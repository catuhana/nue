use std::{fs, time};

use clap::Args;
use indicatif::ProgressBar;

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

        let progress_bar = ProgressBar::new_spinner().with_message("Cleaning up...");
        progress_bar.enable_steady_tick(time::Duration::from_millis(120));

        for download in cached_downloads {
            fs::remove_dir_all(download)?;
        }

        progress_bar.finish_with_message("Cleaned up unused Node downloads.");

        Ok(())
    }
}
