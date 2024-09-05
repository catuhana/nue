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

        let mut possible_shell_profiles: Vec<&str> = vec!["~/.profile"];
        match get_current_shell() {
            Some(shell) => match shell.as_str() {
                "zsh" => {
                    possible_shell_profiles.append(&mut vec![
                        "~/.zprofile",
                        "~/.zshenv",
                        "~/.zshrc",
                    ]);
                }
                "bash" => {
                    possible_shell_profiles.append(&mut vec!["~/.bash_profile", "~/.bashrc"]);
                }
                _ => {}
            },
            None => {}
        }

        println!(
            "Created env script at `$HOME/.nue/env`. Source it in your shell profile ({}) to use nue.",
            possible_shell_profiles.join(", ")
        );

        Ok(())
    }
}

fn get_current_shell() -> Option<String> {
    env::var("SHELL").ok().and_then(|shell_path| {
        Path::new(&shell_path)
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_string())
    })
}
