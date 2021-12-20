use super::{BaseParser, BaseParserOnce};
use crate::error::Result;
use crate::stream::Stream;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

/// Dynamically dispatched type which implements `BaseParser`.
pub type BoxedParser<'a, I, O> = Box<&'a dyn BaseParser<Item = I, Output = O>>;

/// Dynamically dispatched reference type which implements `BaseParserOnce`.
pub type BoxedParserOnce<'a, I, O> = Box<dyn BaseParserOnce<Item = I, Output = O> + 'a>;

impl<P: BaseParser> BaseParser for Box<P> {
    type Item = P::Item;
    type Output = P::Output;

    fn parse_iter(&self, input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        (**self).parse_iter(input)
    }
}

impl<P: BaseParserOnce> BaseParserOnce for Box<P> {
    type Item = P::Item;
    type Output = P::Output;

    fn parse_iter_once(self, input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        (*self).parse_iter_once(input)
    }
}
