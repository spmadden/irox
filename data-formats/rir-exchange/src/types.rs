// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::net::{Ipv4Addr, Ipv6Addr};
use std::time::Duration;

use irox_carto::countrycodes::CountryCode;
use irox_enums::EnumName;
use irox_time::format::iso8601::BASIC_CALENDAR_DATE;
use irox_time::gregorian::Date;

use crate::{Error, RegionalRegistry};

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumName)]
pub enum EntryType {
    ASN,
    IPv4,
    IPv6,
}

impl TryFrom<&String> for EntryType {
    type Error = Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "asn" => EntryType::ASN,
            "ipv4" => EntryType::IPv4,
            "ipv6" => EntryType::IPv6,
            e => return Error::invalid(format!("Invalid value for EntryType: {e}")),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EntryValue {
    ASN(u64),
    IPv4(Ipv4Addr),
    IPv6(Ipv6Addr),
}

impl EntryValue {
    pub fn try_into(val: &str, entry: EntryType) -> Result<EntryValue, Error> {
        Ok(match entry {
            EntryType::ASN => EntryValue::ASN(val.parse()?),
            EntryType::IPv4 => EntryValue::IPv4(val.parse()?),
            EntryType::IPv6 => EntryValue::IPv6(val.parse()?),
        })
    }
    pub fn value(&self) -> String {
        match self {
            EntryValue::ASN(e) => format!("{e}"),
            EntryValue::IPv4(e) => format!("{e}"),
            EntryValue::IPv6(e) => format!("{e}"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Status {
    Allocated,
    Available,
    Other,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VersionLine {
    version: String,
    registry: RegionalRegistry,
    serial: String,
    record_count: u64,
    start_date: Date,
    end_date: Date,
    utc_offset: Duration,
}

impl TryFrom<Vec<String>> for VersionLine {
    type Error = Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.len() != 7 {
            return Error::invalid(format!(
                "Invalid number of entries for version, expecting 7 was {}",
                value.len()
            ));
        }
        let Some(version) = value.get(0) else {
            return Error::missing("Missing value for version");
        };
        let Some(registry) = value.get(1) else {
            return Error::missing("Missing value for registry");
        };
        let registry: RegionalRegistry = registry.try_into()?;
        let Some(serial) = value.get(2) else {
            return Error::missing("Missing value for serial");
        };
        let Some(record_count) = value.get(3) else {
            return Error::missing("Missing value for record count");
        };
        let record_count: u64 = record_count.parse()?;
        let Some(start_date) = value.get(4) else {
            return Error::missing("Missing value for start date");
        };
        let start_date = Date::parse_from(&BASIC_CALENDAR_DATE, start_date).unwrap_or_default();
        let Some(end_date) = value.get(5) else {
            return Error::missing("Missing value for end date");
        };
        let end_date = Date::parse_from(&BASIC_CALENDAR_DATE, end_date)?;
        let Some(_utc_offset) = value.get(6) else {
            return Error::missing("Missing value for UTC Offset");
        };

        let utc_offset = Duration::default();
        Ok(VersionLine {
            version: version.clone(),
            registry,
            serial: serial.clone(),
            record_count,
            start_date,
            end_date,
            utc_offset,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SummaryLine {
    registry: RegionalRegistry,
    entry_type: EntryType,
    count: u32,
}

impl TryFrom<Vec<String>> for SummaryLine {
    type Error = Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.len() != 6 {
            return Error::invalid(format!(
                "Invalid number of entries for summary, expecting 6 was {}",
                value.len()
            ));
        }
        let Some(registry) = value.get(0) else {
            return Error::missing("Missing value for registry");
        };
        let registry: RegionalRegistry = registry.try_into()?;
        let Some(entry_type) = value.get(2) else {
            return Error::missing("Missing value for entry");
        };
        let entry_type: EntryType = entry_type.try_into()?;
        let Some(count) = value.get(4) else {
            return Error::missing("Missing value for count");
        };
        let count: u32 = count.parse()?;
        Ok(SummaryLine {
            registry,
            entry_type,
            count,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Record {
    registry: RegionalRegistry,
    country_code: Option<CountryCode>,
    entry_type: EntryType,
    start_value: EntryValue,
    value: u32,
    date: Option<Date>,
    opaque_id: Option<String>,
}

impl Record {
    pub fn get_registry(&self) -> RegionalRegistry {
        self.registry
    }
    pub fn get_country_code(&self) -> Option<CountryCode> {
        self.country_code
    }
    pub fn get_entry_type(&self) -> EntryType {
        self.entry_type
    }
    pub fn get_start_value(&self) -> EntryValue {
        self.start_value
    }
    pub fn get_count(&self) -> u32 {
        self.value
    }
    pub fn get_date(&self) -> Option<Date> {
        self.date
    }
    pub fn get_opaque_id(&self) -> Option<&String> {
        self.opaque_id.as_ref()
    }
}

impl TryFrom<Vec<String>> for Record {
    type Error = Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let Some(registry) = value.get(0) else {
            return Error::missing("Missing value for registry");
        };
        let registry: RegionalRegistry = registry.try_into()?;
        let Some(cc) = value.get(1) else {
            return Error::missing("Missing value for cc");
        };
        let cc: Option<CountryCode> = match cc.len() {
            0 => None,
            _ => cc.as_str().try_into().ok(),
        };
        let Some(entry_type) = value.get(2) else {
            return Error::missing("Missing value for type");
        };
        let entry_type: EntryType = entry_type.try_into()?;
        let Some(start_value) = value.get(3) else {
            return Error::missing("Missing value for start value");
        };
        let start_value: EntryValue = EntryValue::try_into(start_value, entry_type)?;
        let Some(val) = value.get(4) else {
            return Error::missing("Missing value for value");
        };
        let val: u32 = val.parse()?;
        let Some(date) = value.get(5) else {
            return Error::missing("Missing value for date");
        };
        let date = Date::parse_from(&BASIC_CALENDAR_DATE, date).ok();
        let Some(_status) = value.get(6) else {
            return Error::missing("Missing value for status");
        };
        let opaque_id = value.get(7).cloned();
        Ok(Record {
            registry,
            country_code: cc,
            entry_type,
            start_value,
            value: val,
            date,
            opaque_id,
        })
    }
}

pub enum RecordType {
    VersionLine(VersionLine),
    SummaryLine(SummaryLine),
    Record(Record),
}
