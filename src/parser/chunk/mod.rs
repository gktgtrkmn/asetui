pub mod cel;
pub mod colorprofile;
pub mod externalfile;
pub mod layer;
pub mod mask;
pub mod oldpalette;
pub mod palette;
pub mod path;
pub mod slice;
pub mod tag;
pub mod tileset;
pub mod userdata;

use nom::IResult;
use nom::bytes::complete::take;
use nom::error::ErrorKind;

use crate::parser::primitives::{DWORD, WORD, parse_dword, parse_word};

#[derive(Debug, PartialEq)]
pub struct AsepriteChunk<'a> {
    pub chunk_size: DWORD,
    pub chunk_type: WORD,
    pub raw: &'a [u8],
}

pub fn parse_aseprite_chunk(input: &[u8]) -> IResult<&[u8], AsepriteChunk<'_>> {
    let (input, chunk_size) = parse_dword(input)?;
    let (input, chunk_type) = parse_word(input)?;
    if chunk_size < 6 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Verify,
        )));
    }
    let body_len: usize = (chunk_size - 6) as usize;
    let (input, raw) = take(body_len)(input)?;
    Ok((
        input,
        AsepriteChunk {
            chunk_size,
            chunk_type,
            raw,
        },
    ))
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
