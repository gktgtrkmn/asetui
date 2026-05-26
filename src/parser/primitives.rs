use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::verify,
    number::complete::{le_f32, le_f64, le_i16, le_i32, le_i64, le_u8, le_u16, le_u32, le_u64},
};

pub type BYTE = u8;
pub type WORD = u16;
pub type DWORD = u32;
pub type SHORT = i16;
pub type LONG = i32;
pub type FIXED = i32;
pub type LONG64 = i64;
pub type QWORD = u64;
pub type UUID = [u8; 16];

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

#[inline]
pub fn parse_long64(input: &[u8]) -> IResult<&[u8], LONG64> {
    le_i64(input)
}

#[inline]
pub fn parse_qword(input: &[u8]) -> IResult<&[u8], QWORD> {
    le_u64(input)
}

#[inline]
pub fn parse_float(input: &[u8]) -> IResult<&[u8], f32> {
    le_f32(input)
}

#[inline]
pub fn parse_double(input: &[u8]) -> IResult<&[u8], f64> {
    le_f64(input)
}

pub fn parse_uuid(input: &[u8]) -> IResult<&[u8], UUID> {
    let (input, bytes) = take(16usize)(input)?;
    let mut uuid = [0u8; 16];
    uuid.copy_from_slice(bytes);
    Ok((input, uuid))
}

pub fn parse_string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, len) = parse_word(input)?;
    let (input, bytes) = take(len as usize)(input)?;
    let s = String::from_utf8(bytes.to_vec()).map_err(|_| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify))
    })?;
    Ok((input, s))
}

#[inline]
pub fn parse_magic_word<const MAGIC: WORD>(input: &[u8]) -> IResult<&[u8], WORD> {
    verify(parse_word, |&m| m == MAGIC).parse(input)
}
