use std::iter::Peekable;
use std::iter::Iterator;

pub struct PrevPeekable<I> where 
    I: Iterator, 
    <I as ::std::iter::Iterator>::Item: ::std::clone::Clone, {

    iterator: Peekable<I>,
    prev: Option<I::Item>,
    current: Option<I::Item>,
}

impl<I> PrevPeekable<I> where
    I: Iterator,
    <I as ::std::iter::Iterator>::Item: ::std::clone::Clone, {

    pub fn new(iterator: I) -> Self {
        PrevPeekable {
            iterator: iterator.peekable(),
            prev: None,
            current: None,
        }
    }
}

impl<I> Iterator for PrevPeekable<I> where 
    I: Iterator,
    <I as ::std::iter::Iterator>::Item: ::std::clone::Clone, {

    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        // TODO: store value in prev before releasing it
        self.iterator.next()
    }
}
