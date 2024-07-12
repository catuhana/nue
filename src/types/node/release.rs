use crate::{exts::HyperlinkExt, types};
use async_compression::tokio::bufread::XzDecoder;
use futures::TryStreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{de::Error as DeError, Deserialize, Deserializer};
use tokio::io::BufReader;
use tokio_tar::Archive;
use tokio_util::io::StreamReader;

use super::LTS;

#[derive(Deserialize, Clone, Debug)]
pub struct NodeRelease {
    #[serde(deserialize_with = "deserialise_version_v_prefix")]
    pub version: node_semver::Version,
    pub files: Vec<String>,
    pub lts: LTS,
}

impl NodeRelease {
    pub async fn install(&self) -> anyhow::Result<()> {
        if !self.is_supported_by_current_platform() {
            anyhow::bail!("This release is not supported by the current platform.");
        }

        let response = reqwest::get(self.get_download_url()).await?;
        if !response.status().is_success() {
            anyhow::bail!("Failed to download release: {}", response.status());
        }

        let download_progress_bar = ProgressBar::new(response.content_length().unwrap_or_default())
            .with_style(
                ProgressStyle::default_bar()
                    .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} {bytes_per_sec} ({eta})")?
                    .progress_chars("-Cc·")
            )
            .with_message(
                format!(
                    "Downloading and unpacking version {}",
                    format!("v{}", self.version).hyperlink(self.get_github_release_url())
                )
            );

        let data_stream = response
            .bytes_stream()
            .map_ok(|chunk| {
                download_progress_bar.inc(chunk.len() as u64);
                chunk
            })
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));

        let unpack_temporary_folder = types::temp::Folder::new()?;
        let decompressed = XzDecoder::new(BufReader::new(StreamReader::new(data_stream)));
        Archive::new(decompressed)
            .unpack(unpack_temporary_folder.path())
            .await?;

        let nue_dir = dirs::home_dir()
            .expect("Failed to get home directory")
            .join(".nue");
        dircpy::copy_dir(
            unpack_temporary_folder
                .path()
                .join(self.get_archive_string()),
            nue_dir,
        )?;

        Ok(())
    }

    pub fn get_download_url(&self) -> String {
        format!(
            "{}/v{}/{}.tar.xz",
            types::node::URLs::default().get_distribution_path(),
            self.version,
            self.get_archive_string()
        )
    }

    pub fn get_github_release_url(&self) -> String {
        format!(
            "{}/releases/tag/v{}",
            types::node::URLs::default().github,
            self.version
        )
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
