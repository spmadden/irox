// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_protobuf::irox_bits::{Bits, BitsError, MutBits, WriteToBEBits};
use irox_protobuf::{ProtoMessage, ProtobufBinary, ToProtoFieldData};
use irox_protobuf_derive::ProtobufBinary;
use irox_tools::{assert_eq_hex_slice, hex};

#[derive(Debug, ProtobufBinary)]
struct TestMsg1 {
    #[id = 1]
    pub val: u32,
}

#[test]
fn test_derive() -> Result<(), BitsError> {
    let mut out = Vec::<u8>::new();
    let msg = TestMsg1 { val: 150 };
    msg.write_to(&mut out)?;

    assert_eq_hex_slice!(&out, hex!("089601"));
    Ok(())
}

pub struct TestMsg2 {
    pub val: u32,
}
impl ProtobufBinary for TestMsg2 {
    fn write_to<T: MutBits>(&self, output: &mut T) -> Result<usize, BitsError> {
        let mut out = 0;
        let mut msg = ProtoMessage::default();
        msg.fields
            .push(ToProtoFieldData::to_proto_field(&self.val, "val", 1));
        out += msg.write_be_to(output)?;
        Ok(out)
    }

    fn read_from<T: Bits>(_input: &mut T) -> Result<Self, BitsError>
    where
        Self: Sized,
    {
        todo!()
    }
}
