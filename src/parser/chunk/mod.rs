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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ParseContext {
    pub layers_have_uuid: bool,
}

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
    pub fn parse_as<T>(&self) -> IResult<&'a [u8], T>
    where
        T: AsepriteChunkParser<'a, Need = NoCtx>,
    {
        self.dispatch::<T>(())
    }

    pub fn parse_as_ctx<T>(&self, ctx: ParseContext) -> IResult<&'a [u8], T>
    where
        T: AsepriteChunkParser<'a, Need = WithCtx>,
    {
        self.dispatch::<T>(ctx)
    }

    fn dispatch<T: AsepriteChunkParser<'a>>(
        &self,
        arg: <T::Need as CtxNeed>::Arg,
    ) -> IResult<&'a [u8], T> {
        if self.chunk_type != T::CHUNK_TYPE {
            return Err(nom::Err::Error(nom::error::Error::new(
                self.raw,
                ErrorKind::Verify,
            )));
        }
        let (rest, value) = T::parse_data(self.raw, arg)?;
        if !rest.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                rest,
                ErrorKind::Eof,
            )));
        }
        Ok((rest, value))
    }
}

mod sealed {
    pub trait Sealed {}
}

pub trait CtxNeed: sealed::Sealed {
    type Arg;
}

pub struct NoCtx;
pub struct WithCtx;

impl sealed::Sealed for NoCtx {}
impl sealed::Sealed for WithCtx {}
impl CtxNeed for NoCtx {
    type Arg = ();
}
impl CtxNeed for WithCtx {
    type Arg = ParseContext;
}

pub trait AsepriteChunkParser<'a>: Sized {
    const CHUNK_TYPE: WORD;
    type Need: CtxNeed;
    fn parse_data(
        input: &'a [u8],
        arg: <Self::Need as CtxNeed>::Arg,
    ) -> IResult<&'a [u8], Self>;
}
