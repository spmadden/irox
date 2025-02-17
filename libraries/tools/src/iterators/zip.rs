// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::array;
use crate::buf::{Buffer, RoundBuffer};
use crate::iterators::LendingIterator;

///
/// Struct that allows zipping iterators in quantities larger than 2.
pub struct Zipping<'a, const N: usize, T> {
    iters: [core::slice::Iter<'a, T>; N],
}

macro_rules! impl_zipping {
    (<$n: tt>) => {
        impl<'a, T: Copy> Iterator for Zipping<'a, $n, T> {
            type Item = [T;$n];

            fn next(&mut self) -> Option<Self::Item> {
                let mut i = self.iters.iter_mut();
                let out : [T;$n] = array![
                    i.next()?.next().copied()?;$n
                ];
                Some(out)
            }
        }
        impl<'a, T> From<[core::slice::Iter<'a, T>; $n]> for Zipping<'a, $n, T> {
            fn from(iters: [core::slice::Iter<'a, T>; $n]) -> Self {
                Self {
                    iters
                }
            }
        }
    };
    ($($n:tt),+) => (
        $(
            impl_zipping!(<$n>);
        )*
    )
    ;
}
impl_zipping!(2, 3, 4, 5, 6, 7, 8, 16, 32, 64);

pub struct Windows<'a, const N: usize, T> {
    iter: core::slice::Iter<'a, T>,
    buf: RoundBuffer<N, T>,
}
impl<'a, const N: usize, T> Windows<'a, N, T> {
    pub fn new(iter: core::slice::Iter<'a, T>) -> Self {
        Self {
            iter,
            buf: RoundBuffer::new(),
        }
    }
}

macro_rules! impl_windows {
    (<$n: tt>) => {
        impl<'a, T: Copy> LendingIterator for Windows<'a, $n, T> where Self: 'a {
        type Item<'b> = [&'b T; $n] where Self: 'b;

        fn next_ref<'b>(&'b mut self) -> Option<Self::Item<'b>> {
            if !self.buf.is_full() {
                while !self.buf.is_full() {
                    self.buf.push(self.iter.next().copied()?).ok()?;
                }
            } else {
                self.buf.pop_front()?;
                self.buf.push(self.iter.next().copied()?).ok()?;
            }
            let mut iter = self.buf.iter();
            let arr = array![iter.next()?;$n];
            Some(arr)
        }
    }
    };
    ($($n:tt),+) => (
        $(
            impl_windows!(<$n>);
        )*
    )
    ;
}
impl_windows!(2, 3, 4, 5, 6, 7, 8, 16, 32, 64);
#[cfg(test)]
mod tests {
    use crate::hex;
    use crate::iterators::{LendingIterator, Windows};

    #[test]
    pub fn testwindows() {
        let a: [u8; 16] = hex!("000102030405060708090A0B0C0D0E0F");
        let mut iter = Windows::<2, _>::new(a.iter());
        assert_eq!(iter.next_ref(), Some([&0x00, &0x01]));
        assert_eq!(iter.next_ref(), Some([&0x01, &0x02]));
        assert_eq!(iter.next_ref(), Some([&0x02, &0x03]));
        assert_eq!(iter.next_ref(), Some([&0x03, &0x04]));
        assert_eq!(iter.next_ref(), Some([&0x04, &0x05]));
        assert_eq!(iter.next_ref(), Some([&0x05, &0x06]));
        assert_eq!(iter.next_ref(), Some([&0x06, &0x07]));
        assert_eq!(iter.next_ref(), Some([&0x07, &0x08]));
        assert_eq!(iter.next_ref(), Some([&0x08, &0x09]));
        assert_eq!(iter.next_ref(), Some([&0x09, &0x0A]));
        assert_eq!(iter.next_ref(), Some([&0x0A, &0x0B]));
        assert_eq!(iter.next_ref(), Some([&0x0B, &0x0C]));
        assert_eq!(iter.next_ref(), Some([&0x0C, &0x0D]));
        assert_eq!(iter.next_ref(), Some([&0x0D, &0x0E]));
        assert_eq!(iter.next_ref(), Some([&0x0E, &0x0F]));
    }
}
