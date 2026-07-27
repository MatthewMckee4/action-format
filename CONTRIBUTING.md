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

## Release Process

Run the `Prepare release` workflow with the version bump to perform, such as `alpha` or an
explicit version. The workflow runs `seal bump <version>` and opens the release pull request.

To prepare a release locally, install [`seal`](https://github.com/MatthewMckee4/seal), then run:

```shell
seal bump alpha
seal bump <version>
```

Seal creates the release branch, commits and pushes the changes, and opens a pull request. Merge
the pull request, then run the `Release` workflow to publish that version.

## Documentation

To build the documentation locally:

```shell
uv run --isolated --with-requirements docs/requirements.txt zensical serve
```
