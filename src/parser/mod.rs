pub mod chunk;
pub mod frame;
pub mod header;
pub mod primitives;

pub use chunk::{
    AsepriteChunk, AsepriteChunkParser, CtxNeed, NoCtx, ParseContext, WithCtx, parse_aseprite_chunk,
};
pub use frame::{AsepriteFrameHeader, Frame, parse_aseprite_frame, parse_aseprite_frame_header};
pub use header::{AsepriteHeader, parse_aseprite_header};
pub use primitives::{
    BYTE, DWORD, LONG, SHORT, WORD, parse_byte, parse_dword, parse_long, parse_magic_word,
    parse_short, parse_word, skip_bytes,
};
