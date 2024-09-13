use std::{fs, path};

use crate::globals::NUE_PATH;

pub fn find_cached_node_downloads() -> anyhow::Result<Vec<path::PathBuf>> {
    let mut caches = vec![];
    for entry in fs::read_dir(&*NUE_PATH)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }
        if path.file_name().unwrap() == "node" {
            continue;
        }

        if entry.file_name().to_string_lossy().starts_with("node") {
            caches.push(path);
        }
    }

    Ok(caches)
}
