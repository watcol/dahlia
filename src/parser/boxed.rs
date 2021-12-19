use crate::error::Result;
use crate::parser::BaseParser;
use crate::stream::Stream;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::boxed::Box;

pub type BoxedParser<'a, I, O> = Box<dyn BaseParser<Item = I, Output = O> + 'a>;

impl<'a, I, O> BaseParser for BoxedParser<'a, I, O>
where
    I: Clone,
{
    type Item = I;
    type Output = O;

    fn parse_iter(&self, input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        (**self).parse_iter(input)
    }
}

pub type RefBoxedParser<'a, I, O> = Box<&'a dyn BaseParser<Item = I, Output = O>>;

impl<'a, I, O> BaseParser for RefBoxedParser<'a, I, O>
where
    I: Clone,
{
    type Item = I;
    type Output = O;

    fn parse_iter(&self, input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        (**self).parse_iter(input)
    }
}
