use std::{
    io::{Read, Write},
    net::TcpStream,
};

use native_tls::TlsConnector;

use crate::{
    parse::{parse_rfc, parse_rfcs_index},
    RfcEntry,
};

const RFC_INDEX_URL: &str = "https://www.ietf.org/rfc/rfc-index.txt";
const RFC_EDITOR_URL_BASE: &str = "https://www.rfc-editor.org/rfc/rfc";

pub fn fetch(url: &str) -> anyhow::Result<String> {
    let parts: Vec<&str> = url.split("://").collect();
    if parts.len() == 2 {
        let _scheme = parts[0];
        let remaining = parts[1];
        let (domain, path) = match remaining.split_once("/") {
            Some((domain, path)) => (domain, path),
            None => anyhow::bail!("Unable to parse domain and path"),
        };
        let path_with_prefix = format!("/{path}");
        let addr = format!("{domain}:443");

        let connector = TlsConnector::new()?;
        let stream = TcpStream::connect(&addr)?;
        let mut stream = connector.connect(domain, stream)?;
        let req = format!(
        "GET {path_with_prefix} HTTP/1.1\r\nHost: {domain}\r\nUser-Agent: rfsee/0.0.1\r\nConnection: close\r\n\r\n"
    );
        stream.write_all(req.as_bytes())?;
        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;

        Ok(buf)
    } else {
        anyhow::bail!("Invalid url: {url}");
    }
}

/// Return the raw `String` contents of IETF RFC index
pub fn fetch_rfc_index() -> anyhow::Result<String> {
    let rfc_index_content = fetch(RFC_INDEX_URL)?;
    Ok(rfc_index_content)
}

pub fn fetch_rfcs() -> anyhow::Result<Vec<RfcEntry>> {
    let rfc_index_content = fetch(RFC_INDEX_URL)?;
    let rfcs = parse_rfcs_index(rfc_index_content)?;
    Ok(rfcs)
}

pub fn fetch_rfc(raw_rfc: &str) -> anyhow::Result<RfcEntry> {
    if let Ok((rfc_num, title)) = parse_rfc(raw_rfc) {
        let url = format!("{RFC_EDITOR_URL_BASE}{rfc_num}.txt");
        if let Ok(content) = fetch(&url) {
            Ok(RfcEntry {
                number: rfc_num,
                url: url.clone(),
                title: title.replace("\n     ", " ").to_string(),
                content: Some(content),
            })
        } else {
            anyhow::bail!("Unable to fetch RFC")
        }
    } else {
        anyhow::bail!("Unable to parse raw RFC")
    }
}

fn _fetch_urls(_urls: &[&str]) -> anyhow::Result<Vec<String>> {
    // Use single connection
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_empty_path() {
        let url = "https://www.rfc-editor.org/";
        let res = fetch(url).unwrap();
        if let Some(first_line) = res.lines().next() {
            assert_eq!(first_line, "HTTP/1.1 200 OK")
        }
    }

    #[test]
    fn fetch_ietf_path() {
        let url = "https://www.ietf.org/rfc/rfc-index.txt";
        let res = fetch(url).unwrap();
        if let Some(first_line) = res.lines().next() {
            assert_eq!(first_line, "HTTP/1.1 200 OK")
        }
    }

    #[test]
    fn fetch_rfc() {
        let url = "https://www.rfc-editor.org/rfc/rfc8124.txt";
        let res = fetch(url).unwrap();
        if let Some(first_line) = res.lines().next() {
            assert_eq!(first_line, "HTTP/1.1 200 OK")
        }
    }
}
