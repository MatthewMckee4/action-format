# CLAUDE.md

This file provides guidance to Claude Code when working with this repository.

## Project Overview

action-format is a GitHub Actions workflow formatter written in Rust. Cargo workspace with:
- `crates/action-format` - CLI binary
- `crates/action-format-cli` - CLI argument parsing
- `crates/action-format-core` - Core formatting logic

## Style

Do not add too many comments. Only add comments where necessary, if the code is complicated and cannot be simplified.

## Development Commands

```shell
cargo build
cargo test
cargo fmt
cargo run -p action-format
```

## Architecture

### Core Library (action-format-core)

- `lib.rs` - Public API exports
- `config.rs` - Formatter configuration
- `formatter.rs` - Core formatting logic (line-based parsing)
- `parser.rs` - Error types

### CLI Library (action-format-cli)

- `lib.rs` - CLI argument structs with clap

### CLI Binary (action-format)

- `main.rs` - Entry point, finds workflows in `.github/workflows`
- `printer.rs` - Output abstraction (stdout/stderr with quiet mode)

### Formatting Approach

Uses line-based parsing rather than a full YAML parser to preserve comments naturally. Tracks indentation levels, detects `steps:` sections, and normalizes accordingly.

## Code Conventions

- Edition 2024, MSRV 1.90
- Strict clippy pedantic enabled
- Use Printer abstraction instead of direct print! calls
