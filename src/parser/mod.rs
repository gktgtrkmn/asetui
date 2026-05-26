pub mod chunk;
pub mod frame;
pub mod header;
pub mod primitives;

pub use chunk::Chunk;
pub use frame::{AsepriteFrameHeader, Frame};
pub use header::{AsepriteHeader, parse_aseprite_header};
pub use primitives::{
    BYTE, DWORD, LONG, SHORT, WORD, parse_byte, parse_dword, parse_long, parse_short, parse_word,
    skip_bytes,
};
