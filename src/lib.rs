//! Dhalia, a reusable parser combinator.
//!
//! Dhalia is a parser combinator implemented as a trait. Parsers implements `Parser`
//! is not consumed when parsing an input. (So parsers can be defined as an `once_cell`
//! constant.)

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod error;
mod stream;

pub use error::{ParseError, Result};
pub use stream::Stream;

pub mod parser;

pub use parser::{any, function, is, is_not, position, value, BoxedParser, Parser};
