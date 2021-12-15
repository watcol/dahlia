pub enum ParseError<I> {
    Remain {
        start: usize,
        end: usize,
    },
    Expected {
        position: usize,
        expected: String,
        found: Option<I>,
    },
}
