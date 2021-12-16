use super::Parser;
use crate::error::Result;
use crate::stream::Stream;
use std::marker::PhantomData;

pub struct Value<I, T: Clone>(T, PhantomData<I>);

pub fn value<I, T: Clone>(value: T) -> Value<I, T> {
    Value(value, PhantomData)
}

impl<I, T: Clone> Parser for Value<I, T> {
    type Item = I;
    type Output = T;

    fn parse_at<S>(&self, _input: &mut Stream<S>) -> Result<Self::Output>
    where
        S: Iterator<Item = Self::Item>,
    {
        Ok(self.0.clone())
    }
}
