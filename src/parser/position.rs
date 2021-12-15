use super::Parser;
use crate::error::Result;
use std::marker::PhantomData;

pub struct Position<I>(PhantomData<I>);

pub fn position<I>() -> Position<I> {
    Position(PhantomData)
}

impl<'a, I> Parser<'a> for Position<I> {
    type Item = I;
    type Output = usize;

    fn parse_at(&self, _input: &'a [I], start: usize) -> Result<(usize, usize)> {
        Ok((start, start))
    }
}
