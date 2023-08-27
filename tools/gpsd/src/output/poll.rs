//!
//! Structs around the Poll command

use crate::output::{SKY, TPV};
use time::OffsetDateTime;

/// The POLL command requests data from the last-seen fixes on all active GPS devices. Devices must
/// previously have been activated by ?WATCH to be pollable.
pub struct Poll {
    time: OffsetDateTime,
    active: u32,
    tpv: Vec<TPV>,
    sky: Vec<SKY>,
}
