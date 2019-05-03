#![warn(rust_2018_idioms)]

use std::path::{Path, PathBuf};

use winapi::shared::minwindef::{MAX_PATH, TRUE, UINT};
use winapi::um::winnt::{HRESULT, PVOID, WCHAR};
use winapi::um::winuser::{
    SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_GETDESKWALLPAPER,
    SPI_SETDESKWALLPAPER,
};

use widestring::{U16CStr, U16CString};

mod error;

pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Set desktop image.
///
/// ```no_run
/// use wallpaper_windows_user32::set;
/// use std::path::{Path, PathBuf};
///
/// let path = Path::new(r#"C:\Users\User\AppData\Local\Temp\qwerty.jpg"#);
/// let result = set(path);
/// assert!(result.is_ok())
/// ```
pub fn set<T: AsRef<Path>>(full_path: T) -> Result<()> {
    let full_path: U16CString = U16CString::from_os_str(full_path.as_ref())?;
    let ret = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            full_path.as_ptr() as PVOID,
            SPIF_SENDCHANGE | SPIF_UPDATEINIFILE,
        )
    };
    check_result(ret)?;
    Ok(())
}

/// Get desktop image.
///
/// ```no_run
/// use wallpaper_windows_user32::get;
/// use std::path::{Path, PathBuf};
///
/// let wallpaper_path: PathBuf = get().unwrap();
/// assert_eq!(Path::new(r#"C:\Users\User\AppData\Local\Temp\qwerty.jpg"#), wallpaper_path)
/// ```
pub fn get() -> Result<PathBuf> {
    let mut full_path_buf = [0 as WCHAR; MAX_PATH];
    let ret = unsafe {
        SystemParametersInfoW(
            SPI_GETDESKWALLPAPER,
            full_path_buf.len() as UINT,
            full_path_buf.as_mut_ptr() as PVOID,
            0,
        )
    };
    check_result(ret)?;
    let full_path: &U16CStr = U16CStr::from_slice_with_nul(&full_path_buf)?;
    Ok(full_path.to_os_string().into())
}

fn check_result(result: HRESULT) -> Result<()> {
    match result {
        TRUE => Ok(()),
        _ => Err(std::io::Error::from_raw_os_error(result))?,
    }
}
