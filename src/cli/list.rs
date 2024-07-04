use anyhow::Context;
use clap::Args;
use inquire::Select;

use super::NueCommand;

use crate::{exts::HyperlinkExt, types};

#[derive(Debug, Default, Clone)]
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
}

impl NueCommand for CommandArguments {
    type Arguments = Self;

    async fn run(&self) -> anyhow::Result<()> {
        let progress_bar = indicatif::ProgressBar::new_spinner();
        progress_bar.enable_steady_tick(std::time::Duration::from_millis(120));

        progress_bar.set_message("Fetching releases...");
        let response = reqwest::get("https://nodejs.org/download/release/index.json")
            .await
            .context(
                "Failed to fetch releases from `https://nodejs.org/download/release/index.json`",
            )?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch releases: {}", response.status());
        }

        progress_bar.set_message("Parsing releases...");
        let releases_json: Vec<types::node::Release> = response
            .json()
            .await
            .context("Failed to parse releases JSON")?;

        progress_bar.set_message("Filtering releases...");
        let releases: Vec<_> = releases_json.into_iter().filter(|release| {
            if !release.is_supported_by_current_platform() {
                return false
            }

            match &self.version {
                VersionInputs::VersionString(version) => format!("{}", release.version).starts_with(version),
                VersionInputs::Lts(Some(code_name)) => matches!(&release.lts, types::node::LTS::CodeName(name) if *name.to_lowercase() == *code_name),
                VersionInputs::Lts(None) => release.lts.is_code_name(),
                VersionInputs::All => true,
            }
        }).collect();
        progress_bar.finish_and_clear();

        if self.latest {
            let latest_version = &releases
                .first()
                .expect("release not found, somehow.")
                .version;

            println!("v{latest_version}");
        } else if releases.is_empty() {
            anyhow::bail!("No release found with given version or LTS code name.");
        } else if let Ok(Some(selected_version)) = Select::new(
            "Select Node Version",
            releases
                .iter()
                .map(|release| {
                    if release.lts.is_code_name() {
                        format!("v{} ({} LTS)", release.version, release.lts)
                    } else {
                        format!("v{}", release.version)
                    }
                })
                .collect(),
        )
        .with_page_size(16)
        .prompt_skippable()
        {
            let selected_version = if selected_version.contains(' ') {
                selected_version
                    .split_whitespace()
                    .nth(0)
                    .expect("version not found, somehow.")
            } else {
                &selected_version
            };

            println!(
                "Run `nue install {}` to install this version.",
                selected_version.hyperlink(format!(
                    "https://github.com/nodejs/node/releases/tag/{selected_version}"
                ))
            );
        }

        Ok(())
    }
}

impl std::fmt::Display for VersionInputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VersionString(version) => write!(f, "{version}"),
            Self::All => write!(f, "all"),
            Self::Lts(Some(code_name)) => write!(f, "{code_name}"),
            Self::Lts(None) => write!(f, "lts"),
        }
    }
}

impl std::str::FromStr for VersionInputs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.as_str() {
            "all" => Ok(Self::All),
            "lts" => Ok(Self::Lts(None)),
            _ if s.starts_with('v') && s[1..].parse::<node_semver::Range>().is_ok() => {
                Ok(Self::VersionString(s[1..].to_string()))
            }
            _ if s.parse::<node_semver::Range>().is_ok() => Ok(Self::VersionString(s)),
            _ => Ok(Self::Lts(Some(s))),
        }
    }
}
