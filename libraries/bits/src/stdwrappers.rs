// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use core::ops::{Deref, DerefMut};

///
/// Wraps a borrowed value and provides implementations of [`Bits`] and [`MutBits`] where applicable.
pub enum BitsWrapper<'a, T> {
    Owned(T),
    Borrowed(&'a mut T),
}
impl<B> Deref for BitsWrapper<'_, B> {
    type Target = B;

    fn deref(&self) -> &Self::Target {
        match self {
            BitsWrapper::Borrowed(v) => v,
            BitsWrapper::Owned(v) => v,
        }
    }
}
impl<B> DerefMut for BitsWrapper<'_, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            BitsWrapper::Borrowed(v) => v,
            BitsWrapper::Owned(v) => v,
        }
    }
}

#[cfg(feature = "std")]
mod stds {
    use crate::{Bits, BitsWrapper, Error, MutBits};

    impl<T> Bits for BitsWrapper<'_, T>
    where
        T: std::io::Read,
    {
        fn next_u8(&mut self) -> Result<Option<u8>, Error> {
            let mut byte: u8 = 0;
            let read = self.read(core::slice::from_mut(&mut byte))?;
            if read < 1 {
                return Ok(None);
            }
            Ok(Some(byte))
        }
    }

    impl<T> MutBits for BitsWrapper<'_, T>
    where
        T: std::io::Write,
    {
        fn write_u8(&mut self, val: u8) -> Result<(), Error> {
            Ok(self.write_all(&[val])?)
        }
    }
}
