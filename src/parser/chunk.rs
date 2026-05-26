use nom::IResult;
use nom::Parser;
use nom::bytes::complete::take;
use nom::error::ErrorKind;
use nom::multi::count;

use crate::parser::primitives::{
    BYTE, DWORD, FIXED, LONG, LONG64, QWORD, SHORT, UUID, WORD, parse_byte, parse_double,
    parse_dword, parse_float, parse_long, parse_long64, parse_qword, parse_short, parse_string,
    parse_uuid, parse_word, skip_bytes,
};

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: LONG,
    pub y: LONG,
}

#[derive(Debug, PartialEq)]
pub struct Size {
    pub w: LONG,
    pub h: LONG,
}

#[derive(Debug, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

#[derive(Debug, PartialEq)]
pub struct OldPaletteColor {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
}

#[derive(Debug, PartialEq)]
pub struct OldPalettePacket {
    pub skip: BYTE,
    pub count: BYTE,
    pub colors: Vec<OldPaletteColor>,
}

#[derive(Debug, PartialEq)]
pub struct OldPalette04Chunk {
    pub packets: Vec<OldPalettePacket>,
}

#[derive(Debug, PartialEq)]
pub struct OldPalette11Chunk {
    pub packets: Vec<OldPalettePacket>,
}

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
    pub uuid: Option<UUID>,
}

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

#[derive(Debug, PartialEq)]
pub struct ColorProfileChunk<'a> {
    pub kind: WORD,
    pub flags: WORD,
    pub gamma: FIXED,
    pub icc: Option<&'a [u8]>,
}

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

#[derive(Debug, PartialEq)]
pub struct MaskChunk<'a> {
    pub x: SHORT,
    pub y: SHORT,
    pub width: WORD,
    pub height: WORD,
    pub name: String,
    pub bitmap: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct PathChunk;

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub from_frame: WORD,
    pub to_frame: WORD,
    pub direction: BYTE,
    pub repeat: WORD,
    pub color: [BYTE; 3],
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct TagsChunk {
    pub tags: Vec<Tag>,
}

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

#[derive(Debug, PartialEq)]
pub enum PropValue {
    Bool(bool),
    I8(i8),
    U8(BYTE),
    I16(SHORT),
    U16(WORD),
    I32(LONG),
    U32(DWORD),
    I64(LONG64),
    U64(QWORD),
    Fixed(FIXED),
    Float(f32),
    Double(f64),
    String(String),
    Point(Point),
    Size(Size),
    Rect(Rect),
    Vector(Vec<PropValue>),
    Map(Vec<(String, PropValue)>),
    Uuid(UUID),
}

#[derive(Debug, PartialEq)]
pub struct PropertiesMap {
    pub key: DWORD,
    pub props: Vec<(String, PropValue)>,
}

#[derive(Debug, PartialEq)]
pub struct UserDataChunk {
    pub text: Option<String>,
    pub color: Option<(BYTE, BYTE, BYTE, BYTE)>,
    pub maps: Vec<PropertiesMap>,
}

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

#[derive(Debug, PartialEq)]
pub struct AsepriteChunk<'a> {
    pub chunk_size: DWORD,
    pub chunk_type: WORD,
    pub raw: &'a [u8]
}

pub fn parse_aseprite_chunk(input: &[u8]) -> IResult<&[u8], AsepriteChunk<'_>> {
    let (input, chunk_size) = parse_dword(input)?;
    let (input, chunk_type) = parse_word(input)?;
    if chunk_size < 6 {
        return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)));
    }
    let body_len: usize = (chunk_size - 6) as usize;
    let (input, raw) = take(body_len)(input)?;
    Ok((input, AsepriteChunk { chunk_size, chunk_type, raw }))
}

impl<'a> AsepriteChunk<'a> {
    pub fn parse_as<T: AsepriteChunkParser<'a>>(&self) -> IResult<&'a [u8], T> {
        if self.chunk_type != T::CHUNK_TYPE {
            return Err(nom::Err::Error(nom::error::Error::new(
                self.raw,
                ErrorKind::Verify,
            )));
        }
        let (rest, value) = T::parse_data(self.raw)?;
        if !rest.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                rest,
                ErrorKind::Eof,
            )));
        }
        Ok((rest, value))
    }
}

pub trait AsepriteChunkParser<'a>: Sized {
    const CHUNK_TYPE: WORD;
    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self>;
}

fn parse_old_palette_color(input: &[u8]) -> IResult<&[u8], OldPaletteColor> {
    let (input, r) = parse_byte(input)?;
    let (input, g) = parse_byte(input)?;
    let (input, b) = parse_byte(input)?;
    Ok((input, OldPaletteColor { r, g, b }))
}

fn parse_old_palette_packet(input: &[u8]) -> IResult<&[u8], OldPalettePacket> {
    let (input, skip) = parse_byte(input)?;
    let (input, count_byte) = parse_byte(input)?;
    let actual = if count_byte == 0 { 256 } else { count_byte as usize };
    let (input, colors) = count(parse_old_palette_color, actual).parse(input)?;
    Ok((
        input,
        OldPalettePacket {
            skip,
            count: count_byte,
            colors,
        },
    ))
}

fn parse_old_palette_packets(input: &[u8]) -> IResult<&[u8], Vec<OldPalettePacket>> {
    let (input, n) = parse_word(input)?;
    count(parse_old_palette_packet, n as usize).parse(input)
}

impl<'a> AsepriteChunkParser<'a> for OldPalette04Chunk {
    const CHUNK_TYPE: WORD = 0x0004;

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (input, packets) = parse_old_palette_packets(input)?;
        Ok((input, OldPalette04Chunk { packets }))
    }
}

impl<'a> AsepriteChunkParser<'a> for OldPalette11Chunk {
    const CHUNK_TYPE: WORD = 0x0011;

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (input, packets) = parse_old_palette_packets(input)?;
        Ok((input, OldPalette11Chunk { packets }))
    }
}

impl<'a> AsepriteChunkParser<'a> for LayerChunk {
    const CHUNK_TYPE: WORD = 0x2004;

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
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
        let (input, uuid) = if input.is_empty() {
            (input, None)
        } else {
            let (i, id) = parse_uuid(input)?;
            (i, Some(id))
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

impl<'a> AsepriteChunkParser<'a> for CelChunk<'a> {
    const CHUNK_TYPE: WORD = 0x2005;

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
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

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
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

impl<'a> AsepriteChunkParser<'a> for ColorProfileChunk<'a> {
    const CHUNK_TYPE: WORD = 0x2007;

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (input, kind) = parse_word(input)?;
        let (input, flags) = parse_word(input)?;
        let (input, gamma) = parse_long(input)?;
        let (input, _) = skip_bytes(input, 8)?;
        let (input, icc) = if kind == 2 {
            let (i, len) = parse_dword(input)?;
            let (i, data) = take(len as usize)(i)?;
            (i, Some(data))
        } else {
            (input, None)
        };
        Ok((
            input,
            ColorProfileChunk {
                kind,
                flags,
                gamma,
                icc,
            },
        ))
    }
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
        let (input, entries) = count(parse_external_file_entry, num_entries as usize).parse(input)?;
        Ok((input, ExternalFilesChunk { entries }))
    }
}

impl<'a> AsepriteChunkParser<'a> for MaskChunk<'a> {
    const CHUNK_TYPE: WORD = 0x2016;

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (input, x) = parse_short(input)?;
        let (input, y) = parse_short(input)?;
        let (input, width) = parse_word(input)?;
        let (input, height) = parse_word(input)?;
        let (input, _) = skip_bytes(input, 8)?;
        let (input, name) = parse_string(input)?;
        let bitmap_size = (height as usize) * (((width as usize) + 7) / 8);
        let (input, bitmap) = take(bitmap_size)(input)?;
        Ok((
            input,
            MaskChunk {
                x,
                y,
                width,
                height,
                name,
                bitmap,
            },
        ))
    }
}

impl<'a> AsepriteChunkParser<'a> for PathChunk {
    const CHUNK_TYPE: WORD = 0x2017;

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        Ok((input, PathChunk))
    }
}

fn parse_tag(input: &[u8]) -> IResult<&[u8], Tag> {
    let (input, from_frame) = parse_word(input)?;
    let (input, to_frame) = parse_word(input)?;
    let (input, direction) = parse_byte(input)?;
    let (input, repeat) = parse_word(input)?;
    let (input, _) = skip_bytes(input, 6)?;
    let (input, color_slice) = take(3usize)(input)?;
    let color: [u8; 3] = color_slice.try_into().unwrap();
    let (input, _) = skip_bytes(input, 1)?;
    let (input, name) = parse_string(input)?;
    Ok((
        input,
        Tag {
            from_frame,
            to_frame,
            direction,
            repeat,
            color,
            name,
        },
    ))
}

impl<'a> AsepriteChunkParser<'a> for TagsChunk {
    const CHUNK_TYPE: WORD = 0x2018;

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (input, num_tags) = parse_word(input)?;
        let (input, _) = skip_bytes(input, 8)?;
        let (input, tags) = count(parse_tag, num_tags as usize).parse(input)?;
        Ok((input, TagsChunk { tags }))
    }
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

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
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

fn parse_prop_value(input: &[u8], prop_type: WORD) -> IResult<&[u8], PropValue> {
    match prop_type {
        0x0001 => parse_byte(input).map(|(i, b)| (i, PropValue::Bool(b != 0))),
        0x0002 => parse_byte(input).map(|(i, b)| (i, PropValue::I8(b as i8))),
        0x0003 => parse_byte(input).map(|(i, b)| (i, PropValue::U8(b))),
        0x0004 => parse_short(input).map(|(i, v)| (i, PropValue::I16(v))),
        0x0005 => parse_word(input).map(|(i, v)| (i, PropValue::U16(v))),
        0x0006 => parse_long(input).map(|(i, v)| (i, PropValue::I32(v))),
        0x0007 => parse_dword(input).map(|(i, v)| (i, PropValue::U32(v))),
        0x0008 => parse_long64(input).map(|(i, v)| (i, PropValue::I64(v))),
        0x0009 => parse_qword(input).map(|(i, v)| (i, PropValue::U64(v))),
        0x000A => parse_long(input).map(|(i, v)| (i, PropValue::Fixed(v))),
        0x000B => parse_float(input).map(|(i, v)| (i, PropValue::Float(v))),
        0x000C => parse_double(input).map(|(i, v)| (i, PropValue::Double(v))),
        0x000D => parse_string(input).map(|(i, v)| (i, PropValue::String(v))),
        0x000E => {
            let (input, x) = parse_long(input)?;
            let (input, y) = parse_long(input)?;
            Ok((input, PropValue::Point(Point { x, y })))
        }
        0x000F => {
            let (input, w) = parse_long(input)?;
            let (input, h) = parse_long(input)?;
            Ok((input, PropValue::Size(Size { w, h })))
        }
        0x0010 => {
            let (input, x) = parse_long(input)?;
            let (input, y) = parse_long(input)?;
            let (input, w) = parse_long(input)?;
            let (input, h) = parse_long(input)?;
            Ok((
                input,
                PropValue::Rect(Rect {
                    origin: Point { x, y },
                    size: Size { w, h },
                }),
            ))
        }
        0x0011 => {
            let (input, num_elements) = parse_dword(input)?;
            let (input, element_type) = parse_word(input)?;
            let (input, elements) = count(
                move |i| {
                    if element_type == 0 {
                        let (i, t) = parse_word(i)?;
                        parse_prop_value(i, t)
                    } else {
                        parse_prop_value(i, element_type)
                    }
                },
                num_elements as usize,
            )
            .parse(input)?;
            Ok((input, PropValue::Vector(elements)))
        }
        0x0012 => {
            let (input, num_props) = parse_dword(input)?;
            let (input, props) = parse_props(input, num_props)?;
            Ok((input, PropValue::Map(props)))
        }
        0x0013 => parse_uuid(input).map(|(i, id)| (i, PropValue::Uuid(id))),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            input,
            ErrorKind::Verify,
        ))),
    }
}

fn parse_prop(input: &[u8]) -> IResult<&[u8], (String, PropValue)> {
    let (input, name) = parse_string(input)?;
    let (input, prop_type) = parse_word(input)?;
    let (input, value) = parse_prop_value(input, prop_type)?;
    Ok((input, (name, value)))
}

fn parse_props(input: &[u8], num_props: DWORD) -> IResult<&[u8], Vec<(String, PropValue)>> {
    count(parse_prop, num_props as usize).parse(input)
}

fn parse_property_map(input: &[u8]) -> IResult<&[u8], PropertiesMap> {
    let (input, key) = parse_dword(input)?;
    let (input, num_props) = parse_dword(input)?;
    let (input, props) = parse_props(input, num_props)?;
    Ok((input, PropertiesMap { key, props }))
}

impl<'a> AsepriteChunkParser<'a> for UserDataChunk {
    const CHUNK_TYPE: WORD = 0x2020;

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (input, flags) = parse_dword(input)?;
        let (input, text) = if flags & 1 != 0 {
            let (i, t) = parse_string(input)?;
            (i, Some(t))
        } else {
            (input, None)
        };
        let (input, color) = if flags & 2 != 0 {
            let (i, r) = parse_byte(input)?;
            let (i, g) = parse_byte(i)?;
            let (i, b) = parse_byte(i)?;
            let (i, a) = parse_byte(i)?;
            (i, Some((r, g, b, a)))
        } else {
            (input, None)
        };
        let (input, maps) = if flags & 4 != 0 {
            let (i, _size) = parse_dword(input)?;
            let (i, num_maps) = parse_dword(i)?;
            count(parse_property_map, num_maps as usize).parse(i)?
        } else {
            (input, Vec::new())
        };
        Ok((input, UserDataChunk { text, color, maps }))
    }
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
            let (i, px) = parse_long(input)?;
            let (i, py) = parse_long(i)?;
            (i, Some(Point { x: px, y: py }))
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

    fn parse_data(input: &'a [u8]) -> IResult<&'a [u8], Self> {
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
