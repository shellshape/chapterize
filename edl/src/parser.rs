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
pub fn parse(data: &str, frame_rate: f32) -> Result<Vec<Entry>, Error> {
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

fn scan_entry(line: &str, frame_rate: f32) -> Result<Entry, Error> {
    let lines: Vec<&str> = line.split("\r\n").collect();
    if lines.len() != 2 {
        return Err(Error::InvalidEntryFormat);
    }

    let fields: Vec<&str> = lines.first().unwrap().split_whitespace().collect();
    if fields.len() < 6 {
        return Err(Error::InvalidEntryFormat);
    }

    let index = fields[0]
        .parse::<usize>()
        .map_err(|_| Error::InvalidIndexFormat)?;

    let timestamp = parse_duration(fields[4], frame_rate)?;
    let duration = parse_duration(fields[5], frame_rate)? - timestamp;

    let fields: Vec<&str> = lines[1].split('|').collect();

    let mut description = fields.first().map(|v| v.trim().to_string());
    if let Some(desc) = description {
        description = if desc.is_empty() { None } else { Some(desc) };
    }

    let color = fields
        .iter()
        .find(|v| v.starts_with("C:"))
        .map(|v| v.trim()[2..].to_string());
    let name = fields
        .iter()
        .find(|v| v.starts_with("M:"))
        .map(|v| v.trim()[2..].to_string());

    Ok(Entry {
        index,
        timestamp,
        duration,
        color,
        name,
        description,
    })
}

fn parse_duration(v: &str, frame_rate: f32) -> Result<Duration, Error> {
    if v.is_empty() {
        return Err(Error::InvalidTimestamp("empty".into()));
    }

    let mut parts = v
        .split(':')
        .map(|e| e.parse::<f32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| Error::InvalidTimestamp(err.to_string()))?;

    parts.resize(4, 0f32);
    parts.reverse();

    println!(
        "{} => {}",
        parts[0],
        (parts[0] * 1000f32 / frame_rate) as i64
    );
    let d = Duration::microseconds((parts[0] * 1000000f32 / frame_rate).floor() as i64)
        + Duration::seconds(parts[1] as i64)
        + Duration::minutes(parts[2] as i64)
        + Duration::hours(parts[3] as i64);

    Ok(d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = &"
TITLE: Example Video\r
FCM: NON-DROP FRAME\r
\r
001  001      V     C        00:00:00:00 00:00:00:01 00:00:00:00 00:00:00:01  \r
 |C:ResolveColorBlue |M:Intro |D:1\r
\r
002  001      V     C        00:02:10:15 00:02:10:16 00:02:10:15 00:02:10:16  \r
 |C:ResolveColorBlue |M:Stuff |D:1\r
\r
003  001      V     C        00:03:03:54 00:03:03:55 00:02:10:15 00:02:10:16  \r
 |C:ResolveColorGreen |M:Don't export this! |D:1\r
\r
004  001      V     C        00:04:32:13 00:04:32:14 00:04:32:13 00:04:32:14  \r
 |C:ResolveColorBlue |M:Outro |D:1\r
\r
"[1..];

        let expected = vec![
            Entry {
                color: Some("ResolveColorBlue".into()),
                description: None,
                duration: Duration::microseconds((1000000f32 / 60f32) as i64),
                index: 1,
                name: Some("Intro".into()),
                timestamp: Duration::milliseconds(0),
            },
            Entry {
                color: Some("ResolveColorBlue".into()),
                description: None,
                duration: Duration::microseconds((1000000f32 / 60f32) as i64),
                index: 2,
                name: Some("Stuff".into()),
                timestamp: Duration::microseconds((15f32 * 1000000f32 / 60f32) as i64)
                    + Duration::seconds(10)
                    + Duration::minutes(2),
            },
            Entry {
                color: Some("ResolveColorGreen".into()),
                description: None,
                duration: Duration::microseconds((1000000f32 / 60f32) as i64),
                index: 3,
                name: Some("Don't export this!".into()),
                timestamp: Duration::microseconds((54f32 * 1000000f32 / 60f32) as i64)
                    + Duration::seconds(3)
                    + Duration::minutes(3),
            },
            Entry {
                color: Some("ResolveColorBlue".into()),
                description: None,
                duration: Duration::microseconds((1000000f32 / 60f32) as i64),
                index: 4,
                name: Some("Outro".into()),
                timestamp: Duration::microseconds((13f32 * 1000000f32 / 60f32) as i64)
                    + Duration::seconds(32)
                    + Duration::minutes(4),
            },
        ];
        let entries = parse(data, 60f32).unwrap();

        assert_eq!(entries.len(), expected.len());

        for (i, e) in entries.iter().enumerate() {
            assert_eq!(e.color, expected[i].color, "entry: {i}");
            assert_eq!(e.description, expected[i].description, "entry: {i}");
            assert_eq!(
                e.duration.num_milliseconds(),
                expected[i].duration.num_milliseconds(),
                "entry: {i}"
            );
            assert_eq!(e.index, expected[i].index, "entry: {i}");
            assert_eq!(e.name, expected[i].name, "entry: {i}");
            assert_eq!(e.timestamp, expected[i].timestamp, "entry: {i}");
        }
    }
}
