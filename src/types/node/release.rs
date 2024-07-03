use serde::{de::Error as DeError, Deserialize, Deserializer};

use crate::types;

use super::LTS;

#[derive(Deserialize, Clone, Debug)]
pub struct NodeRelease {
    #[serde(deserialize_with = "deserialise_version_v_prefix")]
    pub version: node_semver::Version,
    pub files: Vec<String>,
    pub lts: LTS,
}

impl NodeRelease {
    pub fn is_supported_by_current_platform(&self) -> bool {
        self.files.iter().any(|file| {
            file.contains(&types::platforms::Platform::get_system_platform().to_string())
        })
    }
}

fn deserialise_version_v_prefix<'de, D>(deserializer: D) -> Result<node_semver::Version, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.trim_start_matches('v').parse().map_err(DeError::custom)
}
