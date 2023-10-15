// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_enums::{EnumIterItem, EnumName, EnumTryFromStr};

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumName, EnumTryFromStr, EnumIterItem)]
pub enum RegionalRegistry {
    afrinic,
    apnic,
    arin,
    lacnic,
    ripencc,
}

impl RegionalRegistry {
    pub fn url(&self) -> &'static str {
        match self {
            RegionalRegistry::afrinic => {
                "https://ftp.afrinic.net/stats/afrinic/delegated-afrinic-extended-latest"
            }
            RegionalRegistry::apnic => {
                "https://ftp.apnic.net/stats/apnic/delegated-apnic-extended-latest"
            }
            RegionalRegistry::arin => {
                "https://ftp.arin.net/pub/stats/arin/delegated-arin-extended-latest"
            }
            RegionalRegistry::lacnic => {
                "https://ftp.lacnic.net/pub/stats/lacnic/delegated-lacnic-extended-latest"
            }
            RegionalRegistry::ripencc => {
                "https://ftp.ripe.net/pub/stats/ripencc/delegated-ripencc-extended-latest"
            }
        }
    }
}
