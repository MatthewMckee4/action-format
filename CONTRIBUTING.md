# Contributing

## Setup

[Rust](https://rustup.rs/) is required to build and work on the project.

## Development

```shell
# Build
cargo build

# Test
cargo test

# Format
cargo fmt

# Run (from a repo with .github/workflows)
cargo run -p action-format
```

## Testing

For running tests, we recommend [nextest](https://nexte.st/).

```shell
cargo nextest run
```

### Snapshot testing

We use [insta](https://insta.rs/) for snapshot testing. Install `cargo-insta` for a better review experience.

```shell
cargo test
cargo insta review
```

## Documentation

To build the documentation locally:

```shell
uv run --isolated --with-requirements docs/requirements.txt zensical serve
```
