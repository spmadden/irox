// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::gsa::GNSSSystemID;
use crate::{
    maybe_date, maybe_latitude, maybe_longitude, maybe_speed, maybe_timestamp, maybe_track,
    MessageType, ModeIndicator,
};
use irox_bits::{Bits, Error};
use irox_carto::coordinate::{Latitude, Longitude};
use irox_time::gregorian::Date;
use irox_time::Time;
use irox_tools::packetio::{Packet, PacketBuilder};
use irox_units::units::angle::Angle;
use irox_units::units::compass::Track;
use irox_units::units::speed::Speed;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum RMCStatus {
    Valid,
    Warning,
    #[default]
    UnknownUnset,
}
impl From<Option<&str>> for RMCStatus {
    fn from(value: Option<&str>) -> Self {
        if let Some(val) = value {
            return match val {
                "A" => RMCStatus::Valid,
                "V" => RMCStatus::Warning,
                _ => RMCStatus::UnknownUnset,
            };
        }
        Default::default()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RMC {
    system_id: GNSSSystemID,
    timestamp: Option<Time>,
    status: RMCStatus,
    latitude: Option<Latitude>,
    longitude: Option<Longitude>,
    speed: Option<Speed>,
    track: Option<Track>,
    date: Option<Date>,
    magvar: Option<Angle>,
    faa_mode: ModeIndicator,
    nav_mode: ModeIndicator,
}
impl Display for RMC {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SYSTEM[{}] ", self.system_id)?;
        write!(f, "FIX[{:?}] ", self.status)?;
        if let Some(ts) = self.timestamp {
            write!(f, "Time[{ts}] ")?;
        }
        if let Some(lat) = self.latitude {
            write!(f, " {lat} ")?;
        }
        if let Some(lon) = self.longitude {
            write!(f, " {lon} ")?;
        }
        if let Some(spd) = self.speed {
            write!(f, "SPD[{spd}] ")?;
        }
        if let Some(trk) = self.track {
            write!(f, "TRK[{trk}] ",)?;
        }
        if let Some(date) = self.date {
            write!(f, "Date[{date}] ")?;
        }
        if let Some(mv) = self.magvar {
            write!(f, "MV[{mv}] ")?;
        }
        write!(f, "MODE1[{:?}] ", self.faa_mode)?;
        write!(f, "MODE2[{:?}] ", self.nav_mode)?;
        Ok(())
    }
}
impl Packet for RMC {
    type PacketType = MessageType;

    fn get_bytes(&self) -> Result<Vec<u8>, Error> {
        todo!()
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct RMCBuilder;
impl PacketBuilder<RMC> for RMCBuilder {
    type Error = Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<RMC, Self::Error> {
        let buf = input.read_all_str_lossy()?;

        let mut split = buf.split(',');
        let system_id = split
            .next()
            .map(GNSSSystemID::from_sender)
            .unwrap_or_default();
        let timestamp = maybe_timestamp(split.next());
        let status = split.next().into();
        let latitude = maybe_latitude(split.next(), split.next());
        let longitude = maybe_longitude(split.next(), split.next());
        let speed = maybe_speed(split.next());
        let track = maybe_track(split.next());
        let date = maybe_date(split.next());
        let magvar = maybe_longitude(split.next(), split.next()).map(|v| v.0);
        let faa_mode = split.next().into();
        let nav_mode = split.next().into();

        Ok(RMC {
            system_id,
            timestamp,
            status,
            latitude,
            longitude,
            speed,
            track,
            date,
            magvar,
            faa_mode,
            nav_mode,
        })
    }
}
