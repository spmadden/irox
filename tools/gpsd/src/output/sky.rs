//!
//! Structs around the SKY message
use time::OffsetDateTime;

use irox_units::units::length::Length;

pub struct Satellite {}

pub enum QualityIndicator {
    NoSignal = 0,
    SearchingSignal = 1,
    SignalAcquired = 2,
    SignalUnusable = 3,
    CodeLockedTimeSynched = 4,
    CodeCarrierLockedTimeSynched = 5,
}

/// A SKY object reports a sky view of the GPS satellite positions.
pub struct SKY {
    pub n_sat: Option<u32>,
    pub gdop: Option<f32>,
    pub hdop: Option<f32>,
    pub pdop: Option<f32>,

    pub pr_res: Option<Length>,
    pub qual: Option<QualityIndicator>,
    pub satellites: Option<Vec<Satellite>>,

    pub tdop: Option<f32>,
    pub time: Option<OffsetDateTime>,

    pub u_sat: Option<u32>,
    pub vdop: Option<f32>,
    pub xdop: Option<f32>,
    pub ydop: Option<f32>,
}
