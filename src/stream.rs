#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

pub struct Stream<T: Clone> {
    position: usize,
    inner: Vec<T>,
}

impl<T: Clone> Clone for Stream<T> {
    fn clone(&self) -> Self {
        Self {
            position: self.position,
            inner: self.inner.clone(),
        }
    }
}

impl<T: Clone> From<Vec<T>> for Stream<T> {
    fn from(v: Vec<T>) -> Self {
        Self {
            position: 0,
            inner: v,
        }
    }
}

impl<T: Clone> From<&[T]> for Stream<T> {
    fn from(s: &[T]) -> Self {
        Self {
            position: 0,
            inner: s.to_vec(),
        }
    }
}

impl From<String> for Stream<char> {
    fn from(s: String) -> Self {
        Self {
            position: 0,
            inner: s.chars().collect(),
        }
    }
}

impl From<&str> for Stream<char> {
    fn from(s: &str) -> Self {
        Self {
            position: 0,
            inner: s.chars().collect(),
        }
    }
}

impl<T: Clone> Iterator for Stream<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let val = self.inner.get(self.position);
        self.position += 1;
        val.cloned()
    }
}

impl<T: Clone> Stream<T> {
    pub fn pos(&self) -> usize {
        self.position
    }
}
