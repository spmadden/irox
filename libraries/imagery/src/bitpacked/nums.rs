// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::stacked::LinearStackedImage;
use irox_bits::MutBits;
use irox_tools::buf::FixedU8Buf;
use irox_tools::hex;

// 16x29 pixel images packed into 58 bytes, 1 bit/pixel
pub static ZERO: [u8;58] = hex!("000000000000000003800C60183018303018301830183018301830183018301830183018183018300C6003800000000000000000000000000000");
pub static ONE: [u8;58] = hex!("00000000000000000080018007800180018001800180018001800180018001800180018001800180018007E00000000000000000000000000000");
pub static TWO: [u8;58] = hex!("000000000000000007E00FF0103820180018001800300030006000C00180030006000C00080010083FF83FF80000000000000000000000000000");
pub static THREE: [u8;58] = hex!("000000000000000007E00C7010300030002000400080010003E004700038001800180018001800300030306038C01F0000000000000000000000");
pub static FOUR: [u8;58] = hex!("0000000000000000000800180030006000C00190033006300C30183030303FFC0030003000300030003000FC0000000000000000000000000000");
pub static FIVE: [u8;58] = hex!("00000000000000001FF01FF0100010001C001F0007C001E0007000300038001800180018001800300030006001803E0000000000000000000000");
pub static SIX: [u8;58] = hex!("00000000000000600180030006000C0018001800300033E0347030383018301830183018181818100C2003C00000000000000000000000000000");
pub static SEVEN: [u8;58] = hex!("00000000000000007FFC7FFC40180010003000200060004000C000800180010003000200060004000C0008000000000000000000000000000000");
pub static EIGHT: [u8;58] = hex!("000000000000000007C00C601830183018301C200E40078003C00CE0187018303018301830181010082007C00000000000000000000000000000");
pub static NINE: [u8;58] = hex!("000000000000000007800C6018303030301830183018301818180C580798001800300030006000C0018006000000000000000000000000000000");
pub static ALL: [&[u8; 58]; 10] = [
    &ZERO, &ONE, &TWO, &THREE, &FOUR, &FIVE, &SIX, &SEVEN, &EIGHT, &NINE,
];

pub fn expand(img: &[u8; 58]) -> [u8; 464] {
    let mut i = FixedU8Buf::<464>::new();
    for j in 0..58 {
        let px = img.get(j).copied().unwrap_or_default();
        for k in 0..8 {
            let sh = 7 - k;
            let v = (px >> sh) & 0x01;
            let _ = i.write_u8(v * 0xFF);
        }
    }
    i.take()
}
pub const fn get_image() -> LinearStackedImage<464> {
    todo!()
}

#[cfg(all(test, feature = "std"))]
mod test {
    use crate::bitpacked::nums::ALL;
    use crate::ColorDepth;
    use irox_bits::{BitStreamDecoder, BitsWrapper, Error};
    use irox_tools::buf::{Buffer, FixedU8Buf};
    use irox_tools::hex::HexDump;

    #[test]
    pub fn test() -> Result<(), Error> {
        for v in ALL {
            let mut v = v.as_ref();
            let mut dec = BitStreamDecoder::new(BitsWrapper::Borrowed(&mut v));
            let mut out = FixedU8Buf::<464>::new();
            for _ in 0..464 {
                let v = ColorDepth::OneBitPerColor.next_byte_stretched_color(&mut dec)?;
                let _ = out.push(v);
            }
            out.hexdump();
        }
        Ok(())
    }
}
