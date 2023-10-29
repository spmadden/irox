// SPDX-License-Identifier: MIT
// Copyright ${YEAR} IROX Contributors
//

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
        let Some(arr) = arr.get(start_pos..) else {
            return None;
        };
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
