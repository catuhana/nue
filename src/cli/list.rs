use clap::Args;

use crate::types;

use super::NueCommand;

#[derive(Args, Debug)]
pub struct CommandArguments {
    /// List all available versions of a specific one.
    version: Option<types::node::Version>,

    /// Show all releases, no matter if current machine supports it or not.
    #[arg(long)]
    all: bool,

    /// List LTS only releases
    #[arg(long)]
    lts: bool,
}

impl NueCommand for CommandArguments {
    type Arguments = Self;

    fn run(&self) -> anyhow::Result<()> {
        let response: Vec<types::node::Release> =
            ureq::get("https://nodejs.org/download/release/index.json")
                .call()?
                .into_json()?;

        let releases = match &self.version {
            Some(version) => match version {
                types::node::Version::Semver(version) => response
                    .into_iter()
                    .filter(|release| format!("{}", release.version).starts_with(version))
                    .collect::<Vec<_>>(),
                types::node::Version::Latest => response
                    .iter()
                    .max_by_key(|release| &release.version)
                    .map_or_else(Vec::new, |max| vec![max.clone()]),
                types::node::Version::Lts => {
                    anyhow::bail!("Use the `--lts` flag to list LTS releases")
                }
            },
            None => {
                if self.lts {
                    response
                        .into_iter()
                        .filter(|release| matches!(release.lts, types::node::LTS::CodeName(_)))
                        .collect()
                } else if self.all {
                    response
                } else {
                    let current_platform =
                        types::platforms::Platform::get_system_platform().to_string();
                    response
                        .into_iter()
                        .filter(|release| release.files.contains(&current_platform))
                        .collect()
                }
            }
        };

        if releases.is_empty() {
            anyhow::bail!("Version not found");
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
                    types::node::LTS::CodeName(code_name) => format!(" (LTS, {})", code_name),
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

        if release.version.patch > 0 {
            tree_string.push_str(&format!(
                "    - v{}.{}.{}\n",
                release.version.major, release.version.minor, release.version.patch
            ));
        }
    }

    tree_string
}
