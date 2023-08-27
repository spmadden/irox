//!
//! Structs around the devices/device messages

use time::OffsetDateTime;

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
    activated: Option<OffsetDateTime>,
    bps: Option<u32>,
    cycle: Option<f32>,
    driver: Option<String>,
    flags: Option<Vec<DeviceFlags>>,
    hexdata: Option<String>,
    mincycle: Option<f32>,
    native: Option<NativeMode>,
    parity: Option<Parity>,
    path: Option<String>,
    readonly: Option<bool>,
    stopbits: Option<StopBits>,
    subtype: Option<String>,
    subtype1: Option<String>,
}

/// Returns a device list object with the following elements:
pub struct Devices {
    /// List of device descriptions
    devices: Vec<Device>,

    /// URL of the remote daemon reporting the device set. If empty, this is a DEVICES response
    /// from the local daemon.
    remote: Option<String>,
}
