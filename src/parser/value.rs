use std::marker::PhantomData;

use super::Parser;
use crate::error::Result;

pub struct Value<I, T: Clone>(T, PhantomData<I>);

pub fn value<I, T: Clone>(value: T) -> Value<I, T> {
    Value(value, PhantomData)
}

impl<'a, I, T: Clone> Parser<'a> for Value<I, T> {
    type Item = I;
    type Output = T;

    fn parse_at(&self, _input: &'a [I], start: usize) -> Result<(T, usize)> {
        Ok((self.0.clone(), start))
    }
}
