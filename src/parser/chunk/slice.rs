use nom::{IResult, Parser, multi::count};

use crate::parser::{
    DWORD, LONG, WORD,
    chunk::{AsepriteChunkParser, NoCtx},
    parse_dword, parse_long,
    primitives::{Point, Rect, Size, parse_point, parse_string},
};

#[derive(Debug, PartialEq)]
pub struct SliceKey {
    pub frame: DWORD,
    pub x: LONG,
    pub y: LONG,
    pub width: DWORD,
    pub height: DWORD,
    pub ninepatch: Option<Rect>,
    pub pivot: Option<Point>,
}

#[derive(Debug, PartialEq)]
pub struct SliceChunk {
    pub flags: DWORD,
    pub name: String,
    pub keys: Vec<SliceKey>,
}

fn parse_slice_key(
    has_ninepatch: bool,
    has_pivot: bool,
) -> impl FnMut(&[u8]) -> IResult<&[u8], SliceKey> {
    move |input: &[u8]| {
        let (input, frame) = parse_dword(input)?;
        let (input, x) = parse_long(input)?;
        let (input, y) = parse_long(input)?;
        let (input, width) = parse_dword(input)?;
        let (input, height) = parse_dword(input)?;
        let (input, ninepatch) = if has_ninepatch {
            let (i, cx) = parse_long(input)?;
            let (i, cy) = parse_long(i)?;
            let (i, cw) = parse_dword(i)?;
            let (i, ch) = parse_dword(i)?;
            (
                i,
                Some(Rect {
                    origin: Point { x: cx, y: cy },
                    size: Size {
                        w: cw as LONG,
                        h: ch as LONG,
                    },
                }),
            )
        } else {
            (input, None)
        };
        let (input, pivot) = if has_pivot {
            let (i, p) = parse_point(input)?;
            (i, Some(p))
        } else {
            (input, None)
        };
        Ok((
            input,
            SliceKey {
                frame,
                x,
                y,
                width,
                height,
                ninepatch,
                pivot,
            },
        ))
    }
}

impl<'a> AsepriteChunkParser<'a> for SliceChunk {
    const CHUNK_TYPE: WORD = 0x2022;
    type Need = NoCtx;

    fn parse_data(input: &'a [u8], _: ()) -> IResult<&'a [u8], Self> {
        let (input, num_keys) = parse_dword(input)?;
        let (input, flags) = parse_dword(input)?;
        let (input, _reserved) = parse_dword(input)?;
        let (input, name) = parse_string(input)?;
        let (input, keys) = count(
            parse_slice_key(flags & 1 != 0, flags & 2 != 0),
            num_keys as usize,
        )
        .parse(input)?;
        Ok((input, SliceChunk { flags, name, keys }))
    }
}
