// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Functions and tools for manipulating arrays of items.
//!

use crate::cfg_feature_alloc;
cfg_feature_alloc! {
    extern crate alloc;
}
#[macro_export]
macro_rules! array {
    (@accum (0, $($_es:expr),*) -> ($($body:tt)*))
        => {array!(@as_expr [$($body)*])};
    (@accum (1, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (0, $($es),*) -> ($($body)* $($es,)*))};
    (@accum (2, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (0, $($es),*) -> ($($body)* $($es,)* $($es,)*))};
    (@accum (3, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (2, $($es),*) -> ($($body)* $($es,)*))};
    (@accum (4, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (2, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (5, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (4, $($es),*) -> ($($body)* $($es,)*))};
    (@accum (6, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (4, $($es),*) -> ($($body)* $($es,)* $($es,)*))};
    (@accum (7, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (4, $($es),*) -> ($($body)* $($es,)* $($es,)* $($es,)*))};
    (@accum (8, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (4, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (16, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (8, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (32, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (16, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (64, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (32, $($es,)* $($es),*) -> ($($body)*))};

    (@as_expr $e:expr) => {$e};

    [$e:expr; $n:tt] => { array!(@accum ($n, $e) -> ()) };
}

///
/// Scans through the array, looking for the maximum value contained within it, using the
/// [`PartialOrd`] operation.  Returns the first index of the maximum detected value.
///
/// # Example
/// ```
/// use irox_tools::arrays::max_index;
/// let arr = &[8,2,7,4,6,3,8,2,15,1,6];
/// let max_idx = max_index(arr);
///
/// assert_eq!(max_idx, Some(8));
/// ```
pub fn max_index<T: PartialOrd>(arr: &[T]) -> Option<usize> {
    let mut max_idx: Option<usize> = None;
    let mut max_val: Option<&T> = None;
    for (idx, val) in arr.iter().enumerate() {
        if max_idx.is_none() {
            max_idx = Some(idx);
            max_val = Some(val);
        } else if let Some(mv) = max_val {
            if val.gt(mv) {
                max_val = Some(val);
                max_idx = Some(idx);
            }
        }
    }
    max_idx
}

///
/// Copies from the provided source slice into a statically sized output array.  Uses [`&T::copy_from_slice`]
/// under-the-hood, and will panic if `N` is greater than the provided slice length.
pub fn copy_subset<T: Copy + Default, const N: usize>(arr: &[T]) -> [T; N] {
    let mut out = [T::default(); N];
    out.copy_from_slice(arr.split_at(N).0);
    out
}

pub trait SliceTools<T> {
    ///
    /// Copies from the provided source slice into a statically sized output array.  Uses [`&T::copy_from_slice`]
    /// under-the-hood, and will panic if `N` is greater than the provided slice length.
    fn copy_subset<const N: usize>(&self) -> [T; N]
    where
        T: Copy + Default;
    fn limit(&self, limit: usize) -> &[T];
    cfg_feature_alloc! {
        fn reversed(&self) -> alloc::boxed::Box<[T]> where T: Copy+Default;
    }
}
impl<T> SliceTools<T> for [T]
where
    T: Copy + Default,
{
    fn copy_subset<const N: usize>(&self) -> [T; N]
    where
        T: Copy + Default,
    {
        copy_subset(self)
    }

    fn limit(&self, limit: usize) -> &[T] {
        if limit < self.len() {
            self.split_at(limit).0
        } else {
            self
        }
    }

    cfg_feature_alloc! {
        fn reversed(&self) -> alloc::boxed::Box<[T]>
        where
            T: Copy + Default
        {
            let mut v : alloc::boxed::Box<[T]> = alloc::boxed::Box::from(self);
            v.as_mut().reverse();
            v
        }
    }
}

pub trait ArrayTools<T: Copy + Default, const N: usize> {
    fn reversed(&self) -> [T; N];
}
impl<T: Copy + Default, const N: usize> ArrayTools<T, N> for [T; N] {
    fn reversed(&self) -> [T; N] {
        let mut c = *self;
        c.reverse();
        c
    }
}

///
/// Searches the provided array for the longest consecutive repeated sequence of 'val'.
///
/// # Example
/// ```
/// # use irox_tools::arrays::longest_consecutive_values;
/// let (position,length) = longest_consecutive_values(&[1,0,0,0,0,0,1,2,3,4,5,6], &0).unwrap();
///
/// assert_eq!(position, 1);
/// assert_eq!(length, 5);
/// ```
pub fn longest_consecutive_values<T: PartialOrd>(arr: &[T], val: &T) -> Option<(usize, usize)> {
    let len = arr.len();
    let mut best_count: usize = 0;
    let mut best_position: usize = 0;
    let mut start_pos = 0;
    while start_pos < len {
        let mut count: usize = 0;
        let arr = arr.get(start_pos..)?;
        for (idx, v) in arr.iter().enumerate() {
            if v.ne(val) {
                start_pos += idx;
                break;
            }
            count += 1;
        }
        if count > best_count {
            best_count = count;
            if start_pos != 0 {
                best_position = start_pos - count;
            }
        }
        start_pos += 1;
    }
    if best_count > 0 {
        return Some((best_position, best_count));
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::arrays::longest_consecutive_values;

    #[test]
    pub fn test1() {
        let (position, length) =
            longest_consecutive_values(&[1, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6], &0).unwrap();

        assert_eq!(position, 1);
        assert_eq!(length, 5);
    }

    #[test]
    pub fn test2() {
        let (position, length) =
            longest_consecutive_values(&[1, 0, 0, 0, 1, 2, 0, 0, 0, 0, 0, 4, 5, 6], &0).unwrap();

        assert_eq!(position, 6);
        assert_eq!(length, 5);
    }
}
