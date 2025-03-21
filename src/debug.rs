//! Helper functions and structures for debugging purpose

use alloc::format;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::fmt;
#[cfg(feature = "std")]
use nom::{
    combinator::{map, peek, rest},
    HexDisplay, IResult,
};

/// Dump the remaining bytes to stderr, formatted as hex
#[cfg(feature = "std")]
pub fn dbg_dmp_rest(i: &[u8]) -> IResult<&[u8], ()> {
    use nom::Parser;

    map(peek(rest), |r: &[u8]| eprintln!("\n{}\n", r.to_hex(16))).parse(i)
}

/// Wrapper for printing value as u8 hex data
pub struct HexU8(pub u8);

impl fmt::Debug for HexU8 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "0x{:02x}", self.0)
    }
}

/// Wrapper for printing value as u16 hex data
pub struct HexU16(pub u16);

impl fmt::Debug for HexU16 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "0x{:04x}", self.0)
    }
}

/// Wrapper for printing slice as hex data
pub struct HexSlice<'a>(pub &'a [u8]);

impl fmt::Debug for HexSlice<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s: Vec<_> = self.0.iter().map(|&i| format!("{:02x}", i)).collect();
        write!(fmt, "[{}]", s.join(" "))
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "std"))]
    use alloc::format;

    use crate::debug;

    #[test]
    fn debug_print_hexu8() {
        assert_eq!(format!("{:?}", debug::HexU8(18)), "0x12");
    }

    #[test]
    fn debug_print_hexu16() {
        assert_eq!(format!("{:?}", debug::HexU16(32769)), "0x8001");
    }

    #[test]
    fn debug_print_hexslice() {
        assert_eq!(
            format!("{:?}", debug::HexSlice(&[15, 16, 17, 18, 19, 20])),
            "[0f 10 11 12 13 14]"
        );
    }
}
