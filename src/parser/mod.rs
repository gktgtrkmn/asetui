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

#[derive(Debug)]
pub struct AsepriteHeader {
    pub file_size: DWORD,
    pub magic_number: WORD, // must be 0xA5E0
    pub frames: WORD,
    pub width: WORD,
    pub height: WORD,
    pub color_depth: WORD,
    pub flags: DWORD,
    pub speed: WORD, // deprecated
    pub transparent_index: BYTE,
    pub number_of_colors: WORD,
    pub pixel_width: BYTE,
    pub pixel_height: BYTE,
    pub grid_x: SHORT,
    pub grid_y: SHORT,
    pub grid_width: WORD,
    pub grid_height: WORD,
}

pub fn parse_main_header(input: &[u8]) -> IResult<&[u8], AsepriteHeader> {
    let (input, file_size) = parse_dword(input)?;
    let (input, magic_number) = parse_word(input)?;
    let (input, frames) = parse_word(input)?;
    let (input, width) = parse_word(input)?;
    let (input, height) = parse_word(input)?;
    let (input, color_depth) = parse_word(input)?;
    let (input, flags) = parse_dword(input)?;
    let (input, speed) = parse_word(input)?;
    let (input, _) = skip_bytes(input, 8)?; // two deprecated dwords (8 bytes), set to zero in doc
    let (input, transparent_index) = parse_byte(input)?;
    let (input, _) = skip_bytes(input, 3)?; // ignore next 3 bytes
    let (input, number_of_colors) = parse_word(input)?;
    let (input, pixel_width) = parse_byte(input)?;
    let (input, pixel_height) = parse_byte(input)?;
    let (input, grid_x) = parse_short(input)?;
    let (input, grid_y) = parse_short(input)?;
    let (input, grid_width) = parse_word(input)?;
    let (input, grid_height) = parse_word(input)?;
    let (input, _) = skip_bytes(input, 84)?; // 84 bytes reserved for future, skip

    Ok((
        input,
        AsepriteHeader {
            file_size,
            magic_number,
            frames,
            width,
            height,
            color_depth,
            flags,
            speed,
            transparent_index,
            number_of_colors,
            pixel_width,
            pixel_height,
            grid_x,
            grid_y,
            grid_width,
            grid_height,
        },
    ))
}
