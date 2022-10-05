use crate::chunk::compact_array::{CompactArray, CompactArrayIndex};

pub mod compact_array;

pub const BITS_PER_BLOCK: u8 = 15;
pub const MINIMUM_BITS_PER_BLOCK: u8 = 4;

pub const CHUNK_HEIGHT: u16 = 384;
pub const CHUNK_WIDTH: u8 = 16;

pub const SECTION_HEIGHT: u8 = 16;
pub const SECTION_WIDTH: u8 = 16;
pub const SECTION_LENGTH: u8 = 16;

pub const BLOCKS_PER_SECTION: u32 =
    (SECTION_HEIGHT as u32 * SECTION_WIDTH as u32 * SECTION_LENGTH as u32);
#[derive(Debug, Clone)]
pub struct Chunk {
    pub x_pos: i32,
    pub y_pos: i32,
    pub z_pos: i32,
    pub last_update: i64,
}
#[derive(Debug, Clone)]
pub struct ChunkSection {
    pub y_pos: i32,
}
#[derive(Debug, Clone)]
pub struct BlockStates {
    pub data: CompactArray,
    pub pallet: Palette,
}
#[derive(Debug, Clone)]
pub struct Palette {}
