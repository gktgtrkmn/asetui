use nom::IResult;
use uuid::Uuid;

use crate::parser::chunk::{AsepriteChunkParser, ParseContext, WithCtx};
use crate::parser::primitives::{parse_string, parse_uuid};
use crate::parser::{BYTE, DWORD, WORD, parse_byte, parse_dword, parse_word, skip_bytes};

#[derive(Debug, PartialEq)]
pub struct LayerChunk {
    pub flags: WORD,
    pub layer_type: WORD,
    pub child_level: WORD,
    pub default_width: WORD,
    pub default_height: WORD,
    pub blend_mode: WORD,
    pub opacity: BYTE,
    pub name: String,
    pub tileset_index: Option<DWORD>,
    pub uuid: Option<Uuid>,
}

impl<'a> AsepriteChunkParser<'a> for LayerChunk {
    const CHUNK_TYPE: WORD = 0x2004;
    type Need = WithCtx;

    fn parse_data(input: &'a [u8], ctx: ParseContext) -> IResult<&'a [u8], Self> {
        let (input, flags) = parse_word(input)?;
        let (input, layer_type) = parse_word(input)?;
        let (input, child_level) = parse_word(input)?;
        let (input, default_width) = parse_word(input)?;
        let (input, default_height) = parse_word(input)?;
        let (input, blend_mode) = parse_word(input)?;
        let (input, opacity) = parse_byte(input)?;
        let (input, _) = skip_bytes(input, 3)?;
        let (input, name) = parse_string(input)?;
        let (input, tileset_index) = if layer_type == 2 {
            let (i, idx) = parse_dword(input)?;
            (i, Some(idx))
        } else {
            (input, None)
        };
        let (input, uuid) = if ctx.layers_have_uuid {
            let (i, id) = parse_uuid(input)?;
            (i, Some(id))
        } else {
            (input, None)
        };
        Ok((
            input,
            LayerChunk {
                flags,
                layer_type,
                child_level,
                default_width,
                default_height,
                blend_mode,
                opacity,
                name,
                tileset_index,
                uuid,
            },
        ))
    }
}
