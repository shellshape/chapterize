use chrono::Duration;

/// An EDL entry.
#[derive(Debug)]
pub struct Entry {
    pub index: usize,
    pub timestamp: Duration,
    pub duration: Duration,
    pub color: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}
