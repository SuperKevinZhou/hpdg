//! Experimental C ABI bindings.

#[cfg(feature = "c-bindings")]
use std::os::raw::c_char;

#[cfg(feature = "c-bindings")]
#[unsafe(no_mangle)]
/// Return the crate version string for C callers.
pub extern "C" fn hpdg_version() -> *const c_char {
    std::ptr::null()
}
