use nom::{IResult, bytes::complete::take};

use crate::parser::{
    SHORT, WORD,
    chunk::{AsepriteChunkParser, NoCtx},
    parse_short, parse_word,
    primitives::parse_string,
    skip_bytes,
};

#[derive(Debug, PartialEq)]
pub struct MaskChunk<'a> {
    pub x: SHORT,
    pub y: SHORT,
    pub width: WORD,
    pub height: WORD,
    pub name: String,
    pub bitmap: &'a [u8],
}

impl<'a> AsepriteChunkParser<'a> for MaskChunk<'a> {
    const CHUNK_TYPE: WORD = 0x2016;
    type Need = NoCtx;

    fn parse_data(input: &'a [u8], _: ()) -> IResult<&'a [u8], Self> {
        let (input, x) = parse_short(input)?;
        let (input, y) = parse_short(input)?;
        let (input, width) = parse_word(input)?;
        let (input, height) = parse_word(input)?;
        let (input, _) = skip_bytes(input, 8)?;
        let (input, name) = parse_string(input)?;
        let bitmap_size = (height as usize) * (width as usize).div_ceil(8);
        let (input, bitmap) = take(bitmap_size)(input)?;
        Ok((
            input,
            MaskChunk {
                x,
                y,
                width,
                height,
                name,
                bitmap,
            },
        ))
    }
}
