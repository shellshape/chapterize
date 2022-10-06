use chrono::Duration;
use clap::{command, Parser};
use edl::entry::Entry;
use std::fmt::Write as FmtWrite;
use std::io;
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
    /// When not specified, input will be read from STDIN.
    #[arg()]
    input: Option<String>,

    /// The output TXT file.
    /// When not specified, output will be printed to STDOUT.
    #[arg(short, long)]
    output: Option<String>,

    /// The frame rate (FPS) of the timeline.
    #[arg(short, long, default_value = "60")]
    frame_rate: f32,

    /// Color filter.
    #[arg(short, long)]
    color_filter: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut f: Box<dyn Read> = if let Some(input_file) = args.input {
        Box::new(fs::File::open(Path::new(&input_file))?)
    } else {
        Box::new(io::stdin())
    };

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

    if let Some(output_file) = args.output {
        let mut out = fs::File::create(Path::new(&output_file))?;
        wtite_entries(&entries, &mut out)?;
        println!("{output_file} has been generated.");
    } else {
        wtite_entries(&entries, &mut io::stdout())?;
    }

    Ok(())
}

fn wtite_entries(entries: &Vec<&Entry>, w: &mut impl Write) -> io::Result<()> {
    let with_hours =
        entries.last().is_some() && entries.last().unwrap().timestamp >= Duration::hours(1);

    let def_name = String::from("-");

    for e in entries {
        let name = e.name.as_ref().unwrap_or(&def_name);
        writeln!(w, "{} {}", to_timestamp(e.timestamp, with_hours), name)?;
    }

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
