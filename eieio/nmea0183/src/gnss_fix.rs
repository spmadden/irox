// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_eieio_api::carto::coordinate::EllipticalCoordinateBuilder;
use irox_eieio_api::carto::geo::standards::wgs84::WGS84_SHAPE;
use irox_eieio_api::carto::position_type::{Positions, WGS84Position};
use irox_eieio_api::codec::Codec;
use irox_eieio_api::error::Error;
use irox_eieio_api::gnss_fix::{GNSSFix, GNSSFixBuilder, OwnedGNSSFix};
use irox_eieio_api::io::{SupportedWriters, SupportedWritersBuilder, WriteToBytes, WriteToString};
use irox_eieio_api::time::datetime::UTCDateTime;
use irox_eieio_api::{BaseMessage, Message, MessageType};
use irox_nmea0183::gga::{GGABuilder, GGA};
use irox_nmea0183::zda::{ZDABuilder, ZDA};
use irox_nmea0183::Packet;
use std::sync::Arc;

use crate::NMEA0183Codec;

#[derive(Clone)]
pub struct GNSSFixImpl {
    pub gga: GGA,
    pub zda: ZDA,
    pub codec: Arc<NMEA0183Codec>,
}

impl WriteToBytes for GNSSFixImpl {
    fn get_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut buf: Vec<u8> = Vec::new();
        self.gga.write_to(&mut buf)?;
        self.zda.write_to(&mut buf)?;
        Ok(buf)
    }
}
impl WriteToString for GNSSFixImpl {
    fn write_to_string(&self) -> Result<String, Error> {
        Ok(String::from_utf8(self.get_bytes()?)?)
    }
}

impl BaseMessage for GNSSFixImpl {
    fn get_supported_writers(&self) -> SupportedWriters<'_> {
        SupportedWritersBuilder::new()
            .with_bytes(self)
            .with_string(self)
            .build()
    }

    fn get_message_type(&self) -> MessageType {
        MessageType::GnssFix
    }

    fn as_message(&self) -> Message {
        Message::GnssFix(Box::new(Clone::clone(self)))
    }

    fn get_codec(&self) -> Arc<dyn Codec> {
        self.codec.clone()
    }
}

impl GNSSFix for GNSSFixImpl {
    fn get_super(&self) -> &dyn BaseMessage {
        self
    }

    fn get_positions(&self) -> Positions {
        let mut bldr = EllipticalCoordinateBuilder::new();
        if let Some(lat) = &self.gga.latitude() {
            bldr.with_latitude(*lat);
        }
        if let Some(lon) = &self.gga.longitude() {
            bldr.with_longitude(*lon);
        }
        if let Some(alt) = &self.gga.ant_alt() {
            bldr.with_altitude(*alt);
        }
        bldr.with_reference_frame(WGS84_SHAPE);

        Positions {
            ecef: None,
            latlon: bldr.build().ok().map(WGS84Position),
            enu: None,
            ned: None,
        }
    }

    fn get_timestamp(&self) -> UTCDateTime {
        self.zda.try_into().unwrap_or_default()
    }
}

pub struct Nmea0183GnssFixBuilder {
    gga: GGABuilder,
    zda: ZDABuilder,
    codec: Arc<NMEA0183Codec>,
}

impl Nmea0183GnssFixBuilder {
    pub fn new(codec: Arc<NMEA0183Codec>) -> Self {
        Self {
            gga: Default::default(),
            zda: Default::default(),
            codec,
        }
    }
}

impl GNSSFixBuilder for Nmea0183GnssFixBuilder {
    fn set_timestamp(&mut self, timestamp: UTCDateTime) {
        self.gga.set_timestamp(timestamp.get_time());
        self.zda.with_datetime(timestamp);
    }

    fn set_positions(&mut self, positions: Positions) {
        if let Some(wgs84) = positions.latlon {
            let latlon = wgs84.0;
            self.gga.set_latitude(*latlon.get_latitude());
            self.gga.set_longitude(*latlon.get_longitude());
        }
    }

    fn build(&self) -> Result<OwnedGNSSFix, Error> {
        Ok(Box::new(GNSSFixImpl {
            gga: self.gga.build(),
            zda: self.zda.build(),
            codec: self.codec.clone(),
        }))
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct GNSSFixCollector {
    pub gga: Option<GGA>,
    pub zda: Option<ZDA>,
}

impl GNSSFixCollector {
    pub fn with_gga(&mut self, gga: GGA, codec: Arc<NMEA0183Codec>) -> Option<GNSSFixImpl> {
        self.gga = Some(gga);
        if let Some(zda) = self.zda {
            return Some(GNSSFixImpl { gga, zda, codec });
        }
        None
    }

    pub fn with_zda(&mut self, zda: ZDA, codec: Arc<NMEA0183Codec>) -> Option<GNSSFixImpl> {
        self.zda = Some(zda);
        if let Some(gga) = self.gga {
            return Some(GNSSFixImpl { gga, zda, codec });
        }
        None
    }
}
