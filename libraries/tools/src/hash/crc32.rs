// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

pub static CRC32_TABLE: [u32; 256] = crc32_init();
const fn crc32_init() -> [u32; 256] {
    let mut out = [0u32; 256];
    let mut i = 0;
    let mut c;
    while i < 256 {
        c = i;
        let mut k = 0;
        while k < 8 {
            if c & 1 == 1 {
                c = (c >> 1) ^ 0xEDB88320;
            } else {
                c >>= 1;
            }
            k += 1;
        }
        out[i] = c as u32;
        i += 1;
    }

    out
}

pub struct CRC32 {
    crc: u32,
}
impl Default for CRC32 {
    fn default() -> Self {
        Self { crc: 0xFFFFFFFF }
    }
}

impl CRC32 {
    pub fn new() -> Self {
        Self { crc: 0xFFFFFFFF }
    }
    pub fn update(&mut self, data: &[u8]) {
        for d in data {
            let idx = (self.crc ^ *d as u32) as usize & 0xFF;
            let t = CRC32_TABLE[idx];
            self.crc = t ^ (self.crc >> 8);
        }
    }
    pub fn finalize(self) -> u32 {
        self.crc ^ 0xFFFFFFFF
    }

    pub fn crc32(data: &[u8]) -> u32 {
        let mut crc = Self::new();
        crc.update(data);
        crc.finalize()
    }
}
