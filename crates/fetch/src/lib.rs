use std::{
    io::{Read, Write},
    net::TcpStream,
};

use native_tls::TlsConnector;

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
