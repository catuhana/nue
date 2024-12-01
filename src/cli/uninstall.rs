use std::fs;

use clap::Args;

use crate::{globals::NUE_PATH, utils};

use super::NueCommand;

#[derive(Args, Debug)]
pub struct CommandArguments;

impl NueCommand for CommandArguments {
    fn run(&self) -> anyhow::Result<()> {
        let nue_node_path = NUE_PATH.join("node");

        if nue_node_path.try_exists()? {
            fs::remove_dir_all(nue_node_path)?;
            println!("Node uninstalled successfully.");

            return Ok(());
        }

        println!("Node is not installed.");

        if utils::check::is_node_in_path() {
            let platform_specific_help = if cfg!(unix) {
                format!(
                    "Remove the sourced env script from your shell profile ({}).",
                    files_in_home_containing("$HOME/.nue/env")?.join(", ")
                )
            } else {
                "Remove the entry `\\nue\\node` from your user `Path`.".to_string()
            };

            println!("Node is still in your `PATH`. {platform_specific_help}");
        }

        Ok(())
    }
}

fn files_in_home_containing(substring: &str) -> anyhow::Result<Vec<String>> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("failed to get home directory"))?;

    let mut matching_files = Vec::new();
    for entry in fs::read_dir(home_dir)? {
        let Ok(entry) = entry else { continue };

        let path = entry.path();
        if path.is_dir() {
            continue;
        }

        match fs::read_to_string(&path) {
            Ok(contents) if contents.contains(substring) => {
                matching_files.push(format!("~/{}", path.file_name().unwrap().to_string_lossy()));
            }
            _ => {}
        }
    }

    Ok(matching_files)
}
