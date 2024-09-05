use std::{env, path::Path};

use clap::Args;
use tokio::fs;

use crate::globals::NUE_PATH;

use super::NueCommand;

#[derive(Args, Debug)]
pub struct CommandArguments;

impl NueCommand for CommandArguments {
    async fn run(&self) -> anyhow::Result<()> {
        let environment_script = include_str!("../../resources/env.sh");

        if !NUE_PATH.try_exists()? {
            fs::create_dir_all(&*NUE_PATH).await?;
        }

        if !NUE_PATH.join("env").exists() {
            fs::write(NUE_PATH.join("env"), environment_script).await?;
        }

        println!(
            "Created env script at `$HOME/.nue/env`. Source it in your shell profile ({}) to use nue.",
            available_shell_profiles().join(", ")
        );

        Ok(())
    }
}

fn available_shell_profiles() -> Vec<&'static str> {
    let shell = env::var("SHELL").ok().and_then(|shell_path| {
        Path::new(&shell_path)
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_string())
    });

    let mut profiles = vec!["~/.profile"];
    let additional_profiles = match shell.as_deref() {
        Some("zsh") => vec!["~/.zprofile", "~/.zshenv", "~/.zshrc"],
        Some("bash") => vec!["~/.bash_profile", "~/.bashrc"],
        _ => vec![],
    };

    profiles.extend(additional_profiles);
    profiles
}
