use clap::Args;

use crate::utils;

use super::NueCommand;

#[derive(Args, Debug)]
pub struct CommandArguments;

impl NueCommand for CommandArguments {
    async fn run(&self) -> anyhow::Result<()> {
        let nue_dir = dirs::home_dir()
            .expect("failed to get home directory")
            .join(".nue");
        if !nue_dir.try_exists()? {
            println!("Node is not installed.");
            return Ok(());
        }
        tokio::fs::remove_dir_all(nue_dir).await?;
        println!("Node uninstalled successfully.");

        if utils::check::path_contains(".nue/bin")? {
            println!("Node is still on your PATH. Please remove the environment file from your shell configuration.");
        }

        Ok(())
    }
}
