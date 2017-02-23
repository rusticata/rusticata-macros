//! Helper macros

/// Helper macro for nom parsers: raise error if the condition is false
#[macro_export]
macro_rules! error_if (
  ($i:expr, $cond:expr, $err:expr) => (
    {
      if $cond {
        IResult::Error($err)
      } else {
        IResult::Done($i, ())
      }
    }
  );
  ($i:expr, $cond:expr, $err:expr) => (
    error!($i, $cond, $err);
  );
);


/// Read an entire slice as a big-endian value.
///
/// Returns the value as `u64`. This function checks for integer overflows, and returns a
/// `Result::Err` value if the value is too big.
pub fn bytes_to_u64(s: &[u8]) -> Result<u64, &'static str> {
    let mut u : u64 = 0;

    for &c in s {
        let (u1,f1) = u.overflowing_mul(256);
        let (u2,f2) = u1.overflowing_add(c as u64);
        if f1 || f2 { return Err("overflow"); }
        u = u2;
    }

    Ok(u)
}

/// Read a slice as a big-endian value.
#[macro_export]
macro_rules! parse_hex_to_u64 (
    ( $i:expr, $size:expr ) => (
        map_res!($i, take!(($size as usize)), $crate::bytes_to_u64)
    );
);

named_attr!(#[doc = "Read 3 bytes as an unsigned integer"],
            pub parse_uint24<&[u8], u64>, parse_hex_to_u64!(3));

//named!(parse_hex4<&[u8], u64>, parse_hex_to_u64!(4));

