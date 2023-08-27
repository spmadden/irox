use std::thread::sleep;
use std::time::Duration;

use log::{debug, error};
use tokio::runtime;
use windows::core::Error;
use windows::Devices::Geolocation::{
    Geocoordinate, GeolocationAccessStatus, Geolocator, PositionAccuracy,
    PositionChangedEventArgs,
};
use windows::Foundation::{IReference, TypedEventHandler};

use irox_carto::coordinate::{EllipticalCoordinate, Latitude, Longitude};
use irox_carto::geo::standards::wgs84::WGS84_SHAPE;
use irox_carto::units::angle::Angle;
use irox_carto::units::length::Length;

use crate::error::GPSdError;
use crate::output::{Frame, FramePayload, TPV};
use crate::transport::TCPServer;

pub struct WinErr(GPSdError);
impl WinErr {
    pub fn err(msg: &str) -> Result<(), WinErr> {
        Err(WinErr(GPSdError::new(msg)))
    }
}
impl From<Error> for WinErr {
    fn from(value: Error) -> Self {
        WinErr(GPSdError::new_str(value.to_string()))
    }
}
impl From<WinErr> for GPSdError {
    fn from(value: WinErr) -> Self {
        value.0
    }
}
impl From<std::io::Error> for WinErr {
    fn from(value: std::io::Error) -> Self {
        WinErr(value.into())
    }
}

pub fn opt_f64_ref(opt: windows::core::Result<IReference<f64>>) -> f64 {
    let Ok(refv) = opt else {
        return 0.0;
    };
    let Ok(val) = refv.GetDouble() else {
        return 0.0;
    };
    return val;
}

pub fn print_point(coord: &Geocoordinate) -> windows::core::Result<Frame> {
    let point = coord.Point()?;
    let pos = point.Position()?;
    let source = coord.PositionSource()?;
    let acc = coord.Accuracy()?;

    let lat = Latitude(Angle::new_degrees(pos.Latitude));
    let lon = Longitude(Angle::new_degrees(pos.Longitude));
    let alt = Length::new_meters(pos.Altitude);
    let ecc = EllipticalCoordinate::new(lat, lon, WGS84_SHAPE).with_altitude(alt);

    debug!("{pos:?} :: {source:?} :: {acc}");
    if let Ok(sats) = coord.SatelliteData() {
        let hdop = opt_f64_ref(sats.HorizontalDilutionOfPrecision());
        let pdop = opt_f64_ref(sats.PositionDilutionOfPrecision());
        let gdop = opt_f64_ref(sats.GeometricDilutionOfPrecision());
        let tdop = opt_f64_ref(sats.PositionDilutionOfPrecision());
        let vdop = opt_f64_ref(sats.VerticalDilutionOfPrecision());
        debug!(
            "DOPs: HDOP({}) VDOP({}) GDOP({}) TDOP({}) PDOP({})",
            hdop, vdop, gdop, tdop, pdop
        );
    };

    let tpv = TPV {
        ..Default::default()
    };
    let frame = Frame {
        device: Some(String::from("Windows")),
        payload: FramePayload::TPV(Box::new(tpv)),
    };

    Ok(frame)
}

pub fn open(server: TCPServer) -> Result<(), GPSdError> {
    let mut runtime = runtime::Builder::new_current_thread().build()?;
    let mut locator = GPSdError::conv(Geolocator::new())?;

    let access = GPSdError::conv(Geolocator::RequestAccessAsync())?;
    runtime.block_on(async move {
        let result = access.await;
        let Ok(status) = result else {
            return WinErr::err("Error checking geoaccess status");
        };
        match status {
            GeolocationAccessStatus::Unspecified => {}
            GeolocationAccessStatus::Denied => return WinErr::err("Geolocation not allowed"),
            GeolocationAccessStatus::Allowed => {}
            _ => return WinErr::err("Unknown geoloc status {other}"),
        }

        locator.SetDesiredAccuracy(PositionAccuracy::High)?;

        let pos = locator.GetGeopositionAsync()?.await?;
        let coord = pos.Coordinate()?;
        let frame = print_point(&coord)?;

        if let Err(e) = server.send(frame) {
            error!("Unable to send first frame: {e:?}");
        };

        locator.PositionChanged(&TypedEventHandler::new(
            move |_l, args: &Option<PositionChangedEventArgs>| {
                if let Some(arg) = args {
                    let pos = arg.Position()?;
                    let coord = pos.Coordinate()?;
                    let frame = print_point(&coord)?;
                    if let Err(e) = server.send(frame) {
                        error!("Unable to send periodic frame: {e:?}");
                    }
                }
                Ok(())
            },
        ))?;

        sleep(Duration::from_secs(86400));

        Ok(())
    })?;

    Ok(())
}
