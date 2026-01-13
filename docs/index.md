# action-format

A fast GitHub Actions workflow formatter.

## Features

- Consistent 2-space indentation normalization
- Blank line separation between workflow steps
- Preserves comments, anchors, and YAML semantics
- Fast execution (<100ms for typical workflows)

## Installation

```shell
# macOS and Linux
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/MatthewMckee4/action-format/releases/latest/download/action-format-installer.sh | sh
```

```shell
# Windows
powershell -ExecutionPolicy Bypass -c "irm https://github.com/MatthewMckee4/action-format/releases/latest/download/action-format-installer.ps1 | iex"
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
