#![warn(rust_2018_idioms)]

mod com;
mod error;

pub use com::desktop_wallpaper::{DesktopWallpaper, MonID, MonitorID};
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "singleton")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "singleton")]
mod singleton {
    use crate::DesktopWallpaper;
    use std::sync::Mutex;

    lazy_static! {
        static ref DESKTOP_WALLPAPER: Mutex<DesktopWallpaper> =
            Mutex::new(DesktopWallpaper::new().unwrap());
    }

    /// Provides access to the singleton `DesktopWallpaper`.
    /// Only available if the `singleton` feature is enabled.
    pub fn desktop_wallpaper() -> &'static Mutex<DesktopWallpaper> {
        &DESKTOP_WALLPAPER
    }
}

#[cfg(feature = "singleton")]
pub use self::singleton::desktop_wallpaper;
