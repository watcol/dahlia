use super::BaseParser;
use crate::error::Result;
use crate::stream::Stream;
#[cfg(not(feature = "std"))]
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::marker::PhantomData;

/// A parser consumes no input, returns a value.
///
/// See `value()`.
pub struct Value<T: Clone, I: Clone>(T, PhantomData<I>);

/// A parser consumes no input, returns a value.
///
/// Note that `value` will be cloned per parsing.
pub fn value<T: Clone, I: Clone>(value: T) -> Value<T, I> {
    Value(value, PhantomData)
}

impl<T: Clone, I: Clone> BaseParser for Value<T, I> {
    type Item = I;
    type Output = T;

    fn parse_iter(&self, _input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        Ok(self.0.clone())
    }
}
