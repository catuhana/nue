use core::time::Duration;

use std::{fs, io::Read as _, os, path, process, time::Instant};

use demand::Spinner;
use serde::{Deserialize, Deserializer};
use ureq::http::StatusCode;

use crate::{
    constants::{NODE_DISTRIBUTIONS_INDEX_URL, NODE_DISTRIBUTIONS_URL, NODE_GITHUB_URL},
    exts::HyperlinkExt as _,
    globals::{NUE_PATH, NUE_RELEASES_PATH},
    types,
};

use super::Lts;

const SPINNER_DOWNLOADING_MESSAGE: fn(&Release, Option<&str>) -> String = |release, progress| {
    format!(
        "Downloading version {}{}",
        release.version.hyperlink(release.get_github_release_url()),
        progress.map_or_else(String::default, |progress| { format!(" ({progress})") })
    )
};
const BUFFER_SIZE: usize = 1024 * 1024;
const BYTES_PER_MB: f64 = 1_048_576.0;
const SPINNER_UPDATE_INTERVAL: Duration = Duration::from_millis(100);

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

        let mut response = ureq::get(&self.get_download_url()).call()?;
        if response.status() != StatusCode::OK {
            anyhow::bail!("Failed to download release: {}", response.status());
        }

        Spinner::new(SPINNER_DOWNLOADING_MESSAGE(self, None)).run(
            |spinner| -> anyhow::Result<()> {
                let content_length = response.headers().get("Content-Length").unwrap().to_str()?.parse::<usize>()?;

                let mut file_chunks = Vec::with_capacity(content_length);
                let mut buffer = vec![0; BUFFER_SIZE];
                let mut reader = response.body_mut().as_reader();

                let mut last_update = Instant::now();

                while let Ok(read_bytes) = reader.read(&mut buffer) {
                    if read_bytes == 0 {
                        break;
                    }

                    file_chunks.extend_from_slice(&buffer[..read_bytes]);

                    if last_update.elapsed() >= SPINNER_UPDATE_INTERVAL {
                      spinner.title(SPINNER_DOWNLOADING_MESSAGE(
                          self,
                          Some(&format!("{:.2}/{:.2}MiB",
                              file_chunks.len() as f64 / BYTES_PER_MB,
                              content_length as f64 / BYTES_PER_MB
                          )),
                      ))?;

                      last_update = Instant::now();
                    }
                }

                spinner.title("Unpacking archive...")?;
                extract_node_archive(file_chunks.as_slice())?;

                spinner.title("Linking node folder...")?;
                #[cfg(unix)]
                os::unix::fs::symlink(
                    NUE_RELEASES_PATH.join(self.get_archive_string()),
                    NUE_PATH.join("node"),
                )?;

                #[cfg(windows)]
                if let Err(error) = os::windows::fs::symlink_dir(
                    NUE_RELEASES_PATH.join(self.get_archive_string()),
                    NUE_PATH.join("node"),
                ) {
                    if error.raw_os_error() == Some(1314) {
                        anyhow::bail!(
                            "Developer mode must be enabled to install nue. For more information: https://learn.microsoft.com/en-us/windows/apps/get-started/enable-your-device-for-development"
                        );
                    }

                    anyhow::bail!(error);
                }

                Ok(())
            },
        )??;

        Ok(())
    }

    pub fn install_from_cache(&self, cached_downloads: &[path::PathBuf]) -> anyhow::Result<()> {
        Spinner::new("Looking for a cached release...").run(|spinner| -> anyhow::Result<()> {
            for cache in cached_downloads {
                if cache.try_exists()? && cache.ends_with(self.get_archive_string()) {
                    spinner.title("Linking cached version...")?;

                    if NUE_PATH.join("node").exists() {
                        fs::remove_dir_all(NUE_PATH.join("node"))?;
                    }

                    #[cfg(unix)]
                    os::unix::fs::symlink(cache, NUE_PATH.join("node"))?;

                    #[cfg(windows)]
                    os::windows::fs::symlink_dir(cache, NUE_PATH.join("node"))?;

                    return Ok(());
                }
            }

            anyhow::bail!("No cached release found.");
        })?
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
        let mut response = ureq::get(NODE_DISTRIBUTIONS_INDEX_URL).call()?;
        if response.status() != StatusCode::OK {
            anyhow::bail!("Failed to fetch releases: {}", response.status());
        }

        let releases = response.body_mut().read_json()?;
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
                .node_archive_extension()
        )
    }

    pub fn get_github_release_url(&self) -> String {
        format!("{}/releases/tag/v{}", NODE_GITHUB_URL, self.version)
    }

    pub fn get_archive_string(&self) -> String {
        format!(
            "node-v{}-{}",
            self.version,
            types::platforms::Platform::current()
                .expect("unsupported platform")
                .node_platform_string()
        )
    }

    pub fn is_supported_by_current_platform(&self) -> bool {
        self.files.contains(
            &types::platforms::Platform::current()
                .expect("unsupported platform")
                .node_index_platform_string(),
        )
    }
}

fn deserialise_version_v_prefix<'de, D>(deserializer: D) -> Result<node_semver::Version, D::Error>
where
    D: Deserializer<'de>,
{
    let version: String = Deserialize::deserialize(deserializer)?;
    version
        .trim_start_matches('v')
        .parse()
        .map_err(serde::de::Error::custom)
}

fn extract_node_archive(file_chunks: &[u8]) -> Result<(), anyhow::Error> {
    if !NUE_RELEASES_PATH.try_exists()? {
        fs::create_dir_all(&*NUE_RELEASES_PATH)?;
    }

    #[cfg(unix)]
    {
        use binstall_tar::Archive;
        use liblzma::decode_all;

        let decoded = decode_all(file_chunks)?;
        Archive::new(decoded.as_slice()).unpack(&*NUE_RELEASES_PATH)?;
    }

    #[cfg(windows)]
    {
        use sevenz_rust2::decompress;
        use std::io;

        decompress(io::Cursor::new(file_chunks), &*NUE_RELEASES_PATH)?;
    }

    Ok(())
}
