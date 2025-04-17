#![cfg(windows)]

use std::path::PathBuf;

use sys_traits::impls::RealSys;

pub trait EnvProgramsDir {
    fn env_programs_dir(&self) -> Option<PathBuf>;
}

// We could just do `env_cache_dir().join()` like the Unix implementation,
// but it's recommended to use `FOLDERID_UserProgramFiles` directly.
// Implementation originates from
// https://github.com/emilk/egui/blob/3f731ec79407ed1a116ec354dbf8c1bbf10773ed/crates/eframe/src/native/file_storage.rs#L46
impl EnvProgramsDir for RealSys {
    fn env_programs_dir(&self) -> Option<PathBuf> {
        use std::{ffi::OsString, os::windows::ffi::OsStringExt as _};
        use windows_sys::Win32::{
            Foundation::S_OK,
            System::Com::CoTaskMemFree,
            UI::Shell::{FOLDERID_UserProgramFiles, KF_FLAG_DEFAULT, SHGetKnownFolderPath},
        };

        unsafe extern "C" {
            fn wcslen(s: *const u16) -> usize;
        }

        let mut path_raw = std::ptr::null_mut();
        let result = unsafe {
            SHGetKnownFolderPath(
                &FOLDERID_UserProgramFiles,
                KF_FLAG_DEFAULT as u32,
                std::ptr::null_mut(),
                &mut path_raw,
            )
        };

        let path = if result == S_OK {
            let path_slice = unsafe { std::slice::from_raw_parts(path_raw, wcslen(path_raw)) };
            Some(PathBuf::from(OsString::from_wide(path_slice)))
        } else {
            None
        };

        unsafe {
            CoTaskMemFree(path_raw.cast());
        }

        path
    }
}
