use super::Parser;
use crate::error::{ParseError, Result};
use std::marker::PhantomData;

pub struct Any<I>(PhantomData<I>);

pub fn any<I>() -> Any<I> {
    Any(PhantomData)
}

impl<'a, I: Clone> Parser<'a> for Any<I> {
    type Item = I;
    type Output = I;

    fn parse_at(&self, input: &'a [Self::Item], start: usize) -> Result<(Self::Output, usize)> {
        if let Some(i) = input.get(start) {
            Ok((i.clone(), start + 1))
        } else {
            Err(ParseError::Expected {
                position: start,
                expected: String::from("any character"),
            })
        }
    }
}
