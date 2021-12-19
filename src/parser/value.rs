use super::{BaseParser, BaseParserOnce};
use crate::error::Result;
use crate::stream::Stream;
#[cfg(not(feature = "std"))]
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::marker::PhantomData;

pub struct Value<T, I: Clone>(T, PhantomData<I>);

pub fn value<T, I: Clone>(value: T) -> Value<T, I> {
    Value(value, PhantomData)
}

impl<T: Copy, I: Clone> BaseParser for Value<T, I> {
    type Item = I;
    type Output = T;

    fn parse_iter(&self, _input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        Ok(self.0)
    }
}

impl<T, I: Clone> BaseParserOnce for Value<T, I> {
    type Item = I;
    type Output = T;

    fn parse_iter_once(self, _input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        Ok(self.0)
    }
}

pub struct ValueClone<T, I: Clone>(T, PhantomData<I>);

pub fn value_clone<T, I: Clone>(value: T) -> ValueClone<T, I> {
    ValueClone(value, PhantomData)
}

impl<T: Clone, I: Clone> BaseParser for ValueClone<T, I> {
    type Item = I;
    type Output = T;

    fn parse_iter(&self, _input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        Ok(self.0.clone())
    }
}

impl<T, I: Clone> BaseParserOnce for ValueClone<T, I> {
    type Item = I;
    type Output = T;

    fn parse_iter_once(self, _input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        Ok(self.0)
    }
}
