#[derive(Debug)]
pub enum GenError {
    BufferTooSmall(usize),
    InvalidOffset,

    CustomError(u32),
    NotYetImplemented,
}


#[inline]
pub fn set_be_u8(x:(&mut [u8],usize),v:u8) -> Result<(&mut [u8],usize),GenError> {
    let (i,idx) = x;
    match i.len() <= idx {
        true  => Err(GenError::BufferTooSmall(idx+1)),
        false => {
            i[idx] = v;
            Ok((i,idx+1))
        }
    }
}

#[inline]
pub fn set_be_u16(x:(&mut [u8],usize),v:u16) -> Result<(&mut [u8],usize),GenError> {
    let (i,idx) = x;
    match i.len() <= idx+1 {
        true  => Err(GenError::BufferTooSmall(idx+2)),
        false => {
            let v1 : u8 = ((v >>  8) & 0xff) as u8;
            let v2 : u8 = ((v      ) & 0xff) as u8;
            i[idx  ] = v1;
            i[idx+1] = v2;
            Ok((i,idx+2))
        }
    }
}

#[inline]
pub fn set_be_u32(x:(&mut [u8],usize),v:u32) -> Result<(&mut [u8],usize),GenError> {
    let (i,idx) = x;
    match i.len() <= idx+3 {
        true  => Err(GenError::BufferTooSmall(idx+4)),
        false => {
            let v1 : u8 =  (v >> 24)         as u8;
            let v2 : u8 = ((v >> 16) & 0xff) as u8;
            let v3 : u8 = ((v >>  8) & 0xff) as u8;
            let v4 : u8 = ((v      ) & 0xff) as u8;
            i[idx  ] = v1;
            i[idx+1] = v2;
            i[idx+2] = v3;
            i[idx+3] = v4;
            Ok((i,idx+4))
        }
    }
}

#[macro_export]
macro_rules! gen_align(
    (($i:expr, $idx:expr), $val:expr) => (
        {
            let aligned = $val - ($idx % $val);
            match $i.len() <= $idx+aligned {
                true  => Err(GenError::BufferTooSmall($idx+aligned)),
                false => { Ok(($i,($idx+aligned))) },
            }
        }
    );
    ($i:expr, $val:expr) => ( gen_skip!(($i.0, $i.1), $val) );
);

#[macro_export]
macro_rules! gen_skip(
    (($i:expr, $idx:expr), $val:expr) => (
        match $i.len() < $idx+$val {
            true  => Err(GenError::BufferTooSmall($idx+$val)),
            false => { Ok(($i,($idx+$val))) },
        }
    );
    ($i:expr, $val:expr) => ( gen_skip!(($i.0, $i.1), $val) );
);


#[macro_export]
macro_rules! gen_be_u8(
    (($i:expr, $idx:expr), $val:expr) => (
        match $i.len() <= $idx {
            true  => Err(GenError::BufferTooSmall($idx+1)),
            false => {
                $i[$idx] = $val;
                Ok(($i,($idx+1)))
            }
        }
    );
    ($i:expr, $val:expr) => (
        gen_be_u8!(($i.0, $i.1), $val)
    );
);

#[macro_export]
macro_rules! gen_be_u16(
    (($i:expr, $idx:expr), $val:expr) => (
        match $i.len() <= $idx + 1 {
            true  => Err(GenError::BufferTooSmall($idx+2)),
            false => {
                let v1 : u8 = (($val >>  8) & 0xff) as u8;
                let v2 : u8 = (($val      ) & 0xff) as u8;
                $i[$idx  ] = v1;
                $i[$idx+1] = v2;
                Ok(($i,($idx+2)))
            }
        }
    );
    ($i:expr, $val:expr) => (
        gen_be_u16!(($i.0, $i.1), $val)
    );
);

#[macro_export]
macro_rules! gen_be_u24(
    (($i:expr, $idx:expr), $val:expr) => (
        match $i.len() <= $idx + 2 {
            true  => Err(GenError::BufferTooSmall($idx+3)),
            false => {
                let v1 : u8 = (($val >> 16) & 0xff) as u8;
                let v2 : u8 = (($val >>  8) & 0xff) as u8;
                let v3 : u8 = (($val      ) & 0xff) as u8;
                $i[$idx  ] = v1;
                $i[$idx+1] = v2;
                $i[$idx+2] = v3;
                Ok(($i,($idx+3)))
            }
        }
    );
    ($i:expr, $val:expr) => (
        gen_be_u24!(($i.0, $i.1), $val)
    );
);

#[macro_export]
macro_rules! gen_be_u32(
    (($i:expr, $idx:expr), $val:expr) => (
        match $i.len() <= $idx + 3 {
            true  => Err(GenError::BufferTooSmall($idx+4)),
            false => {
                let v1 : u8 = (($val >> 24) & 0xff) as u8;
                let v2 : u8 = (($val >> 16) & 0xff) as u8;
                let v3 : u8 = (($val >>  8) & 0xff) as u8;
                let v4 : u8 = (($val      ) & 0xff) as u8;
                $i[$idx  ] = v1;
                $i[$idx+1] = v2;
                $i[$idx+2] = v3;
                $i[$idx+3] = v4;
                Ok(($i,($idx+4)))
            }
        }
    );
    ($i:expr, $val:expr) => (
        gen_be_u32!(($i.0, $i.1), $val)
    );
);

#[macro_export]
macro_rules! gen_copy(
    (($i:expr, $idx:expr), $val:expr, $l:expr) => (
        match $i.len() <= $idx+$l {
            true  => Err(GenError::BufferTooSmall($idx+$l+1)),
            false => {
                $i[$idx..$idx+$l].clone_from_slice(&$val[0..$l]);
                Ok(($i,($idx+$l)))
            }
        }
    );
);


#[macro_export]
macro_rules! gen_call(
    (($i:expr, $idx:expr), $fun:expr) => ( $fun( ($i,$idx) ) );
    (($i:expr, $idx:expr), $fun:expr, $($args:expr),* ) => ( $fun( ($i,$idx), $($args),* ) );
);


#[macro_export]
macro_rules! do_gen(
    (__impl $i:expr, $idx:expr, ( $($rest:expr),* )) => (
        {
            $($rest)*;
            Ok(($i,$idx))
        }
    );
    (__impl $i:expr, $idx:expr, $e:ident >> $($rest:tt)*) => (
        do_gen!(__impl $i, $idx, gen_call!($e) >> $($rest)*)
    );
    (__impl $i:expr, $idx:expr, $e:ident( $($args:tt)* )) => (
        do_gen!(__impl $i, $idx, gen_call!($e,$($args)*))
    );
    (__impl $i:expr, $idx:expr, $e:ident( $($args:tt)* ) >> $($rest:tt)*) => (
        do_gen!(__impl $i, $idx, gen_call!($e,$($args)*) >> $($rest)*)
    );
    (__impl $i:expr, $idx:expr, $submac:ident!( $($args:tt)* ) >> $($rest:tt)*) => (
        {
            match $submac!(($i,$idx), $($args)*) {
                Ok((j,idx2)) => {
                    do_gen!(__impl j, idx2, $($rest)*)
                },
                Err(e) => Err(e),
            }
        }
    );
    (__impl $i:expr, $idx:expr, $submac:ident!( $($args:tt)* )) => (
        $submac!(($i,$idx), $($args)*)
    );

    ( ($i:expr, $idx:expr), $($rest:tt)*) => (
        do_gen!(__impl $i, $idx, $($rest)*)
    );
    ( $i:expr, $($rest:tt)*) => (
        do_gen!(__impl $i.0, $i.1, $($rest)*)
    );
);




#[macro_export]
macro_rules! gen_cond(
    (($i:expr, $idx:expr), $cond:expr, $submac:ident!( $($args:tt)* )) => (
        {
            if $cond {
                $submac!(($i,$idx), $($args)*)
            } else {
                Ok(($i,$idx))
            }
        }
    );
    (($i:expr, $idx:expr), $cond:expr, $f:expr) => (
        gen_cond!(($i,$idx), $cond, gen_call!($f))
    );
);

#[macro_export]
macro_rules! gen_if_else(
    (($i:expr, $idx:expr), $cond:expr, $submac_if:ident!( $($args_if:tt)* ), $submac_else:ident!( $($args_else:tt)* )) => (
        {
            if $cond {
                $submac_if!(($i,$idx), $($args_if)*)
            } else {
                $submac_else!(($i,$idx), $($args_else)*)
            }
        }
    );
    (($i:expr, $idx:expr), $cond:expr, $f:expr, $g:expr) => (
        gen_cond!(($i,$idx), $cond, gen_call!($f), gen_call!($g))
    );
);

#[macro_export]
macro_rules! gen_many_ref(
    (($i:expr, $idx:expr), $l:expr, $f:expr) => (
        $l.iter().fold(
            Ok(($i,$idx)),
            |r,ref v| {
                match r {
                    Err(e) => Err(e),
                    Ok(x) => { $f(x, v) },
                }
            }
        )
    );
);

#[macro_export]
macro_rules! gen_many(
    (($i:expr, $idx:expr), $l:expr, $f:expr) => (
        $l.iter().fold(
            Ok(($i,$idx)),
            |r,&v| {
                match r {
                    Err(e) => Err(e),
                    Ok(x) => { $f(x, v) },
                }
            }
        )
    );
);


/// Write the length taken from (start) to (current position) at
/// (offset)
/// Then, returns to current offset
#[macro_export]
macro_rules! gen_adjust_length_u16(
    (($i:expr, $idx:expr), $start:expr, $offset:expr) => (
        {
            let r = ($idx as u32).overflowing_sub($start);
            if r.1 == false && r.0 < (std::u16::MAX as u32) {
                match gen_be_u16!(($i,$offset),r.0 as u16) {
                    Err(e)    => Err(e),
                    Ok((s,_)) => Ok((s,$idx))
                }
            } else {
                Err(GenError::InvalidOffset)
            }
        }
    );
);


#[cfg(test)]
mod tests {
    use super::*;
    use std;

    #[test]
    fn test_do_gen() {
        let mut mem : [u8; 8] = [0; 8];
        let s = &mut mem[..];
        let expected = [1, 2, 3, 4, 5, 6, 7, 8];
        let r = do_gen!(
            (s,0),
            gen_be_u8!(1) >>
            gen_be_u8!(2) >>
            gen_be_u16!(0x0304) >>
            gen_be_u32!(0x05060708)
        );
        match r {
            Ok((b,idx)) => {
                assert_eq!(idx,8);
                assert_eq!(b,&expected);
            },
            Err(e) => panic!("error {:?}",e),
        }
    }

    #[test]
    fn test_gen_skip() {
        let mut mem : [u8; 8] = [0; 8];
        let s = &mut mem[..];
        let expected = [0, 0, 0, 0, 0, 0, 0, 0];
        let r = gen_skip!((s,0),5);
        match r {
            Ok((b,idx)) => {
                assert_eq!(idx,5);
                assert_eq!(b,&expected);
            },
            Err(e) => panic!("error {:?}",e),
        }
    }

    #[test]
    fn test_gen_adjust_length_u16() {
        let mut mem : [u8; 8] = [0; 8];
        let s = &mut mem[..];
        let expected = [0, 8, 0, 0, 0, 0, 0, 0];
        let r = do_gen!(
            (s,0),
            gen_skip!(8) >>
            gen_adjust_length_u16!(0,0)
        );
        match r {
            Ok((b,idx)) => {
                assert_eq!(idx,8);
                assert_eq!(b,&expected);
            },
            Err(e) => panic!("error {:?}",e),
        }

        let mut mem2 : [u8; 8] = [0; 8];
        let s2 = &mut mem2[..];
        let r2 = gen_adjust_length_u16!((s2,0),4,0);
        match r2 {
            Err(GenError::InvalidOffset) => (),
            _ => panic!("gen_adjust_length_u16 failed"),
        };
    }

    #[test]
    fn test_gen_be_u8() {
        let mut mem : [u8; 8] = [0; 8];
        let s = &mut mem[..];
        let expected = [1, 2, 0, 0, 0, 0, 0, 0];
        let r = do_gen!(
            (s,0),
            gen_be_u8!(1) >>
            gen_be_u8!(2)
        );
        match r {
            Ok((b,idx)) => {
                assert_eq!(idx,2);
                assert_eq!(b,&expected);
            },
            Err(e) => panic!("error {:?}",e),
        }
    }

    #[test]
    fn test_set_be_u8() {
        let mut mem : [u8; 8] = [0; 8];
        let s = &mut mem[..];
        let expected = [1, 2, 0, 0, 0, 0, 0, 0];
        let r = do_gen!(
            (s,0),
            set_be_u8(1) >>
            set_be_u8(2)
        );
        match r {
            Ok((b,idx)) => {
                assert_eq!(idx,2);
                assert_eq!(b,&expected);
            },
            Err(e) => panic!("error {:?}",e),
        }
    }

    #[test]
    fn test_gen_align() {
        let mut mem : [u8; 8] = [0; 8];
        let s = &mut mem[..];
        let expected = [1, 0, 0, 0, 1, 0, 0, 0];
        let r = do_gen!(
            (s,0),
            gen_be_u8!(1) >>
            gen_align!(4) >>
            gen_be_u8!(1)
        );
        match r {
            Ok((b,idx)) => {
                assert_eq!(idx,5);
                assert_eq!(b,&expected);
            },
            Err(e) => panic!("error {:?}",e),
        }
    }

    #[test]
    fn test_gen_many() {
        let mut mem : [u8; 8] = [0; 8];
        let s = &mut mem[..];
        let v : Vec<u16> = vec![1, 2, 3, 4];
        let expected = [0, 1, 0, 2, 0, 3, 0, 4];
        let r = gen_many!((s,0),v,set_be_u16);
        match r {
            Ok((b,idx)) => {
                assert_eq!(idx,8);
                assert_eq!(b,&expected);
            },
            Err(e) => panic!("error {:?}",e),
        }
    }

    #[test]
    fn test_gen_copy() {
        let mut mem : [u8; 8] = [0; 8];
        let s = &mut mem[..];
        let v = [1, 2, 3, 4];
        let expected = [1, 2, 3, 4, 0, 0, 0, 0];
        let r = gen_copy!((s,0),v,v.len());
        match r {
            Ok((b,idx)) => {
                assert_eq!(idx,4);
                assert_eq!(b,&expected);
            },
            Err(e) => panic!("error {:?}",e),
        }
    }
}
