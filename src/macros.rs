//! Helper macros

use nom::{IResult,rest};
use nom::HexDisplay;

/// Helper macro for newtypes: declare associated constants and implement Display trait
#[macro_export]
macro_rules! newtype_enum (
    (@collect_impl, $name:ident, $($key:ident = $val:expr),* $(,)*) => {
        $( pub const $key : $name = $name($val); )*
    };

    (@collect_disp, $name:ident, $f:ident, $m:expr, $($key:ident = $val:expr),* $(,)*) => {
        match $m {
            $( $val => write!($f, stringify!{$key}), )*
            n => write!($f, "{}({} / 0x{:x})", stringify!{$name}, n, n)
        }
    };

    // entry
    (impl $name:ident {$($body:tt)*}) => (
        #[allow(non_upper_case_globals)]
        impl $name {
            newtype_enum!{@collect_impl, $name, $($body)*}
        }
    );

    // entry with display
    (impl display $name:ident {$($body:tt)*}) => (
        newtype_enum!(impl $name { $($body)* });

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                newtype_enum!(@collect_disp, $name, f, self.0, $($body)*)
            }
        }
    );

    // entry with display and debug
    (impl debug $name:ident {$($body:tt)*}) => (
        newtype_enum!(impl display $name { $($body)* });

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self)
            }
        }
    );
);

/// Helper macro for nom parsers: raise error if the condition is false
#[macro_export]
macro_rules! error_if (
  ($i:expr, $cond:expr, $err:expr) => (
    {
      if $cond {
        Err(::nom::Err::Error(error_position!($i, $err)))
      } else {
        Ok(($i, ()))
      }
    }
  );
);

/// Helper macro for nom parsers: raise error if input is not empty
#[macro_export]
macro_rules! empty (
  ($i:expr,) => (
    {
      use nom::{Err,ErrorKind};

      if ($i).len() == 0 {
        Ok(($i, $i))
      } else {
        Err(Err::Error(error_position!($i, ErrorKind::Eof::<u32>)))
      }
    }
  );
);

/// Helper macro for nom parsers: run first parser if condition is true, else second parser
#[macro_export]
macro_rules! cond_else (
  ($i:expr, $cond:expr, $expr_then:ident!($($args_then:tt)*), $expr_else:ident!($($args_else:tt)*)) => (
    {
      if $cond { $expr_then!($i, $($args_then)*) }
      else { $expr_else!($i, $($args_else)*) }
    }
  );
  ($i:expr, $cond:expr, $expr_then:expr, $expr_else:ident!($($args_else:tt)*)) => (
      cond_else!($i, $cond, call!($expr_then), $expr_else!($($args_else)*))
  );
  ($i:expr, $cond:expr, $expr_then:ident!($($args_then:tt)*), $expr_else:expr) => (
      cond_else!($i, $cond, $expr_then!($($args_then)*), call!($expr_else))
  );
  ($i:expr, $cond:expr, $expr_then:expr, $expr_else:expr) => (
      cond_else!($i, $cond, call!($expr_then), call!($expr_else))
  );
);

/// Dump the remaining bytes to stderr, formatted as hex
pub fn dbg_dmp_rest(i:&[u8]) -> IResult<&[u8],()> {
    map!(
        i,
        peek!(rest),
        |r| eprintln!("\n{}\n", r.to_hex(16))
    )
}

/// Read an entire slice as a big-endian value.
///
/// Returns the value as `u64`. This function checks for integer overflows, and returns a
/// `Result::Err` value if the value is too big.
pub fn bytes_to_u64(s: &[u8]) -> Result<u64, &'static str> {
    let mut u : u64 = 0;

    if s.len() == 0 { return Err("empty"); };
    if s.len() > 8 { return Err("overflow"); }
    for &c in s {
        let u1 = u << 8;
        u = u1 | (c as u64);
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

named_attr!(#[doc = "Read 3 bytes as an unsigned integer"]
#[deprecated(since="0.5.0", note="please use `be_u24` instead")],
            pub parse_uint24<&[u8], u64>, parse_hex_to_u64!(3));

//named!(parse_hex4<&[u8], u64>, parse_hex_to_u64!(4));


/// Parse a slice and return a fixed-sized array of bytes
///
/// This creates a copy of input data
/// Uses unsafe code
#[macro_export]
macro_rules! slice_fixed(
    ( $i:expr, $count:expr ) => (
        {
            let cnt = $count;
            let ires: IResult<_,_> = if $i.len() < cnt {
                Err(::nom::Err::Incomplete(Needed::Size(cnt)))
            } else {
                let mut res: [u8; $count] = unsafe{[::std::mem::uninitialized(); $count as usize]};
                unsafe{::std::ptr::copy($i.as_ptr(), res.as_mut_ptr(), cnt)};
                Ok((&$i[cnt..],res))
            };
            ires
        }
    );
);



#[cfg(test)]
mod tests{

    use nom::{be_u8,IResult,Needed,Err,ErrorKind};

#[test]
#[allow(unsafe_code)]
fn test_slice_fixed() {
    let empty = &b""[..];
    let b = &[0x01, 0x02, 0x03, 0x04, 0x05];

    let res = slice_fixed!(b, 4);
    assert_eq!(res, Ok((&b[4..], [1, 2, 3, 4])));

    // can we still use the result ?
    match res {
        Ok((rem, _)) => {
            let res2 = be_u8(rem);
            assert_eq!(res2, Ok((empty,5)));
        },
        _ => (),
    }
}

#[test]
#[allow(unsafe_code)]
fn test_slice_fixed_incomplete() {
    let b = &[0x01, 0x02, 0x03, 0x04, 0x05];
    let res = slice_fixed!(b, 8);
    assert_eq!(res, Err(Err::Incomplete(Needed::Size(8))));
}

#[test]
fn test_error_if() {
    let empty = &b""[..];
    let res : IResult<&[u8],(),u32> = error_if!(empty, true, ErrorKind::Tag);
    assert_eq!(res, Err(Err::Error(error_position!(empty, ErrorKind::Tag))));
}

#[test]
fn test_empty() {
    let input = &[0x01][..];
    assert_eq!(empty!(input,), Err(Err::Error(error_position!(input, ErrorKind::Eof))));
    let empty = &b""[..];
    assert_eq!(empty!(empty,), Ok((empty,empty)));
}

#[test]
fn test_cond_else() {
    let input = &[0x01][..];
    let empty = &b""[..];
    let a = 1;
    assert_eq!(cond_else!(input,a == 1,call!(be_u8),value!(0x02)), Ok((empty,0x01)));
    assert_eq!(cond_else!(input,a == 1,be_u8,value!(0x02)), Ok((empty,0x01)));
    assert_eq!(cond_else!(input,a == 2,be_u8,value!(0x02)), Ok((input,0x02)));
    assert_eq!(cond_else!(input,a == 1,value!(0x02),be_u8), Ok((input,0x02)));
    assert_eq!(cond_else!(input,a == 1,be_u8,be_u8), Ok((empty,0x01)));
}

#[test]
fn test_newtype_enum() {
    #[derive(Debug, PartialEq, Eq)]
    struct MyType(pub u8);

    newtype_enum!{
        impl display MyType {
            Val1 = 0,
            Val2 = 1
        }
    }

    assert_eq!(MyType(0), MyType::Val1);
    assert_eq!(MyType(1), MyType::Val2);

    assert_eq!(
        format!("{}", MyType(0)),
        "Val1"
    );
    assert_eq!(
        format!("{}", MyType(4)),
        "MyType(4 / 0x4)"
    );
}

}
