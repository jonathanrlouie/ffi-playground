use std::ffi::{CStr, CString, c_char};

// There isn't a way to pass in an invalid int here that would
// trigger UB, so this fn does not need to be marked as unsafe.
#[unsafe(no_mangle)]
extern "C" fn pass_int(_i: u32) -> bool {
    return true;
}

// Note: Since the caller can pass an arbitrary *const c_char, we need to
// add safety docs here and mark the function as unsafe. The Rust FFI Omnibus
// does not do this, but it really should, since it would be UB if a Rust
// function called this with an invalid pointer without using an unsafe block.

/// Returns true if `s` contains the substring "rust" and false otherwise.
///
/// # Panics
///
/// This function will panic if `s` is null or not a valid UTF-8 string.
///
/// # Safety
///
/// It is undefined behaviour to call this function for any non-null pointer `s` that cannot be converted safely to a `CStr`.
/// See the safety docs for [CStr::from_ptr](https://doc.rust-lang.org/src/core/ffi/c_str.rs.html#263)
#[unsafe(no_mangle)]
unsafe extern "C" fn contains_rust(s: *const c_char) -> bool {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    // panic if the string isn't valid UTF-8
    let r_str = c_str.to_str().unwrap();
    r_str.contains("rust")
}

#[unsafe(no_mangle)]
extern "C" fn get_rust_string() -> *mut c_char {
    let rust_string = String::from("rust");
    let c_string = CString::new(rust_string).unwrap();
    c_string.into_raw()
}

/// ## Safety
///
/// This should only ever be called with a pointer earlier obtained by calling `get_rust_string`.
#[unsafe(no_mangle)]
unsafe extern "C" fn free_rust_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        drop(CString::from_raw(s))
    };
}
