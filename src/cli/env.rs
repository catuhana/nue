use std::fs;

use clap::Args;

use crate::globals::NUE_PATH;

use super::NueCommand;

#[derive(Args, Debug)]
pub struct CommandArguments;

impl NueCommand for CommandArguments {
    fn run(&self) -> anyhow::Result<()> {
        if !NUE_PATH.try_exists()? {
            fs::create_dir_all(&*NUE_PATH)?;
        } else if !NUE_PATH.join("node").try_exists()? {
            println!("Node is not installed yet and won't be available until its installed. Use `nue install` to install.");
        }

        #[cfg(unix)]
        {
            let environment_script = include_str!("../../resources/env.sh");
            fs::write(NUE_PATH.join("env"), environment_script)?;

            println!(
                "Created env script at `$HOME/.nue/env`. Source it in your shell profile ({}) to use nue.",
                available_shell_profiles().join(", ")
            );
        }

        #[cfg(windows)]
        {
            let environment_script = include_str!("../../resources/env.ps1");
            fs::write(NUE_PATH.join("env.ps1"), environment_script)?;

            println!(
                "Created env script at `%LocalAppData%\\nue\\env.ps1`. Run this script once to add Node binaries to your user path.",
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
