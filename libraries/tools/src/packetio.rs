// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Traits for packetization of data and movement of packets of data

use std::collections::VecDeque;
use std::io::{ErrorKind, Read};

use crate::bits::{Bits, MutBits};

/// A packet is a series of bytes
pub type PacketData = Vec<u8>;

pub trait Packet {
    type PacketType;
    type Error;

    fn write_to<T: MutBits>(&self, out: &mut T) -> Result<(), Self::Error>;

    fn get_bytes(&self) -> Result<Vec<u8>, Self::Error>;

    fn get_type(&self) -> Self::PacketType;
}

pub trait PacketBuilder<P: Packet> {
    type Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<P, Self::Error>;
}

///
/// This trait represents a way to packetize a stream of data
///
pub trait Packetization {
    type Error;

    /// Reads the next packet from the source reader
    fn read_next_packet<T: Bits>(&mut self, source: &mut T) -> Result<PacketData, Self::Error>;
}

///
/// Represents an underlying message packet transport
///
pub trait PacketTransport {
    type Error;

    /// Polls the next packet from the underlying transport
    fn poll_next_packet(&mut self) -> Result<PacketData, Self::Error>;

    /// Start the underlying transport up
    fn start(&mut self) -> Result<(), Self::Error>;

    /// Stop the underlying transport
    fn stop(&mut self) -> Result<(), Self::Error>;
}

///
/// A packetizer binds a Read stream and a Packetization strategy
pub struct Packetizer<'a, R: Read, P: Packetization> {
    reader: &'a mut R,
    chunker: &'a mut P,
}

impl<'a, R, P> PacketTransport for Packetizer<'a, R, P>
where
    R: Read,
    P: Packetization<Error = std::io::Error>,
{
    type Error = std::io::Error;

    /// Polls the next packet from the underlying transport
    fn poll_next_packet(&mut self) -> Result<PacketData, Self::Error> {
        self.chunker.read_next_packet(&mut self.reader)
    }

    /// Start the underlying transport up
    fn start(&mut self) -> Result<(), Self::Error> {
        // noop.
        Ok(())
    }

    /// Stop the underlying transport
    fn stop(&mut self) -> Result<(), Self::Error> {
        // noop.
        Ok(())
    }
}

///
/// A delimited packetizer searches for a delimiter in the underlying data stream
#[derive(Default)]
pub struct DelimitedPacketizer {
    /// The delimiter to search for
    pub delimiter: Vec<u8>,

    /// Include the delimiter in the packet output?
    pub include_delimiter: bool,

    /// Will scan up to max_buffer_size bytes in memory before failing.
    pub max_buffer_size: usize,

    /// Internal data buffer
    buffer: Vec<u8>,
}

impl Packetization for DelimitedPacketizer {
    type Error = std::io::Error;

    fn read_next_packet<T: Bits>(&mut self, source: &mut T) -> Result<PacketData, Self::Error> {
        if self.delimiter.is_empty() {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "Delimiter is empty",
            ));
        }

        self.buffer.clear();
        let del_len = self.delimiter.len();

        let mut ringbuf: VecDeque<u8> = VecDeque::with_capacity(del_len);
        source.read_exact(ringbuf.as_mut_slices().0)?;

        let mut onebuf: [u8; 1] = [0; 1];
        loop {
            if ringbuf.eq(&self.delimiter) {
                let mut outbuf = self.buffer.clone();
                if self.include_delimiter {
                    outbuf.extend(&self.delimiter);
                }
                return Ok(outbuf);
            }

            if self.buffer.len() == self.max_buffer_size {
                return Err(std::io::Error::new(
                    ErrorKind::OutOfMemory,
                    "Packet exceeded max buffer size",
                ));
            }

            if source.read(&mut onebuf)? == 0 {
                return Ok(self.buffer.clone());
            }

            ringbuf.pop_front();
            ringbuf.push_back(onebuf[0]);
        }
    }
}
