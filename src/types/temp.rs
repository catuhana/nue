use std::{fs, path};

use anyhow::Ok;

use crate::utils;

#[derive(Debug)]
pub struct Folder {
    system_temp_path: path::PathBuf,
    pub prefix: String,
    pub uid: String,
}

impl Folder {
    pub fn new() -> anyhow::Result<Self> {
        let uid = utils::random::generate_string(8);

        let system_temp_path = std::env::temp_dir();
        let prefix = String::from("nue");
        let path = system_temp_path.join(format!("{prefix}-{uid}"));

        fs::create_dir(&path)?;

        Ok(Self {
            system_temp_path,
            prefix,
            uid,
        })
    }

    pub fn find_caches(&self) -> anyhow::Result<Vec<path::PathBuf>> {
        let mut caches = vec![];
        for entry in fs::read_dir(&self.system_temp_path)? {
            let entry = entry?;

            if !entry.path().is_dir() {
                continue;
            }

            if entry
                .file_name()
                .to_string_lossy()
                .starts_with(&self.prefix)
            {
                caches.push(entry.path());
            }
        }

        Ok(caches)
    }

    pub fn get_full_path(&self) -> path::PathBuf {
        self.system_temp_path.join(format!("nue-{}", self.uid))
    }
}
