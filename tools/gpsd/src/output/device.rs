//!
//! Structs around the devices/device messages

use irox_time::datetime::UTCDateTime;

pub enum NativeMode {
    NMEAMode = 0,
    BinaryMode = 1,
}

pub enum Parity {
    None,
    Odd,
    Even,
}

pub enum StopBits {
    Unknown = 0,
    One = 1,
    Two = 2,
}

pub enum DeviceFlags {
    SeenGPS = 1,
    SeenRTCM2 = 2,
    SeenRTCM3 = 4,
    SeenAIS = 8,
}

pub struct Device {
    pub activated: Option<UTCDateTime>,
    pub bps: Option<u32>,
    pub cycle: Option<f32>,
    pub driver: Option<String>,
    pub flags: Option<Vec<DeviceFlags>>,
    pub hexdata: Option<String>,
    pub mincycle: Option<f32>,
    pub native: Option<NativeMode>,
    pub parity: Option<Parity>,
    pub path: Option<String>,
    pub readonly: Option<bool>,
    pub stopbits: Option<StopBits>,
    pub subtype: Option<String>,
    pub subtype1: Option<String>,
}

/// Returns a device list object with the following elements:
pub struct Devices {
    /// List of device descriptions
    pub devices: Vec<Device>,

    /// URL of the remote daemon reporting the device set. If empty, this is a DEVICES response
    /// from the local daemon.
    pub remote: Option<String>,
}
