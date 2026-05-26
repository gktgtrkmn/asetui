use nom::IResult;

use crate::parser::chunk::Chunk;
use crate::parser::{parse_dword, parse_magic_word, parse_word, skip_bytes};
use crate::parser::primitives::{DWORD, WORD};

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
pub struct Frame<'a> {
    pub header: AsepriteFrameHeader,
    pub chunks: Vec<Chunk<'a>>,
}

pub fn parse_aseprite_frame_header(input: &[u8]) -> IResult<&[u8], AsepriteFrameHeader> {
    let (input, bytes_in_frame) = parse_dword(input)?;
    let (input, magic_number) = parse_magic_word::<0xF1FA>(input)?;
    let (input, number_of_chunks_old) = parse_word(input)?;
    let (input, frame_duration) = parse_word(input)?;
    let (input, _) = skip_bytes(input, 2)?;
    let (input, number_of_chunks_new) = parse_dword(input)?;
    Ok((
        input,
        AsepriteFrameHeader {
            bytes_in_frame,
            magic_number,
            number_of_chunks_old,
            frame_duration,
            number_of_chunks_new
        }
    ))
}