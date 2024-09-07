use std::{env, fs, path};

pub fn find_cached_node_downloads() -> anyhow::Result<Vec<path::PathBuf>> {
    let nue_temp_path = env::temp_dir().join("nue");

    let mut caches = vec![];
    for entry in fs::read_dir(&nue_temp_path)? {
        let entry = entry?;

        if !entry.path().is_dir() {
            continue;
        }

        if entry.file_name().to_string_lossy().starts_with("node") {
            caches.push(entry.path());
        }
    }

    Ok(caches)
}
