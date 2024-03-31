// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

extern crate alloc;

use alloc::collections::VecDeque;
use alloc::vec::Vec;

pub struct Joining<I, E>
where
    I: Iterator<Item = E>,
    E: Clone,
{
    wrapped: I,
    delimiter: E,
    peeked: Option<E>,
    delim: Option<E>,
}

impl<I, E> Joining<I, E>
where
    I: Iterator<Item = E>,
    E: Clone,
{
    pub fn new(mut wrapped: I, delim: E) -> Self {
        let peeked = wrapped.next();
        Self {
            wrapped,
            delimiter: delim,
            peeked,
            delim: None,
        }
    }
}

impl<I, E> Iterator for Joining<I, E>
where
    I: Iterator<Item = E>,
    E: Clone,
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(delim) = self.delim.take() {
            return Some(delim);
        }
        if let Some(peek) = self.peeked.take() {
            self.peeked = self.wrapped.next();
            if self.peeked.is_some() {
                self.delim = Some(self.delimiter.clone());
            }
            return Some(peek);
        }
        None
    }
}

pub struct MultiJoining<I, E>
where
    I: Iterator<Item = E>,
    E: Clone,
{
    wrapped: I,
    delimiters: Vec<E>,
    peeked: Option<E>,
    delim: VecDeque<E>,
}

impl<I, E> MultiJoining<I, E>
where
    I: Iterator<Item = E>,
    E: Clone,
{
    pub fn new(mut wrapped: I, delim: &[E]) -> Self {
        let peeked = wrapped.next();
        Self {
            wrapped,
            delimiters: Vec::from(delim),
            peeked,
            delim: VecDeque::with_capacity(delim.len()),
        }
    }
}

impl<I, E> Iterator for MultiJoining<I, E>
where
    I: Iterator<Item = E>,
    E: Clone,
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(delim) = self.delim.pop_front() {
            return Some(delim);
        }
        if let Some(peek) = self.peeked.take() {
            self.peeked = self.wrapped.next();
            if self.peeked.is_some() {
                for elem in &self.delimiters {
                    self.delim.push_back(elem.clone());
                }
            }
            return Some(peek);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::iterators::Itertools;

    #[test]
    pub fn test_joining() {
        let mut iter = ["A", "B", "C", "D"].iter().joining(&"z");

        assert_eq!(iter.next().unwrap(), &"A");
        assert_eq!(iter.next().unwrap(), &"z");
        assert_eq!(iter.next().unwrap(), &"B");
        assert_eq!(iter.next().unwrap(), &"z");
        assert_eq!(iter.next().unwrap(), &"C");
        assert_eq!(iter.next().unwrap(), &"z");
        assert_eq!(iter.next().unwrap(), &"D");
        assert_eq!(iter.next(), None);
    }

    #[test]
    pub fn test_multi_joining() {
        let mut iter = ["A", "B", "C", "D"].iter().joining_multi(&[&"y", &"z"]);

        assert_eq!(iter.next().unwrap(), &"A");
        assert_eq!(iter.next().unwrap(), &"y");
        assert_eq!(iter.next().unwrap(), &"z");
        assert_eq!(iter.next().unwrap(), &"B");
        assert_eq!(iter.next().unwrap(), &"y");
        assert_eq!(iter.next().unwrap(), &"z");
        assert_eq!(iter.next().unwrap(), &"C");
        assert_eq!(iter.next().unwrap(), &"y");
        assert_eq!(iter.next().unwrap(), &"z");
        assert_eq!(iter.next().unwrap(), &"D");
        assert_eq!(iter.next(), None);
    }
}
