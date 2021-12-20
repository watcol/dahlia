use crate::error::Result;
use crate::parser::BaseParser;
use crate::stream::Stream;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

/// Dynamically dispatched type which implements `BaseParser`.
pub type BoxedParser<'a, I, O> = Box<dyn BaseParser<Item = I, Output = O> + 'a>;

impl<'a, I: Clone, O> BaseParser for BoxedParser<'a, I, O> {
    type Item = I;
    type Output = O;

    fn parse_iter(&self, input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        (**self).parse_iter(input)
    }
}

/// Dynamically dispatched reference type which implements `BaseParser`.
pub type RefBoxedParser<'a, I, O> = Box<&'a dyn BaseParser<Item = I, Output = O>>;

impl<'a, I: Clone, O> BaseParser for RefBoxedParser<'a, I, O> {
    type Item = I;
    type Output = O;

    fn parse_iter(&self, input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        (**self).parse_iter(input)
    }
}
