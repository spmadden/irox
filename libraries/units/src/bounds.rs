// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Bounding Boxes and Range Checks
//!

use core::fmt::{Debug, Display, Formatter};

///
/// Defines a basic four-corner bounding box
pub struct Bounds<T> {
    /// The upper left coordinate
    upper_left: T,
    /// The upper right coordinate
    upper_right: T,
    /// The lower left coordinate
    lower_left: T,
    /// The lower right coordinate
    lower_right: T,
}

impl<T> Bounds<T> {
    ///
    /// Creates a new bounding box using the specified four corners
    pub const fn new(upper_left: T, upper_right: T, lower_left: T, lower_right: T) -> Bounds<T> {
        Bounds {
            upper_left,
            upper_right,
            lower_left,
            lower_right,
        }
    }

    pub fn upper_left_corner(&self) -> &T {
        &self.upper_left
    }

    pub fn lower_left_corner(&self) -> &T {
        &self.lower_left
    }

    pub fn upper_right_corner(&self) -> &T {
        &self.upper_right
    }

    pub fn lower_right_corner(&self) -> &T {
        &self.lower_right
    }
}

///
/// A trait to check if a particular value is within range
pub trait Range<T>
where
    T: Debug + Display + Clone + PartialOrd,
{
    type Error;

    ///
    /// Returns true if the value is valid for the specified range
    fn value_is_valid(&self, value: &T) -> bool;

    ///
    /// Checks if the value is valid, if valid, returns `Ok(())`
    /// If not valid, returns `Err(Error)`
    fn check_value_is_valid(&self, value: &T) -> Result<(), Self::Error>;
}

///
/// A [`Range`] implementation to verify a value is less than the reference value
#[derive(Debug, Clone)]
pub struct LessThanValue<T>
where
    T: Debug + Display + Clone + PartialOrd,
{
    pub(crate) value: T,
}
impl<T: Debug + Display + Clone + PartialOrd> LessThanValue<T> {
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    /// Returns the first invalid value for this range.  All values MUST be less than this value
    pub fn value(&self) -> &T {
        &self.value
    }
}
impl<T: Debug + Display + Clone + PartialOrd> Range<T> for LessThanValue<T> {
    type Error = GreaterThanEqualToValueError<T>;
    fn value_is_valid(&self, value: &T) -> bool {
        value.lt(&self.value)
    }

    fn check_value_is_valid(&self, value: &T) -> Result<(), Self::Error> {
        if self.value_is_valid(value) {
            return Ok(());
        }
        Err(GreaterThanEqualToValueError {
            value: value.clone(),
            valid_range: self.clone(),
        })
    }
}

///
/// Error type for when a value is less than or equal to the reference value
#[derive(Debug, Clone)]
pub struct LessThanEqualToValueError<T>
where
    T: Debug + Display + Clone + PartialOrd,
{
    pub(crate) value: T,
    pub(crate) valid_range: GreaterThanValue<T>,
}

impl<T: Debug + Display + Clone + PartialOrd> Display for LessThanEqualToValueError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "value {} is less than or equal to {}",
            self.value, self.valid_range.value
        ))
    }
}

impl<T: Debug + Display + Clone + PartialOrd> LessThanEqualToValueError<T> {
    /// Returns the offending value that is out of range
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns the range of valid values this value breaks
    pub fn valid_range(&self) -> &GreaterThanValue<T> {
        &self.valid_range
    }
}
#[cfg(feature = "std_errors")]
impl<T: Debug + Display + Clone + PartialOrd> std::error::Error for LessThanEqualToValueError<T> {}

///
/// A [`Range`] implementation to verify a value is greater than the reference value
#[derive(Debug, Clone)]
pub struct GreaterThanValue<T>
where
    T: Debug + Display + Clone + PartialOrd,
{
    pub(crate) value: T,
}

impl<T: Debug + Display + Clone + PartialOrd> GreaterThanValue<T> {
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    /// Returns the first smallest invalid value for the range.  All values MUST be greater than
    /// this value.
    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T: Debug + Display + Clone + PartialOrd> Range<T> for GreaterThanValue<T> {
    type Error = LessThanEqualToValueError<T>;

    fn value_is_valid(&self, value: &T) -> bool {
        value.gt(&self.value)
    }

    fn check_value_is_valid(&self, value: &T) -> Result<(), Self::Error> {
        if self.value_is_valid(value) {
            return Ok(());
        }
        Err(LessThanEqualToValueError {
            valid_range: self.clone(),
            value: value.clone(),
        })
    }
}

///
/// An error type to indicate that the checked value is greater than or equal to
/// the valid reference value
#[derive(Debug, Clone)]
pub struct GreaterThanEqualToValueError<T>
where
    T: Debug + Display + Clone + PartialOrd,
{
    pub(crate) value: T,
    pub(crate) valid_range: LessThanValue<T>,
}
impl<T: Debug + Display + Clone + PartialOrd> Display for GreaterThanEqualToValueError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "the value {} is greater than or equal to {}",
            self.value, self.valid_range.value
        ))
    }
}
#[cfg(feature = "std_errors")]
impl<T: Debug + Display + Clone + PartialOrd> std::error::Error
    for GreaterThanEqualToValueError<T>
{
}

impl<T: Debug + Display + Clone + PartialOrd> GreaterThanEqualToValueError<T> {
    #[must_use]
    pub fn new(value: T, valid_range: LessThanValue<T>) -> Self {
        Self { value, valid_range }
    }

    pub fn err<O>(value: T, valid_range: LessThanValue<T>) -> Result<O, Self> {
        Err(Self::new(value, valid_range))
    }

    /// Returns the value that is out of range.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns the valid range that the value is outside of.
    pub fn valid_range(&self) -> &LessThanValue<T> {
        &self.valid_range
    }
}
macro_rules! upconvert_error_type {
    ($lower:tt,$upper:tt,$error:tt) => {
        impl From<$error<$lower>> for $error<$upper> {
            fn from(value: $error<$lower>) -> Self {
                $error {
                    value: value.value as $upper,
                    valid_range: LessThanValue {
                        value: value.valid_range.value as $upper,
                    },
                }
            }
        }
    };
}
upconvert_error_type!(u8, u16, GreaterThanEqualToValueError);
upconvert_error_type!(u8, u32, GreaterThanEqualToValueError);
upconvert_error_type!(u8, u64, GreaterThanEqualToValueError);
upconvert_error_type!(u8, u128, GreaterThanEqualToValueError);
upconvert_error_type!(u8, f32, GreaterThanEqualToValueError);
upconvert_error_type!(u8, f64, GreaterThanEqualToValueError);
upconvert_error_type!(u16, u32, GreaterThanEqualToValueError);
upconvert_error_type!(u16, u64, GreaterThanEqualToValueError);
upconvert_error_type!(u16, u128, GreaterThanEqualToValueError);
upconvert_error_type!(u16, f32, GreaterThanEqualToValueError);
upconvert_error_type!(u16, f64, GreaterThanEqualToValueError);
upconvert_error_type!(u32, u64, GreaterThanEqualToValueError);
upconvert_error_type!(u32, u128, GreaterThanEqualToValueError);
upconvert_error_type!(u32, f32, GreaterThanEqualToValueError);
upconvert_error_type!(u32, f64, GreaterThanEqualToValueError);

///
/// A [`Range`] implementation to verify a value is between two reference values
#[derive(Debug, Clone)]
pub struct WithinRange<T>
where
    T: Debug + Display + Clone + PartialOrd,
{
    pub(crate) lower_bound: GreaterThanValue<T>,
    pub(crate) upper_bound: LessThanValue<T>,
}
impl<T: Debug + Display + Clone + PartialOrd> WithinRange<T> {
    /// Creates a new [`WithinRange`], which is implemented as the union of a [`LessThanValue<T>`]
    /// and a [`GreaterThanValue<T>`].
    ///
    /// Somewhat confusingly, `lower_bound` and `upper_bound` are the first two INVALID values
    /// bounding this range.
    #[must_use]
    pub const fn new(lower_bound: T, upper_bound: T) -> Self {
        Self {
            lower_bound: GreaterThanValue::new(lower_bound),
            upper_bound: LessThanValue::new(upper_bound),
        }
    }

    /// Returns the lower bound of this range
    #[must_use]
    pub fn lower_bound(&self) -> &GreaterThanValue<T> {
        &self.lower_bound
    }
    /// Returns the upper bound of this range
    #[must_use]
    pub fn upper_bound(&self) -> &LessThanValue<T> {
        &self.upper_bound
    }
}
impl<T: Debug + Display + Clone + PartialOrd> Range<T> for WithinRange<T> {
    type Error = OutsideRangeError<T>;

    fn value_is_valid(&self, value: &T) -> bool {
        self.lower_bound.value_is_valid(value) && self.upper_bound.value_is_valid(value)
    }

    fn check_value_is_valid(&self, value: &T) -> Result<(), Self::Error> {
        if self.value_is_valid(value) {
            return Ok(());
        }
        Err(OutsideRangeError {
            value: value.clone(),
            valid_range: self.clone(),
        })
    }
}

///
/// A [`Range`] implementation to verify a value is outside two reference values
#[derive(Debug, Clone)]
pub struct OutsideRange<T>
where
    T: Debug + Display + Clone + PartialOrd,
{
    pub(crate) lower_bound: LessThanValue<T>,
    pub(crate) upper_bound: GreaterThanValue<T>,
}

impl<T: Debug + Display + Clone + PartialOrd> OutsideRange<T> {
    /// Creates a new Outside Range.  Valid values must be less than the lower bound, or greater
    /// than the upper bound.  Both bounds specify the first invalid values ob either end.
    #[must_use]
    pub fn new(lower_bound: T, upper_bound: T) -> Self {
        Self {
            lower_bound: LessThanValue::new(lower_bound),
            upper_bound: GreaterThanValue::new(upper_bound),
        }
    }

    /// Returns the lower bound of this range, valid values are less than this value
    pub fn lower_bound(&self) -> &LessThanValue<T> {
        &self.lower_bound
    }

    /// Returns the upper bound of this range, valid values are greater than this value
    pub fn upper_bound(&self) -> &GreaterThanValue<T> {
        &self.upper_bound
    }
}

impl<T: Debug + Display + Clone + PartialOrd> Range<T> for OutsideRange<T> {
    type Error = InsideRangeError<T>;

    fn value_is_valid(&self, value: &T) -> bool {
        self.lower_bound.value_is_valid(value) || self.upper_bound.value_is_valid(value)
    }

    fn check_value_is_valid(&self, value: &T) -> Result<(), Self::Error> {
        if !self.lower_bound.value_is_valid(value) && !self.upper_bound.value_is_valid(value) {
            return InsideRangeError::err(value.clone(), self.clone());
        }
        Ok(())
    }
}

///
/// An error type to indicate that the checked value is outside the specified
/// value range
#[derive(Debug, Clone)]
pub struct OutsideRangeError<T>
where
    T: Debug + Display + Clone + PartialOrd,
{
    pub(crate) value: T,
    pub(crate) valid_range: WithinRange<T>,
}

impl<T: Debug + Display + Clone + PartialOrd> OutsideRangeError<T> {
    pub fn new(value: T, valid_range: WithinRange<T>) -> Self {
        Self { value, valid_range }
    }

    pub fn err<O>(value: T, valid_range: WithinRange<T>) -> Result<O, Self> {
        Err(Self::new(value, valid_range))
    }
}

impl<T: Debug + Display + Clone + PartialOrd> Display for OutsideRangeError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "Value {} is outside valid range {} -> {}",
            self.value, self.valid_range.lower_bound.value, self.valid_range.upper_bound.value
        ))
    }
}
#[cfg(feature = "std_errors")]
impl<T: Debug + Display + Clone + PartialOrd> std::error::Error for OutsideRangeError<T> {}

///
/// An error type to indicate that the value is inside the prohibited value range
#[derive(Debug, Clone)]
pub struct InsideRangeError<T>
where
    T: Debug + Display + Clone + PartialOrd,
{
    pub(crate) value: T,
    pub(crate) valid_range: OutsideRange<T>,
}
impl<T: Debug + Display + Clone + PartialOrd> InsideRangeError<T> {
    pub fn new(value: T, valid_range: OutsideRange<T>) -> Self {
        Self { value, valid_range }
    }

    pub fn err<O>(value: T, valid_range: OutsideRange<T>) -> Result<O, Self> {
        Err(Self::new(value, valid_range))
    }
}

impl<T: Debug + Display + Clone + PartialOrd> Display for InsideRangeError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "Value {} is inside invalid range {} -> {}",
            self.value, self.valid_range.lower_bound.value, self.valid_range.upper_bound.value
        ))
    }
}
#[cfg(feature = "std_errors")]
impl<T: Debug + Display + Clone + PartialOrd> std::error::Error for InsideRangeError<T> {}
