use std::collections::HashMap;
use std::ffi::*;
use std::os::raw::c_char;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[repr(C)]
pub struct SaveResult {
    error: bool,
}

#[repr(C)]
pub struct TermFrequencies {
    inner: HashMap<String, f64>,
}

#[no_mangle]
pub extern "C" fn create_term_frequencies() -> *mut TermFrequencies {
    let map: HashMap<String, f64> = HashMap::new();
    let term_freqs = TermFrequencies { inner: map };
    let boxed = Box::new(term_freqs);
    Box::into_raw(boxed)
}

#[no_mangle]
pub extern "C" fn hello_world() -> i32 {
    42
}

#[no_mangle]
pub extern "C" fn save_json() -> SaveResult {
    match api::save_json() {
        Ok(_) => SaveResult { error: false },
        Err(_) => SaveResult { error: true },
    }
}

#[no_mangle]
pub extern "C" fn save_input_number_as_json(val: i32) -> SaveResult {
    match api::save_input_number_as_json(val) {
        Ok(_) => SaveResult { error: false },
        Err(_) => SaveResult { error: true },
    }
}

/// Take input number and save it to provided path as JSON
/// # Safety
/// This function is marked as `unsafe` because it takes a raw pointer (`path`) as an argument.
/// The caller must ensure the following:
///
/// - `path` must be non-null. Passing a null pointer will result in undefined behavior.
/// - `path` must point to a valid null-terminated C-style string.
/// - The string referenced by `path` must remain valid for the duration of the function call.
/// - The caller must ensure proper synchronization if this function is called concurrently,
///   as it could interact with shared resources in `api::save_input_number_as_json_to_custom_path`.
///
/// Failure to adhere to these requirements may result in undefined behavior or program crashes.
#[no_mangle]
pub unsafe extern "C" fn save_input_number_as_json_to_custom_path(
    val: i32,
    path: *const c_char,
) -> SaveResult {
    if path.is_null() {
        return SaveResult { error: true };
    }

    let c_str = unsafe { CStr::from_ptr(path) };
    match c_str.to_str() {
        Ok(s) => match api::save_input_number_as_json_to_custom_path(val, s) {
            Ok(_) => SaveResult { error: false },
            Err(_) => SaveResult { error: true },
        },
        Err(_) => SaveResult { error: true },
    }
}

pub unsafe extern "C" fn print_table() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
