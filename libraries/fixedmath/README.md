IROX-FIXEDMATH
===========

*Fixed Precision Math Primitives*

Wikipedia's article on [Fixed-point arithmetic](https://en.wikipedia.org/wiki/Fixed-point_arithmetic)

All of the fractional components are scaled by the max value of the fractional size.

Works acceptably well:
* `FixedU32` - Q16.16 - `u16`/`u16`
* `FixedI32` - Q15.16 - `i16`/`u16`
  * Each fractional value represents `1/u16::MAX` ~= `1.5259e-5` or `0.000_015_259`, or about `15.3 micro`, and can accurately represent SI-prefixes: `milli/1e-3` with no loss of accuracy.
* `FixedU64` - Q32.32 - `u32`/`u32`
* `FixedI64` - Q31.32 - `i32`/`u32`
  * Each fractional value represents `1/u32::MAX` ~= `2.328306e-10` or `0.000_000_000_238_306`, or about `238.3 pico`, and can accurately represent SI-prefixes `milli/1e-3`, `micro/1e-6`, and `nano/1e-9` with no loss of accuracy.

Beta quality/Some known issues:
* `FixedU128` - Q64.64 - `u64`/`u64` [1]
* `FixedI128` - Q63.64 - `i64`/`u64` [1]
  * Each fractional value represents `1/u64::MAX`] ~= `5.4210e-20` or `0.000_000_000_000_000_000_054_210`, or about `54.2 zepto`, and can accurately represent SI-prefixes `milli/1e-3`, `micro/1e-6`, `nano/1e-9`, `pico/1e-12`, `femto/1e-15`, and `atto/1e-18` with no loss of accuracy.

[1]: Multiplication doesn't roll over properly, so [`FloatExt`](https://docs.rs/irox-tools/latest/irox_tools/f64/trait.FloatExt.html) aren't supported.

### No-STD support:

* Fully No-STD, No-Alloc

### Cargo Features:

* None

