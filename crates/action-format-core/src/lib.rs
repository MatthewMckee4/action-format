mod config;
mod formatter;
mod parser;

pub use config::FormatterConfig;
pub use formatter::{format_file, format_string};
pub use parser::FormatError;
