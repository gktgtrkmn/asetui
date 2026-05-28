use nom::{IResult, Parser, error::ErrorKind, multi::count};
use uuid::Uuid;

use crate::parser::{
    BYTE, DWORD, LONG, SHORT, WORD,
    chunk::{AsepriteChunkParser, NoCtx},
    parse_byte, parse_dword, parse_long, parse_short, parse_word,
    primitives::{
        FIXED, LONG64, Point, QWORD, Rect, Size, parse_double, parse_float, parse_long64,
        parse_point, parse_qword, parse_rect, parse_size, parse_string, parse_uuid,
    },
};

#[derive(Debug, PartialEq)]
pub struct UserDataChunk {
    pub text: Option<String>,
    pub color: Option<(BYTE, BYTE, BYTE, BYTE)>,
    pub maps: Vec<PropertiesMap>,
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
    Uuid(Uuid),
}

#[derive(Debug, PartialEq)]
pub struct PropertiesMap {
    pub key: DWORD,
    pub props: Vec<(String, PropValue)>,
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
        0x000E => parse_point(input).map(|(i, p)| (i, PropValue::Point(p))),
        0x000F => parse_size(input).map(|(i, s)| (i, PropValue::Size(s))),
        0x0010 => parse_rect(input).map(|(i, r)| (i, PropValue::Rect(r))),
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
    type Need = NoCtx;

    fn parse_data(input: &'a [u8], _: ()) -> IResult<&'a [u8], Self> {
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
