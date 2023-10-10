// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Debug, Display, Formatter};

pub struct Bounds<T> {
    upper_left: T,
    upper_right: T,
    lower_left: T,
    lower_right: T,
}

impl<T> Bounds<T> {
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
    pub const fn new(value: T) -> Self {
        Self { value }
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "value {} is less than or equal to {}",
            self.value, self.valid_range.value
        ))
    }
}

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
    pub const fn new(value: T) -> Self {
        Self { value }
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "the value {} is greater than or equal to {}",
            self.value, self.valid_range.value
        ))
    }
}
impl<T: Debug + Display + Clone + PartialOrd> std::error::Error
    for GreaterThanEqualToValueError<T>
{
}

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
    pub const fn new(lower_bound: T, upper_bound: T) -> Self {
        Self {
            lower_bound: GreaterThanValue::new(lower_bound),
            upper_bound: LessThanValue::new(upper_bound),
        }
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Value {} is outside valid range {} -> {}",
            self.value, self.valid_range.lower_bound.value, self.valid_range.upper_bound.value
        ))
    }
}
impl<T: Debug + Display + Clone + PartialOrd> std::error::Error for OutsideRangeError<T> {}

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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Value {} is inside invalid range {} -> {}",
            self.value, self.valid_range.lower_bound.value, self.valid_range.upper_bound.value
        ))
    }
}
impl<T: Debug + Display + Clone + PartialOrd> std::error::Error for InsideRangeError<T> {}
