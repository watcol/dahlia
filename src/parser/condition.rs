use super::{BaseParser, BaseParserOnce};
use crate::error::{ParseError, Result};
use crate::stream::Stream;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::string::String;

type Func<'a, I> = dyn for<'b> Fn(&'b I) -> bool + 'a;
pub struct Condition<'a, I: Clone>(Box<Func<'a, I>>);

pub fn is<'a, I: Clone, F>(cond: F) -> Condition<'a, I>
where
    F: for<'b> Fn(&'b I) -> bool + 'a,
{
    Condition(Box::new(cond))
}

pub fn is_not<'a, I: Clone, F>(cond: F) -> Condition<'a, I>
where
    F: for<'b> Fn(&'b I) -> bool + 'a,
{
    Condition(Box::new(move |i| !cond(i)))
}

impl<'a, I: Clone> BaseParser for Condition<'a, I> {
    type Item = I;
    type Output = I;

    fn parse_iter(&self, input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        match input.next() {
            Some(i) if (self.0)(&i) => Ok(i),
            _ => Err(ParseError::Expected {
                position: input.pos(),
                expected: String::from("<condition>"),
            }),
        }
    }
}

type FuncOnce<'a, I> = dyn for<'b> FnOnce(&'b I) -> bool + 'a;
pub struct ConditionOnce<'a, I: Clone>(Box<FuncOnce<'a, I>>);

pub fn is_once<'a, I: Clone, F>(cond: F) -> ConditionOnce<'a, I>
where
    F: for<'b> FnOnce(&'b I) -> bool + 'a,
{
    ConditionOnce(Box::new(cond))
}

pub fn is_not_once<'a, I: Clone, F>(cond: F) -> ConditionOnce<'a, I>
where
    F: for<'b> FnOnce(&'b I) -> bool + 'a,
{
    ConditionOnce(Box::new(move |i| !cond(i)))
}

impl<'a, I: Clone> BaseParserOnce for ConditionOnce<'a, I> {
    type Item = I;
    type Output = I;

    fn parse_iter_once(self, input: &mut Stream<Self::Item>) -> Result<Self::Output> {
        match input.next() {
            Some(i) if (self.0)(&i) => Ok(i),
            _ => Err(ParseError::Expected {
                position: input.pos(),
                expected: String::from("<condition>"),
            }),
        }
    }
}
