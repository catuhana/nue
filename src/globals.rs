use std::{path::PathBuf, sync::LazyLock};

pub static NUE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    #[cfg(unix)]
    {
        dirs::home_dir()
            .expect("failed to get home directory")
            .join(".nue")
    }
    #[cfg(windows)]
    {
        dirs::data_local_dir()
            .expect("failed to get data directory")
            .join("nue")
    }
});

pub static NUE_RELEASES_PATH: LazyLock<PathBuf> = LazyLock::new(|| NUE_PATH.join("releases"));
