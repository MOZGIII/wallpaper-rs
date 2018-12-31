use std::ffi::{OsStr, OsString};
use std::io::Error;
use std::iter::once;
use std::os::windows::prelude::*;

use winapi::shared::minwindef::{MAX_PATH, TRUE, UINT};
use winapi::um::winnt::{PVOID, WCHAR};
use winapi::um::winuser::{
    SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_GETDESKWALLPAPER,
    SPI_SETDESKWALLPAPER,
};

type Result<T> = std::result::Result<T, std::io::Error>;

pub fn set(full_path: &OsStr) -> Result<()> {
    let mut full_path_vec: Vec<u16> = proper_to_wide(full_path);
    let ret = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            full_path_vec.as_mut_ptr() as PVOID,
            SPIF_SENDCHANGE | SPIF_UPDATEINIFILE,
        )
    };
    match ret {
        TRUE => Ok(()),
        _ => Err(Error::last_os_error().into()),
    }
}

pub fn get() -> Result<OsString> {
    let mut full_path_vec = [0 as WCHAR; MAX_PATH];
    let ret = unsafe {
        SystemParametersInfoW(
            SPI_GETDESKWALLPAPER,
            full_path_vec.len() as UINT,
            full_path_vec.as_mut_ptr() as PVOID,
            0,
        )
    };
    match ret {
        TRUE => Ok(proper_from_wide(&full_path_vec)),
        _ => Err(Error::last_os_error().into()),
    }
}

fn proper_to_wide(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(once(0)).collect::<Vec<_>>()
}

fn proper_from_wide(s: &[u16]) -> OsString {
    // Panic if there's no null terminator.
    let pos = s.iter().position(|a| *a == 0).unwrap();
    OsString::from_wide(&s[..pos])
}
