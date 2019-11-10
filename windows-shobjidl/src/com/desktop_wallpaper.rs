use super::helpers::*;
use super::init::ensure_com_initialized;
use crate::Result;
use std::ffi::{OsStr, OsString};
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::ptr;
use widestring::U16CString;
use winapi::shared::minwindef::{BOOL, FALSE, LPVOID, TRUE, UINT};
use winapi::shared::windef::{COLORREF, RECT};
use winapi::um::combaseapi::{CoCreateInstance, CoTaskMemFree, CLSCTX_ALL};
use winapi::um::shobjidl_core::{
    CLSID_DesktopWallpaper, IDesktopWallpaper, DESKTOP_SLIDESHOW_DIRECTION,
    DESKTOP_SLIDESHOW_OPTIONS, DESKTOP_SLIDESHOW_STATE, DESKTOP_WALLPAPER_POSITION,
};
use winapi::um::winnt::{LPCWSTR, LPWSTR};
use winapi::Interface;

/// Provides access to the IDesktopWallpaper COM interface.
pub struct DesktopWallpaper(*mut IDesktopWallpaper);

// We initialize COM locally per-thread, but we only have to initialize it in
// the thread we're creating objects from.
unsafe impl Send for DesktopWallpaper {}
unsafe impl Sync for DesktopWallpaper {}

impl Drop for DesktopWallpaper {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            (*self.0).Release();
        }
    }
}

pub type MonitorID = OsString;
pub type MonID = OsStr;

impl DesktopWallpaper {
    /// Creates a DesktopWallpaper backed by a COM object.
    pub fn new() -> Result<DesktopWallpaper> {
        // COM initialization is thread local, but we only need to have COM
        // initialized in the thread we create the objects in.
        ensure_com_initialized();

        // Create the desktop wallpaper.
        unsafe {
            let mut desktop_wallpaper: *mut IDesktopWallpaper = ptr::null_mut();

            let hresult = CoCreateInstance(
                &CLSID_DesktopWallpaper,
                ptr::null_mut(),
                CLSCTX_ALL,
                &IDesktopWallpaper::uuidof(),
                &mut desktop_wallpaper as *mut *mut IDesktopWallpaper as *mut LPVOID,
            );

            check_result(hresult)?;
            Ok(DesktopWallpaper(desktop_wallpaper))
        }
    }

    /// Sets the desktop wallpaper.
    /// If `monitor_id` is `None` sets the wallpaper on all monitors.
    pub fn set_wallpaper<MID, WP>(&mut self, monitor_id: Option<MID>, wallpaper: WP) -> Result<()>
    where
        MID: AsRef<MonID>,
        WP: AsRef<Path>,
    {
        let monitor_id: Option<U16CString> = match monitor_id {
            Some(val) => Some(U16CString::from_os_str(val.as_ref())?),
            None => None,
        };
        let wallpaper: U16CString = U16CString::from_os_str(wallpaper.as_ref())?;
        check_result(unsafe {
            (*self.0).SetWallpaper(
                match monitor_id {
                    Some(val) => val.as_ptr(),
                    None => ptr::null(),
                },
                wallpaper.as_ptr() as LPCWSTR,
            )
        })
    }

    /// Gets the current desktop wallpaper.
    ///
    /// The `monitor_id` can be `None`. In that case, if a single wallpaper
    /// image is displayed on all of the system's monitors, the method returns
    /// successfully. If `monitor_id` is set to `None` and different monitors
    /// are displaying different wallpapers or a slideshow is running, the
    /// method returns an error with `S_FALSE` `Error::IOError`.
    ///
    /// The resulting `PathBuf` can will be empty if no wallpaper image is being
    /// displayed or if a monitor is displaying a solid color.
    pub fn get_wallpaper<MID>(&mut self, monitor_id: Option<MID>) -> Result<PathBuf>
    where
        MID: AsRef<MonID>,
    {
        let monitor_id: Option<U16CString> = match monitor_id {
            Some(val) => Some(U16CString::from_os_str(val.as_ref())?),
            None => None,
        };
        unsafe {
            let mut wallpaper_buf_ptr: LPWSTR = ptr::null_mut();
            check_result((*self.0).GetWallpaper(
                match monitor_id {
                    Some(val) => val.as_ptr() as LPCWSTR,
                    None => ptr::null(),
                },
                &mut wallpaper_buf_ptr,
            ))?;
            let wallpaper = U16CString::from_ptr_str(wallpaper_buf_ptr);
            CoTaskMemFree(wallpaper_buf_ptr as LPVOID);
            Ok(wallpaper.to_os_string().into())
        }
    }

    /// Retrieves the unique ID of one of the system's monitors.
    ///
    /// This method can be called on monitors that are currently detached but
    /// that have an image assigned to them. Call `get_monitor_rect` to
    /// distinguish between attached and detached monitors.
    pub fn get_monitor_device_path_at(&mut self, monitor_index: UINT) -> Result<MonitorID> {
        unsafe {
            let mut monitor_id_buf_ptr: LPWSTR = ptr::null_mut();
            check_result((*self.0).GetMonitorDevicePathAt(monitor_index, &mut monitor_id_buf_ptr))?;
            let monitor_id = U16CString::from_ptr_str(monitor_id_buf_ptr);
            CoTaskMemFree(monitor_id_buf_ptr as LPVOID);
            Ok(monitor_id.to_os_string())
        }
    }

    /// Retrieves the number of monitors that are associated with the system.
    ///
    /// The count retrieved through this method includes monitors that are
    /// currently detached but that have an image assigned to them.
    /// Call `get_monitor_rect` to distinguish between attached and detached
    /// monitors.
    pub fn get_monitor_device_path_count(&mut self) -> Result<UINT> {
        let mut count: UINT = 0;
        check_result(unsafe { (*self.0).GetMonitorDevicePathCount(&mut count) })?;
        Ok(count)
    }

    /// Retrieves the display rectangle of the specified monitor.
    pub fn get_monitor_rect<MID>(&mut self, monitor_id: MID) -> Result<RECT>
    where
        MID: AsRef<MonID>,
    {
        let monitor_id: U16CString = U16CString::from_os_str(monitor_id.as_ref())?;
        let mut display_rect = MaybeUninit::<RECT>::uninit();
        unsafe {
            check_result((*self.0).GetMonitorRECT(monitor_id.as_ptr(), display_rect.as_mut_ptr()))?;
            Ok(display_rect.assume_init())
        }
    }

    /// Sets the color that is visible on the desktop when no image is displayed
    /// or when the desktop background has been disabled. This color is also
    /// used as a border when the desktop wallpaper does not fill the entire
    /// screen.
    pub fn set_background_color(&mut self, color: COLORREF) -> Result<()> {
        check_result(unsafe { (*self.0).SetBackgroundColor(color) })
    }

    /// Retrieves the color that is visible on the desktop when no image is
    /// displayed or when the desktop background has been disabled. This color
    /// is also used as a border when the desktop wallpaper does not fill the
    /// entire screen.
    pub fn get_background_color(&mut self) -> Result<COLORREF> {
        let mut color = MaybeUninit::<COLORREF>::uninit();
        unsafe {
            check_result((*self.0).GetBackgroundColor(color.as_mut_ptr()))?;
            Ok(color.assume_init())
        }
    }

    /// Sets the display option for the desktop wallpaper image, determining
    /// whether the image should be centered, tiled, or stretched.
    pub fn set_position(&mut self, position: DESKTOP_WALLPAPER_POSITION) -> Result<()> {
        check_result(unsafe { (*self.0).SetPosition(position) })
    }

    /// Retrieves the current display value for the desktop background image.
    pub fn get_position(&mut self) -> Result<DESKTOP_WALLPAPER_POSITION> {
        let mut position = MaybeUninit::<DESKTOP_WALLPAPER_POSITION>::uninit();
        unsafe {
            check_result((*self.0).GetPosition(position.as_mut_ptr()))?;
            Ok(position.assume_init())
        }
    }

    // TODO: implement.
    // Specifies the images to use for the desktop wallpaper slideshow.
    // fn SetSlideshow(
    //     items: *mut IShellItemArray,
    // ) -> HRESULT,

    // TODO: implement.
    // Gets the images that are being displayed in the desktop wallpaper
    // slideshow.
    // fn GetSlideshow(
    //     items: *mut *mut IShellItemArray,
    // ) -> HRESULT,

    /// Sets the desktop wallpaper slideshow settings for shuffle and timing.
    pub fn set_slideshow_options(
        &mut self,
        options: DESKTOP_SLIDESHOW_OPTIONS,
        slideshow_tick: UINT,
    ) -> Result<()> {
        check_result(unsafe { (*self.0).SetSlideshowOptions(options, slideshow_tick) })
    }

    /// Gets the current desktop wallpaper slideshow settings for shuffle and
    /// timing.
    pub fn get_slideshow_options(&mut self) -> Result<(DESKTOP_WALLPAPER_POSITION, UINT)> {
        let mut options = MaybeUninit::<DESKTOP_SLIDESHOW_OPTIONS>::uninit();
        let mut slideshow_tick = MaybeUninit::<UINT>::uninit();
        unsafe {
            check_result(
                (*self.0).GetSlideshowOptions(options.as_mut_ptr(), slideshow_tick.as_mut_ptr()),
            )?;
            Ok((options.assume_init(), slideshow_tick.assume_init()))
        }
    }

    /// Switches the wallpaper on a specified monitor to the next image in the
    /// slideshow.
    /// If `monitor_id` is `None`, the monitor scheduled to change next is used.
    pub fn advance_slideshow<MID>(
        &mut self,
        monitor_id: Option<MID>,
        direction: DESKTOP_SLIDESHOW_DIRECTION,
    ) -> Result<()>
    where
        MID: AsRef<MonID>,
    {
        let monitor_id: Option<U16CString> = match monitor_id {
            Some(val) => Some(U16CString::from_os_str(val.as_ref())?),
            None => None,
        };
        check_result(unsafe {
            (*self.0).AdvanceSlideshow(
                match monitor_id {
                    Some(val) => val.as_ptr(),
                    None => ptr::null(),
                },
                direction,
            )
        })
    }

    /// Gets the current status of the slideshow.
    pub fn get_status(&mut self) -> Result<DESKTOP_SLIDESHOW_STATE> {
        let mut state = MaybeUninit::<DESKTOP_SLIDESHOW_STATE>::uninit();
        unsafe {
            check_result((*self.0).GetStatus(state.as_mut_ptr()))?;
            Ok(state.assume_init())
        }
    }

    /// Enables or disables the desktop background.
    ///
    /// This method would normally be called to disable the desktop background
    /// for performance reasons.
    ///
    /// When the desktop background is disabled, a solid color is shown in its
    /// place. To get or set the specific color, use the `get_background_color`
    /// and `set_background_color` methods.
    pub fn enable(&mut self, enable: bool) -> Result<()> {
        let val: BOOL = if enable { TRUE } else { FALSE };
        check_result(unsafe { (*self.0).Enable(val) })
    }
}
