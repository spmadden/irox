// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! IROX EIEIO API Traits - A system of abstracting common data-structures behind facades
//!

#![forbid(unsafe_code)]

pub use irox_carto as carto;
pub use irox_time as time;

use crate::codec::SharedCodec;
use crate::gnss_fix::OwnedGNSSFix;

pub use codec::Codec;
pub use gnss_fix::{GNSSFix, GNSSFixBuilder};
pub use io::*;

pub mod codec;
pub mod error;
pub mod gnss_fix;
pub mod io;

///
/// The various different message types that could be supported by a [`Codec`]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MessageType {
    /// A Global Navigational Satellite System Fix, a position from a GNSS Satellite
    GnssFix,

    /// Last case, unknown, not specified, or 'other' message.
    UnknownOther(&'static str),
}

///
/// A typed shuttle to allow owned messages to be passed around without generics or dynamic types.
#[non_exhaustive]
pub enum Message {
    /// A Global Navigational Satellite System Fix, a position from a GNSS Satellite
    GnssFix(OwnedGNSSFix),

    /// An owned "generic" message that can shuttle any message type around.
    UnknownOther(Box<dyn BaseMessage>),
}

///
/// The top-level common metadata for all messages.  If Rust were a real polymorphic object-oriented
/// language, then this would be the top of the polymorphism tree.
pub trait BaseMessage {
    /// Returns a set of encoding methods that this particular message supports.  If no encodings
    /// are possible, then this will be an empty struct full of [`None`]s.
    fn get_supported_writers(&self) -> SupportedWriters<'_>;

    ///
    /// Returns the [`MessageType`] for this message.
    fn get_message_type(&self) -> MessageType;

    ///
    /// Converts this message into a generic "Owned" type.
    fn as_message(&self) -> Message;

    ///
    /// Returns a reference to the [`Codec`] that generated this message.
    fn get_codec(&self) -> SharedCodec;
}
