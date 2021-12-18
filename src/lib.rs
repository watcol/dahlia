#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

pub mod error;
pub mod parser;
pub mod stream;

pub use parser::{any, position, value, value_clone, Parser, ParserOnce};

#[cfg(feature = "alloc")]
pub use parser::{is, is_not, is_not_once, is_once};
