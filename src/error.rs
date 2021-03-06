#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use core::fmt;
#[cfg(feature = "std")]
use std::fmt;

/// The Result type for this crate.
#[cfg(feature = "std")]
pub type Result<T> = std::result::Result<T, ParseError>;
/// The Result type for this crate.
#[cfg(not(feature = "std"))]
pub type Result<T> = core::result::Result<T, ParseError>;

/// An enum expresses an parse error.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    /// Expected something at this position.
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

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}
