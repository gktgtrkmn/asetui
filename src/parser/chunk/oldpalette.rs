use nom::{IResult, Parser, multi::count};

use crate::parser::{BYTE, WORD, chunk::AsepriteChunkParser, parse_byte, parse_word};

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

fn parse_old_palette_color(input: &[u8]) -> IResult<&[u8], OldPaletteColor> {
    let (input, r) = parse_byte(input)?;
    let (input, g) = parse_byte(input)?;
    let (input, b) = parse_byte(input)?;
    Ok((input, OldPaletteColor { r, g, b }))
}

fn parse_old_palette_packet(input: &[u8]) -> IResult<&[u8], OldPalettePacket> {
    let (input, skip) = parse_byte(input)?;
    let (input, count_byte) = parse_byte(input)?;
    let actual = if count_byte == 0 {
        256
    } else {
        count_byte as usize
    };
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
