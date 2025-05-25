use indicatif::{ProgressBar, ProgressStyle};
use nue_common::globals::{NUE_CACHE_PATH, NUE_NODE_PATH};

use crate::Command;

#[derive(Debug, clap::Args)]
/// Clean cached Node installs
pub struct Arguments;

impl Command for Arguments {
    fn run(&self) -> anyhow::Result<()> {
        if !NUE_CACHE_PATH.try_exists()? {
            println!("Cache directory does not exist yet. Nothing to clean.");
            return Ok(());
        }

        let progress = ProgressBar::new_spinner();
        progress.enable_steady_tick(std::time::Duration::from_millis(100));
        progress.set_message("Scanning cache directory...");

        let active_node_dir = if NUE_NODE_PATH.try_exists()? {
            std::fs::read_link(&*NUE_NODE_PATH).ok()
        } else {
            None
        };

        let node_dirs: Vec<_> = std::fs::read_dir(&*NUE_CACHE_PATH)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_dir()
                    && path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .map_or(false, |name| name.starts_with("node-v"))
                    && !active_node_dir
                        .as_ref()
                        .map_or(false, |active_dir| path == active_dir)
            })
            .collect();

        if node_dirs.is_empty() {
            progress.finish_with_message("No cached Node extracts found.");
            return Ok(());
        }

        let total = node_dirs.len();
        progress.set_length(total as u64);
        progress.set_style(
            ProgressStyle::default_bar()
                .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
                .progress_chars("█▉▊▋▌▍▎▏  ")
        );

        progress.set_message(format!("Cleaning {total} cached Node extract(s)"));

        let mut failures = Vec::new();
        for (index, dir) in node_dirs.iter().enumerate() {
            let name = dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            progress.set_message(format!("Deleting {name} ({}/{})", index + 1, total));

            if let Err(error) = std::fs::remove_dir_all(dir) {
                failures.push((name.to_string(), error));
            }
            progress.inc(1);
        }

        let cleaned = total - failures.len();
        if failures.is_empty() {
            progress.finish_with_message(format!(
                "Successfully cleaned {cleaned} cached Node install(s)."
            ));
        } else {
            progress.finish_with_message(format!(
                "Cleaned {cleaned} out of {total} cached Node installs."
            ));
            println!("\nFailed to delete:");
            for (name, error) in failures {
                println!("  {name}: {error}");
            }
        }

        Ok(())
    }
}
