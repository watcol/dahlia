use super::BaseParser;
use crate::error::{ParseError, Result};
use crate::stream::Stream;

#[cfg(not(feature = "std"))]
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::marker::PhantomData;

#[cfg(not(feature = "std"))]
use alloc::string::String;

/// A parser consumes one item.
///
/// See `any()`.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Any<I: Clone>(PhantomData<I>);

/// A parser consumes one item.
///
/// This parser consume one item unless it reaches EOF.
pub fn any<I: Clone>() -> Any<I> {
    Any(PhantomData)
}

impl<I: Clone> BaseParser for Any<I> {
    type Item = I;
    type Output = I;

    fn parse_iter(&self, input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        match input.next() {
            Some(i) => Ok(i),
            None => Err(ParseError::Expected {
                position: input.pos(),
                expected: String::from("something"),
            }),
        }
    }
}
