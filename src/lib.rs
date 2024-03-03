//! `prev-iter` contains an iterator which allows you to view the previous element.

use std::iter::Iterator;
use std::iter::Peekable;

/// An iterator with `prev()`, `prev_peek()`, and `peek()` functions that return the previous element, a
/// reference to the previous element, or a reference to the next element, respectively.
///
/// This `struct` is created by passing an [`Iterator`] whose `Item` implements [`Clone`] to the
/// [`new`] function.
///
/// [`Iterator`]: https://doc.rust-lang.org/nightly/core/iter/iterator/trait.Iterator.html
/// [`Clone`]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html
/// [`new`]: struct.PrevPeekable.html#method.new
#[derive(Debug)]
pub struct PrevPeekable<I>
where
    I: Iterator,
    <I as ::std::iter::Iterator>::Item: ::std::clone::Clone,
{
    /// Iterator that `PrevPeekable` wraps
    iterator: Peekable<I>,
    /// The element before the one we just returned. Initially it's `None`.
    prev: Option<I::Item>,
    /// The current element we just returned.
    current: Option<I::Item>,
    /// Keeps track of whether the iterator has reached the end or not
    finished: bool,
}

impl<I> PrevPeekable<I>
where
    I: Iterator,
    <I as ::std::iter::Iterator>::Item: ::std::clone::Clone,
{
    /// Creates a new `PrevPeekable`. It takes an [`Iterator`] whose `Item` implements [`Clone`].
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/nightly/core/iter/iterator/trait.Iterator.html
    /// [`Clone`]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use prev_iter::PrevPeekable;
    ///
    /// let v = vec![1, 2, 3];
    /// let mut iter = PrevPeekable::new(v.iter());
    ///
    /// assert_eq!(Some(&1), iter.next());
    /// assert_eq!(Some(&2), iter.next());
    /// assert_eq!(Some(&1), iter.prev());
    /// ```
    pub fn new(iterator: I) -> Self {
        PrevPeekable {
            iterator: iterator.peekable(),
            prev: None,
            current: None,
            finished: false,
        }
    }

    /// Returns a reference to the `next()` value without advancing the iterator.
    ///
    /// Like [`next`], if there is a value, it is wrapped in a `Some(T)`.
    /// But if the iteration is over, `None` is returned.
    ///
    /// [`next`]: https://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html#tymethod.next
    ///
    /// Because `peek()` returns a reference, and many iterators iterate over
    /// references, there can be a possibly confusing situation where the
    /// return value is a double reference. You can see this effect in the
    /// examples below.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let xs = [1, 2, 3];
    ///
    /// let mut iter = xs.iter().peekable();
    ///
    /// // peek() lets us see into the future
    /// assert_eq!(iter.peek(), Some(&&1));
    /// assert_eq!(iter.next(), Some(&1));
    ///
    /// assert_eq!(iter.next(), Some(&2));
    ///
    /// // The iterator does not advance even if we `peek` multiple times
    /// assert_eq!(iter.peek(), Some(&&3));
    /// assert_eq!(iter.peek(), Some(&&3));
    ///
    /// assert_eq!(iter.next(), Some(&3));
    ///
    /// // After the iterator is finished, so is `peek()`
    /// assert_eq!(iter.peek(), None);
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn peek(&mut self) -> Option<&I::Item> {
        self.iterator.peek()
    }

    /// Returns the previous value in the iterator without moving the iterator backwards.
    /// When the end is reached, it will always return the last element.
    ///
    /// This function performs a `clone()` when returning the data.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use prev_iter::PrevPeekable;
    ///
    /// let v = vec![1, 2];
    /// let mut it = PrevPeekable::new(v.iter());
    ///
    /// // When the iterator is initialized there is not previous value
    /// assert_eq!(None, it.prev());
    /// assert_eq!(Some(&1), it.next());
    ///
    /// // There is no value before the first element
    /// assert_eq!(None, it.prev());
    /// assert_eq!(Some(&2), it.next());
    ///
    /// // Previous value before 2 is 1
    /// assert_eq!(Some(&1), it.prev());
    ///
    /// // The iterator doesn't have anymore values so the prev() will always
    /// // return the last element
    /// assert_eq!(None, it.next());
    /// assert_eq!(Some(&2), it.prev());
    /// ```
    pub fn prev(&self) -> Option<I::Item> {
        self.prev.clone()
    }

    /// Returns a reference to the previous value in the iterator without moving the iterator
    /// backwards. When the end is reached, it will always return the last element.
    ///
    /// Because `prev_peek()` returns a reference, and many iterators iterate over
    /// references, there can be a possibly confusing situation where the
    /// return value is a double reference. You can see this effect in the
    /// examples below.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use prev_iter::PrevPeekable;
    ///
    /// let v = vec![1, 2];
    /// let mut it = PrevPeekable::new(v.iter());
    ///
    /// // Initially there is nothing to peek at
    /// assert_eq!(None, it.prev_peek());
    /// assert_eq!(Some(&1), it.next());
    ///
    /// // There is nothing before the first element
    /// assert_eq!(None, it.prev_peek());
    /// assert_eq!(Some(&2), it.next());
    ///
    /// // 1 comes before 2
    /// assert_eq!(Some(&&1), it.prev_peek());
    /// assert_eq!(None, it.next());
    ///
    /// // 2 will always be returned as the last element
    /// assert_eq!(Some(&&2), it.prev_peek());
    /// ```
    pub fn prev_peek(&self) -> Option<&I::Item> {
        self.prev.as_ref()
    }
}

impl<I> Iterator for PrevPeekable<I>
where
    I: Iterator,
    <I as ::std::iter::Iterator>::Item: ::std::clone::Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        // If self.iterator.peek() is None, we've reached the end, and self.prev should
        // the second last element
        if let Some(_) = self.iterator.peek() {
            self.prev = std::mem::replace(&mut self.current, self.iterator.next());
            return self.current.clone();
        } else if !self.finished {
            self.prev = std::mem::replace(&mut self.current, self.iterator.next());
            self.finished = true;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! iter {
        ($v: expr) => {{
            PrevPeekable::new($v.iter())
        }};
    }

    #[test]
    fn test_next() {
        let v = vec![1, 2, 3];
        let mut iter = iter!(v);

        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.prev);
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&1), iter.prev);
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&2), iter.prev);
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_peek() {
        let v = vec![1, 2];
        let mut iter = iter!(v);

        assert_eq!(Some(&&1), iter.peek());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&&2), iter.peek());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_prev() {
        let v = vec![1, 2];
        let mut it = iter!(v);

        assert_eq!(None, it.prev());
        assert_eq!(Some(&1), it.next());
        assert_eq!(None, it.prev());
        assert_eq!(Some(&2), it.next());
        assert_eq!(Some(&1), it.prev());
        assert_eq!(None, it.next());
        assert_eq!(Some(&2), it.prev());

        assert_eq!(None, it.next());
        assert_eq!(Some(&2), it.prev());
    }

    #[test]
    fn test_prev_peek() {
        let v = vec![1, 2];
        let mut it = iter!(v);

        assert_eq!(None, it.prev_peek());
        assert_eq!(Some(&1), it.next());
        assert_eq!(None, it.prev_peek());
        assert_eq!(Some(&2), it.next());
        assert_eq!(Some(&&1), it.prev_peek());
        assert_eq!(None, it.next());
        assert_eq!(Some(&&2), it.prev_peek());

        assert_eq!(None, it.next());
        assert_eq!(Some(&&2), it.prev_peek());
    }
}
