pub mod writer;

use crate::region::reader::RegionReader;
use crate::region::{ChunkHeader, CompressionType, RegionHeader, RegionLocation};
use axolotl_nbt::value::Value;
use byteorder::{BigEndian, ReadBytesExt};
use flate2::read::{GzDecoder, ZlibDecoder};
use std::fmt::Debug;
use std::io::{BufReader, Error, Read, Seek, SeekFrom};
use axolotl_nbt::NBTDataType;

impl<Reader: Read + ReadBytesExt + Debug> RegionReader<Reader> {
    pub fn read_region_header(&mut self) -> Result<RegionHeader, Error> {
        let mut locations = Vec::with_capacity(1024);
        let mut timestamps = Vec::with_capacity(1024);
        self.read_chunk_header_to_location(&mut locations, &mut timestamps)?;
        Ok(RegionHeader {
            locations,
            timestamps,
        })
    }
    pub fn read_chunk_header_to_location(
        &mut self,
        locations: &mut Vec<RegionLocation>,
        timestamps: &mut Vec<u32>,
    ) -> Result<(), Error> {
        for _ in 0..1024 {
            let offset = self.src.read_u24::<BigEndian>()?;
            let size = self.src.read_u8()?;
            locations.push(RegionLocation(offset, size));
        }
        for _ in 0..1024 {
            timestamps.push(self.src.read_u32::<BigEndian>()?);
        }
        Ok(())
    }
}

impl<Reader: Read + ReadBytesExt + Seek + Debug> RegionReader<Reader> {
    pub fn read_chunk_header(&mut self, location: RegionLocation) -> Result<ChunkHeader, Error> {
        if location.0 == 0 {
            return Ok(ChunkHeader {
                length: 0,
                compression_type: CompressionType::Gzip,
            });
        }
        let calc_offset = location.calc_offset();
        self.src.seek(SeekFrom::Start(calc_offset as u64))?;

        let length = self.src.read_u32::<BigEndian>()?;
        let compression_type = self.src.read_u8()?;
        Ok(ChunkHeader {
            length,
            compression_type: compression_type.into(),
        })
    }
    pub fn read_chunk_to_bytes(
        &mut self,
        location: RegionLocation,
    ) -> Result<(ChunkHeader, Vec<u8>), Error> {
        let result = self.read_chunk_header(location)?;
        if result.length == 0 {
            return Ok((result, Vec::new()));
        }
        let mut data = Vec::with_capacity(result.length as usize);

        (&mut self.src)
            .take(result.length as u64)
            .read_to_end(&mut data)?;
        Ok((result, data))
    }
    pub fn read_chunk_to_value(
        &mut self,
        location: RegionLocation,
    ) -> Result<(ChunkHeader, Value), Error> {
        let result = self.read_chunk_header(location)?;
        if result.length == 0 {
            return Ok((result, Value::End));
        }

        let mut take = (&mut self.src).take((result.length - 1) as u64);
        let value = match &result.compression_type {
            CompressionType::Gzip => {
                Value::read(&mut GzDecoder::new(take))?
            }
            CompressionType::Zlib => {
                Value::read(&mut ZlibDecoder::new(take))?
            }
            CompressionType::Uncompressed => Value::read(&mut take)?,
            CompressionType::Custom(_) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Custom compression not supported",
                ));
            }
        };
        Ok((result, value))
    }
}

pub mod test {
    use crate::region::reader::RegionReader;
    use crate::region::{ChunkHeader, RegionLocation};
    use axolotl_nbt::value::{NameLessValue, Value};
    use std::fs::File;
    use std::io::Error;
    use std::mem::size_of;
    use std::path::Path;

    #[test]
    pub fn test() {
        let path =
            Path::new(r"C:\Users\wherk\Desktop\make_my_server\purpur\world\region\r.0.0.mca");
        let reader = File::open(path).unwrap();
        let mut reader = RegionReader::new(reader);
        let header = reader.read_region_header().unwrap();
        //println!("{:#?}", header);
        assert_eq!(header.locations.len(), 1024);
        assert_eq!(header.timestamps.len(), 1024);
        for x in header.locations {
            println!("Chunk Location: {:?}", x);
            match reader.read_chunk_to_value(x) {
                Ok((header, v)) => {
                    println!("{:#?}", header);
                    let string = format!("{:#?}", v);
                    if string.contains("minecraft:warped_nylium") {
                        println!("{:#?}", v);
                        break;
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                    continue;
                }
            };
        }
    }
}
