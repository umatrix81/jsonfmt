# Project Overview

## Purpose
jsonfmt is a simple command-line utility that formats JSON from the clipboard. It reads JSON text from the system clipboard, pretty-prints it, and can optionally save it to a file.

## Tech Stack
- Language: Rust (2021 edition)
- Dependencies:
  - arboard 3.4.1 - clipboard access
  - serde_json 1.0.137 - JSON parsing and formatting

## Project Structure
- Single binary application with main.rs in src/
- No library components or modules
- Simple single-file architecture

## Behavior
- Default: reads clipboard, pretty-prints JSON to console
- With any command-line argument: also saves to formatted.json file
- Error messages in Russian