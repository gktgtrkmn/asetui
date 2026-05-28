use nom::IResult;
use nom::Parser;
use nom::multi::count;

use crate::parser::chunk::{AsepriteChunk, parse_aseprite_chunk};
use crate::parser::primitives::{DWORD, WORD};
use crate::parser::{parse_dword, parse_magic_word, parse_word, skip_bytes};

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
    pub chunks: Vec<AsepriteChunk<'a>>,
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
            number_of_chunks_new,
        },
    ))
}

pub fn parse_aseprite_frame(input: &[u8]) -> IResult<&[u8], Frame<'_>> {
    let (input, header) = parse_aseprite_frame_header(input)?;
    let chunk_count: usize = if header.number_of_chunks_old == 0xFFFF {
        header.number_of_chunks_new as usize
    } else {
        header.number_of_chunks_old as usize
    };
    let (input, chunks) = count(parse_aseprite_chunk, chunk_count).parse(input)?;
    Ok((input, Frame { header, chunks }))
}
