// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! An Identifier represents a way to uniquely identify an item, whether as a [`String`], [`u64`],
//! or [`UUID`].
//!

extern crate alloc;
use crate::cfg_feature_alloc;
use crate::hash::murmur3_128;
use crate::uuid::UUID;
use alloc::string::{String, ToString};
use core::fmt::{Display, Formatter};

///
/// Represents a way to uniquely identify an item.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Identifier {
    Integer(u64),
    String(String),
    UUID(UUID),
}

impl Identifier {
    /// Represents this identifier as an [`u64`] type
    #[must_use]
    pub fn as_integer(&self) -> Identifier {
        match self {
            Identifier::Integer(i) => Identifier::Integer(*i),
            Identifier::String(s) => {
                let hash = murmur3_128(s.as_bytes());
                Identifier::Integer(hash as u64)
            }
            Identifier::UUID(u) => {
                let inner: u128 = u.into();
                Identifier::Integer(inner as u64)
            }
        }
    }

    /// Represents this identifier as a [`String`] type
    #[must_use]
    pub fn as_string(&self) -> Identifier {
        match self {
            Identifier::Integer(i) => Identifier::String(crate::format!("{i}")),
            Identifier::String(s) => Identifier::String(s.clone()),
            Identifier::UUID(u) => Identifier::String(crate::format!("{u}")),
        }
    }

    /// Represents this identifier as a [`UUID`] type
    #[must_use]
    pub fn as_uuid(&self) -> Identifier {
        match self {
            Identifier::Integer(i) => {
                let inner: u128 = *i as u128;
                Identifier::UUID(inner.into())
            }
            Identifier::String(s) => {
                let inner: u128 = murmur3_128(s);
                Identifier::UUID(inner.into())
            }
            Identifier::UUID(u) => Identifier::UUID(*u),
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Identifier::Integer(i) => f.write_fmt(format_args!("{i}")),
            Identifier::String(s) => f.write_fmt(format_args!("{s}")),
            Identifier::UUID(u) => f.write_fmt(format_args!("{u}")),
        }
    }
}

impl From<u64> for Identifier {
    fn from(value: u64) -> Self {
        Identifier::Integer(value)
    }
}

impl From<&u64> for Identifier {
    fn from(value: &u64) -> Self {
        Identifier::Integer(*value)
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Identifier::String(value)
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Identifier::String(value.to_string())
    }
}

impl From<UUID> for Identifier {
    fn from(value: UUID) -> Self {
        Identifier::UUID(value)
    }
}

impl From<&UUID> for Identifier {
    fn from(value: &UUID) -> Self {
        Identifier::UUID(*value)
    }
}

cfg_feature_alloc! {
    crate::cfg_feature_std!{
        use crate::random::PRNG;
        impl Identifier {
            pub fn random_int() -> Identifier {
                let id = crate::random::system_random().prng(PRNG::next_u64);
                Identifier::Integer(id)
            }
            pub fn random_string() -> Identifier {
                let b : [u8;4]= crate::random::system_random().next_u32().to_be_bytes();
                let mut s = String::with_capacity(19);
                let _ = crate::hash::bytewords::write_words(&b, "-", &mut s);
                Identifier::String(s)
            }
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct SharedIdentifier(alloc::sync::Arc<Identifier>);
    impl core::ops::Deref for SharedIdentifier {
        type Target = Identifier;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl From<Identifier> for SharedIdentifier {
        fn from(value: Identifier) -> Self {
            SharedIdentifier(alloc::sync::Arc::new(value))
        }
    }
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use crate::identifier::Identifier;

    #[test]
    pub fn test_identifier() {
        let i = Identifier::random_int();
        let s = Identifier::random_string();
        println!("{i}, {s}");
        assert_ne!(i, s);
    }
}
