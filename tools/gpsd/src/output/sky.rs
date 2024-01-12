//!
//! Structs around the SKY message

use irox_time::datetime::UTCDateTime;
use irox_units::units::length::Length;

#[derive(Debug, Clone, PartialEq)]
pub struct Satellite {
    pub prn: u8,
    pub az: Option<f64>,
    pub el: Option<f64>,
    pub ss: Option<f64>,
    pub used: bool,
    pub gnss_id: Option<u8>,
    pub sv_id: Option<u8>,
    pub sig_id: Option<u8>,
    pub freq_id: Option<u8>,
    pub health: Option<u8>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum QualityIndicator {
    NoSignal = 0,
    SearchingSignal = 1,
    SignalAcquired = 2,
    SignalUnusable = 3,
    CodeLockedTimeSynched = 4,
    CodeCarrierLockedTimeSynched = 5,
}

/// A SKY object reports a sky view of the GPS satellite positions.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SKY {
    pub n_sat: Option<u32>,
    pub gdop: Option<f32>,
    pub hdop: Option<f32>,
    pub pdop: Option<f32>,

    pub pr_res: Option<Length>,
    pub qual: Option<QualityIndicator>,
    pub satellites: Option<Vec<Satellite>>,

    pub tdop: Option<f32>,
    pub time: Option<UTCDateTime>,

    pub u_sat: Option<u32>,
    pub vdop: Option<f32>,
    pub xdop: Option<f32>,
    pub ydop: Option<f32>,
}
