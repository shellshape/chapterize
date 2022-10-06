use crate::{entry::Entry, errors::Error};
use chrono::Duration;

/// Parses the passed EDL encoded content into a vector
/// of entries.
///
/// ```rust
/// let mut f = fs::File::open(Path::new("timeline.edl")).unwrap();
///
/// let mut data = String::new();
/// f.read_to_string(&mut data).unwrap();
///
/// let mut entries = edl::parser::parse(&data, 60)?;
/// ```
pub fn parse(data: &str, frame_rate: u32) -> Result<Vec<Entry>, Error> {
    let lines: Vec<&str> = data.split("\r\n\r\n").collect();
    if lines.len() < 2 {
        return Err(Error::NoEntries);
    }

    let end = if (*lines.last().unwrap()).is_empty() {
        lines.len() - 1
    } else {
        lines.len()
    };

    let lines = lines[1..end]
        .to_vec()
        .iter()
        .map(|line| scan_entry(line, frame_rate))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(lines)
}

fn scan_entry(line: &str, frame_rate: u32) -> Result<Entry, Error> {
    let lines: Vec<&str> = line.split("\r\n").collect();
    if lines.len() != 2 {
        return Err(Error::InvalidEntryFormat);
    }

    let fields: Vec<&str> = lines.first().unwrap().split_whitespace().collect();
    if fields.len() < 5 {
        return Err(Error::InvalidEntryFormat);
    }

    let index = fields[0]
        .parse::<usize>()
        .map_err(|_| Error::InvalidIndexFormat)?;

    let timestamp = parse_duration(fields[4], frame_rate)?;

    let fields: Vec<&str> = lines[1].split('|').collect();
    let description = fields.first().map(|v| v.trim().to_string());
    let color = fields
        .iter()
        .find(|v| v.starts_with("C:"))
        .map(|v| v.trim()[2..].to_string());
    let name = fields
        .iter()
        .find(|v| v.starts_with("M:"))
        .map(|v| v.trim()[2..].to_string());

    let duration = match fields
        .iter()
        .find(|v| v.starts_with("D:"))
        .map(|v| parse_duration(&v.trim()[2..], frame_rate))
    {
        Some(v) => match v {
            Ok(v) => Some(v),
            Err(e) => return Err(e),
        },
        None => None,
    };

    Ok(Entry {
        index,
        timestamp,
        duration,
        color,
        name,
        description,
    })
}

fn parse_duration(v: &str, frame_rate: u32) -> Result<Duration, Error> {
    if v.is_empty() {
        return Err(Error::InvalidTimestamp("empty".into()));
    }

    let mut parts = v
        .split(':')
        .map(|e| e.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| Error::InvalidTimestamp(err.to_string()))?;

    parts.resize(4, 0);
    parts.reverse();

    let frame_rate: i64 = frame_rate.into();
    let d = Duration::milliseconds(parts[0] * 1000 / frame_rate)
        + Duration::seconds(parts[1])
        + Duration::minutes(parts[2])
        + Duration::hours(parts[3]);

    Ok(d)
}
