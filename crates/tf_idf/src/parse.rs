use std::num::ParseIntError;

use crate::error::{RFSeeError, RFSeeResult};

const RFC_DELIMITER: &str = "\n\n";

/// Parse raw `String` contents of RFC index and return `Vec` of `&str` for each item after
/// splitting on `RFC_DELIMITER`
pub fn parse_rfc_index(content: &str) -> RFSeeResult<Vec<&str>> {
    let found = content.find("0001");
    match found {
        Some(idx) => {
            let raw_rfcs = &content[idx..];
            let splitted = raw_rfcs.split(RFC_DELIMITER).collect();
            Ok(splitted)
        }
        None => Err(RFSeeError::ParseError(
            "Unable to parse RFC index".to_string(),
        )),
    }
}

/// Parse raw RFC `String` contents into the RFC number and its title
pub fn parse_rfc_details(rfc_content: &str) -> RFSeeResult<(i32, &str)> {
    if let Some((rfc_num, title)) = rfc_content.split_once(" ") {
        let parsed_num: i32 = rfc_num
            .parse()
            .map_err(|e: ParseIntError| RFSeeError::ParseError(e.to_string()))?;
        Ok((parsed_num, title))
    } else {
        Err(RFSeeError::ParseError(
            "Unable to parse RFC number {rfc_content}".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_rfc_details, parse_rfc_index};

    #[test]
    fn test_parse_index() {
        let index_contents = std::fs::read_to_string("../../data/rfc_index.txt").unwrap();
        let parsed = parse_rfc_index(&index_contents).unwrap();

        assert_eq!(parsed, vec!["0001 Host Software. S. Crocker. April 1969. (Format: TXT, HTML) (Status:\n     UNKNOWN) (DOI: 10.17487/RFC0001) ", "0002 Host software. B. Duvall. April 1969. (Format: TXT, PDF, HTML)\n     (Status: UNKNOWN) (DOI: 10.17487/RFC0002) ", ""]);
    }

    #[test]
    fn test_parse_rfc_index_and_details() {
        let index_contents = std::fs::read_to_string("../../data/rfc_index.txt").unwrap();
        let parsed = parse_rfc_index(&index_contents).unwrap();
        let first = parsed.first().unwrap();
        let (num, title) = parse_rfc_details(first).unwrap();
        assert_eq!(num, 1);
        assert_eq!(title, "Host Software. S. Crocker. April 1969. (Format: TXT, HTML) (Status:\n     UNKNOWN) (DOI: 10.17487/RFC0001) ");
    }
}
