use std::{path::PathBuf, sync::LazyLock};

pub static NUE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    dirs::home_dir()
        .expect("failed to get home directory")
        .join(".nue")
});
