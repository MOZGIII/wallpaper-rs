//! Handles COM initialization and cleanup.

use super::helpers::*;

use winapi::um::combaseapi::{CoInitializeEx, CoUninitialize};
use winapi::um::objbase::COINIT_MULTITHREADED;

use std::ptr;

/// RAII object that guards the fact that COM is initialized.
///
// We store a raw pointer because it's the only way at the moment to remove
// `Send`/`Sync` from the object.
struct ComInitialized(*mut ());

impl Drop for ComInitialized {
    #[inline]
    fn drop(&mut self) {
        unsafe { CoUninitialize() };
    }
}

thread_local! {
    static COM_INITIALIZED: ComInitialized = {
        unsafe {
            // This call can fail if another library initialized COM in
            // single-threaded mode.
            // TODO: handle this situation properly.
            check_result(CoInitializeEx(ptr::null_mut(), COINIT_MULTITHREADED)).unwrap();
            ComInitialized(ptr::null_mut())
        }
    }
}

/// Ensures that COM is initialized in this thread.
#[inline]
pub fn ensure_com_initialized() {
    COM_INITIALIZED.with(|_| {});
}
