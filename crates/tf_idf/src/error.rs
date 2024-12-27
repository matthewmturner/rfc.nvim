use std::error::Error;

#[derive(Debug)]
pub enum RFSeeError {
    ParseError(String),
    FetchError(String),
    IOError(String),
    RuntimeError(String),
}

impl std::fmt::Display for RFSeeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            RFSeeError::ParseError(m) => format!("ParseError: {m}"),
            RFSeeError::FetchError(m) => format!("FetchError: {m}"),
            RFSeeError::IOError(m) => format!("IOError: {m}"),
            RFSeeError::RuntimeError(m) => format!("RuntimeError: {m}"),
        };
        writeln!(f, "RfSeeError: {msg}")
    }
}

impl Error for RFSeeError {}

pub type RFSeeResult<T> = Result<T, RFSeeError>;
