use std::ffi::*;
use std::os::raw::c_char;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[repr(C)]
pub struct SaveResult {
    error: bool,
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
/// This is safe
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
