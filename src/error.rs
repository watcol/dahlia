use std::fmt;

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    Expected { position: usize, expected: String },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Expected { expected, .. } => {
                write!(f, "expected {}", expected)
            }
        }
    }
}

impl std::error::Error for ParseError {}
