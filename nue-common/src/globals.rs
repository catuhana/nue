use std::{path::PathBuf, sync::LazyLock};

use sys_traits::{FsCreateDirAll, impls::RealSys};

pub static ARGV_0: LazyLock<String> =
    LazyLock::new(|| std::env::args().next().unwrap_or_else(|| "nue".to_string()));

pub static NUE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = {
        #[cfg(windows)]
        {
            use crate::env_programs_dir::EnvProgramsDir as _;

            RealSys
                .env_programs_dir()
                .expect("failed to get programs directory")
                .join("nue")
        }
        #[cfg(unix)]
        {
            use sys_traits::EnvHomeDir as _;

            RealSys
                .env_home_dir()
                .expect("failed to get home directory")
                .join(".nue")
        }
        #[cfg(not(any(unix, windows)))]
        {
            panic!("unsupported platform")
        }
    };

    if !path.exists() {
        RealSys
            .fs_create_dir_all(&path)
            .expect("failed to create nue directory");
    }

    path
});

pub static NUE_NODE_PATH: LazyLock<PathBuf> = LazyLock::new(|| NUE_PATH.join("node"));

pub static NUE_CACHE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = NUE_PATH.join("cache");

    if !path.exists() {
        RealSys
            .fs_create_dir_all(&path)
            .expect("failed to create cache directory");
    }

    path
});
