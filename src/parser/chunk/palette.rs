use nom::{IResult, Parser, multi::count};

use crate::parser::{
    BYTE, DWORD, WORD,
    chunk::{AsepriteChunkParser, NoCtx},
    parse_byte, parse_dword, parse_word,
    primitives::parse_string,
    skip_bytes,
};

#[derive(Debug, PartialEq)]
pub struct PaletteEntry {
    pub flags: WORD,
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
    pub name: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct PaletteChunk {
    pub new_size: DWORD,
    pub first_index: DWORD,
    pub last_index: DWORD,
    pub entries: Vec<PaletteEntry>,
}

fn parse_palette_entry(input: &[u8]) -> IResult<&[u8], PaletteEntry> {
    let (input, flags) = parse_word(input)?;
    let (input, r) = parse_byte(input)?;
    let (input, g) = parse_byte(input)?;
    let (input, b) = parse_byte(input)?;
    let (input, a) = parse_byte(input)?;
    let (input, name) = if flags & 1 != 0 {
        let (i, n) = parse_string(input)?;
        (i, Some(n))
    } else {
        (input, None)
    };
    Ok((
        input,
        PaletteEntry {
            flags,
            r,
            g,
            b,
            a,
            name,
        },
    ))
}

impl<'a> AsepriteChunkParser<'a> for PaletteChunk {
    const CHUNK_TYPE: WORD = 0x2019;
    type Need = NoCtx;

    fn parse_data(input: &'a [u8], _: ()) -> IResult<&'a [u8], Self> {
        let (input, new_size) = parse_dword(input)?;
        let (input, first_index) = parse_dword(input)?;
        let (input, last_index) = parse_dword(input)?;
        let (input, _) = skip_bytes(input, 8)?;
        let n = last_index.saturating_sub(first_index) + 1;
        let (input, entries) = count(parse_palette_entry, n as usize).parse(input)?;
        Ok((
            input,
            PaletteChunk {
                new_size,
                first_index,
                last_index,
                entries,
            },
        ))
    }
}
