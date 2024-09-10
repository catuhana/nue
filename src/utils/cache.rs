use std::{fs, path};

use crate::globals::NUE_PATH;

pub fn find_cached_node_downloads() -> anyhow::Result<Vec<path::PathBuf>> {
    let mut caches = vec![];

    let Ok(entries) = fs::read_dir(&*NUE_PATH) else {
        return Ok(caches);
    };

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        } else if path.file_name().unwrap() == "node" {
            continue;
        }

        if entry.file_name().to_string_lossy().starts_with("node") {
            caches.push(entry.path());
        }
    }

    Ok(caches)
}
