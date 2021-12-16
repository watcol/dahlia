use super::{Parser, ParserOnce};
use crate::error::Result;
use crate::stream::Stream;
#[cfg(not(feature = "std"))]
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::marker::PhantomData;

pub struct Value<I, T>(T, PhantomData<I>);

pub fn value<I, T>(value: T) -> Value<I, T> {
    Value(value, PhantomData)
}

impl<I, T: Copy> Parser for Value<I, T> {
    type Item = I;
    type Output = T;

    fn parse_iter<S>(&self, _input: &mut Stream<S>) -> Result<Self::Output>
    where
        S: Iterator<Item = Self::Item>,
    {
        Ok(self.0)
    }
}

impl<I, T> ParserOnce for Value<I, T> {
    type Item = I;
    type Output = T;

    fn parse_iter_once<S>(self, _input: &mut Stream<S>) -> Result<Self::Output>
    where
        S: Iterator<Item = Self::Item>,
    {
        Ok(self.0)
    }
}

pub struct ValueClone<I, T>(T, PhantomData<I>);

pub fn value_clone<I, T>(value: T) -> ValueClone<I, T> {
    ValueClone(value, PhantomData)
}

impl<I, T: Clone> Parser for ValueClone<I, T> {
    type Item = I;
    type Output = T;

    fn parse_iter<S>(&self, _input: &mut Stream<S>) -> Result<Self::Output>
    where
        S: Iterator<Item = Self::Item>,
    {
        Ok(self.0.clone())
    }
}

impl<I, T> ParserOnce for ValueClone<I, T> {
    type Item = I;
    type Output = T;

    fn parse_iter_once<S>(self, _input: &mut Stream<S>) -> Result<Self::Output>
    where
        S: Iterator<Item = Self::Item>,
    {
        Ok(self.0)
    }
}
