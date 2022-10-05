#[derive(Debug)]
pub enum Error {
    NoEntries,
    InvalidEntryFormat,
    InvalidIndexFormat,
    InvalidTimestamp(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoEntries => write!(f, "No entries"),
            Error::InvalidEntryFormat => write!(f, "Invalid entry format"),
            Error::InvalidIndexFormat => write!(f, "Invalid index format"),
            Error::InvalidTimestamp(v) => write!(f, "Invalid timestamp: {v}"),
        }
    }
}
