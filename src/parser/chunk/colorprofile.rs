use nom::{IResult, bytes::complete::take};

use crate::parser::{
    WORD, chunk::AsepriteChunkParser, parse_dword, parse_long, parse_word, primitives::FIXED,
    skip_bytes,
};

#[derive(Debug, PartialEq)]
pub struct ColorProfileChunk<'a> {
    pub kind: WORD,
    pub flags: WORD,
    pub gamma: FIXED,
    pub icc: Option<&'a [u8]>,
}

impl<'a> AsepriteChunkParser<'a> for ColorProfileChunk<'a> {
    const CHUNK_TYPE: WORD = 0x2007;

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (input, kind) = parse_word(input)?;
        let (input, flags) = parse_word(input)?;
        let (input, gamma) = parse_long(input)?;
        let (input, _) = skip_bytes(input, 8)?;
        let (input, icc) = if kind == 2 {
            let (i, len) = parse_dword(input)?;
            let (i, data) = take(len as usize)(i)?;
            (i, Some(data))
        } else {
            (input, None)
        };
        Ok((
            input,
            ColorProfileChunk {
                kind,
                flags,
                gamma,
                icc,
            },
        ))
    }
}
