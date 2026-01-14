# Configuration

action-format can be configured using a TOML file at `.github/action-format.toml`.

## Configuration File

Create a file at `.github/action-format.toml` in your repository:

```toml
# .github/action-format.toml

# Number of spaces for indentation (default: 2)
indent_size = 2

# Add blank lines between steps (default: true)
separate_steps = true

# Add blank lines between jobs (default: true)
separate_jobs = true

# Files to ignore
ignore = []
```

All options are optional. If not specified, the default values are used.

## Options

### `indent_size`

Number of spaces to use for indentation.

**Default:** `2`

```toml
indent_size = 4
```

### `separate_steps`

Whether to add blank lines between steps in a job.

**Default:** `true`

```toml
separate_steps = false
```

### `separate_jobs`

Whether to add blank lines between jobs in a workflow.

**Default:** `true`

```toml
separate_jobs = false
```

### `ignore`

List of files to ignore. You can specify files by:

- **Filename only:** `"ci.yml"` - matches any file with this name
- **Full path:** `".github/workflows/ci.yml"` - matches the exact path

**Default:** `[]`

```toml
# Ignore specific files
ignore = [
    "release.yml",
    ".github/workflows/generated.yml",
]
```

## Examples

### Disable step separation

```toml
separate_steps = false
```

### Use 4-space indentation

```toml
indent_size = 4
```

### Ignore auto-generated workflows

```toml
ignore = ["release.yml", "generated.yml"]
```

### Minimal formatting

```toml
separate_steps = false
separate_jobs = false
```
