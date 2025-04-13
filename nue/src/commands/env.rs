use nue_common::globals::{ARGV_0, NUE_NODE_PATH, NUE_PATH};
use nue_resources::create_env_script;

use crate::Command;

#[derive(Debug, clap::Args)]
pub struct Arguments;

impl Command for Arguments {
    fn run(&self) -> anyhow::Result<()> {
        if !NUE_NODE_PATH.try_exists()? {
            println!(
                "Node is not installed yet and won't be available until its installed. Use `{} install` to install.",
                &*ARGV_0
            );
        }

        #[cfg(windows)]
        {
            let path = NUE_PATH.join("env.ps1");
            create_env_script(&path)?;

            println!(
                "Environment script created at `{}`. Run this script once to add Node binaries to your user path.",
                path.display()
            );
        }

        #[cfg(unix)]
        {
            let path = NUE_PATH.join("env.sh");
            create_env_script(&path)?;

            println!(
                "Environment script created at `{}`. Source it in your shell profile ({}) to use nue.",
                path.display(),
                available_shell_profiles().join(", ")
            );
        }

        Ok(())
    }
}

#[cfg(unix)]
fn available_shell_profiles() -> Vec<&'static str> {
    use std::{env, path};

    let shell = env::var("SHELL").ok().and_then(|shell_path| {
        path::Path::new(&shell_path)
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .map(std::string::ToString::to_string)
    });

    let mut profiles = vec![];
    let additional_profiles = match shell.as_deref() {
        Some("zsh") => vec!["~/.zprofile", "~/.zshenv", "~/.zshrc"],
        Some("bash") => vec!["~/.bash_profile", "~/.bashrc"],
        _ => vec!["~/.profile"],
    };

    profiles.extend(additional_profiles);
    profiles
}
