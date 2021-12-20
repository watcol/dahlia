//! Variable parsers and combinators.

// combinators
mod boxed;

pub use boxed::BoxedParser;

// parsers
mod any;
mod condition;
mod position;
mod value;

pub use any::{any, Any};
pub use condition::{is, is_not, Condition};
pub use position::{position, Position};
pub use value::{value, Value};

use crate::error::{ParseError, Result};
use crate::stream::Stream;

#[cfg(not(feature = "std"))]
use alloc::string::String;

/// A parser which can be reused.
///
/// By implementing `BaseParser`, the type will be a parser which refers an input and
/// returns an output. `Parser` trait will be implemented automatically to provide more
/// functions. This trait is not required (`use` is unneeded) for basic usage.
pub trait BaseParser {
    /// The type of items that the input will provide. (normally `u8` or `char`.)
    type Item: Clone;
    /// The type of output that the parser will provide.
    type Output;

    /// Parses an input stream and returns an output or a parsing error.
    ///
    /// > This function is for implementors, users should use `parse` or `parse_complete` from
    /// > `Parser` trait.
    fn parse_iter(&self, input: &mut Stream<Self::Item>) -> Result<Self::Output>;
}

impl<T: BaseParser> Parser for T {}

/// An extention of `BaseParser`.
///
/// `Parser` is a trait that will implemented automatically for implementors of
/// `BaseParser` to extend functions. They are separated to implement [object
/// safety](https://rust-lang.github.io/rfcs/0255-object-safety.html).
pub trait Parser: BaseParser {
    /// Puts the reference of myself into the `Box`.
    ///
    /// This function converts myself to `Box<dyn BaseParser>`, so it's useful when type
    /// matching or describing the type of `once_cell` constant.
    #[cfg(feature = "std")]
    fn boxed(&self) -> boxed::BoxedParser<'_, Self::Item, Self::Output>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    /// Parse the input.
    ///
    /// This function will parse an input (`Vec<Item>`, `&[Item]`, `String` or `&str`
    /// (if `Item` == `char`) are allowed.) and returns output and remained input.
    fn parse<I, B>(&self, input: I) -> Result<(Self::Output, B)>
    where
        I: Into<Stream<Self::Item>>,
        B: FromIterator<Self::Item>,
    {
        let mut input = input.into();
        let output = self.parse_iter(&mut input)?;
        Ok((output, input.collect()))
    }

    /// Parse the input completely.
    ///
    /// This function will parse an input (`Vec<Item>`, `&[Item]`, `String` or `&str`
    /// (if `Item` == `char`) are allowed.) and returns output. Unlike `parse`,
    /// this will fail if the input remains at least one item.
    fn parse_complete<I>(&self, input: I) -> Result<Self::Output>
    where
        I: Into<Stream<Self::Item>>,
    {
        let mut input = input.into();
        let output = self.parse_iter(&mut input)?;
        match input.next() {
            Some(_) => Err(ParseError::Expected {
                position: input.pos(),
                expected: String::from("EOF"),
            }),
            None => Ok(output),
        }
    }
}
