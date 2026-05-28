use nom::{IResult, Parser, multi::count};

use crate::parser::{
    BYTE, DWORD, WORD, chunk::AsepriteChunkParser, parse_byte, parse_dword,
    primitives::parse_string, skip_bytes,
};

#[derive(Debug, PartialEq)]
pub struct ExternalFileEntry {
    pub id: DWORD,
    pub kind: BYTE,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct ExternalFilesChunk {
    pub entries: Vec<ExternalFileEntry>,
}

fn parse_external_file_entry(input: &[u8]) -> IResult<&[u8], ExternalFileEntry> {
    let (input, id) = parse_dword(input)?;
    let (input, kind) = parse_byte(input)?;
    let (input, _) = skip_bytes(input, 7)?;
    let (input, name) = parse_string(input)?;
    Ok((input, ExternalFileEntry { id, kind, name }))
}

impl<'a> AsepriteChunkParser<'a> for ExternalFilesChunk {
    const CHUNK_TYPE: WORD = 0x2008;

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (input, num_entries) = parse_dword(input)?;
        let (input, _) = skip_bytes(input, 8)?;
        let (input, entries) =
            count(parse_external_file_entry, num_entries as usize).parse(input)?;
        Ok((input, ExternalFilesChunk { entries }))
    }
}
