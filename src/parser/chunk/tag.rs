use nom::{IResult, Parser, bytes::complete::take, multi::count};

use crate::parser::{
    BYTE, WORD,
    chunk::{AsepriteChunkParser, NoCtx},
    parse_byte, parse_word,
    primitives::parse_string,
    skip_bytes,
};

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub from_frame: WORD,
    pub to_frame: WORD,
    pub direction: BYTE,
    pub repeat: WORD,
    pub color: [BYTE; 3],
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct TagsChunk {
    pub tags: Vec<Tag>,
}

fn parse_tag(input: &[u8]) -> IResult<&[u8], Tag> {
    let (input, from_frame) = parse_word(input)?;
    let (input, to_frame) = parse_word(input)?;
    let (input, direction) = parse_byte(input)?;
    let (input, repeat) = parse_word(input)?;
    let (input, _) = skip_bytes(input, 6)?;
    let (input, color_slice) = take(3usize)(input)?;
    let color: [u8; 3] = color_slice.try_into().unwrap();
    let (input, _) = skip_bytes(input, 1)?;
    let (input, name) = parse_string(input)?;
    Ok((
        input,
        Tag {
            from_frame,
            to_frame,
            direction,
            repeat,
            color,
            name,
        },
    ))
}

impl<'a> AsepriteChunkParser<'a> for TagsChunk {
    const CHUNK_TYPE: WORD = 0x2018;
    type Need = NoCtx;

    fn parse_data(input: &'a [u8], _: ()) -> IResult<&'a [u8], Self> {
        let (input, num_tags) = parse_word(input)?;
        let (input, _) = skip_bytes(input, 8)?;
        let (input, tags) = count(parse_tag, num_tags as usize).parse(input)?;
        Ok((input, TagsChunk { tags }))
    }
}
