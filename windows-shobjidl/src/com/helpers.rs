use crate::Result;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::winnt::HRESULT;

pub fn check_result(result: HRESULT) -> Result<()> {
    if SUCCEEDED(result) {
        Ok(())
    } else {
        Err(std::io::Error::from_raw_os_error(result))?
    }
}
