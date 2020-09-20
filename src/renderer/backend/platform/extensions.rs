use ash::extensions::khr;
//----------------------------------------------------------------------------------------------------------------------

use crate::utils::ffi;
//----------------------------------------------------------------------------------------------------------------------

#[cfg(target_os = "windows")]
pub fn get_platform_instance_extensions() -> Vec<ffi::CString> {
    vec![khr::Win32Surface::name()]
        .into_iter()
        .map(|extension| ffi::cstr_to_cstring(extension))
        .collect()
}
//----------------------------------------------------------------------------------------------------------------------

#[cfg(target_os = "linux")]
pub fn get_platform_instance_extensions() -> Vec<ffi::CString> {
    vec![khr::XcbSurface::name()]
        .into_iter()
        .map(|extension| ffi::cstr_to_cstring(extension))
        .collect()
}
//----------------------------------------------------------------------------------------------------------------------
