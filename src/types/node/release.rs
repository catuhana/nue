use std::{io::Read, path, process, time};

use dircpy::CopyBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{de::Error as DeError, Deserialize, Deserializer};
use tar::Archive;

use crate::{
    constants::{NODE_DISTRIBUTIONS_INDEX_URL, NODE_DISTRIBUTIONS_URL, NODE_GITHUB_URL},
    exts::HyperlinkExt,
    globals::NUE_PATH,
    types,
};

use super::LTS;

#[derive(Deserialize, Clone, Debug)]
pub struct NodeRelease {
    #[serde(deserialize_with = "deserialise_version_v_prefix")]
    pub version: node_semver::Version,
    pub files: Vec<String>,
    pub lts: LTS,
}

impl NodeRelease {
    pub fn install(&self) -> anyhow::Result<()> {
        if !self.is_supported_by_current_platform() {
            anyhow::bail!("This release is not supported by the current platform.");
        }

        let mut response = reqwest::blocking::get(self.get_download_url())?;
        if !response.status().is_success() {
            anyhow::bail!("Failed to download release: {}", response.status());
        }

        let download_progress_bar = ProgressBar::new(response.content_length().unwrap_or_default())
            .with_style(
                ProgressStyle::default_bar()
                    .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} {bytes_per_sec} ({eta})")?
                    .progress_chars("-Cc·")
            );

        download_progress_bar.set_message(format!(
            "Downloading version {}",
            format!("v{}", self.version).hyperlink(self.get_github_release_url())
        ));
        let mut file_chunks = Vec::new();
        let mut buffer = vec![0; 8192];
        while let Ok(read_bytes) = response.read(&mut buffer) {
            if read_bytes == 0 {
                break;
            }

            file_chunks.extend_from_slice(&buffer[..read_bytes]);
            download_progress_bar.inc(read_bytes as u64);
        }
        download_progress_bar.finish_and_clear();

        let progress_bar = ProgressBar::new_spinner();
        progress_bar.enable_steady_tick(time::Duration::from_millis(120));

        progress_bar.set_message("Decoding archive...");
        let decoded = liblzma::decode_all(file_chunks.as_slice())?;

        let temporary_folder = types::temp::Folder::new()?;

        progress_bar.set_message("Unpacking archive...");
        Archive::new(decoded.as_slice()).unpack(temporary_folder.get_full_path())?;
        CopyBuilder::new(
            temporary_folder
                .get_full_path()
                .join(self.get_archive_string()),
            NUE_PATH.join("node"),
        )
        .overwrite(true)
        .run()?;
        progress_bar.finish_and_clear();

        Ok(())
    }

    pub fn install_from_cache(&self, cached_downloads: Vec<path::PathBuf>) -> anyhow::Result<()> {
        let progress_bar = ProgressBar::new_spinner();
        progress_bar.enable_steady_tick(time::Duration::from_millis(120));

        progress_bar.set_message("Looking for caches to install from...");
        for cache in cached_downloads {
            let cached_node = cache.join(self.get_archive_string());
            if cached_node.try_exists()? {
                progress_bar.set_message("Installing from cache...");
                CopyBuilder::new(cached_node, NUE_PATH.join("node"))
                    .overwrite(true)
                    .run()?;
                progress_bar.finish_and_clear();

                return Ok(());
            }
        }

        anyhow::bail!("No cached release found.");
    }

    pub fn check_installed(&self) -> anyhow::Result<bool> {
        let nue_node_path = NUE_PATH.join("node");
        if !nue_node_path.try_exists()? {
            return Ok(false);
        }

        let version = process::Command::new(nue_node_path.join("bin").join("node"))
            .arg("--version")
            .output()?
            .stdout;

        if !String::from_utf8_lossy(&version).contains(&self.version.to_string()) {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn get_all_releases() -> anyhow::Result<Vec<Self>> {
        let response = reqwest::blocking::get(NODE_DISTRIBUTIONS_INDEX_URL)?;
        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch releases: {}", response.status());
        }

        let releases: Vec<Self> = response.json()?;
        Ok(releases)
    }

    pub fn get_download_url(&self) -> String {
        format!(
            "{}/v{}/{}.tar.xz",
            NODE_DISTRIBUTIONS_URL,
            self.version,
            self.get_archive_string()
        )
    }

    pub fn get_github_release_url(&self) -> String {
        format!("{}/releases/tag/v{}", NODE_GITHUB_URL, self.version)
    }

    pub fn get_archive_string(&self) -> String {
        format!(
            "node-v{}-{}",
            self.version,
            types::platforms::Platform::get_system_platform()
        )
    }

    pub fn is_supported_by_current_platform(&self) -> bool {
        self.files
            .contains(&types::platforms::Platform::get_system_platform().to_string())
    }
}

fn deserialise_version_v_prefix<'de, D>(deserializer: D) -> Result<node_semver::Version, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.trim_start_matches('v').parse().map_err(DeError::custom)
}
