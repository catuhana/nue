use crate::utils;

pub struct Folder(pub std::path::PathBuf);

impl Folder {
    pub fn create() -> anyhow::Result<Self> {
        let system_temp_folder = std::env::temp_dir();
        let temp_folder_path =
            system_temp_folder.join(format!("nue-{}", utils::random::generate_string(8)));

        std::fs::create_dir_all(&temp_folder_path)?;

        Ok(Self(temp_folder_path))
    }

    pub fn delete(&self) -> anyhow::Result<()> {
        std::fs::remove_dir_all(&self.0)?;

        Ok(())
    }

    pub fn path(&self) -> &std::path::Path {
        &self.0
    }
}

impl Drop for Folder {
    fn drop(&mut self) {
        if let Err(err) = self.delete() {
            eprintln!("Failed to delete temporary folder: {err}");
        }
    }
}
