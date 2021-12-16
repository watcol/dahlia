use super::{Parser, ParserOnce};
use crate::error::Result;
use crate::stream::Stream;
#[cfg(not(feature = "std"))]
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::marker::PhantomData;

pub struct Position<I>(PhantomData<I>);

pub fn position<I>() -> Position<I> {
    Position(PhantomData)
}

impl<I> Parser for Position<I> {
    type Item = I;
    type Output = usize;

    fn parse_iter<S>(&self, input: &mut Stream<S>) -> Result<Self::Output>
    where
        S: Iterator<Item = Self::Item>,
    {
        Ok(input.pos())
    }
}

impl<I> ParserOnce for Position<I> {
    type Item = I;
    type Output = usize;

    fn parse_iter_once<S>(self, input: &mut Stream<S>) -> Result<Self::Output>
    where
        S: Iterator<Item = Self::Item>,
    {
        self.parse_iter(input)
    }
}
