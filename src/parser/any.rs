use super::{BaseParser, BaseParserOnce};
use crate::error::{ParseError, Result};
use crate::stream::Stream;

#[cfg(not(feature = "std"))]
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::marker::PhantomData;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::string::String;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Any<I: Clone>(PhantomData<I>);

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
                #[cfg(feature = "alloc")]
                expected: String::from("something"),
            }),
        }
    }
}

impl<I: Clone> BaseParserOnce for Any<I> {
    type Item = I;
    type Output = I;

    fn parse_iter_once(self, input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        self.parse_iter(input)
    }
}
