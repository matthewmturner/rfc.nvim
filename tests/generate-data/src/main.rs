use std::{path::PathBuf, str::FromStr};

use rfsee_tf_idf::{RfcEntry, TfIdf};

fn main() {
    let mut tf_idf = TfIdf::default();
    let rfc1 = RfcEntry {
        number: 1,
        url: "https://rfsee.com/1".to_string(),
        title: "Test 1".to_string(),
        content: Some("Hello world".to_string()),
    };
    let rfc2 = RfcEntry {
        number: 2,
        url: "https://rfsee.com/2".to_string(),
        title: "Test 2".to_string(),
        content: Some("Goodbye car".to_string()),
    };

    tf_idf.add_rfc_entry(rfc1);
    tf_idf.add_rfc_entry(rfc2);

    tf_idf.finish();
    let path_buf = PathBuf::from_str("/tmp/index.json").unwrap();
    tf_idf.save(&path_buf);
}
