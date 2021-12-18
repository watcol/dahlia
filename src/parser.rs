use crate::error::{ParseError, Result};
use crate::stream::Stream;

mod any;
#[cfg(feature = "std")]
mod condition;
mod position;
mod value;

#[cfg(feature = "std")]
mod boxed;

pub use any::{any, Any};
#[cfg(feature = "std")]
pub use condition::{is, is_not, is_not_once, is_once, Condition, ConditionOnce};
pub use position::{position, Position};
pub use value::{value, value_clone, Value, ValueClone};

pub trait Parser: BaseParser {
    #[cfg(feature = "std")]
    fn boxed(&self) -> boxed::RefBoxedParser<'_, Self::Iter, Self::Output>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    #[cfg(feature = "std")]
    fn boxed_clone(&self) -> boxed::BoxedParser<'_, Self::Iter, Self::Output>
    where
        Self: Clone,
    {
        Box::new(self.clone())
    }

    fn parse<I, B>(&self, input: I) -> Result<(Self::Output, B)>
    where
        I: IntoIterator<IntoIter = Self::Iter>,
        B: FromIterator<<Self::Iter as Iterator>::Item>,
    {
        let mut input = Stream::from(input);
        let output = self.parse_iter(&mut input)?;
        Ok((output, input.collect()))
    }

    fn parse_complete<I>(&self, input: I) -> Result<Self::Output>
    where
        I: IntoIterator<IntoIter = Self::Iter>,
    {
        let mut input = Stream::from(input);
        let output = self.parse_iter(&mut input)?;
        match input.next() {
            Some(_) => Err(ParseError::Expected {
                position: input.pos(),
                #[cfg(feature = "std")]
                expected: String::from("EOF"),
            }),
            None => Ok(output),
        }
    }
}

pub trait BaseParser {
    type Iter: Iterator;
    type Output;

    fn parse_iter(&self, input: &mut Stream<Self::Iter>) -> Result<Self::Output>;
}

impl<T: BaseParser> Parser for T {}

pub trait ParserOnce: BaseParserOnce {
    fn parse_once<S, B>(self, input: S) -> Result<(Self::Output, B)>
    where
        Self: Sized,
        S: IntoIterator<IntoIter = Self::Iter>,
        B: FromIterator<<Self::Iter as Iterator>::Item>,
    {
        let mut input = Stream::from(input);
        let output = self.parse_iter_once(&mut input)?;
        Ok((output, input.collect()))
    }

    fn parse_complete_once<S>(self, input: S) -> Result<Self::Output>
    where
        Self: Sized,
        S: IntoIterator<IntoIter = Self::Iter>,
    {
        let mut input = Stream::from(input);
        let output = self.parse_iter_once(&mut input)?;
        match input.next() {
            Some(_) => Err(ParseError::Expected {
                position: input.pos(),
                #[cfg(feature = "std")]
                expected: String::from("EOF"),
            }),
            None => Ok(output),
        }
    }
}

pub trait BaseParserOnce {
    type Iter: Iterator;
    type Output;

    fn parse_iter_once(self, input: &mut Stream<Self::Iter>) -> Result<Self::Output>;
}
