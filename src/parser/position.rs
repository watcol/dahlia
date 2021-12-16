use super::Parser;
use crate::error::Result;
use crate::stream::Stream;
use std::marker::PhantomData;

pub struct Position<I>(PhantomData<I>);

pub fn position<I>() -> Position<I> {
    Position(PhantomData)
}

impl<I> Parser for Position<I> {
    type Item = I;
    type Output = usize;

    fn parse_at<S>(&self, input: &mut Stream<S>) -> Result<Self::Output>
    where
        S: Iterator<Item = Self::Item>,
    {
        Ok(input.pos())
    }
}
