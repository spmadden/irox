// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

///
/// Like TryInto<T> but instead of returning a Result<Ok, Err> it returns simply an
/// Option<T>.  Semantically identical to Result<Ok, Err>.ok()
pub trait MaybeInto<T> {
    fn maybe_into(self) -> Option<T>;
}

///
/// Like TryFrom<T> but instead of returning a Result<Ok, Err> it returns simply an
/// Option<T>.  Semantically identical to Result<Ok, Err>.ok()
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
