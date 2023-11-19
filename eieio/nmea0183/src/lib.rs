// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//!
//!

#![forbid(unsafe_code)]

use std::sync::{Arc, RwLock};

use irox_eieio_api::codec::{Codec, CodecIdentifier};
use irox_eieio_api::error::{Error, ErrorType};
use irox_eieio_api::gnss_fix::OwnedGNSSFixBuilder;
use irox_eieio_api::io::{ReadFromBytes, SupportedReaders, SupportedReadersBuilder};
use irox_eieio_api::{BaseMessage, Message, MessageType};
use irox_nmea0183::{FramePayload, NMEAParser, PacketBuilder};

use crate::gnss_fix::{GNSSFixCollector, Nmea0183GnssFixBuilder};

pub mod gnss_fix;

#[derive(Debug, Default, Clone)]
pub struct NMEA0183CodecInner {
    pub gnss_collector: GNSSFixCollector,
}

impl Codec for NMEA0183Codec {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Arc<dyn Codec>
    where
        Self: Sized,
    {
        NMEA0183Codec::new()
    }

    fn get_codec_id(&self) -> CodecIdentifier {
        CodecIdentifier::new_desc("NMEA-0183", "NMEA-0183 GPS and AIS messages")
    }

    fn get_supported_builders(&self) -> Vec<MessageType> {
        vec![MessageType::GnssFix]
    }

    fn get_supported_readers(&self) -> SupportedReaders {
        SupportedReadersBuilder::new().with_bytes(self).build()
    }

    fn get_gnss_fix_builder(self: Arc<Self>) -> Option<OwnedGNSSFixBuilder> {
        Some(Box::new(Nmea0183GnssFixBuilder::new(self)))
    }
}

#[derive(Default, Clone)]
pub struct NMEA0183Codec {
    inner: Arc<RwLock<NMEA0183CodecInner>>,
}

impl NMEA0183Codec {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Arc<dyn Codec> {
        Arc::new(Self::default())
    }
}

impl ReadFromBytes for NMEA0183Codec {
    fn read_from_bytes(&mut self, mut bytes: &[u8]) -> Result<Vec<Message>, Error> {
        let frame = match NMEAParser.build_from(&mut bytes) {
            Ok(f) => f,
            Err(e) => {
                return ErrorType::ParserError("Error").source(Box::new(e));
            }
        };

        if let FramePayload::GGA(gga) = frame.payload {
            if let Ok(mut write) = self.inner.write() {
                if let Some(msg) = write
                    .gnss_collector
                    .with_gga(gga, Arc::new(Clone::clone(self)))
                {
                    return Ok(vec![msg.as_message()]);
                }
            }
        }
        Ok(vec![])
    }
}
