use crate::error::Result;
use crate::parser::BaseParser;
use crate::stream::Stream;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::boxed::Box;

pub type BoxedParser<'a, I, O> = Box<dyn BaseParser<Iter = I, Output = O> + 'a>;

impl<'a, I, O> BaseParser for BoxedParser<'a, I, O>
where
    I: Iterator,
{
    type Iter = I;
    type Output = O;

    fn parse_iter(&self, input: &mut Stream<Self::Iter>) -> Result<Self::Output> {
        (**self).parse_iter(input)
    }
}

pub type RefBoxedParser<'a, I, O> = Box<&'a dyn BaseParser<Iter = I, Output = O>>;

impl<'a, I, O> BaseParser for RefBoxedParser<'a, I, O>
where
    I: Iterator,
{
    type Iter = I;
    type Output = O;

    fn parse_iter(&self, input: &mut Stream<Self::Iter>) -> Result<Self::Output> {
        (**self).parse_iter(input)
    }
}
