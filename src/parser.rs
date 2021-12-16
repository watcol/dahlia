use crate::error::{ParseError, Result};
use crate::stream::Stream;

mod any;
mod condition;
mod position;
mod value;

pub use any::{any, Any};
pub use condition::{is, is_not, Condition};
pub use position::{position, Position};
pub use value::{value, Value};

pub trait Parser {
    type Item;
    type Output;

    fn parse<S, B>(&self, input: S) -> Result<(Self::Output, B)>
    where
        S: IntoIterator<Item = Self::Item>,
        B: FromIterator<Self::Item>,
    {
        let mut input = Stream::from(input);
        let output = self.parse_at(&mut input)?;
        Ok((output, input.collect()))
    }

    fn parse_complete<S>(&self, input: S) -> Result<Self::Output>
    where
        S: IntoIterator<Item = Self::Item>,
    {
        let mut input = Stream::from(input);
        let output = self.parse_at(&mut input)?;
        match input.next() {
            Some(_) => Err(ParseError::Expected {
                position: input.pos(),
                expected: String::from("EOF"),
            }),
            None => Ok(output),
        }
    }

    fn parse_at<S>(&self, input: &mut Stream<S>) -> Result<Self::Output>
    where
        S: Iterator<Item = Self::Item>;
}
