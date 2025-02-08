use std::{fmt, str};

use clap::Args;
use demand::{DemandOption, Select, Spinner};

use crate::types;

use super::{install, NueCommand};

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

    /// Force install.
    #[arg(long)]
    force: bool,
}

impl NueCommand for CommandArguments {
    fn run(&self) -> anyhow::Result<()> {
        let mut releases: Vec<types::node::Release> = Vec::new();
        Spinner::new("Fetching releases...").run(|spinner| -> anyhow::Result<()> {
            let all_releases = types::node::Release::get_all_releases()?;

            spinner.title("Filtering releases...")?;
            releases = all_releases.into_iter().filter(|release| {
                    if !release.is_supported_by_current_platform() {
                        return false;
                    }

                    match &self.version {
                        VersionInputs::VersionString(version) => format!("{}", release.version).starts_with(version),
                        VersionInputs::Lts(Some(code_name)) => matches!(&release.lts, types::node::Lts::CodeName(name) if *name.to_lowercase() == *code_name),
                        VersionInputs::Lts(None) => release.lts.is_code_name(),
                        VersionInputs::All => true,
                    }
                }).collect();

            Ok(())
        })??;

        if releases.is_empty() {
            anyhow::bail!("No release found with given version or LTS code name.");
        } else if let Ok(selected_version) = Select::new("Select Node Version")
            .filterable(true)
            .options(
                releases
                    .iter()
                    .map(|release| {
                        if release.lts.is_code_name() {
                            DemandOption::new(&release.version).label(
                                format!("v{} ({} LTS)", release.version, release.lts).as_str(),
                            )
                        } else {
                            DemandOption::new(&release.version)
                        }
                    })
                    .collect(),
            )
            .run()
        {
            let release = releases
                .iter()
                .find(|release| release.version == *selected_version)
                .unwrap();

            install::CommandArguments {
                version: install::VersionInputs::VersionString(release.version.to_string()),
                force: self.force,
            }
            .run()?;
        }

        Ok(())
    }
}

impl fmt::Display for VersionInputs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VersionString(version) => write!(f, "{version}"),
            Self::All => write!(f, "all"),
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
