// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use std::collections::VecDeque;
use std::sync::Arc;

use irox_log::log::debug;
use irox_tools::bits::{Bits, Error, MutBits};

use crate::inode::{BlockIter, Inode};

pub struct DataStream {
    inode: Arc<Inode>,
    position: u64,
    length: u64,
    block_iter: BlockIter,
    data_buf: VecDeque<u8>,
}

impl DataStream {
    pub fn new(inode: &Arc<Inode>) -> Result<Self, Error> {
        let length = inode.get_data_length()?;
        let block_iter = BlockIter::new(inode)?;
        let ds = DataStream {
            inode: inode.clone(),
            position: 0,
            length,
            block_iter,
            data_buf: VecDeque::new(),
        };
        Ok(ds)
    }
}

impl Bits for DataStream {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        if self.position >= self.length {
            debug!(
                "Datastream bailing out because {} >= {}",
                self.position, self.length
            );
            return Ok(None);
        }

        if let Some(nxt) = self.data_buf.pop_front() {
            self.position += 1;
            return Ok(Some(nxt));
        }
        if let Some(nxt_block) = self.block_iter.next() {
            let blk = self.inode.fs.read_raw_4k_block(nxt_block)?;
            self.data_buf.write_all_bytes(blk.as_slice())?;
            debug!("Datastream loaded next block {nxt_block}");
            return self.next_u8();
        }
        Ok(None)
    }
}

impl Iterator for DataStream {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_u8().ok().flatten()
    }
}

pub struct MutDataStream {
    
    position: u64,
    data_buf: VecDeque<u8>,

}

impl MutBits for MutDataStream {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.data_buf.push_back(val);
        
        Ok(())
    }
}