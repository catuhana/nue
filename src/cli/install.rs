use std::{fmt, str};

use clap::Args;

use super::NueCommand;
use crate::types;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum VersionInputs {
    VersionString(String),
    #[default]
    Latest,
    Lts(Option<String>),
}

#[derive(Args, Debug)]
pub struct CommandArguments {
    /// Optional version of Node to install.
    #[arg(default_value_t = VersionInputs::default())]
    version: VersionInputs,
}

impl NueCommand for CommandArguments {
    type Arguments = Self;

    fn run(&self) -> anyhow::Result<()> {
        let releases_json: Vec<types::node::Release> =
            ureq::get("https://nodejs.org/download/release/index.json")
                .call()?
                .into_json()?;

        let release_branch: &str;
        let latest_release = match &self.version {
            VersionInputs::VersionString(version) => {
                release_branch = version;

                releases_json
                    .iter()
                    .find(|release| format!("{}", release.version).starts_with(version))
            }
            VersionInputs::Lts(Some(code_name)) => {
                release_branch = code_name;

                releases_json.iter().find(|release| {
                    matches!(
                        &release.lts,
                        types::node::LTS::CodeName(name) if *name.to_lowercase() == *code_name
                    )
                })
            }
            VersionInputs::Lts(None) => {
                release_branch = "lts";

                releases_json
                    .iter()
                    .find(|release| matches!(release.lts, types::node::LTS::CodeName(_)))
            }
            VersionInputs::Latest => {
                release_branch = "latest";

                releases_json.iter().max_by_key(|release| &release.version)
            }
        };

        match latest_release {
            Some(release) => {
                println!(
                    "Installing version {} from {} branch",
                    release.version,
                    if release_branch == "latest" {
                        "current"
                    } else if release_branch == "lts" {
                        "LTS"
                    } else {
                        release_branch
                    }
                )
            }
            None => {}
        }

        Ok(())
    }
}

impl fmt::Display for VersionInputs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VersionString(version) => write!(f, "{}", version),
            Self::Latest => write!(f, "latest"),
            Self::Lts(Some(code_name)) => write!(f, "{}", code_name),
            Self::Lts(None) => write!(f, "lts"),
        }
    }
}

impl std::str::FromStr for VersionInputs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.as_str() {
            "latest" => Ok(Self::Latest),
            "lts" => Ok(Self::Lts(None)),
            _ if s.starts_with('v') && s[1..].parse::<node_semver::Range>().is_ok() => {
                Ok(Self::VersionString(s[1..].to_string()))
            }
            _ if (s.parse::<node_semver::Range>().is_ok()) => Ok(Self::VersionString(s)),
            _ => Ok(Self::Lts(Some(s))),
        }
    }
}
