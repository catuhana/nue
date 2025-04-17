#![cfg(windows)]

use std::path::PathBuf;

use sys_traits::impls::RealSys;
use windows::Win32::UI::Shell::{FOLDERID_UserProgramFiles, KF_FLAG_DEFAULT, SHGetKnownFolderPath};

pub trait EnvProgramsDir {
    fn env_programs_dir(&self) -> Option<PathBuf>;
}

// We could just do `env_cache_dir().join()` like the Unix implementation,
// but it's recommended to use `FOLDERID_UserProgramFiles` directly.
impl EnvProgramsDir for RealSys {
    fn env_programs_dir(&self) -> Option<PathBuf> {
        unsafe {
            match SHGetKnownFolderPath(&FOLDERID_UserProgramFiles, KF_FLAG_DEFAULT, None) {
                Ok(path) => Some(PathBuf::from(path.to_string().ok()?)),
                Err(_) => None,
            }
        }
    }
}
