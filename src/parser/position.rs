use super::{BaseParser, BaseParserOnce};
use crate::error::Result;
use crate::stream::Stream;
#[cfg(not(feature = "std"))]
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::marker::PhantomData;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Position<I>(PhantomData<I>);

pub fn position<I>() -> Position<I>
where
    I: Iterator,
{
    Position(PhantomData)
}

impl<I> BaseParser for Position<I>
where
    I: Iterator,
{
    type Iter = I;
    type Output = usize;

    fn parse_iter(&self, input: &mut Stream<Self::Iter>) -> Result<Self::Output> {
        Ok(input.pos())
    }
}

impl<I> BaseParserOnce for Position<I>
where
    I: Iterator,
{
    type Iter = I;
    type Output = usize;

    fn parse_iter_once(self, input: &mut Stream<Self::Iter>) -> Result<Self::Output> {
        self.parse_iter(input)
    }
}
