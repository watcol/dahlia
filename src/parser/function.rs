use super::Parser;
use crate::error::ParseError;

pub struct Function<'a, I, O>(Box<Func<'a, I, O>>);
type Func<'a, I, O> = dyn Fn(&'a [I], usize) -> Result<(O, usize), ParseError<I>> + 'a;

pub fn function<'a, F, I, O>(function: F) -> Function<'a, I, O>
where
    F: Fn(&'a [I], usize) -> Result<(O, usize), ParseError<I>> + 'a,
{
    Function(Box::new(function))
}

impl<'a, I, O> Parser<'a> for Function<'a, I, O> {
    type Item = I;
    type Output = O;

    fn parse_at(
        &self,
        input: &'a [Self::Item],
        start: usize,
    ) -> Result<(Self::Output, usize), ParseError<Self::Item>> {
        (self.0)(input, start)
    }
}
