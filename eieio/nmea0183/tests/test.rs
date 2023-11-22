// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use irox_eieio_api::carto::coordinate::{Latitude, Longitude};
use irox_eieio_api::carto::irox_units::units::angle::Angle;
use irox_eieio_api::carto::position_type::{PositionsBuilder, WGS84PositionBuilder};
use irox_eieio_api::time::datetime::UTCDateTime;
use irox_eieio_api::time::gregorian::Date;
use irox_eieio_api::time::Time;
use irox_eieio_api::BaseMessage;
use irox_eieio_nmea0183::NMEA0183Codec;

#[test]
pub fn test() {
    let codec = NMEA0183Codec::new();
    println!("{codec:#?}");

    let Some(mut fix_bldr) = codec.get_gnss_fix_builder() else {
        return;
    };

    fix_bldr.set_timestamp(UTCDateTime::new(
        Date::new(2023, 11).unwrap(),
        Time::from_hms(12, 34, 56).unwrap(),
    ));
    fix_bldr.set_positions(
        PositionsBuilder::new()
            .with_latlon(
                WGS84PositionBuilder::new()
                    .with_latitude(Latitude(Angle::new_degrees(41.)))
                    .with_longitude(Longitude(Angle::new_degrees(-71.)))
                    .build()
                    .unwrap(),
            )
            .build(),
    );

    let res = fix_bldr.build().unwrap();

    let string = res
        .get_supported_writers()
        .string()
        .unwrap()
        .write_to_string()
        .unwrap();
    assert_eq!("$GPGGA,123456.00,4100.00000,N,7100.00000,W,,,,,,,,,*65\r\n$GPZDA,123456.00,11,01,2023,00,00*63\r\n", string);
}
