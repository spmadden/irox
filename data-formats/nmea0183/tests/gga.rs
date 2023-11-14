// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use irox_carto::altitude::{Altitude, AltitudeReferenceFrame};
use irox_carto::coordinate::{Latitude, Longitude};
use irox_nmea0183::{Error, FramePayload, NMEAParser};
use irox_nmea0183::gga::GPSQualityIndicator;
use irox_time::Time;
use irox_tools::packetio::{Packet, PacketBuilder};
use irox_units::units::angle::Angle;
use irox_units::units::length::Length;

#[test]
pub fn test_gga() -> Result<(), Error> {
    let test = "$GNGGA,001043.00,4404.14036,N,12118.85961,W,1,12,0.98,1113.0,M,-21.3,M*47\r\n";
    let mut out = NMEAParser.build_from(&mut test.as_bytes())?;

    match &out.payload {
        FramePayload::GGA(gga) => {
            println!("{gga:#?}");
            assert_eq!(gga.timestamp(), Some(Time::new(643, 0).unwrap()));
            assert_eq!(
                gga.latitude(),
                Some(Latitude(Angle::new_degrees(44.069006)))
            );
            assert_eq!(
                gga.longitude(),
                Some(Longitude(Angle::new_degrees(
                    -121.31432683333333333333333333333333
                )))
            );
            assert_eq!(gga.quality(), Some(GPSQualityIndicator::GPSFix));
            assert_eq!(gga.num_sats(), Some(12));
            assert_eq!(gga.hdop(), Some(Length::new_meters(0.98)));
            assert_eq!(
                gga.ant_alt(),
                Some(Altitude::new(
                    Length::new_meters(1113.),
                    AltitudeReferenceFrame::Geoid,
                ))
            );
            assert_eq!(gga.geoid_sep(), Some(Length::new_meters(-21.3)));
        }
        e => {
            assert!(false, "Expecting GGA but was {e:?}");
        }
    };

    let bytes = out.get_bytes()?;
    assert_eq!(test.trim(), String::from_utf8_lossy(&bytes));

    out.raw = None;
    let bytes = out.get_bytes()?;
    assert_eq!(
        "$GPGGA,001043.00,4404.14036,N,12118.85961,W,1,12,0.98,1113.00,M,-21.30,M,,*59\r\n",
        String::from_utf8_lossy(&bytes)
    );

    Ok(())
}
