use super::Parser;
use crate::error::{ParseError, Result};
use crate::stream::Stream;

pub struct Condition<'a, I>(Box<dyn for<'b> Fn(&'b I) -> bool + 'a>);

pub fn is<'a, I, F>(cond: F) -> Condition<'a, I>
where
    F: for<'b> Fn(&'b I) -> bool + 'a,
{
    Condition(Box::new(cond))
}

pub fn is_not<'a, I, F>(cond: F) -> Condition<'a, I>
where
    F: for<'b> Fn(&'b I) -> bool + 'a,
{
    Condition(Box::new(move |i| !cond(i)))
}

impl<'a, I> Parser for Condition<'a, I> {
    type Item = I;
    type Output = I;

    fn parse_at<S>(&self, input: &mut Stream<S>) -> Result<Self::Output>
    where
        S: Iterator<Item = Self::Item>,
    {
        match input.next() {
            Some(i) if (self.0)(&i) => Ok(i),
            _ => Err(ParseError::Expected {
                position: input.pos(),
                expected: String::from("<condition>"),
            }),
        }
    }
}
