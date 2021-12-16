use super::{Parser, ParserOnce};
use crate::error::{ParseError, Result};
use crate::stream::Stream;
use std::marker::PhantomData;

pub struct Any<I>(PhantomData<I>);

pub fn any<I>() -> Any<I> {
    Any(PhantomData)
}

impl<I> Parser for Any<I> {
    type Item = I;
    type Output = I;

    fn parse_iter<S>(&self, input: &mut Stream<S>) -> Result<Self::Output>
    where
        S: Iterator<Item = Self::Item>,
    {
        match input.next() {
            Some(i) => Ok(i),
            None => Err(ParseError::Expected {
                position: input.pos(),
                expected: String::from("something"),
            }),
        }
    }
}

impl<I> ParserOnce for Any<I> {
    type Item = I;
    type Output = I;

    fn parse_iter_once<S>(self, input: &mut Stream<S>) -> Result<Self::Output>
    where
        S: Iterator<Item = Self::Item>,
    {
        self.parse_iter(input)
    }
}
