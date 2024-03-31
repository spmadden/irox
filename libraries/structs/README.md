IROX-STRUCTS
===========

*Traits for Struct Types - linearly serialized big/little endian bytes*

A very simple 'Struct' library that provides a single trait. This is intended to be most
generally used in concert with `irox_structs_derive` to generate the impls.

A Struct is a linear sequence in memory of serialized bytes - serialized in the order the
fields are present in the struct with no adjacency or packing.

## Strict Sizing

By default, `irox_structs_derive` will allow variably sized types like `String` and `Vec<u8>`.
If you know you don't need/use these, apply a `#[strict_sizing]` attribute, and the generator
will add a `pub const STRUCT_SIZE: usize` to the struct

## Choosing Endianness:

Apply either the `#[big_endian]` or `#[little_endian]` attributes when deriving `Struct` and
it will use the appropriate serializers. If not specified, it defaults to big endian.

### Example Big Endian:

```rust
use irox_structs::Struct;
use irox_tools::bits::Error;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Struct)]
#[strict_sizing]
pub struct UdpHeader {
    source_port: u16,
    dest_port: u16,
    length: u16,
    checksum: u16,
}

pub fn main() -> Result<(), Error> {
    assert_eq!(8, UdpHeader::STRUCT_SIZE);

    let header = UdpHeader {
        source_port: 0x0064,
        dest_port: 0x0400,
        length: 0x1388,
        checksum: 0x01C2,
    };
    let mut output_buf: Vec<u8> = Vec::new();

    header.write_to(&mut output_buf)?;
    assert_eq!(output_buf.len(), 8);
    assert_eq!(&[0x00u8, 0x64, 0x04, 0x00, 0x13, 0x88, 0x01, 0xC2],
               output_buf.as_slice());

    let parsed = UdpHeader::parse_from(&mut output_buf.as_slice())?;
    assert_eq!(header, parsed);
    Ok(())
}
```

### Example Little Endian:

```rust
use irox_structs::Struct;
use irox_tools::bits::Error;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Struct)]
#[little_endian]
#[strict_sizing]
pub struct UdpHeader {
    source_port: u16,
    dest_port: u16,
    length: u16,
    checksum: u16,
}

pub fn main() -> Result<(), Error> {
    assert_eq!(8, UdpHeader::STRUCT_SIZE);
    let header = UdpHeader {
        source_port: 0x0064,
        dest_port: 0x0400,
        length: 0x1388,
        checksum: 0x01C2,
    };
    let mut output_buf: Vec<u8> = Vec::new();
    
    header.write_to(&mut output_buf)?;
    assert_eq!(output_buf.len(), 8);
    assert_eq!(&[0x64u8, 0x00, 0x00, 0x04, 0x88, 0x13, 0xC2, 0x01],
               output_buf.as_slice());

    let parsed = UdpHeader::parse_from(&mut output_buf.as_slice())?;
    assert_eq!(header, parsed);
    Ok(())
}
```

### No-STD support:

* Fully no-std compliant

### Features:

* None

### Modules:

| Module                     | `[no_std]`? |                            |
|----------------------------|-------------|----------------------------|
| [`irox_structs`](./lib.rs) | ![no_std]   | Contains the 'Struct' type |

[no_std]: https://img.shields.io/badge/no__std-yes-green

[std]: https://img.shields.io/badge/feature-std-lightgrey



