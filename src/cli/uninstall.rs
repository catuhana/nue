use clap::Args;
use tokio::fs;

use crate::{globals::NUE_PATH, utils};

use super::NueCommand;

#[derive(Args, Debug)]
pub struct CommandArguments;

impl NueCommand for CommandArguments {
    async fn run(&self) -> anyhow::Result<()> {
        if !NUE_PATH.join("node").try_exists()? {
            println!("Node is not installed.");
            return Ok(());
        }

        fs::remove_dir_all(&*NUE_PATH).await?;
        println!("Node uninstalled successfully.");

        if utils::check::path_contains(".nue/node/bin")? {
            println!("Node is still in your `PATH`. Remove the sourced env script from your shell profile ({}).", files_in_home_containing("$HOME/.nue/env").await?.join(", "));
        }

        Ok(())
    }
}

async fn files_in_home_containing(substring: &str) -> anyhow::Result<Vec<String>> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("failed to get home directory"))?;
    let mut matching_files = Vec::new();

    let mut dir = fs::read_dir(home_dir).await?;
    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            continue;
        }

        match fs::read_to_string(&path).await {
            Ok(contents) if contents.contains(substring) => {
                matching_files.push(format!(
                    "~/{}",
                    path.file_name().unwrap().to_string_lossy().to_string()
                ));
            }
            _ => {}
        }
    }

    Ok(matching_files)
}
