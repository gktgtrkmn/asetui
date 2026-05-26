use crate::parser::chunk::Chunk;
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
