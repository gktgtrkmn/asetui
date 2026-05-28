use nom::{IResult, bytes::complete::take};

use crate::parser::{
    DWORD, SHORT, WORD, chunk::AsepriteChunkParser, parse_dword, parse_short, parse_word,
    primitives::parse_string, skip_bytes,
};

#[derive(Debug, PartialEq)]
pub struct TilesetChunk<'a> {
    pub id: DWORD,
    pub flags: DWORD,
    pub num_tiles: DWORD,
    pub tile_width: WORD,
    pub tile_height: WORD,
    pub base_index: SHORT,
    pub name: String,
    pub external: Option<(DWORD, DWORD)>,
    pub compressed_image: Option<&'a [u8]>,
}

impl<'a> AsepriteChunkParser<'a> for TilesetChunk<'a> {
    const CHUNK_TYPE: WORD = 0x2023;

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (input, id) = parse_dword(input)?;
        let (input, flags) = parse_dword(input)?;
        let (input, num_tiles) = parse_dword(input)?;
        let (input, tile_width) = parse_word(input)?;
        let (input, tile_height) = parse_word(input)?;
        let (input, base_index) = parse_short(input)?;
        let (input, _) = skip_bytes(input, 14)?;
        let (input, name) = parse_string(input)?;
        let (input, external) = if flags & 1 != 0 {
            let (i, file_id) = parse_dword(input)?;
            let (i, ts_id) = parse_dword(i)?;
            (i, Some((file_id, ts_id)))
        } else {
            (input, None)
        };
        let (input, compressed_image) = if flags & 2 != 0 {
            let (i, len) = parse_dword(input)?;
            let (i, data) = take(len as usize)(i)?;
            (i, Some(data))
        } else {
            (input, None)
        };
        Ok((
            input,
            TilesetChunk {
                id,
                flags,
                num_tiles,
                tile_width,
                tile_height,
                base_index,
                name,
                external,
                compressed_image,
            },
        ))
    }
}
