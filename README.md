# action-format

[![codecov](https://codecov.io/gh/MatthewMckee4/action-format/graph/badge.svg?token=TDAWCJCVJ4)](https://codecov.io/gh/MatthewMckee4/action-format)

A fast GitHub Actions workflow formatter written in Rust.

## Features

- **2-space indentation** - Normalizes all indentation to 2 spaces
- **Step separation** - Adds blank lines between workflow steps for readability
- **Comment preservation** - Keeps your comments exactly where they are
- **Fast** - Formats 500-line workflows in under 100ms

## Installation

```shell
# macOS and Linux
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/MatthewMckee4/action-format/releases/0.0.0-alpha.0/download/action-format-installer.sh | sh
```

```shell
# Windows
powershell -ExecutionPolicy Bypass -c "irm https://github.com/MatthewMckee4/action-format/releases/0.0.0-alpha.0/download/action-format-installer.ps1 | iex"
```

## Usage

Run from your repository root. The tool automatically finds and formats all YAML files in `.github/workflows/`.

```shell
# Format all workflows
action-format

# Check without modifying
action-format --check

# Show diff
action-format --diff
```
