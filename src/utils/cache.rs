use std::{fs, path};

pub fn find_cached_node_downloads() -> anyhow::Result<Vec<path::PathBuf>> {
    let system_temp_path = std::env::temp_dir();

    let mut caches = vec![];
    for entry in fs::read_dir(&system_temp_path)? {
        let entry = entry?;

        if !entry.path().is_dir() {
            continue;
        }

        if entry.file_name().to_string_lossy().starts_with("nue") {
            caches.push(entry.path());
        }
    }

    Ok(caches)
}
