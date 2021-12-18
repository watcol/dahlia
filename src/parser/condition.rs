use super::{BaseParser, BaseParserOnce};
use crate::error::{ParseError, Result};
use crate::stream::Stream;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::string::String;

type Func<'a, I> = dyn for<'b> Fn(&'b I) -> bool + 'a;
pub struct Condition<'a, I: Iterator>(Box<Func<'a, I::Item>>);

pub fn is<'a, I, F>(cond: F) -> Condition<'a, I>
where
    I: Iterator,
    F: for<'b> Fn(&'b I::Item) -> bool + 'a,
{
    Condition(Box::new(cond))
}

pub fn is_not<'a, I, F>(cond: F) -> Condition<'a, I>
where
    I: Iterator,
    F: for<'b> Fn(&'b I::Item) -> bool + 'a,
{
    Condition(Box::new(move |i| !cond(i)))
}

impl<'a, I> BaseParser for Condition<'a, I>
where
    I: Iterator,
{
    type Iter = I;
    type Output = I::Item;

    fn parse_iter(&self, input: &mut Stream<Self::Iter>) -> Result<Self::Output> {
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
pub struct ConditionOnce<'a, I: Iterator>(Box<FuncOnce<'a, I::Item>>);

pub fn is_once<'a, I, F>(cond: F) -> ConditionOnce<'a, I>
where
    I: Iterator,
    F: for<'b> FnOnce(&'b I::Item) -> bool + 'a,
{
    ConditionOnce(Box::new(cond))
}

pub fn is_not_once<'a, I, F>(cond: F) -> ConditionOnce<'a, I>
where
    I: Iterator,
    F: for<'b> FnOnce(&'b I::Item) -> bool + 'a,
{
    ConditionOnce(Box::new(move |i| !cond(i)))
}

impl<'a, I> BaseParserOnce for ConditionOnce<'a, I>
where
    I: Iterator,
{
    type Iter = I;
    type Output = I::Item;

    fn parse_iter_once(self, input: &mut Stream<Self::Iter>) -> Result<Self::Output> {
        match input.next() {
            Some(i) if (self.0)(&i) => Ok(i),
            _ => Err(ParseError::Expected {
                position: input.pos(),
                expected: String::from("<condition>"),
            }),
        }
    }
}
