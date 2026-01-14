use std::path::Path;

use serde::Deserialize;

/// Configuration for the formatter.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct FormatterConfig {
    /// Number of spaces for indentation (default: 2)
    pub indent_size: usize,
    /// Whether to add blank lines between steps (default: true)
    pub separate_steps: bool,
    /// Whether to add blank lines between jobs (default: true)
    pub separate_jobs: bool,
    /// Files to ignore (can be full paths like `.github/workflows/ci.yml` or just filenames like `ci.yml`)
    pub ignore: Vec<String>,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            indent_size: 2,
            separate_steps: true,
            separate_jobs: true,
            ignore: Vec::new(),
        }
    }
}

impl FormatterConfig {
    /// Load configuration from a TOML file, falling back to defaults if file doesn't exist.
    pub fn from_file(path: &Path) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path).map_err(|e| ConfigError::Read {
            path: path.to_path_buf(),
            source: e,
        })?;

        toml::from_str(&content).map_err(|e| ConfigError::Parse {
            path: path.to_path_buf(),
            source: e,
        })
    }

    /// Check if a file should be ignored based on the ignore patterns.
    pub fn should_ignore(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        let file_name = path.file_name().and_then(|n| n.to_str());

        self.ignore.iter().any(|pattern| {
            // Match full path
            path_str == *pattern
                || path_str.ends_with(pattern)
                // Match just the filename
                || file_name.is_some_and(|name| name == pattern)
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to read config file '{path}'")]
    Read {
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse config file '{path}'")]
    Parse {
        path: std::path::PathBuf,
        #[source]
        source: toml::de::Error,
    },
}
