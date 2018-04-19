use std::iter::Peekable;
use std::iter::Iterator;

pub struct PrevPeekable<I> where 
    I: Iterator, 
    <I as ::std::iter::Iterator>::Item: ::std::clone::Clone, {
    /// Iterator that `PrevPeekable` wraps
    iterator: Peekable<I>,
    /// The element before the one we just returned. Initially it's `None`.
    prev: Option<I::Item>,
    /// The current element we just returned.
    current: Option<I::Item>,
}

impl<I> PrevPeekable<I> where
    I: Iterator,
    <I as ::std::iter::Iterator>::Item: ::std::clone::Clone, {

    pub fn new(iterator: I) -> Self {
        // Initialize current with the first value in the iterator. Basically,
        // the iterator will be 1 element ahead under the hood
        let mut peekable = iterator.peekable();
        let current = peekable.next();

        PrevPeekable {
            iterator: peekable,
            prev: None,
            current: current,
        }
    }
}

impl<I> Iterator for PrevPeekable<I> where 
    I: Iterator,
    <I as ::std::iter::Iterator>::Item: ::std::clone::Clone, {

    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        // If self.current is None, we've reached the end, and self.prev should
        // the second last element
        if let Some(_) = self.current {
            let old_val = self.current.clone();
            self.prev = std::mem::replace(&mut self.current, self.iterator.next());
            return old_val;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let v = vec![1, 2, 3];
        let mut iter = PrevPeekable::new(v.iter());

        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&1), iter.prev);
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&2), iter.prev);
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&3), iter.prev);
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }
}
