use super::Parser;
use crate::error::{ParseError, Result};

pub struct Condition<'a, I>(Box<dyn Fn(I) -> bool + 'a>);

pub fn is<'a, I, F>(cond: F) -> Condition<'a, I>
where
    F: Fn(I) -> bool + 'a,
{
    Condition(Box::new(cond))
}

pub fn is_not<'a, I, F>(cond: F) -> Condition<'a, I>
where
    F: Fn(I) -> bool + 'a,
{
    Condition(Box::new(move |i| !cond(i)))
}

impl<'a, 'b, I: Clone> Parser<'a> for Condition<'b, I> {
    type Item = I;
    type Output = I;

    fn parse_at(&self, input: &'a [Self::Item], start: usize) -> Result<(Self::Item, usize)> {
        match input.get(start) {
            Some(i) if (self.0)(i.clone()) => Ok((i.clone(), start + 1)),
            _ => Err(ParseError::Expected {
                position: start,
                expected: String::from("<condition>"),
            }),
        }
    }
}
