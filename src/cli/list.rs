use clap::Args;
use reqwest::blocking as reqwest;

use super::NueCommand;

use crate::{exts::HyperlinkExt, types};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum VersionInputs {
    VersionString(String),
    #[default]
    All,
    Lts(Option<String>),
}

#[derive(Args, Debug)]
pub struct CommandArguments {
    /// List all available versions of a specific one.
    #[arg(default_value_t = VersionInputs::default())]
    version: VersionInputs,

    /// Show the latest version.
    #[arg(long)]
    latest: bool,

    /// List all versions no matter if the current system is supported or not.
    #[arg(long)]
    list_unsupported: bool,
}

impl NueCommand for CommandArguments {
    type Arguments = Self;

    fn run(&self) -> anyhow::Result<()> {
        let releases_json: Vec<types::node::Release> =
            reqwest::get("https://nodejs.org/download/release/index.json")?.json()?;

        let mut releases = match &self.version {
            VersionInputs::VersionString(version) => releases_json
                .into_iter()
                .filter(|release| format!("{}", release.version).starts_with(version))
                .collect(),
            VersionInputs::Lts(Some(code_name)) => releases_json
                .into_iter()
                .filter(|release| {
                    matches!(&release.lts, types::node::LTS::CodeName(name) if *name.to_lowercase() == *code_name)
                })
                .collect(),
            VersionInputs::Lts(None) => releases_json
                .into_iter()
                .filter(|release| matches!(release.lts, types::node::LTS::CodeName(_)))
                .collect(),
            VersionInputs::All => releases_json,
        };

        if !self.list_unsupported {
            let current_platform = types::platforms::Platform::get_system_platform().to_string();
            releases.retain(|release| release.files.contains(&current_platform));
        }

        if self.latest {
            let latest_version = &releases
                .first()
                .expect("release not found, somehow.")
                .version;

            println!("v{latest_version}");
        } else if releases.is_empty() {
            anyhow::bail!("No release found with given version or LTS code name.");
        } else {
            println!("{}", print_version_tree(&releases));
        }

        Ok(())
    }
}

fn print_version_tree(releases: &[types::node::Release]) -> String {
    let mut tree_string = String::new();

    let mut current_major = None;
    let mut current_minor = None;
    let mut current_patch_written = false;

    for release in releases {
        if current_major != Some(release.version.major) {
            current_major = Some(release.version.major);
            current_minor = None;
            current_patch_written = false;

            tree_string.push_str(&format!(
                "v{}{}\n",
                release.version.major,
                match &release.lts {
                    types::node::LTS::CodeName(code_name) => format!(" ({} LTS)", code_name),
                    types::node::LTS::Bool(_false) => "".to_string(),
                }
            ));
        }

        if current_minor != Some(release.version.minor) || !current_patch_written {
            current_minor = Some(release.version.minor);
            current_patch_written = true;

            tree_string.push_str(&format!(
                "  - v{}.{}\n",
                release.version.major, release.version.minor
            ));
        }

        tree_string.push_str(&format!(
            "    - {}{}\n",
            format!("v{}", release.version).hyperlink(format!(
                "https://github.com/nodejs/node/releases/tag/v{}",
                release.version
            )),
            if release.is_supported_by_current_platform() {
                ""
            } else {
                " (unsupported)"
            }
        ));
    }

    tree_string
}

impl ::std::fmt::Display for VersionInputs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::VersionString(version) => write!(f, "{}", version),
            Self::All => write!(f, "all"),
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
            "all" => Ok(Self::All),
            "lts" => Ok(Self::Lts(None)),
            _ if s.starts_with('v') && s[1..].parse::<node_semver::Range>().is_ok() => {
                Ok(Self::VersionString(s[1..].to_string()))
            }
            _ if (s.parse::<node_semver::Range>().is_ok()) => Ok(Self::VersionString(s)),
            _ => Ok(Self::Lts(Some(s))),
        }
    }
}
