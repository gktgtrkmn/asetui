use crate::parser::primitives::{BYTE, DWORD, WORD};

#[derive(Debug, PartialEq)]
pub struct Chunk<'a> {
    pub chunk_size: DWORD,
    pub chunk_type: WORD,
    pub chunk_data: &'a [BYTE],
}
