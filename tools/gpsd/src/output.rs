use serde::ser::SerializeMap;
use serde::Serializer;

pub use att::*;
pub use device::*;
pub use error::*;
pub use gst::*;
pub use poll::*;
pub use sky::*;
pub use tpv::*;
pub use version::*;
pub use watch::*;

use crate::error::GPSdError;

pub mod att;
pub mod device;
pub mod error;
pub mod gst;
pub mod poll;
pub mod sky;
pub mod tpv;
pub mod version;
pub mod watch;

pub struct Frame {
    /// Name of originating device
    pub device: Option<String>,
    pub payload: FramePayload,
}

pub enum FramePayload {
    /// A TPV object represents a Time/Position value
    TPV(Box<TPV>),

    /// A SKY object reports a sky view of the GPS satellite positions.
    SKY(Box<SKY>),

    /// A GST object is a pseudorange noise report.
    GST(Box<GST>),

    /// An ATT object is a vehicle-attitude report
    ATT(Box<ATT>),
}

impl FramePayload {
    pub fn name(&self) -> &'static str {
        match self {
            FramePayload::TPV(_) => "TPV",
            FramePayload::SKY(_) => "SKY",
            FramePayload::GST(_) => "GST",
            FramePayload::ATT(_) => "ATT",
        }
    }
}

impl serde::ser::Serialize for Frame {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("class", self.payload.name())?;

        if let Some(ref device) = self.device {
            map.serialize_entry("device", device)?;
        }

        match &self.payload {
            FramePayload::TPV(t) => TPV::serialize::<S>(t, &mut map)?,
            FramePayload::SKY(_s) => {}
            FramePayload::GST(_g) => {}
            FramePayload::ATT(_a) => {}
        }

        map.end()
    }
}

impl Frame {
    pub fn to_json(&self) -> Result<String, GPSdError> {
        Ok(serde_json::to_string(self)?)
    }
}

#[cfg(target_os = "windows")]
pub mod windows {
    use irox_winlocation_api::WindowsCoordinate;

    use crate::output::{Frame, FramePayload, TPV};

    impl From<&WindowsCoordinate> for Frame {
        fn from(value: &WindowsCoordinate) -> Self {
            let tpv: TPV = value.into();
            let device = Some(match value.source() {
                Some(s) => {
                    format!("WindowsAPI({s:?})")
                }
                None => String::from("WindowsAPI"),
            });
            Frame {
                device,
                payload: FramePayload::TPV(Box::new(tpv)),
            }
        }
    }
}
