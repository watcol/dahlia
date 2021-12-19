use crate::error::{ParseError, Result};
use crate::stream::Stream;

mod any;
mod condition;
mod position;
mod value;

mod boxed;

pub use any::{any, Any};
pub use condition::{is, is_not, is_not_once, is_once, Condition, ConditionOnce};
pub use position::{position, Position};
pub use value::{value, value_clone, Value, ValueClone};

#[cfg(not(feature = "std"))]
use alloc::string::String;

pub trait Parser: BaseParser {
    #[cfg(feature = "std")]
    fn boxed(&self) -> boxed::RefBoxedParser<'_, Self::Item, Self::Output>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    #[cfg(feature = "std")]
    fn boxed_clone(&self) -> boxed::BoxedParser<'_, Self::Item, Self::Output>
    where
        Self: Clone,
    {
        Box::new(self.clone())
    }

    fn parse<I, B>(&self, input: I) -> Result<(Self::Output, B)>
    where
        I: Into<Stream<Self::Item>>,
        B: FromIterator<Self::Item>,
    {
        let mut input = input.into();
        let output = self.parse_iter(&mut input)?;
        Ok((output, input.collect()))
    }

    fn parse_complete<I>(&self, input: I) -> Result<Self::Output>
    where
        I: Into<Stream<Self::Item>>,
    {
        let mut input = input.into();
        let output = self.parse_iter(&mut input)?;
        match input.next() {
            Some(_) => Err(ParseError::Expected {
                position: input.pos(),
                expected: String::from("EOF"),
            }),
            None => Ok(output),
        }
    }
}

pub trait BaseParser {
    type Item: Clone;
    type Output;

    fn parse_iter(&self, input: &mut Stream<Self::Item>) -> Result<Self::Output>;
}

impl<T: BaseParser> Parser for T {}

pub trait ParserOnce: BaseParserOnce {
    fn parse_once<S, B>(self, input: S) -> Result<(Self::Output, B)>
    where
        Self: Sized,
        S: Into<Stream<Self::Item>>,
        B: FromIterator<Self::Item>,
    {
        let mut input = input.into();
        let output = self.parse_iter_once(&mut input)?;
        Ok((output, input.collect()))
    }

    fn parse_complete_once<S>(self, input: S) -> Result<Self::Output>
    where
        Self: Sized,
        S: Into<Stream<Self::Item>>,
    {
        let mut input = input.into();
        let output = self.parse_iter_once(&mut input)?;
        match input.next() {
            Some(_) => Err(ParseError::Expected {
                position: input.pos(),
                expected: String::from("EOF"),
            }),
            None => Ok(output),
        }
    }
}

pub trait BaseParserOnce {
    type Item: Clone;
    type Output;

    fn parse_iter_once(self, input: &mut Stream<Self::Item>) -> Result<Self::Output>;
}
