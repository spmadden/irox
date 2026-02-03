// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_arch_x86_64::cpu::CpuFeature;
use irox_tools::cfg_feature_std;

cfg_feature_std! {
    use irox_bits::{BitsErrorKind, Error};
    extern crate alloc;
    use crate::{Chacha20};
    use alloc::sync::Arc;
    use std::sync::Mutex;
    use irox_bits::{Bits, MutBits};
    use irox_tools::buf::FixedU8Buf;
    use irox_tools::static_init;

    pub fn rand64() -> Option<u64> {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        {
            let features = irox_arch_x86_64::cpu::cpu_features();
            if features.has_feature(CpuFeature::RDRAND) {
                return irox_arch_x86_64::rand::rdseed64()
            }
        }
        #[cfg(target_os = "linux")]
        {
            std::fs::File::open("/dev/urandom").ok()?
            .read_be_u64().ok()
        }
        #[cfg(target_arch = "wasm32")]
        {
            let mut v = [0u8;8];
            let w = web_sys::window()?;
            let c = w.crypto().ok()?;
            c.get_random_values_with_u8_array(&mut v).ok()?;
            Some(u64::from_be_bytes(v))
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "wasm32")))]
        None
    }

    static_init!(get_crng, Option<Arc<Mutex<Chacha20>>>,
        "Creates and returns a process-wide CRNG seeded from the CPU's entropy generator", {
        for _ in 0..10 {
            if let Some(rnd) = generate_rand_chacha20_stream() {
                return Some(Arc::new(Mutex::new(rnd)));
            }
        }
        None
    });
    /// Generate a random number stream using the `rdseed64` instruction
    ///
    /// # Panics
    /// If the underlying CPU cannot generate enough entropy using `rdseed64`
    #[allow(clippy::unwrap_used)]
    pub fn generate_rand_chacha20_stream() -> Option<Chacha20> {
        let mut buf = FixedU8Buf::<44>::new();
        for _ in 0..=6 {
            let mut rnd1 = None;
            for _ in 0..10 {
                if let Some(rnd) = rand64() {
                   rnd1 = Some(rnd);
                    break;
                }
            };
            let _ = buf.write_be_u64(rnd1?);
        };
        let buf = buf.take();
        let key : [u8;32] = buf[0..32].try_into().unwrap();
        let nonce : [u8;12] = buf[32..44].try_into().unwrap();
        Some(Chacha20::new(key, nonce))
    }

    pub fn get_random_bytes<const N: usize>() -> Option<[u8; N]> {
        let Some(rnd) = get_crng() else {
            return None;
        };
        let Ok(mut lock) = rnd.lock() else {
            return None;
        };
        let mut out = [0u8; N];

        lock.read_exact_into(N, &mut out.as_mut_slice()).ok()?;
        Some(out)
    }

    pub fn fill_random<const N: usize>(out: &mut [u8; N]) -> Result<(), Error> {
        let Some(rnd) = get_crng() else {
            return Err(Error::new(BitsErrorKind::WriteZero, "Failed to generate random number"));
        };
        let Ok(mut lock) = rnd.lock() else {
            return Err(Error::new(BitsErrorKind::WriteZero, "Failed to generate random number"));
        };
        lock.read_exact_into(N, &mut out.as_mut_slice())?;
        Ok(())
    }

    pub fn fill_random_bits<T: MutBits>(out: &mut T) -> Result<(), Error> {
        let Some(rnd) = get_crng() else {
            return Err(Error::new(BitsErrorKind::WriteZero, "Failed to generate random number"));
        };
        let Ok(mut lock) = rnd.lock() else {
            return Err(Error::new(BitsErrorKind::WriteZero, "Failed to generate random number"));
        };
        lock.read_filling(out)?;
        Ok(())
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use irox_bits::{Bits, Error};
    use irox_stats::streaming::SummarizingIterator;
    use irox_tools::assert_eq_eps;

    #[test]
    fn test_rand_chacha20_stream() -> Result<(), Error> {
        let rnd = super::generate_rand_chacha20_stream();
        let Some(mut rnd) = rnd else {
            panic!("Failed to generate random number");
        };
        let mut generated = [0u64; 2048];
        for i in 1..2048 {
            let val = rnd.read_be_u64()?;
            generated[i] = val;
            for j in 0..i {
                assert_ne!(generated[j], val);
            }
        }
        let mut bits = [0f64; 64];
        for w in generated.as_slice().windows(2) {
            let a = w[0] ^ w[1];
            for i in 0..64 {
                bits[i] += ((a >> i) & 0x1) as f64;
            }
        }
        let summary = bits.iter().summarize();
        // println!("{summary:?}");
        let avg = summary.mean().unwrap_or_default() / 2048.;
        let std = summary.stddev().unwrap_or_default() / 2048.;
        assert_eq_eps!(avg, 0.5, 0.01); // 50% chance of any one bit flip (within 1% margin)
        assert_eq_eps!(std, 0.0, 0.05); // +/- 5% bit-for-bit flip jitter

        Ok(())
    }
}
