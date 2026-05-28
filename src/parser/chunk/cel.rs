use nom::{IResult, bytes::complete::take, error::ErrorKind};

use crate::parser::{
    BYTE, DWORD, SHORT, WORD,
    chunk::{AsepriteChunkParser, NoCtx},
    parse_byte, parse_dword, parse_long, parse_short, parse_word,
    primitives::FIXED,
    skip_bytes,
};

#[derive(Debug, PartialEq)]
pub enum CelData<'a> {
    RawImage {
        w: WORD,
        h: WORD,
        pixels: &'a [u8],
    },
    Linked {
        frame: WORD,
    },
    CompressedImage {
        w: WORD,
        h: WORD,
        zlib: &'a [u8],
    },
    CompressedTilemap {
        w: WORD,
        h: WORD,
        bits_per_tile: WORD,
        tile_id_mask: DWORD,
        x_flip_mask: DWORD,
        y_flip_mask: DWORD,
        diag_flip_mask: DWORD,
        zlib_tiles: &'a [u8],
    },
}

#[derive(Debug, PartialEq)]
pub struct CelChunk<'a> {
    pub layer_index: WORD,
    pub x: SHORT,
    pub y: SHORT,
    pub opacity: BYTE,
    pub z_index: SHORT,
    pub data: CelData<'a>,
}

#[derive(Debug, PartialEq)]
pub struct CelExtraChunk {
    pub flags: DWORD,
    pub precise_x: FIXED,
    pub precise_y: FIXED,
    pub width: FIXED,
    pub height: FIXED,
}

impl<'a> AsepriteChunkParser<'a> for CelChunk<'a> {
    const CHUNK_TYPE: WORD = 0x2005;
    type Need = NoCtx;

    fn parse_data(input: &'a [u8], _: ()) -> IResult<&'a [u8], Self> {
        let (input, layer_index) = parse_word(input)?;
        let (input, x) = parse_short(input)?;
        let (input, y) = parse_short(input)?;
        let (input, opacity) = parse_byte(input)?;
        let (input, cel_type) = parse_word(input)?;
        let (input, z_index) = parse_short(input)?;
        let (input, _) = skip_bytes(input, 5)?;
        let (input, data) = match cel_type {
            0 => {
                let (input, w) = parse_word(input)?;
                let (input, h) = parse_word(input)?;
                let (input, pixels) = take(input.len())(input)?;
                (input, CelData::RawImage { w, h, pixels })
            }
            1 => {
                let (input, frame) = parse_word(input)?;
                (input, CelData::Linked { frame })
            }
            2 => {
                let (input, w) = parse_word(input)?;
                let (input, h) = parse_word(input)?;
                let (input, zlib) = take(input.len())(input)?;
                (input, CelData::CompressedImage { w, h, zlib })
            }
            3 => {
                let (input, w) = parse_word(input)?;
                let (input, h) = parse_word(input)?;
                let (input, bits_per_tile) = parse_word(input)?;
                let (input, tile_id_mask) = parse_dword(input)?;
                let (input, x_flip_mask) = parse_dword(input)?;
                let (input, y_flip_mask) = parse_dword(input)?;
                let (input, diag_flip_mask) = parse_dword(input)?;
                let (input, _) = skip_bytes(input, 10)?;
                let (input, zlib_tiles) = take(input.len())(input)?;
                (
                    input,
                    CelData::CompressedTilemap {
                        w,
                        h,
                        bits_per_tile,
                        tile_id_mask,
                        x_flip_mask,
                        y_flip_mask,
                        diag_flip_mask,
                        zlib_tiles,
                    },
                )
            }
            _ => {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    ErrorKind::Verify,
                )));
            }
        };
        Ok((
            input,
            CelChunk {
                layer_index,
                x,
                y,
                opacity,
                z_index,
                data,
            },
        ))
    }
}

impl<'a> AsepriteChunkParser<'a> for CelExtraChunk {
    const CHUNK_TYPE: WORD = 0x2006;
    type Need = NoCtx;

    fn parse_data(input: &'a [u8], _: ()) -> IResult<&'a [u8], Self> {
        let (input, flags) = parse_dword(input)?;
        let (input, precise_x) = parse_long(input)?;
        let (input, precise_y) = parse_long(input)?;
        let (input, width) = parse_long(input)?;
        let (input, height) = parse_long(input)?;
        let (input, _) = skip_bytes(input, 16)?;
        Ok((
            input,
            CelExtraChunk {
                flags,
                precise_x,
                precise_y,
                width,
                height,
            },
        ))
    }
}
