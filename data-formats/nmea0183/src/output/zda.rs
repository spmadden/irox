// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use crate::{calculate_checksum, Error, MessageType};
use irox_time::datetime::UTCDateTime;
use irox_time::gregorian::Date;
use irox_time::Time;
use irox_tools::format::DecimalFormatF64;
use irox_tools::packetio::Packet;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ZDA {
    pub utc_time: Option<Time>,
    pub utc_day: Option<u8>,
    pub utc_month: Option<u8>,
    pub utc_year: Option<i32>,
    pub utc_local_zone_hours: Option<i8>,
    pub utc_local_zone_minute: Option<u8>,
}

impl Packet for ZDA {
    type PacketType = MessageType;

    fn get_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        use std::io::Write;
        let mut buf: Vec<u8> = Vec::new();

        let utctime = self
            .utc_time
            .map(|timestamp| {
                let (hh, mm, ss) = timestamp.as_hms_f64();
                format!("{hh:02}{mm:02}{}", DecimalFormatF64(2, 2, ss))
            })
            .unwrap_or_default();
        let day = self
            .utc_day
            .map(|day| format!("{day:02}"))
            .unwrap_or_default();
        let month = self
            .utc_month
            .map(|month| format!("{month:02}"))
            .unwrap_or_default();
        let year = self
            .utc_year
            .map(|year| format!("{year:04}"))
            .unwrap_or_default();
        let lzh = self
            .utc_local_zone_hours
            .map(|lzh| format!("{lzh:02}"))
            .unwrap_or("00".to_string());
        let lzm = self
            .utc_local_zone_minute
            .map(|lzm| format!("{lzm:02}"))
            .unwrap_or("00".to_string());
        buf.write_fmt(format_args!(
            "$GPZDA,{utctime},{day},{month},{year},{lzh},{lzm}*"
        ))?;

        let csh = calculate_checksum(&buf);
        buf.write_fmt(format_args!("{csh:02X}\r\n"))?;

        Ok(buf)
    }

    fn get_type(&self) -> Self::PacketType {
        MessageType::ZDA
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct ZDABuilder {
    zda: ZDA,
}

impl ZDABuilder {
    pub fn with_utc_time(&mut self, time: Time) -> &mut Self {
        self.zda.utc_time = Some(time);
        self
    }
    pub fn with_utc_day(&mut self, day: u8) -> &mut Self {
        self.zda.utc_day = Some(day);
        self
    }
    pub fn with_utc_month(&mut self, month: u8) -> &mut Self {
        self.zda.utc_month = Some(month);
        self
    }

    pub fn with_utc_year(&mut self, year: i32) -> &mut Self {
        self.zda.utc_year = Some(year);
        self
    }

    pub fn with_utc_local_zone_hours(&mut self, hours: i8) -> &mut Self {
        self.zda.utc_local_zone_hours = Some(hours);
        self
    }

    pub fn with_utc_local_zone_minutes(&mut self, minutes: u8) -> &mut Self {
        self.zda.utc_local_zone_minute = Some(minutes);
        self
    }

    pub fn with_date(&mut self, date: Date) -> &mut Self {
        self.with_utc_day(date.day_of_month());
        self.with_utc_month(date.month_of_year() as u8);
        self.with_utc_year(date.year());
        self
    }

    pub fn with_datetime(&mut self, datetime: UTCDateTime) -> &mut Self {
        self.with_date(datetime.get_date());
        self.with_utc_time(datetime.get_time());
        self
    }

    pub fn build(self) -> ZDA {
        self.zda
    }
}

impl From<UTCDateTime> for ZDA {
    fn from(value: UTCDateTime) -> Self {
        let date = value.get_date();
        let utc_day = Some(date.day_of_month());
        let utc_month = Some(date.month_of_year() as u8);
        let utc_year = Some(date.year());
        ZDA {
            utc_time: Some(value.get_time()),
            utc_day,
            utc_month,
            utc_year,
            utc_local_zone_hours: Some(0),
            utc_local_zone_minute: Some(0),
        }
    }
}

impl TryFrom<ZDA> for UTCDateTime {
    type Error = Error;

    fn try_from(value: ZDA) -> Result<Self, Self::Error> {
        let Some(time) = value.utc_time else {
            return Error::missing_err("time");
        };
        let Some(day) = value.utc_day else {
            return Error::missing_err("day");
        };
        let Some(month) = value.utc_month else {
            return Error::missing_err("month");
        };
        let Some(year) = value.utc_year else {
            return Error::missing_err("year");
        };
        let date = Date::try_from_values(year, month, day)?;

        Ok(UTCDateTime::new(date, time))
    }
}
