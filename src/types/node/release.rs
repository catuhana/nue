use serde::{de::Error as DeError, Deserialize, Deserializer};

use super::LTS;

#[derive(Deserialize, Debug)]
pub struct NodeRelease {
    #[serde(deserialize_with = "deserialise_version_v_prefix")]
    pub version: semver::Version,
    pub files: Vec<String>,
    pub lts: LTS,
}

fn deserialise_version_v_prefix<'de, D>(deserializer: D) -> Result<semver::Version, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.trim_start_matches('v').parse().map_err(DeError::custom)
}
