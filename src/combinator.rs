//! General purpose combinators

use nom::bytes::streaming::take;
use nom::error::ParseError;
use nom::{IResult, InputLength, ToUsize};
use nom::{InputIter, InputTake};

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

#[cfg(test)]
mod tests {
    use super::flat_take;
    use nom::bytes::streaming::take;
    use nom::number::streaming::{be_u16, be_u32};
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
        assert_eq!(res, Err(Err::Incomplete(Needed::Size(4))));
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
        assert_eq!(res, Err(Err::Incomplete(Needed::Size(4))));
    }
}
