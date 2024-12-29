use rfsee_tf_idf::Index;
use std::os::raw::c_char;
use std::{ffi::*, fs::File};

#[repr(C)]
pub struct RfcSearchResult {
    url: *const c_char,
    title: *const c_char,
}

#[repr(C)]
pub struct RfcSearchResults {
    len: i32,
    rfcs: *const RfcSearchResult,
    error: i32,
}

/// A private struct to hold both the public results interface and the heap allocated data for
/// the results.
#[allow(dead_code)]
struct RfcSearchResultsContainer {
    // This is what is returned
    results: RfcSearchResults,
    // Keep RFC results in a Box<[RfcSearchResult]> so they don't move.
    rfc_array: Box<[RfcSearchResult]>,
    // Keep CStrings so their pointers remain valid.
    cstrings: Vec<CString>,
}

#[no_mangle]
pub extern "C" fn build_index() {
    let path = rfsee_tf_idf::get_index_path(None).unwrap();
    let mut index = rfsee_tf_idf::TfIdf::default();
    index.par_load_rfcs().unwrap();
    index.finish();
    index.save(&path);
}

/// Search for the terms in the TF-IDF index and return the results in order with the highest
/// scoring document first.
///
/// # Errors
///
/// This function will return an error if the terms pointer is null, there is no index file, or the
/// provided terms can not be converted to a CStr.
///
/// # Safety
///
/// The terms are provided as a C string which are converted to a `CStr` which has the following
/// safety requirements
///
/// The memory pointed to by `ptr` must contain a valid nul terminator at the
///  end of the string.
///
/// * `ptr` must be [valid] for reads of bytes up to and including the nul terminator.
///     This means in particular:
///
/// * The entire memory range of this `CStr` must be contained within a single allocated object!
/// * `ptr` must be non-null even for a zero-length cstr.
/// * The memory referenced by the returned `CStr` must not be mutated for
///     the duration of lifetime `'a`.
///
/// * The nul terminator must be within `isize::MAX` from `ptr`
#[no_mangle]
pub unsafe extern "C" fn search_terms(terms: *const c_char) -> *mut RfcSearchResults {
    // To convert to `CStr` the pointer must be non-null
    if terms.is_null() {
        return make_error_results(1);
    }

    let index_path = match rfsee_tf_idf::get_index_path(None) {
        Ok(p) => p,
        Err(_) => return make_error_results(2),
    };
    let file = match File::open(index_path) {
        Ok(f) => f,
        Err(_) => {
            return make_error_results(3);
        }
    };

    let index: Index = match simd_json::from_reader(file) {
        Ok(i) => i,
        Err(_) => return make_error_results(4),
    };

    let c_str = unsafe { CStr::from_ptr(terms) };
    let query = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return make_error_results(5),
    };

    let search_results = rfsee_tf_idf::search_index(query.to_string(), index);

    let mut cstrings = Vec::new();
    let mut rfc_results = Vec::with_capacity(search_results.len());

    for result in search_results {
        let c_url = match CString::new(result.url) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let c_title = match CString::new(result.title) {
            Ok(c) => c,
            Err(_) => continue,
        };

        // Push these into cstrings so they're never dropped prematurely
        cstrings.push(c_url);
        cstrings.push(c_title);

        let url_ptr = cstrings[cstrings.len() - 2].as_ptr();
        let title_ptr = cstrings[cstrings.len() - 1].as_ptr();

        rfc_results.push(RfcSearchResult {
            url: url_ptr,
            title: title_ptr,
        });
    }

    let rfc_array = rfc_results.into_boxed_slice();
    let len = rfc_array.len() as i32;

    let container = RfcSearchResultsContainer {
        results: RfcSearchResults {
            len,
            rfcs: rfc_array.as_ptr(),
            error: 0,
        },
        rfc_array,
        cstrings,
    };

    let boxed = Box::new(container);
    let ptr = &boxed.results as *const RfcSearchResults as *mut RfcSearchResults;

    // Leak the container so that the allocations for the results stay around
    std::mem::forget(boxed);

    ptr
}

/// Error types
///
/// 1 -> Null terms
/// 2 -> Unable get index path
/// 3 -> Unable to open index file
/// 4 -> Unable to read index
/// 5 -> Unable to convert search terms to CStr
///
/// # Errors
///
/// This function will return an error if .
fn make_error_results(error: i32) -> *mut RfcSearchResults {
    let my_ffi = RfcSearchResultsContainer {
        results: RfcSearchResults {
            len: 0,
            rfcs: std::ptr::null(),
            error,
        },
        rfc_array: Box::new([]),
        cstrings: Vec::new(),
    };

    let boxed = Box::new(my_ffi);
    let ptr = &boxed.results as *const RfcSearchResults as *mut RfcSearchResults;
    std::mem::forget(boxed);
    ptr
}
