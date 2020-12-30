pub use std::ffi::{CStr, CString};
use std::os::raw::c_char;
//----------------------------------------------------------------------------------------------------------------------

pub type CharArray = [c_char; 256];
pub type CharPtr = *const c_char;
//----------------------------------------------------------------------------------------------------------------------

#[inline]
pub fn string_to_cstring(string: &str) -> CString {
    CString::new(string).expect(&format!("Failed to convert {} to *const c_char!", string))
}
//----------------------------------------------------------------------------------------------------------------------

#[inline]
pub fn cstr_to_string(c_str: &CStr) -> String {
    unsafe {
        CStr::from_ptr(c_str.as_ptr())
            .to_string_lossy()
            .into_owned()
    }
}
//----------------------------------------------------------------------------------------------------------------------

#[inline]
pub fn cstr_to_cstring(c_str: &CStr) -> CString {
    CString::from(c_str)
}
//----------------------------------------------------------------------------------------------------------------------

#[inline]
pub fn char_array_to_string(char_array: CharArray) -> String {
    unsafe {
        CStr::from_ptr(char_array.as_ptr())
            .to_string_lossy()
            .into_owned()
    }
}
//----------------------------------------------------------------------------------------------------------------------

#[inline]
pub fn char_array_to_cstring(char_array: CharArray) -> CString {
    unsafe { CString::from(CStr::from_ptr(char_array.as_ptr())) }
}
//----------------------------------------------------------------------------------------------------------------------

#[inline]
pub fn char_ptr_to_str_ref<'a>(char_ptr: CharPtr) -> &'a str {
    unsafe { CStr::from_ptr(char_ptr).to_str().unwrap() }
}
//----------------------------------------------------------------------------------------------------------------------

#[inline]
pub fn vec_cstring_to_char_ptr(vec: &Vec<CString>) -> Vec<CharPtr> {
    vec.iter().map(|c_string| c_string.as_ptr()).collect()
}
//----------------------------------------------------------------------------------------------------------------------
