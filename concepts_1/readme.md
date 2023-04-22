Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the _prelude_, and you can see everything in it [in the standard library documentation](https://doc.rust-lang.org/std/prelude/index.html).

random number functionality in its standard library. However, the Rust team does provide a [`rand` crate](https://crates.io/crates/rand)

**Table 3-1: Integer Types in Rust**

| Length | Signed | Unsigned |
| --- | --- | --- |
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| arch | `isize` | `usize` |

**Integer Literals in Rust**

| Number literals | Example |
| --- | --- |
| Decimal | `98_222` |
| Hex | `0xff` |
| Octal | `0o77` |
| Binary | `0b1111_0000` |
| Byte (`u8` only) | `b'A'` |

Rust uses the term _panicking_ when a program exits with an error;

To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:

*   Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`.
*   Return the `None` value if there is overflow with the `checked_*` methods.
*   Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
*   Saturate at the value’s minimum or maximum values with the `saturating_*` methods.