use std::fs;

use clap::Args;
use demand::Spinner;

use crate::{globals::NUE_PATH, utils::cache};

use super::NueCommand;

#[derive(Args, Debug)]
pub struct CommandArguments;

impl NueCommand for CommandArguments {
    fn run(&self) -> anyhow::Result<()> {
        let used_node_install = NUE_PATH.join("node").read_link().ok();

        let mut cached_downloads = cache::find_cached_node_downloads()?;
        if let Some(ref used) = used_node_install {
            cached_downloads.retain(|download| used != download);
        }

        if cached_downloads.is_empty() {
            println!("Nothing to clean.");
            return Ok(());
        }

        Spinner::new("Cleaning up...").run(|_| -> anyhow::Result<()> {
            for download in cached_downloads {
                fs::remove_dir_all(download)?;
            }

            Ok(())
        })??;

        println!("Cleaned up unused Node downloads.");

        Ok(())
    }
}
