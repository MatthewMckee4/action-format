mod config;
mod formatter;
mod parser;

pub use config::{ConfigError, FormatterConfig};
pub use formatter::{format_file, format_string};
pub use parser::FormatError;
