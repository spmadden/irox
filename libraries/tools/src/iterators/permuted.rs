// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

extern crate alloc;
use alloc::vec::Vec;

///
/// Iterates through all the permutations of a given set
pub struct Permutation<T> {
    state: Vec<usize>,
    data: Vec<T>,
    position: usize,
    initial: bool,
}
impl<T> Permutation<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self {
            state: alloc::vec![0; data.len()],
            data,
            initial: true,
            position: 1,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&Vec<T>> {
        if self.initial {
            self.initial = false;
            return Some(&self.data);
        }
        loop {
            if self.position >= self.data.len() {
                return None;
            }
            let c = self.state.get_mut(self.position)?;
            if *c < self.position {
                if self.position & 0x01 == 0 {
                    self.data.swap(0, self.position);
                } else {
                    self.data.swap(*c, self.position);
                }
                *c += 1;
                self.position = 1;
                return Some(&self.data);
            }
            *c = 0;
            self.position += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::iterators::permuted::Permutation;

    #[test]
    pub fn test() {
        let set = vec![0, 1, 2, 3];
        let mut p = Permutation::new(set);
        while let Some(s) = p.next() {
            println!("{:?}", s);
        }
    }
}
