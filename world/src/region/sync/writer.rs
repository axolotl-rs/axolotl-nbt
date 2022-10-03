use crate::region::{RegionHeader, RegionWriter};
use byteorder::WriteBytesExt;
use std::fmt::Debug;
use std::io::{Seek, Write};

impl<Writer: Write + WriteBytesExt + Seek + Debug> RegionWriter<Writer> {
    pub fn write_region(&mut self, header: RegionHeader) -> std::io::Result<()> {
        if header.locations.len() != 1024 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid region header length",
            ));
        }
        if header.timestamps.len() != 1024 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid region header length",
            ));
        }
        for location in header.locations.into_iter() {
            self.src.write_u32::<byteorder::BigEndian>(location.0)?;
            self.src.write_u8(location.1)?;
        }
        for timestamp in header.timestamps.into_iter() {
            self.src.write_u32::<byteorder::BigEndian>(timestamp)?;
        }
        Ok(())
    }
    pub fn write_chunk_header(&mut self, length: u32, compression_type: u8) -> std::io::Result<()> {
        self.src.write_u32::<byteorder::BigEndian>(length)?;
        self.src.write_u8(compression_type)?;
        Ok(())
    }
}