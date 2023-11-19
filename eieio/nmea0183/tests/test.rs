// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use irox_eieio_api::carto::coordinate::{Latitude, Longitude};
use irox_eieio_api::carto::irox_units::units::angle::Angle;
use irox_eieio_api::carto::position_type::{PositionsBuilder, WGS84PositionBuilder};
use irox_eieio_api::time::datetime::UTCDateTime;
use irox_eieio_api::BaseMessage;
use irox_eieio_nmea0183::NMEA0183Codec;

#[test]
pub fn test() {
    let codec = NMEA0183Codec::new();
    println!("{codec:#?}");

    let Some(mut fix_bldr) = codec.get_gnss_fix_builder() else {
        return;
    };

    fix_bldr.set_timestamp(UTCDateTime::now());
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

    println!("{res:#?}");

    let string = res
        .get_supported_writers()
        .string()
        .unwrap()
        .write_to_string()
        .unwrap();
    println!("{string}",);
}
