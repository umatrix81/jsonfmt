# Code Style and Conventions

## Rust Conventions
- Edition: 2021
- Follow standard Rust naming conventions (snake_case for functions/variables)
- Use Result types with Box<dyn Error> for error handling
- Prefer explicit error handling over panics

## Current Patterns
- Static constants for configuration (e.g., FILENAME)
- Small, focused functions (save2file, print2console)
- Match expressions for argument handling
- Early returns for error cases

## Release Profile
Optimized for size with:
- strip = true
- opt-level = "z"
- lto = true
- codegen-units = 1
- panic = "abort"