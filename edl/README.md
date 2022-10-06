# EDL

A very simple library to parse EDL (edit decision list) files.

## Usage

Add the edl crate to your `Cargo.toml`.
```toml
[dependencies]
edl = "1"
```

```rust
let mut f = fs::File::open(Path::new("timeline.edl")).unwrap();

let mut data = String::new();
f.read_to_string(&mut data).unwrap();

let mut entries = edl::parser::parse(&data, 60)?;
entries.sort_by_key(|e| e.index);
```

**Attention**  
Any read EDL contents passed to the `parse` function must have
CRLF line endings!