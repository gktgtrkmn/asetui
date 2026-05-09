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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct AsepriteFrameHeader {
    pub bytes_in_frame: DWORD,
    pub magic_number: WORD,
    pub number_of_chunks_old: WORD, // if 0xFFFF, use the new field
    pub frame_duration: WORD,       // in milliseconds
    // BYTE[2] for future (skip, set to zero)
    pub number_of_chunks_new: DWORD,
}

#[derive(Debug, PartialEq)]
pub struct Chunk<'a> {
    pub chunk_size: DWORD,
    pub chunk_type: WORD,
    pub chunk_data: &'a [BYTE],
}

#[derive(Debug, PartialEq)]
pub struct Frame<'a> {
    pub header: AsepriteFrameHeader,
    pub chunks: Vec<Chunk<'a>>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_main_header_success() -> Result<(), String> {
        let mut mock_header = [0u8; 128];
        mock_header[0..4].copy_from_slice(&[0x00, 0x01, 0x00, 0x00]);
        mock_header[4..6].copy_from_slice(&[0xE0, 0xA5]);
        mock_header[6..8].copy_from_slice(&[0x0C, 0x00]);
        mock_header[8..10].copy_from_slice(&[0x20, 0x00]);
        mock_header[10..12].copy_from_slice(&[0x20, 0x00]);
        mock_header[12..14].copy_from_slice(&[0x20, 0x00]);
        mock_header[14..18].copy_from_slice(&[0x01, 0x00, 0x00, 0x00]);
        mock_header[18..20].copy_from_slice(&[0x64, 0x00]);
        mock_header[28] = 0x00;
        mock_header[32..34].copy_from_slice(&[0x00, 0x01]);
        mock_header[34] = 0x01;
        mock_header[35] = 0x01;
        mock_header[36..38].copy_from_slice(&[0x00, 0x00]);
        mock_header[38..40].copy_from_slice(&[0x00, 0x00]);
        mock_header[40..42].copy_from_slice(&[0x10, 0x00]);
        mock_header[42..44].copy_from_slice(&[0x10, 0x00]);

        let (remaining_bytes, header) =
            parse_main_header(&mock_header).map_err(|e| format!("Parsing failed: {:?}", e))?;

        assert_eq!(remaining_bytes.len(), 0);

        assert_eq!(header.file_size, 256);
        assert_eq!(header.magic_number, 0xA5E0);
        assert_eq!(header.frames, 12);
        assert_eq!(header.width, 32);
        assert_eq!(header.height, 32);
        assert_eq!(header.color_depth, 32);
        assert_eq!(header.flags, 1);
        assert_eq!(header.speed, 100);
        assert_eq!(header.number_of_colors, 256);
        assert_eq!(header.pixel_width, 1);
        assert_eq!(header.pixel_height, 1);
        assert_eq!(header.grid_width, 16);
        assert_eq!(header.grid_height, 16);

        Ok(())
    }
}
