//!
//! Structs around the SKY message
use irox_units::units::length::Length;
use time::OffsetDateTime;

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
    n_sat: Option<u32>,
    gdop: Option<f32>,
    hdop: Option<f32>,
    pdop: Option<f32>,

    pr_res: Option<Length>,
    qual: Option<QualityIndicator>,
    satellites: Option<Vec<Satellite>>,

    tdop: Option<f32>,
    time: Option<OffsetDateTime>,

    u_sat: Option<u32>,
    vdop: Option<f32>,
    xdop: Option<f32>,
    ydop: Option<f32>,
}
