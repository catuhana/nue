use clap::Args;
use reqwest::blocking as reqwest;

use super::NueCommand;

use crate::{exts::HyperlinkExt, types};

#[derive(Debug, Default, Clone)]
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
        let progress_bar = indicatif::ProgressBar::new_spinner();
        progress_bar.enable_steady_tick(::std::time::Duration::from_millis(120));

        progress_bar.set_message("Fetching releases...");
        let releases_json: Vec<types::node::Release> =
            reqwest::get("https://nodejs.org/download/release/index.json")?.json()?;

        progress_bar.set_message("Filtering releases based on input...");
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
                release_branch = "LTS";

                releases_json
                    .iter()
                    .find(|release| release.lts.is_code_name())
            }
            VersionInputs::Latest => {
                release_branch = "latest";

                releases_json.iter().max_by_key(|release| &release.version)
            }
        };

        progress_bar.finish_and_clear();

        match latest_release {
            Some(release) => {
                let version_str = format!("v{}", release.version);
                let branch_name = match release_branch {
                    "latest" => "current",
                    "LTS" => release_branch,
                    _ => release_branch,
                };

                println!(
                    "Installing version {} from `{}` branch",
                    version_str.hyperlink(format!(
                        "https://github.com/nodejs/node/releases/tag/{version_str}"
                    )),
                    branch_name
                )
            }
            None => {
                anyhow::bail!("No release found with given version or LTS code name.");
            }
        }

        Ok(())
    }
}

impl ::std::fmt::Display for VersionInputs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::VersionString(version) => write!(f, "{}", version),
            Self::Latest => write!(f, "latest"),
            Self::Lts(Some(code_name)) => write!(f, "{}", code_name),
            Self::Lts(None) => write!(f, "lts"),
        }
    }
}

impl ::std::str::FromStr for VersionInputs {
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
