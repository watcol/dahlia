use crate::error::ParseError;

mod any;
mod function;

pub use any::{any, Any};
pub use function::{function, Function};

pub trait Parser<'a> {
    type Item;
    type Output;

    fn parse(
        &self,
        input: &'a [Self::Item],
    ) -> Result<(Self::Output, usize), ParseError<Self::Item>> {
        self.parse_at(input, 0)
    }

    fn parse_complete(
        &self,
        input: &'a [Self::Item],
    ) -> Result<Self::Output, ParseError<Self::Item>> {
        let (res, end) = self.parse_at(input, 0)?;
        if end == input.len() {
            Ok(res)
        } else {
            Err(ParseError::Remain {
                start: end,
                end: input.len() - 1,
            })
        }
    }

    fn parse_at(
        &self,
        input: &'a [Self::Item],
        start: usize,
    ) -> Result<(Self::Output, usize), ParseError<Self::Item>>;
}
