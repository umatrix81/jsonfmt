# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

jsonfmt is a command-line utility that reads JSON from the system clipboard, formats it with proper indentation, and outputs to console. With any command-line argument, it also saves the formatted JSON to `formatted.json`.

## Architecture

Single-file binary application:
- `src/main.rs` - entire application logic
- Dependencies: `arboard` (clipboard), `serde_json` (JSON handling)
- Error messages are in Russian

Key functions:
- `main()` - orchestrates clipboard reading, JSON parsing, and output
- `print2console()` - displays formatted JSON
- `save2file()` - writes to formatted.json file

## Development Commands

Build and run:
```bash
cargo run              # Format clipboard JSON to console
cargo run -- save      # Format and save to formatted.json
cargo build --release  # Optimized build (stripped, size-optimized)
```

Code quality:
```bash
cargo fmt     # Format code
cargo clippy  # Run linter
cargo check   # Check compilation
cargo test    # Run tests
```

## Release Configuration

The release profile is heavily optimized for binary size:
- Strip symbols enabled
- Size optimization (opt-level = "z")
- Link-time optimization enabled
- Single codegen unit
- Abort on panic
