use std::{fmt, str, time};

use clap::Args;
use indicatif::ProgressBar;

use crate::{
    types,
    utils::{self, cache},
};

use super::NueCommand;

#[derive(Debug, Default, Clone)]
pub enum VersionInputs {
    VersionString(String),
    #[default]
    Latest,
    Lts(Option<String>),
}

#[derive(Args, Debug)]
pub struct CommandArguments {
    /// Optional version of Node to install.
    #[arg(default_value_t = VersionInputs::default())]
    pub version: VersionInputs,

    /// Force re-installation of the entered version.
    #[arg(long)]
    pub force: bool,
}

impl NueCommand for CommandArguments {
    fn run(&self) -> anyhow::Result<()> {
        let progress_bar = ProgressBar::new_spinner();
        progress_bar.enable_steady_tick(time::Duration::from_millis(120));

        progress_bar.set_message("Fetching releases...");
        let releases = types::node::Release::get_all_releases()?;

        progress_bar.set_message("Filtering releases...");
        let selected_release = match &self.version {
            VersionInputs::VersionString(version) => releases
                .iter()
                .find(|release| format!("{}", release.version).starts_with(version)),
            VersionInputs::Lts(Some(code_name)) => releases.iter().find(|release| {
                matches!(
                    &release.lts,
                    types::node::Lts::CodeName(name) if &name.to_lowercase() == code_name
                )
            }),
            VersionInputs::Lts(None) => releases.iter().find(|release| release.lts.is_code_name()),
            VersionInputs::Latest => releases.iter().max_by_key(|release| &release.version),
        };
        progress_bar.finish_and_clear();

        match selected_release {
            Some(release) => {
                if release.check_installed()? && !self.force {
                    println!(
                        "Node v{} is already installed. Use `--force` to re-install.",
                        release.version
                    );
                    return Ok(());
                }

                let cached_downloads = cache::find_cached_node_downloads()?;
                if cached_downloads.is_empty() {
                    release.install()?;
                } else {
                    release.install_from_cache(cached_downloads)?;
                }

                println!("Node v{} is now installed!", release.version);

                if !utils::check::path_contains(".nue/node/bin")? {
                    println!(
                        "Node is installed, but its path isn't in `PATH`. Run `nue env` to fix it."
                    );
                }
            }
            None => {
                anyhow::bail!("No release found with given version or LTS code name.");
            }
        }

        Ok(())
    }
}

impl fmt::Display for VersionInputs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VersionString(version) => write!(f, "{version}"),
            Self::Latest => write!(f, "latest"),
            Self::Lts(Some(code_name)) => write!(f, "{code_name}"),
            Self::Lts(None) => write!(f, "lts"),
        }
    }
}

impl str::FromStr for VersionInputs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.as_str() {
            "latest" => Ok(Self::Latest),
            "lts" => Ok(Self::Lts(None)),
            _ if s.starts_with('v') && s[1..].parse::<node_semver::Range>().is_ok() => {
                Ok(Self::VersionString(s[1..].to_string()))
            }
            _ if s.parse::<node_semver::Range>().is_ok() => Ok(Self::VersionString(s)),
            _ => Ok(Self::Lts(Some(s))),
        }
    }
}
