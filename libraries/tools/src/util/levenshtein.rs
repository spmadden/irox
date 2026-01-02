// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//
extern crate alloc;
use alloc::vec;

#[allow(clippy::indexing_slicing)]
#[allow(clippy::needless_range_loop)]
pub fn levenshtein_distance(a: &[u8], b: &[u8]) -> usize {
    let n = b.len() + 1;
    let mut v0 = vec![0; n];
    let mut v1 = vec![0; n];

    for (i, v) in v0.iter_mut().enumerate().take(n) {
        *v = i;
    }

    for i in 0..a.len() {
        v1[0] = i + 1;

        for j in 0..(n - 1) {
            let dc = v0[j + 1] + 1;
            let ic = v1[j] + 1;
            let sc = v0[j] + if a[i] == b[j] { 0 } else { 1 };
            v1[j + 1] = dc.min(ic).min(sc);
        }

        v0.copy_from_slice(&v1);
    }
    v0.last().copied().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use crate::util::levenshtein::levenshtein_distance;

    #[test]
    pub fn test1() {
        let a = b"hello";
        let b = b"world";
        let distance = levenshtein_distance(a, b);
        assert_eq!(distance, 4);
    }
    #[test]
    pub fn test2() {
        let a = b"kitten";
        let b = b"sitting";
        let distance = levenshtein_distance(a, b);
        assert_eq!(distance, 3);
    }
    #[test]
    pub fn test3() {
        let a = b"saturday";
        let b = b"sunday";
        let distance = levenshtein_distance(a, b);
        assert_eq!(distance, 3);
    }
    #[test]
    pub fn test4() {
        let a = b"uninformed";
        let b = b"uniformed";
        let distance = levenshtein_distance(a, b);
        assert_eq!(distance, 1);
    }
    #[test]
    pub fn test5() {
        let a = b"YHCQPGK";
        let b = b"LAHYQQKPGKA";
        let distance = levenshtein_distance(a, b);
        assert_eq!(distance, 6);
    }
    #[test]
    pub fn test6() {
        let a = b"GUMBO";
        let b = b"GAMBOL";
        let distance = levenshtein_distance(a, b);
        assert_eq!(distance, 2);
    }
}
