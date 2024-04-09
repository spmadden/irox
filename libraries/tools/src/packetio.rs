// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Traits for packetization of data and movement of packets of data

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use irox_bits::{Bits, Error, ErrorKind, MutBits};

/// A packet is a series of bytes
pub type PacketData = Vec<u8>;

pub trait Packet {
    type PacketType;
    fn get_bytes(&self) -> Result<Vec<u8>, Error>;

    fn write_to<T: MutBits>(&self, out: &mut T) -> Result<(), Error> {
        out.write_all_bytes(self.get_bytes()?.as_slice())
    }

    fn get_type(&self) -> Self::PacketType;
}

pub trait PacketBuilder<P> {
    type Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<P, Self::Error>;
}

///
/// This trait represents a way to packetize a stream of data
///
pub trait Packetization<T: Bits> {
    /// Reads the next packet from the source reader
    fn read_next_packet(&mut self, source: &mut T) -> Result<PacketData, Error>;
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
pub struct Packetizer<'a, R: Bits, P: Packetization<R>> {
    reader: &'a mut R,
    chunker: &'a mut P,
}

impl<'a, R, P> PacketTransport for Packetizer<'a, R, P>
where
    R: Bits,
    P: Packetization<R>,
{
    type Error = irox_bits::Error;

    /// Polls the next packet from the underlying transport
    fn poll_next_packet(&mut self) -> Result<PacketData, Self::Error> {
        self.chunker.read_next_packet(self.reader)
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

impl<T: Bits> Packetization<T> for DelimitedPacketizer {
    fn read_next_packet(&mut self, source: &mut T) -> Result<PacketData, Error> {
        if self.delimiter.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "Delimiter is empty"));
        }

        self.buffer.clear();
        let del_len = self.delimiter.len();

        let mut ringbuf: VecDeque<u8> = VecDeque::with_capacity(del_len);
        source.read_exact_into(del_len, &mut ringbuf.as_mut_slices().0)?;

        loop {
            if ringbuf.eq(&self.delimiter) {
                let mut outbuf = self.buffer.clone();
                if self.include_delimiter {
                    outbuf.extend(&self.delimiter);
                }
                return Ok(outbuf);
            }

            if self.buffer.len() == self.max_buffer_size {
                return Err(Error::new(
                    ErrorKind::OutOfMemory,
                    "Packet exceeded max buffer size",
                ));
            }
            let Some(val) = source.next_u8()? else {
                return Ok(self.buffer.clone());
            };

            ringbuf.pop_front();
            ringbuf.push_back(val);
        }
    }
}
