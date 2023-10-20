//!
//! Structs around the Poll command

use irox_time::datetime::UTCDateTime;

use crate::output::{SKY, TPV};

/// The POLL command requests data from the last-seen fixes on all active GPS devices. Devices must
/// previously have been activated by ?WATCH to be pollable.
pub struct Poll {
    pub time: UTCDateTime,
    pub active: u32,
    pub tpv: Vec<TPV>,
    pub sky: Vec<SKY>,
}
