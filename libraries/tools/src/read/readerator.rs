// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

///
/// Super basic implementation of iterator for [`std::io::Read`]
pub struct Readerator<T>(pub T);

impl<T: std::io::Read> Iterator for Readerator<T> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf: [u8; 1] = [0];
        let Ok(val) = self.0.read(&mut buf) else {
            return None;
        };
        if val != 1 {
            return None;
        }
        Some(buf[0])
    }
}
