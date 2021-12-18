use super::{BaseParser, BaseParserOnce};
use crate::error::Result;
use crate::stream::Stream;
#[cfg(not(feature = "std"))]
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::marker::PhantomData;

pub struct Value<T, I>(T, PhantomData<I>);

pub fn value<T, I>(value: T) -> Value<T, I>
where
    I: Iterator,
{
    Value(value, PhantomData)
}

impl<T: Copy, I> BaseParser for Value<T, I>
where
    I: Iterator,
{
    type Iter = I;
    type Output = T;

    fn parse_iter(&self, _input: &mut Stream<Self::Iter>) -> Result<Self::Output> {
        Ok(self.0)
    }
}

impl<T, I> BaseParserOnce for Value<T, I>
where
    I: Iterator,
{
    type Iter = I;
    type Output = T;

    fn parse_iter_once(self, _input: &mut Stream<Self::Iter>) -> Result<Self::Output> {
        Ok(self.0)
    }
}

pub struct ValueClone<T, I>(T, PhantomData<I>);

pub fn value_clone<T, I>(value: T) -> ValueClone<T, I>
where
    I: Iterator,
{
    ValueClone(value, PhantomData)
}

impl<T: Clone, I> BaseParser for ValueClone<T, I>
where
    I: Iterator,
{
    type Iter = I;
    type Output = T;

    fn parse_iter(&self, _input: &mut Stream<Self::Iter>) -> Result<Self::Output> {
        Ok(self.0.clone())
    }
}

impl<T, I> BaseParserOnce for ValueClone<T, I>
where
    I: Iterator,
{
    type Iter = I;
    type Output = T;

    fn parse_iter_once(self, _input: &mut Stream<Self::Iter>) -> Result<Self::Output> {
        Ok(self.0)
    }
}
