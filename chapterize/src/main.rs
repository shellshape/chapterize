use chrono::Duration;
use clap::{command, Parser};
use edl::entry::Entry;
use std::fmt::Write as FmtWrite;
use std::{
    error::Error,
    fs,
    io::{Read, Write},
    path::Path,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input EDL file.
    #[arg()]
    input: String,

    /// The output TXT file (defaults to input file name + .txt).
    #[arg(short, long)]
    output: Option<String>,

    /// The frame rate (FPS) of the timeline.
    #[arg(short, long, default_value = "60")]
    frame_rate: u32,

    /// Color filter.
    #[arg(short, long)]
    color_filter: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut f = fs::File::open(Path::new(&args.input))?;

    let mut data = String::new();
    f.read_to_string(&mut data)?;

    let mut entries = edl::parser::parse(&data, args.frame_rate)?;
    entries.sort_by_key(|v| v.index);

    let entries: Vec<&Entry> = entries
        .iter()
        .filter(|e| {
            args.color_filter.is_none()
                || e.color.is_some()
                    && args
                        .color_filter
                        .as_ref()
                        .unwrap()
                        .contains(e.color.as_ref().unwrap())
        })
        .collect();

    let output_file = args.output.unwrap_or_else(|| format!("{}.txt", args.input));
    let mut out = fs::File::create(Path::new(&output_file))?;

    let with_hours =
        entries.last().is_some() && entries.last().unwrap().timestamp >= Duration::hours(1);

    let def_name = String::from("-");

    for e in entries {
        let name = e.name.as_ref().unwrap_or(&def_name);
        writeln!(out, "{} {}", to_timestamp(e.timestamp, with_hours), name)?;
    }

    println!("{output_file} has been generated.");

    Ok(())
}

fn to_timestamp(d: Duration, with_hours: bool) -> String {
    let st = d.num_seconds();
    let secs = st % 60;
    let mins = (st % 3600) / 60;
    let hours = st / 3600;

    let mut res = String::new();
    if hours > 0 || with_hours {
        write!(res, "{:0>2}:", hours).unwrap();
    }
    write!(res, "{:0>2}:{:0>2}", mins, secs).unwrap();

    res
}
