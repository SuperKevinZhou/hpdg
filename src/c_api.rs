#[cfg(feature = "c-bindings")]
use std::os::raw::c_char;

#[cfg(feature = "c-bindings")]
#[unsafe(no_mangle)]
pub extern "C" fn hpdg_version() -> *const c_char {
    std::ptr::null()
}
