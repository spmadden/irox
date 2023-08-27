// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};

use syn::{Expr, Field, Lit, Type, TypeArray, TypePath};

use crate::{NamedPrimitive, PrimitiveType, Primitives};

#[derive(Debug, Copy, Clone)]
pub enum ErrorType {
    MissingIdent,
    PathElements,
    BadType,
}
#[derive(Debug, Clone)]
pub struct Error {
    error_type: ErrorType,
    error: String,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Error({:?}): {}", self.error_type, self.error))
    }
}
impl std::error::Error for Error {}
impl Error {
    fn new(error_type: ErrorType, error: &'static str) -> Error {
        Error {
            error_type,
            error: String::from(error),
        }
    }
    fn new_str(error_type: ErrorType, error: String) -> Error {
        Error { error_type, error }
    }
    fn missing_ident<T>() -> Result<T, Self> {
        Err(Error::new(
            ErrorType::MissingIdent,
            "Field is missing identifier",
        ))
    }
    fn path_elements<T>(num_elems: usize) -> Result<T, Self> {
        Err(Error::new_str(
            ErrorType::PathElements,
            format!("Wrong number of path elements: {num_elems} expected 1"),
        ))
    }
    fn bad_type<T>(typ: String) -> Result<T, Self> {
        Err(Error::new_str(ErrorType::BadType, typ))
    }
}

impl TryFrom<&TypePath> for Primitives {
    type Error = Error;

    fn try_from(value: &TypePath) -> Result<Self, Self::Error> {
        let len = value.path.segments.len();
        if len != 1 {
            return Error::path_elements(len);
        }
        let Some(elem) = value.path.segments.first() else {
            return Error::path_elements(0);
        };
        let ident = format!("{}", elem.ident);
        Primitives::try_from(ident.as_str())
            .map_err(|_| Error::new_str(ErrorType::BadType, format!("Bad type: {ident}")))
    }
}

impl TryFrom<TypePath> for Primitives {
    type Error = Error;

    fn try_from(value: TypePath) -> Result<Self, Self::Error> {
        Primitives::try_from(&value)
    }
}

impl TryFrom<&Field> for NamedPrimitive {
    type Error = Error;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        let Some(ident) = &value.ident else {
            return Error::missing_ident();
        };
        let ident = format!("{ident}");
        let Type::Path(path) = &value.ty else {
            return Error::bad_type(format!("Not a TypePath: {ident}"));
        };
        let primitive: Primitives = path.try_into()?;
        Ok(NamedPrimitive {
            name: ident,
            primitive,
        })
    }
}

impl TryFrom<Field> for NamedPrimitive {
    type Error = Error;

    fn try_from(value: Field) -> Result<Self, Self::Error> {
        NamedPrimitive::try_from(&value)
    }
}

impl TryFrom<&Field> for Primitives {
    type Error = Error;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        let Some(ident) = &value.ident else {
            return Error::missing_ident();
        };
        let ident = format!("{ident}");
        match &value.ty {
            Type::Path(path) => {
                let prim: Primitives = path.try_into()?;
                Ok(prim)
            }
            Type::Array(ref arr) => match &arr.elem.as_ref() {
                Type::Path(path) => {
                    let prim: Primitives = path.try_into()?;
                    Ok(prim)
                }
                _ => Error::bad_type(format!("ARRAY: {arr:?}")),
            },
            _ => Error::bad_type(format!("Not a TypePath: {ident}")),
        }
    }
}

impl TryFrom<Field> for Primitives {
    type Error = Error;

    fn try_from(value: Field) -> Result<Self, Self::Error> {
        Primitives::try_from(&value)
    }
}

fn try_get_array_size(arr: &TypeArray) -> Result<usize, Error> {
    let Expr::Lit(lit) = &arr.len else {
        return Error::bad_type("Array length not a literal".to_string());
    };
    let Lit::Int(lit) = &lit.lit else {
        return Error::bad_type("Literal was not an int".to_string());
    };

    let Ok(value) = lit.base10_parse::<usize>() else {
        return Error::bad_type("Unable to parse literal as base10".to_string());
    };
    Ok(value)
}

impl TryFrom<&Field> for PrimitiveType {
    type Error = Error;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        let Some(ident) = &value.ident else {
            return Error::missing_ident();
        };
        let ident = format!("{ident}");
        match &value.ty {
            Type::Path(path) => {
                let prim: Primitives = path.try_into()?;
                Ok(PrimitiveType::Primitive(prim))
            }
            Type::Array(ref arr) => {
                let size = try_get_array_size(arr)?;
                match &arr.elem.as_ref() {
                    Type::Path(path) => {
                        let prim: Primitives = path.try_into()?;
                        Ok(PrimitiveType::Array(prim, size))
                    }
                    _ => Error::bad_type(format!("ARRAY: {arr:?}")),
                }
            }
            _ => Error::bad_type(format!("Not a TypePath: {ident}")),
        }
    }
}

impl TryFrom<Field> for PrimitiveType {
    type Error = Error;

    fn try_from(value: Field) -> Result<Self, Self::Error> {
        PrimitiveType::try_from(&value)
    }
}
