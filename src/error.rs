#[cfg(not(feature = "std"))]
use core::fmt;
#[cfg(feature = "std")]
use std::fmt;

pub type Result<T> = core::result::Result<T, ParseError>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    Expected {
        position: usize,
        #[cfg(feature = "std")]
        expected: String,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            #[cfg(feature = "std")]
            Self::Expected { expected, .. } => {
                write!(f, "expected {}", expected)
            }
            #[cfg(not(feature = "std"))]
            Self::Expected { .. } => {
                write!(f, "Input Mismatched")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}
