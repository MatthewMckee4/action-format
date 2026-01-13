/// Configuration for the formatter.
#[derive(Debug, Clone)]
pub struct FormatterConfig {
    /// Number of spaces for indentation (default: 2)
    pub indent_size: usize,
    /// Whether to add blank lines between steps (default: true)
    pub separate_steps: bool,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            indent_size: 2,
            separate_steps: true,
        }
    }
}
