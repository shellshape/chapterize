# chapterize  [![release](https://github.com/zekroTJA/chapterize/actions/workflows/cd.yml/badge.svg)](https://github.com/zekroTJA/chapterize/actions/workflows/cd.yml) [![GitHub release (latest by date)](https://img.shields.io/github/v/release/zekroTJA/chapterize)](https://github.com/zekroTJA/chapterize/releases) [![Crates.io](https://img.shields.io/crates/v/edl)](https://crates.io/edl) [![docs.rs](https://img.shields.io/docsrs/edl)](https://docs.rs/edl/latest/edl)

A CLI tool to convert EDL marker files (for example from DaVinci Resolve) to YouTube video timestamps.

## Usage

```
❯ chapterize --help
Parse EDLs to generate YouTube timestamps.

Usage: chapterize [OPTIONS] [INPUT]

Arguments:
  [INPUT]  The input EDL file. When not specified, input will be read from STDIN

Options:
  -o, --output <OUTPUT>
          The output TXT file. When not specified, output will be printed to STDOUT
  -f, --frame-rate <FRAME_RATE>
          The frame rate (FPS) of the timeline [default: 60]
  -c, --color-filter <COLOR_FILTER>
          Color filter
  -h, --help
          Print help information
  -V, --version
          Print version information
```

You can export your timeline markers in DaVinci Resolve by right-clicking your timeline, navigating to `Timelines` → `Export` → `Timeline Markers to EDL...`.

![image](https://user-images.githubusercontent.com/16734205/194168141-cbcdf1be-a9ed-4e27-9c8b-3c013f793f80.png)

Then simply use the CLI to convert the markers to timestamps. Let's take the following EDL export as example.

> timeline.edl
```edl
TITLE: Example Video
FCM: NON-DROP FRAME

001  001      V     C        00:00:00:00 00:00:00:01 00:00:00:00 00:00:00:01  
 |C:ResolveColorBlue |M:Intro |D:1

002  001      V     C        00:02:10:15 00:02:10:16 00:02:10:15 00:02:10:16  
 |C:ResolveColorBlue |M:Stuff |D:1

003  001      V     C        00:01:03:54 00:02:10:16 00:02:10:15 00:02:10:16  
 |C:ResolveColorGreen |M:Don't export this! |D:1

004  001      V     C        00:04:32:13 00:04:32:14 00:04:32:13 00:04:32:14  
 |C:ResolveColorBlue |M:Outro |D:1
```

Now, you can execute the tool with the following arguments.
```
chapterize timeline.edl \
    -c ResolveColorBlue \
    -f 60 \
    -o timeline.txt
```

The result will look like following.

> timeline.txt
```
00:00 Intro
02:10 Stuff
04:32 Outro
```