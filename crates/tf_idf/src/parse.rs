const RFC_DELIMITER: &str = "\n\n";

/// Parse raw `String` contents of RFC index and return `Vec` of `&str` for each item after
/// splitting on `RFC_DELIMITER`
pub fn parse_rfc_index(content: &str) -> anyhow::Result<Vec<&str>> {
    let found = content.find("0001");
    match found {
        Some(idx) => {
            let raw_rfcs = &content[idx..];
            let splitted = raw_rfcs.split(RFC_DELIMITER).collect();
            Ok(splitted)
        }
        None => anyhow::bail!("Unable to parse RFC index"),
    }
}

/// Parse raw RFC `String` contents into the RFC number and its title
pub fn parse_rfc(rfc_content: &str) -> anyhow::Result<(i32, &str)> {
    if let Some((rfc_num, title)) = rfc_content.split_once(" ") {
        let parsed_num: i32 = rfc_num.parse()?;
        Ok((parsed_num, title))
    } else {
        anyhow::bail!("Unable to parse RFC number {rfc_content}");
    }
}

#[cfg(test)]
mod tests {
    use super::parse_rfc_index;

    #[test]
    fn test_parse_index() {
        let index_contents = std::fs::read_to_string("../../data/rfc_index.txt").unwrap();
        let parsed = parse_rfc_index(&index_contents).unwrap();

        assert_eq!(parsed, vec!["0001 Host Software. S. Crocker. April 1969. (Format: TXT, HTML) (Status:\n     UNKNOWN) (DOI: 10.17487/RFC0001) ", "0002 Host software. B. Duvall. April 1969. (Format: TXT, PDF, HTML)\n     (Status: UNKNOWN) (DOI: 10.17487/RFC0002) ", ""]);
    }
}
