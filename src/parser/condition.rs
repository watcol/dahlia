use super::BaseParser;
use crate::error::{ParseError, Result};
use crate::stream::Stream;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// A parser which consumes an item if condition is matched.
///
/// See `is()` and `is_not()`.
pub struct Condition<'a, I: Clone>(Box<Func<'a, I>>);
type Func<'a, I> = dyn for<'b> Fn(&'b I) -> bool + 'a;

/// A parser which consumes an item if condition is matched.
///
/// If condition is not matched or reached `EOF` it fails.
pub fn is<'a, I: Clone, F>(cond: F) -> Condition<'a, I>
where
    F: for<'b> Fn(&'b I) -> bool + 'a,
{
    Condition(Box::new(cond))
}

/// A parser which consumes an item if condition is not matched.
///
/// If condition is matched or reached `EOF` it fails.
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
