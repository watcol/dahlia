use super::BaseParser;
use crate::error::Result;
use crate::stream::Stream;
#[cfg(not(feature = "std"))]
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::marker::PhantomData;

/// A parser consumes no input, returns current position.
///
/// See `position()`.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Position<I: Clone>(PhantomData<I>);

/// A parser consumes no input, returns current position.
pub fn position<I: Clone>() -> Position<I> {
    Position(PhantomData)
}

impl<I: Clone> BaseParser for Position<I> {
    type Item = I;
    type Output = usize;

    fn parse_iter(&self, input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        Ok(input.pos())
    }
}
