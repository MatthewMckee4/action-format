use thiserror::Error;

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid indentation at line {line}: mixed tabs and spaces")]
    MixedIndentation { line: usize },
}
