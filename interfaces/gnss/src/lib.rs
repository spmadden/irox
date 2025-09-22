// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! GNSS Data Stream parsing and detection
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;
extern crate core;

use alloc::sync::Arc;
use core::fmt::{Display, Formatter};
use core::sync::atomic::AtomicBool;
use irox_bits::{Bits, BitsBuffer, BitsWrapper, BufBits};
use irox_log::log;
use irox_nmea0183::{Frame as NMEAFrame, NMEAParser};
use irox_sirf::packet::PacketParser as SIRFParser;
use irox_sirf::packet::PacketType as SIRFPacket;
use irox_tools::cfg_feature_std;
use irox_tools::packetio::PacketBuilder;
use irox_ubx::{UBXMessage, UBXParser};

cfg_feature_std! {
    pub mod tcp;
}

#[derive(Debug, Clone, PartialEq)]
pub enum GNSSFrame {
    UBX(UBXMessage),
    NMEA(NMEAFrame),
    SIRF(SIRFPacket),
}
impl Display for GNSSFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            GNSSFrame::UBX(a) => core::fmt::Display::fmt(a, f),
            GNSSFrame::NMEA(a) => core::fmt::Display::fmt(a, f),
            GNSSFrame::SIRF(a) => core::fmt::Display::fmt(a, f),
        }
    }
}

pub struct GNSSPacketStream<'a, T: Bits> {
    run: Arc<AtomicBool>,
    buf: BitsBuffer<'a, T>,
}
impl<'a, T: Bits> GNSSPacketStream<'a, T> {
    pub fn new(run: Arc<AtomicBool>, buf: BitsWrapper<'a, T>) -> Self {
        Self {
            run,
            buf: BitsBuffer::new(buf),
        }
    }
    pub fn read_next(&mut self) -> Result<GNSSFrame, irox_bits::Error> {
        let mut skipped = 0;
        while self.run.load(core::sync::atomic::Ordering::Relaxed) {
            let mut bf = match self.buf.fill_buf() {
                Ok(bf) => bf,
                Err(e) => {
                    log::info!("Failed to fill buffer: {e:?}");
                    break;
                }
            };
            // this reads from the slice, but does not consume from the buffer.
            let Ok([a, b]) = bf.read_exact() else {
                log::info!("Failed to peek two bytes from buffer");
                break;
            };
            match (a, b) {
                (b'$', _) => {
                    // assuming NMEA, must read from buf to consume.
                    let Ok(pkt) = NMEAParser.build_from(&mut self.buf) else {
                        log::info!("Failed to read NMEA");
                        break;
                    };
                    if skipped > 0 {
                        log::warn!("skipped {skipped} unused bytes");
                    }
                    log::debug!("read NMEA: {pkt:?}");
                    return Ok(GNSSFrame::NMEA(pkt));
                }
                (0xA0, 0xA2) => {
                    // assuming SIRF, must read from buf to consume.
                    let Ok(pkt) = SIRFParser.build_from(&mut self.buf) else {
                        log::info!("Failed to read SIRF");
                        break;
                    };
                    if skipped > 0 {
                        log::warn!("skipped {skipped} unused bytes");
                    }
                    log::debug!("read SIRF: {pkt:?}");
                    return Ok(GNSSFrame::SIRF(pkt));
                }
                (0xB5, 0x62) => {
                    // assuming UBX, must read from buf to consume.
                    let Ok(pkt) = UBXParser::default().build_from(&mut self.buf) else {
                        log::info!("Failed to read UBX");
                        break;
                    };
                    if skipped > 0 {
                        log::warn!("skipped {skipped} unused bytes");
                    }
                    log::debug!("read UBX: {pkt:?}");
                    return Ok(GNSSFrame::UBX(pkt));
                }
                _ => {
                    // unknown, skip.
                    self.buf.consume(1);
                    skipped += 1;
                    continue;
                }
            }
        }
        Err(irox_bits::ErrorKind::UnexpectedEof.into())
    }
}
