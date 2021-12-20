#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

use super::BaseParser;
use crate::{Result, Stream};

pub struct Function<I: Clone, O>(Box<dyn Fn(&mut Stream<I>) -> Result<O>>);

pub fn function<I: Clone, O, F>(f: F) -> Function<I, O>
where
    F: Fn(&mut Stream<I>) -> Result<O>,
{
    function(Box::new(f))
}

impl<I: Clone, O> BaseParser for Function<I, O> {
    type Item = I;
    type Output = O;

    fn parse_iter(&self, input: &mut Stream<I>) -> Result<O> {
        (self.0)(input)
    }
}
