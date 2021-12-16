pub struct Stream<S> {
    position: usize,
    inner: S,
}

impl<T: IntoIterator> From<T> for Stream<T::IntoIter> {
    fn from(iter: T) -> Self {
        Self {
            position: 0,
            inner: iter.into_iter(),
        }
    }
}

impl<S: Iterator> Iterator for Stream<S> {
    type Item = S::Item;
    fn next(&mut self) -> Option<S::Item> {
        match self.inner.next() {
            Some(i) => {
                self.position += 1;
                Some(i)
            }
            None => None,
        }
    }
}

impl<S: Iterator> Stream<S> {
    pub fn pos(&self) -> usize {
        self.position
    }
}
