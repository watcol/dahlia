use crate::error::{ParseError, Result};
use crate::stream::Stream;

mod any;
mod condition;
mod position;
mod value;

pub use any::{any, Any};
pub use condition::{is, is_not, is_not_once, is_once, Condition, ConditionOnce};
pub use position::{position, Position};
pub use value::{value, value_clone, Value, ValueClone};

pub trait Parser {
    type Item;
    type Output;

    fn parse<S, B>(&self, input: S) -> Result<(Self::Output, B)>
    where
        S: IntoIterator<Item = Self::Item>,
        B: FromIterator<Self::Item>,
    {
        let mut input = Stream::from(input);
        let output = self.parse_iter(&mut input)?;
        Ok((output, input.collect()))
    }

    fn parse_complete<S>(&self, input: S) -> Result<Self::Output>
    where
        S: IntoIterator<Item = Self::Item>,
    {
        let mut input = Stream::from(input);
        let output = self.parse_iter(&mut input)?;
        match input.next() {
            Some(_) => Err(ParseError::Expected {
                position: input.pos(),
                expected: String::from("EOF"),
            }),
            None => Ok(output),
        }
    }

    fn parse_iter<S>(&self, input: &mut Stream<S>) -> Result<Self::Output>
    where
        S: Iterator<Item = Self::Item>;
}

pub trait ParserOnce: Sized {
    type Item;
    type Output;

    fn parse_once<S, B>(self, input: S) -> Result<(Self::Output, B)>
    where
        S: IntoIterator<Item = Self::Item>,
        B: FromIterator<Self::Item>,
    {
        let mut input = Stream::from(input);
        let output = self.parse_iter_once(&mut input)?;
        Ok((output, input.collect()))
    }

    fn parse_complete_once<S>(self, input: S) -> Result<Self::Output>
    where
        S: IntoIterator<Item = Self::Item>,
    {
        let mut input = Stream::from(input);
        let output = self.parse_iter_once(&mut input)?;
        match input.next() {
            Some(_) => Err(ParseError::Expected {
                position: input.pos(),
                expected: String::from("EOF"),
            }),
            None => Ok(output),
        }
    }

    fn parse_iter_once<S>(self, input: &mut Stream<S>) -> Result<Self::Output>
    where
        S: Iterator<Item = Self::Item>;
}
