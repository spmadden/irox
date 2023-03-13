//!
//! A collection of utilities for the u8 built-in
//!

///
/// Finds the minimum and maximum value in the provided iterator.
/// Example:
/// ```
/// let values : Vec<u8> = vec![0, 5, 30, 20, 2];
/// let (min, max) = irox_tools::u8::min_max(&values);
///
/// assert_eq!(min, 0);
/// assert_eq!(max, 30);
/// ```
pub fn min_max(iter: &[u8]) -> (u8, u8) {
    let mut min = u8::MAX;
    let mut max = u8::MIN;

    for val in iter {
        min = min.min(*val);
        max = max.max(*val);
    }

    (min, max)
}
