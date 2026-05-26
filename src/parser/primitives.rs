use nom::{
    IResult,
    bytes::complete::take,
    number::complete::{le_i16, le_i32, le_u8, le_u16, le_u32},
};

pub type BYTE = u8;
pub type WORD = u16;
pub type DWORD = u32;
pub type SHORT = i16;
pub type LONG = i32;

#[inline]
pub fn parse_byte(input: &[u8]) -> IResult<&[u8], BYTE> {
    le_u8(input)
}

#[inline]
pub fn parse_word(input: &[u8]) -> IResult<&[u8], WORD> {
    le_u16(input)
}

#[inline]
pub fn parse_short(input: &[u8]) -> IResult<&[u8], SHORT> {
    le_i16(input)
}

#[inline]
pub fn parse_dword(input: &[u8]) -> IResult<&[u8], DWORD> {
    le_u32(input)
}

#[inline]
pub fn parse_long(input: &[u8]) -> IResult<&[u8], LONG> {
    le_i32(input)
}

#[inline]
pub fn skip_bytes(input: &[u8], count: usize) -> IResult<&[u8], ()> {
    let (input, _) = take(count)(input)?;
    Ok((input, ()))
}
