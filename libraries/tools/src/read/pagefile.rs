// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use irox_bits::{Error, SeekRead, SeekWrite};
use std::fs::File;
use std::path::Path;

pub const FILE_FLAG_NO_BUFFERING: u32 = 0x20000000;
pub const FILE_FLAG_RANDOM_ACCESS: u32 = 0x10000000;
pub const FILE_FLAG_WRITE_THROUGH: u32 = 0x80000000;
pub const FILE_FLAG_OVERLAPPED: u32 = 0x40000000;

pub struct Pagefile {
    backing_file: File,
}

impl Pagefile {
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Pagefile, Error> {
        let backing_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            // .attributes(
            //     FILE_FLAG_NO_BUFFERING
            //         | FILE_FLAG_WRITE_THROUGH
            //         | FILE_FLAG_OVERLAPPED
            // )
            .open(path)?;
        Ok(Pagefile { backing_file })
    }

    pub fn read_page<const N: usize>(&mut self, page_num: u64) -> Result<[u8; N], Error> {
        let mut buf = [0; N];
        let offset = page_num * N as u64;
        let read = self.backing_file.seek_read(&mut buf, offset)?;
        debug_assert!(
            read == 0 || read == N,
            "Read less than expected (0 < {read} < {N}) "
        );
        Ok(buf)
    }
    pub fn write_page<const N: usize>(
        &mut self,
        page_num: u64,
        data: &[u8; N],
    ) -> Result<(), Error> {
        let offset = page_num * N as u64;
        let write = self.backing_file.seek_write(data, offset)?;
        debug_assert!(write == N, "Wrote less than expected ({write} != {N})");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::random::PRNG;
    use crate::read::Pagefile;
    use irox_bits::Error;
    use std::io::Write;

    #[test]
    #[ignore]
    pub fn test() -> Result<(), Error> {
        let mut file = Pagefile::open("test_page.pagefile")?;
        let mut rnd = crate::random::PcgXslRrRr::new_seed(0);

        let mut page = [0u8; 4096];
        let start = std::time::Instant::now();
        for idx in 0..=1_000_000 {
            let mut sli = page.as_mut_slice();
            for _i in 0..32 {
                sli.write_all(&rnd.next_u128().to_be_bytes())?;
            }
            file.write_page(idx, &page)?;
        }
        file.backing_file.sync_data()?;
        let elapsed = start.elapsed();
        let len = page.len() * 1_000_000;
        let dur = elapsed.as_secs_f64();
        println!(
            "Wrote {} in {}s = {} MB/s",
            len,
            dur,
            len as f64 / dur / 1e6
        );
        Ok(())
    }
}
