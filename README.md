# jsonfmt

Command-line utility that reads JSON from the system clipboard, formats it, and optionally applies jq-like filters.

## Usage

```bash
# Pretty-print clipboard JSON
jsonfmt

# Apply a jq filter
jsonfmt '.name'
jsonfmt '.items[]'
jsonfmt '.users[] | select(.active)'
jsonfmt '.items | length'
```

## Install

```bash
cargo install --path .
```

## Build

```bash
cargo build --release
```

## Dependencies

- [arboard](https://crates.io/crates/arboard) — clipboard access
- [jaq-core](https://crates.io/crates/jaq-core), [jaq-std](https://crates.io/crates/jaq-std), [jaq-json](https://crates.io/crates/jaq-json) — jq filter engine
- [serde_json](https://crates.io/crates/serde_json) — JSON pretty-printing
