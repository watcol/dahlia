use crate::error::{ParseError, Result};

mod any;
mod condition;
mod function;
mod position;
mod value;

pub use any::{any, Any};
pub use condition::{is, is_not, Condition};
pub use function::{function, Function};
pub use position::{position, Position};
pub use value::{value, Value};

pub trait Parser<'a> {
    type Item;
    type Output;

    fn parse(&self, input: &'a [Self::Item]) -> Result<(Self::Output, usize)> {
        self.parse_at(input, 0)
    }

    fn parse_complete(&self, input: &'a [Self::Item]) -> Result<Self::Output> {
        let (res, end) = self.parse_at(input, 0)?;
        if end == input.len() {
            Ok(res)
        } else {
            Err(ParseError::Expected {
                position: end,
                expected: String::from("EOF"),
            })
        }
    }

    fn parse_at(&self, input: &'a [Self::Item], start: usize) -> Result<(Self::Output, usize)>;
}
