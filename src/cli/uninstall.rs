use clap::Args;

use crate::{globals::NUE_PATH, utils};

use super::NueCommand;

#[derive(Args, Debug)]
pub struct CommandArguments;

impl NueCommand for CommandArguments {
    async fn run(&self) -> anyhow::Result<()> {
        if !NUE_PATH.try_exists()? {
            println!("Node is not installed.");
            return Ok(());
        }
        tokio::fs::remove_dir_all(&*NUE_PATH).await?;
        println!("Node uninstalled successfully.");

        if utils::check::path_contains(".nue/bin")? {
            println!("Node is still on your PATH. Please remove the environment file from your shell configuration.");
        }

        Ok(())
    }
}
