//! General purpose combinators

use nom::bytes::streaming::take;
use nom::error::ParseError;
use nom::{IResult, InputLength, ToUsize};
use nom::{InputIter, InputTake};

/// Create a combinator that returns the provided value, and input unchanged
pub fn pure<I, O, E: ParseError<I>>(val: O) -> impl Fn(I) -> IResult<I, O, E>
where
    O: Clone,
{
    move |input: I| Ok((input, val.clone()))
}

/// Return a closure that takes `len` bytes from input, and applies `parser`.
pub fn flat_take<I, C, O, E: ParseError<I>, F>(len: C, parser: F) -> impl Fn(I) -> IResult<I, O, E>
where
    I: InputTake + InputLength + InputIter,
    C: ToUsize + Copy,
    F: Fn(I) -> IResult<I, O, E>,
{
    // Note: this is the same as `map_parser(take(len), parser)`
    move |input: I| {
        let (input, o1) = take(len.to_usize())(input)?;
        let (_, o2) = parser(o1)?;
        Ok((input, o2))
    }
}

/// Take `len` bytes from `input`, and apply `parser`.
pub fn flat_takec<I: Clone, O, E: ParseError<I>, C, F>(
    input: I,
    len: C,
    parser: F,
) -> IResult<I, O, E>
where
    C: ToUsize + Copy,
    F: Fn(I) -> IResult<I, O, E>,
    I: InputTake + InputLength + InputIter,
    O: InputLength,
{
    flat_take(len, parser)(input)
}

/// Helper macro for nom parsers: run first parser if condition is true, else second parser
pub fn cond_else<I: Clone, O, E: ParseError<I>, C, F, G>(
    cond: C,
    first: F,
    second: G,
) -> impl Fn(I) -> IResult<I, O, E>
where
    C: Fn() -> bool,
    F: Fn(I) -> IResult<I, O, E>,
    G: Fn(I) -> IResult<I, O, E>,
{
    move |input: I| {
        if cond() {
            first(input)
        } else {
            second(input)
        }
    }
}

/// Align input value to the next multiple of n bytes
/// Valid only if n is a power of 2
pub const fn align_n2(x: usize, n: usize) -> usize {
    (x + (n - 1)) & !(n - 1)
}

/// Align input value to the next multiple of 4 bytes
pub const fn align32(x: usize) -> usize {
    (x + 3) & !3
}

#[cfg(test)]
mod tests {
    use super::{align32, cond_else, flat_take, pure};
    use nom::bytes::streaming::take;
    use nom::number::streaming::{be_u16, be_u32, be_u8};
    use nom::{Err, IResult, Needed};

    #[test]
    fn test_flat_take() {
        let input = &[0x00, 0x01, 0xff];
        // read first 2 bytes and use correct combinator: OK
        let res: IResult<&[u8], u16> = flat_take(2u8, be_u16)(input);
        assert_eq!(res, Ok((&input[2..], 0x0001)));
        // read 3 bytes and use 2: OK (some input is just lost)
        let res: IResult<&[u8], u16> = flat_take(3u8, be_u16)(input);
        assert_eq!(res, Ok((&b""[..], 0x0001)));
        // read 2 bytes and a combinator requiring more bytes
        let res: IResult<&[u8], u32> = flat_take(2u8, be_u32)(input);
        assert_eq!(res, Err(Err::Incomplete(Needed::new(2))));
    }

    #[test]
    fn test_flat_take_str() {
        let input = "abcdef";
        // read first 2 bytes and use correct combinator: OK
        let res: IResult<&str, &str> = flat_take(2u8, take(2u8))(input);
        assert_eq!(res, Ok(("cdef", "ab")));
        // read 3 bytes and use 2: OK (some input is just lost)
        let res: IResult<&str, &str> = flat_take(3u8, take(2u8))(input);
        assert_eq!(res, Ok(("def", "ab")));
        // read 2 bytes and a use combinator requiring more bytes
        let res: IResult<&str, &str> = flat_take(2u8, take(4u8))(input);
        assert_eq!(res, Err(Err::Incomplete(Needed::Unknown)));
    }

    #[test]
    fn test_cond_else() {
        let input = &[0x01][..];
        let empty = &b""[..];
        let a = 1;
        fn parse_u8(i: &[u8]) -> IResult<&[u8], u8> {
            be_u8(i)
        }
        assert_eq!(
            cond_else(|| a == 1, parse_u8, pure(0x02))(input),
            Ok((empty, 0x01))
        );
        assert_eq!(
            cond_else(|| a == 1, parse_u8, pure(0x02))(input),
            Ok((empty, 0x01))
        );
        assert_eq!(
            cond_else(|| a == 2, parse_u8, pure(0x02))(input),
            Ok((input, 0x02))
        );
        assert_eq!(
            cond_else(|| a == 1, pure(0x02), parse_u8)(input),
            Ok((input, 0x02))
        );
        let res: IResult<&[u8], u8> = cond_else(|| a == 1, parse_u8, parse_u8)(input);
        assert_eq!(res, Ok((empty, 0x01)));
    }

    #[test]
    fn test_align32() {
        assert_eq!(align32(3), 4);
        assert_eq!(align32(4), 4);
        assert_eq!(align32(5), 8);
        assert_eq!(align32(5usize), 8);
    }
}
