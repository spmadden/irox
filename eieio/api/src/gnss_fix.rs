// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! A GNSS Fix with a Position and Timestamp

use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use irox_carto::position_type::Positions;
use irox_time::datetime::UTCDateTime;

use crate::codec::Codec;
use crate::error::{Error, ErrorType};
use crate::io::SupportedWriters;
use crate::{BaseMessage, Message, MessageType};

/// An Owned version of a [`GNSSFix`]
pub type OwnedGNSSFix = Box<dyn GNSSFix>;
pub type BorrowedGNSSFix<'a> = &'a Box<dyn GNSSFix>;

/// An Owned version of a [`GNSSFixBuilder`]
pub type OwnedGNSSFixBuilder = Box<dyn GNSSFixBuilder>;

///
/// A GNSS Fix is the result of a Global Navigation Satellite System localization operation.
/// It has a position, timestamp, and some other associated metadata.
pub trait GNSSFix {
    /// Returns a reference to the parent [`BaseMessage`] for this GNSSFix
    fn get_super(&self) -> &dyn BaseMessage;

    /// Returns the position(s) associated with this fix.  If the fix is not yet complete or has
    /// been lost, this structure may be empty.
    fn get_positions(&self) -> Positions;

    /// Returns the UTC Date & Time associated with this fix.
    fn get_timestamp(&self) -> UTCDateTime;

    /// Makes a copy of this object using the Codec's builder.  Most implementations
    /// SHOULD override this with an implementation of [`Clone`]
    fn clone(&self) -> Result<Box<dyn GNSSFix>, Error> {
        let Some(mut bldr) = self.get_super().get_codec().get_gnss_fix_builder() else {
            return ErrorType::BuilderNotSupported("GNSSFixBuilder").error();
        };
        bldr.set_positions(self.get_positions());
        bldr.set_timestamp(self.get_timestamp());
        bldr.build()
    }
}

macro_rules! impl_base {
    ($e:ty) => {
        impl BaseMessage for $e {
            fn get_supported_writers(&self) -> SupportedWriters {
                self.get_super().get_supported_writers()
            }

            fn get_message_type(&self) -> MessageType {
                self.get_super().get_message_type()
            }

            fn as_message(&self) -> Message {
                self.get_super().as_message()
            }

            fn get_codec(&self) -> Arc<dyn Codec> {
                self.get_super().get_codec()
            }
        }
    };
}

impl_base!(&dyn GNSSFix);
impl_base!(&mut dyn GNSSFix);
impl_base!(dyn GNSSFix);
impl Debug for dyn GNSSFix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("dyn GNSSFix")
            .field("positions", &self.get_positions())
            .field("timestamp", &self.get_timestamp())
            .finish_non_exhaustive()
    }
}

///
/// A builder to create a new [`GNSSFix`] message.
pub trait GNSSFixBuilder {
    ///
    /// Seeds this builder with the data from another [`GNSSFix`] message
    fn load_from(&mut self, other: BorrowedGNSSFix) {
        self.set_positions(other.get_positions());
        self.set_timestamp(other.get_timestamp())
    }

    /// Sets the timestamp of the new [`GNSSFix`]
    fn set_timestamp(&mut self, timestamp: UTCDateTime);

    /// Sets the positions of the new [`GNSSFix`]
    fn set_positions(&mut self, positions: Positions);

    /// Attempts to build the new [`GNSSFix`] or an error.  Different implementations may error
    /// in different ways, some implementations may require certain fields be set while others
    /// may not.
    fn build(&self) -> Result<Box<dyn GNSSFix>, Error>;
}
