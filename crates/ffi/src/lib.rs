use std::os::raw::c_char;
use std::{ffi::*, fs::File};
use tf_idf::Index;

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

/// A private struct to hold all data so it won't drop prematurely.
#[allow(dead_code)]
struct RfcSearchResultsContainer {
    results: RfcSearchResults,
    // Keep RFC results in a Box<[RfcSearchResult]> so they don't move.
    rfc_array: Box<[RfcSearchResult]>,
    // Keep CStrings so their pointers remain valid.
    cstrings: Vec<CString>,
}

#[no_mangle]
pub extern "C" fn build_index() {
    let mut index = tf_idf::TfIdf::default();
    index.load_rfcs().unwrap();
    index.finish();
}

#[no_mangle]
pub unsafe extern "C" fn search_terms(terms: *const c_char) -> *mut RfcSearchResults {
    if terms.is_null() {
        return make_error_results();
    }

    let index_path = tf_idf::get_index_path(None);
    let file = match File::open(index_path) {
        Ok(f) => f,
        Err(_) => {
            return make_error_results();
        }
    };

    let index: Index = match simd_json::from_reader(file) {
        Ok(i) => i,
        Err(_) => return make_error_results(),
    };

    let c_str = unsafe { CStr::from_ptr(terms) };
    let query = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return make_error_results(),
    };

    let search_results = tf_idf::compute_search_scores(query.to_string(), index);

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

    let my_ffi = RfcSearchResultsContainer {
        results: RfcSearchResults {
            len,
            rfcs: rfc_array.as_ptr(),
            error: 0,
        },
        rfc_array,
        cstrings,
    };

    let boxed = Box::new(my_ffi);
    let ptr = &boxed.results as *const RfcSearchResults as *mut RfcSearchResults;

    // Leak the Box, passing ownership to the caller
    std::mem::forget(boxed);

    ptr
}

fn make_error_results() -> *mut RfcSearchResults {
    let my_ffi = RfcSearchResultsContainer {
        results: RfcSearchResults {
            len: 0,
            rfcs: std::ptr::null(),
            error: 1,
        },
        rfc_array: Box::new([]),
        cstrings: Vec::new(),
    };

    let boxed = Box::new(my_ffi);
    let ptr = &boxed.results as *const RfcSearchResults as *mut RfcSearchResults;
    std::mem::forget(boxed);
    ptr
}
