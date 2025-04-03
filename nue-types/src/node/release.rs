use nue_common::constants::{NODE_DISTRIBUTIONS_URL, NODE_GITHUB_URL};

use crate::platforms::Platform;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Release {
    pub version: node_semver::Version,
    pub files: Vec<String>,
    pub lts: super::lts::Lts,
}

impl Release {
    pub fn get_download_url(&self, platform: &Platform) -> String {
        format!(
            "{}/v{}/{}.{}",
            NODE_DISTRIBUTIONS_URL,
            self.version,
            self.get_archive_string(platform),
            platform.node_archive_extension()
        )
    }

    pub fn get_archive_string(&self, platform: &Platform) -> String {
        format!("node-v{}-{}", self.version, platform.node_platform_string())
    }

    pub fn get_github_release_url(&self) -> String {
        format!("{}/releases/tag/v{}", NODE_GITHUB_URL, self.version)
    }

    pub fn is_supported_by_current_platform(&self, platform: &Platform) -> anyhow::Result<bool> {
        let platform = platform.node_index_platform_string();

        if self.files.contains(&platform) {
            Ok(true)
        } else {
            anyhow::bail!(
                "Node {} is not supported on this platform.\nSupported platforms: '{}', current platform: {platform}",
                self.version,
                self.files.join(", "),
            );
        }
    }
}
