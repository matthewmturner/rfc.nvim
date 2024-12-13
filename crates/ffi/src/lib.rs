use std::ffi::*;
use std::os::raw::c_char;
use tf_idf::{TermFreqs, TfIdf};

#[no_mangle]
pub extern "C" fn tf_idf_create() -> *mut TfIdf {
    let index = api::TfIdf::default();
    let boxed = Box::new(index);
    Box::into_raw(boxed)
}

/// Add a document's term frequencies to the index
///
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn tf_idf_insert_doc_tfs(
    tf_idf: *mut TfIdf,
    doc: *const c_char,
    term_freqs: *mut TermFreqs,
) {
    if term_freqs.is_null() || doc.is_null() {
        return;
    }
    let tf_idf = unsafe { &mut *tf_idf };
    let key = unsafe { CStr::from_ptr(doc) };
    match key.to_str() {
        Ok(k) => {
            let term_freqs = Box::from_raw(term_freqs);
            tf_idf.doc_tfs.insert(k.to_owned(), *term_freqs);
        }
        Err(_) => {
            // eprintln!("ERROR: Unable to convert key to UTF-8")
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn tf_idf_add_doc(
    tf_idf: *mut TfIdf,
    url: *const c_char,
    doc: *const c_char,
) {
    if url.is_null() || doc.is_null() {}

    let tf_idf = unsafe { &mut *tf_idf };
    let url = unsafe { CStr::from_ptr(url) };
    let doc = unsafe { CStr::from_ptr(doc) };

    match (url.to_str(), doc.to_str()) {
        (Ok(u), Ok(d)) => tf_idf.add_doc(u, d),
        _ => {
            // eprintln!("ERROR: Unable to convert doc or url to utf-8");
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn tf_idf_finish(tf_idf: *mut TfIdf) -> *mut TfIdf {
    if tf_idf.is_null() {}
    let tf_idf = unsafe { &mut *tf_idf };
    tf_idf.finish();
    tf_idf
}

#[no_mangle]
pub unsafe extern "C" fn tf_idf_save(tf_idf: *mut TfIdf, path: *const c_char) {
    if tf_idf.is_null() {}
    let tf_idf = unsafe { &mut *tf_idf };
    let path = unsafe { CStr::from_ptr(path) };
    match path.to_str() {
        Ok(p) => {
            tf_idf.save(p);
        }
        Err(e) => {
            // eprintln!("Error converting path to utf8")
        }
    }
}

// #[no_mangle]
// pub unsafe extern "C" fn extract_tf(doc: *const c_char) -> *mut TermFreqs {
//     if doc.is_null() {}
//
//     let doc = unsafe { CStr::from_ptr(doc) };
//
//     match doc.to_str() {
//         Ok(d) => {
//             let tf = api::extract_tf(d);
//             let boxed = Box::new(tf);
//             Box::into_raw(boxed)
//         }
//         Err(e) => {
//             eprintln!("ERROR: Error converting doc to utf-8");
//             std::ptr::null()
//         }
//     }
// }

#[no_mangle]
pub extern "C" fn tf_create() -> *mut TermFreqs {
    let map: TermFreqs = TermFreqs::new();
    let boxed = Box::new(map);
    Box::into_raw(boxed)
}

/// ChatGPT created the safety docs
/// Inserts a key-value pair into the `TermFrequencies` map.
///
/// # Safety
/// - `term_freqs` must be a valid, non-null pointer to a `TermFrequencies` instance created by Rust.
/// - The caller must ensure that `term_freqs` is not being accessed concurrently or mutably elsewhere during this call.
/// - `key` must be a valid, non-null pointer to a null-terminated C string. The string must remain valid
///   for the duration of this function call.
/// - The function will return immediately if `term_freqs` or `key` is null.
/// - Undefined behavior may occur if the requirements above are not met.
#[no_mangle]
pub unsafe extern "C" fn tf_insert_term(
    term_freqs: *mut TermFreqs,
    term: *const c_char,
    value: f64,
) {
    if term_freqs.is_null() || term.is_null() {
        return;
    }
    let term_freqs = unsafe { &mut *term_freqs };
    let key = unsafe { CStr::from_ptr(term) };
    match key.to_str() {
        Ok(k) => {
            term_freqs.insert(k.to_owned(), value);
        }
        Err(_) => {
            eprintln!("ERROR: Unable to convert key to UTF-8")
        }
    }
}

/// ChatGPT created the safety docs
/// Retrieves a value from the `TermFrequencies` map by key.
///
/// # Safety
/// - `term_freqs` must be a valid, non-null pointer to a `TermFrequencies` instance created by Rust.
/// - The caller must ensure that `term_freqs` is not being accessed concurrently or mutably elsewhere during this call.
/// - `key` must be a valid, non-null pointer to a null-terminated C string. The string must remain valid
///   for the duration of this function call.
/// - The function will return a null pointer if:
///   - `term_freqs` or `key` is null.
///   - The key does not exist in the map.
/// - The returned pointer is valid only as long as `term_freqs` remains valid and is not modified.
/// - Undefined behavior may occur if the requirements above are not met.
#[no_mangle]
pub unsafe extern "C" fn get_term_freqs(
    term_freqs: *const TermFreqs,
    key: *const c_char,
) -> *const f64 {
    if term_freqs.is_null() || key.is_null() {
        // TODO: Maybe return an int that represents error types instead?
        return std::ptr::null();
    }

    let term_freqs = unsafe { &*term_freqs };
    let key = unsafe { CStr::from_ptr(key) };

    // Make sure the key is UTF-8
    match key.to_str() {
        Ok(k) => match term_freqs.get(k) {
            Some(v) => v,
            None => std::ptr::null(),
        },
        Err(_) => std::ptr::null(),
    }
}

/// Frees the term frequencies from memory
///
/// # Safety
/// -`term_freqs` must be a valid, non-null pointer to a `TermFrequencies` instance created by
/// Rust.
/// - The caller must ensure that `term_freqs` is not being accessed concurrently or mutably
///     elsewhere during this call
#[no_mangle]
pub unsafe extern "C" fn free_term_freqs(term_freqs: *mut TermFreqs) {
    if !term_freqs.is_null() {
        drop(Box::from_raw(term_freqs));
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
