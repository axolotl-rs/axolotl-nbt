pub mod reader;
pub mod tokio_impl;

#[derive(Debug, Clone)]
pub struct RegionHeader {
    /// The regions offsets and sizes
    pub locations: Vec<(u32, u8)>,
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

pub trait ChunkSection {
    fn into(self) -> (u32, u8);
}


impl ChunkSection for (u32, u8) {
    fn into(self) -> (u32, u8) {
        self
    }
}