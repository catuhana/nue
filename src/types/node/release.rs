use std::{fs, io::Read as _, os, path, process, time};

use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Deserializer};

use crate::{
    constants::{NODE_DISTRIBUTIONS_INDEX_URL, NODE_DISTRIBUTIONS_URL, NODE_GITHUB_URL},
    exts::HyperlinkExt as _,
    globals::NUE_PATH,
    types,
};

use super::Lts;

#[derive(Deserialize, Clone, Debug)]
pub struct Release {
    #[serde(deserialize_with = "deserialise_version_v_prefix")]
    pub version: node_semver::Version,
    pub files: Vec<String>,
    pub lts: Lts,
}

impl Release {
    pub fn install(&self) -> anyhow::Result<()> {
        if !self.is_supported_by_current_platform() {
            anyhow::bail!("This release is not supported by the current platform.");
        }

        let response = ureq::get(&self.get_download_url()).call()?;
        if !response.status() == 200 {
            anyhow::bail!("Failed to download release: {}", response.status());
        }

        let download_progress_bar = ProgressBar::new(response.header("Content-Length").unwrap().parse::<u64>()?)
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
        let mut reader = response.into_reader();
        while let Ok(read_bytes) = reader.read(&mut buffer) {
            if read_bytes == 0 {
                break;
            }

            file_chunks.extend_from_slice(&buffer[..read_bytes]);
            download_progress_bar.inc(read_bytes as u64);
        }
        download_progress_bar.finish_and_clear();

        let progress_bar = ProgressBar::new_spinner();
        progress_bar.enable_steady_tick(time::Duration::from_millis(120));

        progress_bar.set_message("Unpacking archive...");
        extract_node_archive(file_chunks.as_slice())?;

        if NUE_PATH.join("node").exists() {
            fs::remove_dir_all(NUE_PATH.join("node"))?;
        }

        #[cfg(unix)]
        os::unix::fs::symlink(
            NUE_PATH.join(self.get_archive_string()),
            NUE_PATH.join("node"),
        )?;
        #[cfg(windows)]
        if let Err(error) = os::windows::fs::symlink_dir(
            NUE_PATH.join(self.get_archive_string()),
            NUE_PATH.join("node"),
        ) {
            if error.raw_os_error() == Some(1314) {
                anyhow::bail!(
                    "Developer mode must be enabled to install nue. More information: https://learn.microsoft.com/en-us/windows/apps/get-started/enable-your-device-for-development"
                );
            }

            anyhow::bail!(error);
        }

        progress_bar.finish_and_clear();

        Ok(())
    }

    pub fn install_from_cache(&self, cached_downloads: Vec<path::PathBuf>) -> anyhow::Result<()> {
        let progress_bar = ProgressBar::new_spinner();
        progress_bar.enable_steady_tick(time::Duration::from_millis(120));

        progress_bar.set_message("Looking for caches to install from...");
        for cache in cached_downloads {
            if cache.try_exists()? && cache.ends_with(self.get_archive_string()) {
                progress_bar.set_message("Unpacking from cache...");

                if NUE_PATH.join("node").exists() {
                    fs::remove_dir_all(NUE_PATH.join("node"))?;
                }

                #[cfg(unix)]
                os::unix::fs::symlink(cache, NUE_PATH.join("node"))?;

                #[cfg(windows)]
                {
                    os::windows::fs::symlink_dir(cache, NUE_PATH.join("node"))?;
                }

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

        #[cfg(unix)]
        let version = process::Command::new(nue_node_path.join("bin").join("node"))
            .arg("--version")
            .output()?
            .stdout;
        #[cfg(windows)]
        let version = process::Command::new(nue_node_path.join("node.exe"))
            .arg("--version")
            .output()?
            .stdout;

        if !String::from_utf8_lossy(&version).contains(&self.version.to_string()) {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn get_all_releases() -> anyhow::Result<Vec<Self>> {
        let response = ureq::get(NODE_DISTRIBUTIONS_INDEX_URL).call()?;
        if !response.status() == 200 {
            anyhow::bail!("Failed to fetch releases: {}", response.status());
        }

        let releases: Vec<Self> = response.into_json()?;
        Ok(releases)
    }

    pub fn get_download_url(&self) -> String {
        format!(
            "{}/v{}/{}.{}",
            NODE_DISTRIBUTIONS_URL,
            self.version,
            self.get_archive_string(),
            types::platforms::Platform::current()
                .expect("unsupported platform")
                .archive_extension()
        )
    }

    pub fn get_github_release_url(&self) -> String {
        format!("{}/releases/tag/v{}", NODE_GITHUB_URL, self.version)
    }

    pub fn get_archive_string(&self) -> String {
        let platform = types::platforms::Platform::current().expect("unsupported platform");

        format!("node-v{}-{}", self.version, platform.platform_string())
    }

    pub fn is_supported_by_current_platform(&self) -> bool {
        self.files.contains(
            &types::platforms::Platform::current()
                .expect("unsupported platform")
                .download_index_platform_string(),
        )
    }
}

fn deserialise_version_v_prefix<'de, D>(deserializer: D) -> Result<node_semver::Version, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.trim_start_matches('v')
        .parse()
        .map_err(serde::de::Error::custom)
}

fn extract_node_archive(file_chunks: &[u8]) -> Result<(), anyhow::Error> {
    #[cfg(unix)]
    {
        use binstall_tar::Archive;
        use liblzma::decode_all;

        let decoded = decode_all(file_chunks)?;
        Archive::new(decoded.as_slice()).unpack(&*NUE_PATH)?;
    }

    #[cfg(windows)]
    {
        use std::io;
        use zip::ZipArchive;

        ZipArchive::new(io::Cursor::new(file_chunks))?.extract(&*NUE_PATH)?;
    }

    Ok(())
}
