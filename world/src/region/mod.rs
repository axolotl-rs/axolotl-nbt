use std::fmt::Debug;

pub mod reader;
pub mod sync;

#[derive(Debug)]
pub struct RegionWriter<Src: Debug> {
    pub(crate) src: Src,
}

impl<Src: Debug> RegionWriter<Src> {
    pub fn new(src: Src) -> Self {
        RegionWriter { src }
    }
    pub fn into_inner(self) -> Src {
        self.src
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Copy)]
pub struct RegionLocation(pub u32, pub u8);

impl RegionLocation {
    pub fn calc_offset(&self) -> u64 {
        (self.0 * 4096) as u64
    }
}

impl Into<(u32, u8)> for RegionLocation {
    fn into(self) -> (u32, u8) {
        (self.0, self.1)
    }
}

impl From<(u32, u8)> for RegionLocation {
    fn from((offset, sector_count): (u32, u8)) -> Self {
        RegionLocation(offset, sector_count)
    }
}

#[derive(Debug, Clone)]
pub struct RegionHeader {
    /// The regions offsets and sizes
    pub locations: Vec<RegionLocation>,
    /// The timestamps
    pub timestamps: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct ChunkHeader {
    pub length: u32,
    pub compression_type: CompressionType,
}

#[derive(Debug, Clone)]
pub enum CompressionType {
    Gzip,
    Zlib,
    Uncompressed,
    Custom(u8),
}

impl From<u8> for CompressionType {
    fn from(data: u8) -> Self {
        match data {
            3 => CompressionType::Uncompressed,
            1 => CompressionType::Gzip,
            2 => CompressionType::Zlib,
            _ => CompressionType::Custom(data),
        }
    }
}
