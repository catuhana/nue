use std::{fs, path};

use crate::globals::NUE_RELEASES_PATH;

pub fn find_cached_node_downloads() -> anyhow::Result<Vec<path::PathBuf>> {
    let mut caches = vec![];

    match fs::read_dir(&*NUE_RELEASES_PATH) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if !path.is_dir() {
                    continue;
                }

                if entry.file_name().to_string_lossy().starts_with("node-") {
                    caches.push(path);
                }
            }
        }
        Err(_) => {
            return Ok(caches);
        }
    }

    Ok(caches)
}
