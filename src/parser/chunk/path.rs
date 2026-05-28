use nom::IResult;

use crate::parser::{WORD, chunk::AsepriteChunkParser};

#[derive(Debug, PartialEq)]
pub struct PathChunk;

impl<'a> AsepriteChunkParser<'a> for PathChunk {
    const CHUNK_TYPE: WORD = 0x2017;

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        Ok((input, PathChunk))
    }
}
