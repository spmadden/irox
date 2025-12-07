// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Codec - encodes or decodes a particular data type.  This is one of the main entry points.

use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use crate::gnss_fix::OwnedGNSSFixBuilder;
use crate::io::SupportedReaders;
use crate::MessageType;

pub type OwnedCodec = Box<dyn Codec>;
pub type SharedCodec = Arc<dyn Codec>;

///
/// A Codec is a mechanization of a serialization or deserialization operation.  Essentially,
/// it is a wrapper around some other data format, and provides a consistent API to access that
/// underlying data format.
pub trait Codec {
    /// Constructs a new Codec, forcing the type to be erased.
    #[allow(clippy::new_ret_no_self)]
    fn new() -> SharedCodec
    where
        Self: Sized;

    ///
    /// Returns the unique identifier for this codec implementation
    fn get_codec_id(&self) -> CodecIdentifier<'_>;

    ///
    /// Returns a list of the [`MessageType`]s that are supported.  It is expected that if the
    /// message type is present in this list, then the associated 'get_XYZ_builder' method will
    /// always return `Some` rather than `None`
    fn get_supported_builders(&self) -> Vec<MessageType>;

    ///
    /// Returns a set of supported reading/parsing operations.
    ///
    /// Writing operations are performed on the message object itself.
    fn get_supported_readers(&self) -> SupportedReaders<'_>;

    ///
    /// Returns a new builder for the [`crate::GNSSFix`] type
    fn get_gnss_fix_builder(self: Arc<Self>) -> Option<OwnedGNSSFixBuilder>;
}

macro_rules! impl_codec_debug {
    ($id:ty) => {
        impl Debug for $id {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(self.get_codec_id().name())
                    .field("id", &self.get_codec_id())
                    .field("supported builders", &self.get_supported_builders())
                    .field("supported readers", &self.get_supported_readers())
                    .finish_non_exhaustive()
            }
        }
    };
}

impl_codec_debug!(dyn Codec);

///
/// A Codec Identifier is a way to uniquely identify a particular implementation of a Codec.  It
/// has a mandatory name, with an optional description and version.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct CodecIdentifier<'a> {
    /// Name of this codec.  The name is generally used in user-interfaces or as a grouping element
    /// for codecs with multiple versions
    name: &'a str,

    /// The human-friendly description of this codec.
    description: Option<&'a str>,

    /// The optional version of this codec.
    version: Option<&'a str>,
}

impl<'a> CodecIdentifier<'a> {
    pub fn new(name: &'a str) -> Self {
        Self::new_opt_desc_ver(name, None, None)
    }
    pub fn new_desc(name: &'a str, desc: &'a str) -> Self {
        Self::new_opt_desc_ver(name, Some(desc), None)
    }
    pub fn new_desc_version(name: &'a str, description: &'a str, version: &'a str) -> Self {
        Self::new_opt_desc_ver(name, Some(description), Some(version))
    }

    pub fn new_opt_desc_ver(
        name: &'a str,
        description: Option<&'a str>,
        version: Option<&'a str>,
    ) -> Self {
        CodecIdentifier {
            name,
            description,
            version,
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }
    pub fn description(&self) -> Option<&'a str> {
        self.description
    }
    pub fn version(&self) -> Option<&'a str> {
        self.version
    }
}
