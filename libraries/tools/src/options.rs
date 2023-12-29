// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! `MaybeInto`, `MaybeFrom`, and `MaybeMap` - traits to improve chaining of optional functions and erasing `Result`s
//!

///
/// Like [`TryInto<T>`] but instead of returning a [`Result<Ok, Err>`] it returns simply an
/// [`Option<T>`].  Semantically identical to `Result<Ok, Err>.ok()`
pub trait MaybeInto<T> {
    fn maybe_into(self) -> Option<T>;
}

///
/// Like [`TryFrom<T>`] but instead of returning a [`Result<Ok, Err>`] it returns simply an
/// [`Option<T>`].  Semantically identical to `Result<Ok, Err>.ok()`
pub trait MaybeFrom<T>: Sized {
    fn maybe_from(value: T) -> Option<Self>;
}

impl<T, U> MaybeInto<U> for T
where
    U: MaybeFrom<T>,
{
    fn maybe_into(self) -> Option<U> {
        U::maybe_from(self)
    }
}
macro_rules! maybe_from_prim_str {
    ($type:ident) => {
        impl MaybeFrom<Option<&str>> for $type {
            fn maybe_from(value: Option<&str>) -> Option<$type> {
                value?.parse().ok()
            }
        }
        impl MaybeFrom<&str> for $type {
            fn maybe_from(value: &str) -> Option<$type> {
                value.parse().ok()
            }
        }
        impl MaybeFrom<Option<$type>> for $type {
            fn maybe_from(value: Option<$type>) -> Option<$type> {
                value
            }
        }
    };
}
maybe_from_prim_str!(u8);
maybe_from_prim_str!(i8);
maybe_from_prim_str!(u16);
maybe_from_prim_str!(i16);
maybe_from_prim_str!(u32);
maybe_from_prim_str!(i32);
maybe_from_prim_str!(u64);
maybe_from_prim_str!(i64);
maybe_from_prim_str!(u128);
maybe_from_prim_str!(i128);
maybe_from_prim_str!(f32);
maybe_from_prim_str!(f64);
maybe_from_prim_str!(char);

pub trait MaybeMap<Base, Out>: Sized {
    fn maybe_map<R: FnOnce(Base) -> Option<Out>>(self, map: R) -> Option<Out>;
}

impl<Base, Out> MaybeMap<Base, Out> for Option<Base> {
    fn maybe_map<R: FnOnce(Base) -> Option<Out>>(self, map: R) -> Option<Out> {
        let Some(e) = self else {
            return None;
        };
        map(e)
    }
}
