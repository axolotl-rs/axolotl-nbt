use crate::region::reader::RegionReader;
use crate::region::{ChunkHeader, CompressionType, RegionHeader, RegionLocation};
use async_compression::tokio::bufread::{GzipDecoder, ZlibDecoder};
use axolotl_nbt::value::Value;
use axolotl_nbt::NBTReader;
use byteorder::{BigEndian, ByteOrder};
use std::fmt::Debug;
use std::io::{Error, SeekFrom};
use tokio::io::{AsyncReadExt, AsyncSeekExt, BufReader};

impl<Read: AsyncReadExt + Unpin + Send + Debug> RegionReader<Read> {
    pub async fn read_region_header_async(&mut self) -> Result<RegionHeader, Error> {
        let mut locations = Vec::with_capacity(1024);
        let mut timestamps = Vec::with_capacity(1024);
        self.read_chunk_header_to_location_async(&mut locations, &mut timestamps)
            .await?;
        Ok(RegionHeader {
            locations,
            timestamps,
        })
    }
    pub async fn read_chunk_header_to_location_async(
        &mut self,
        locations: &mut Vec<RegionLocation>,
        timestamps: &mut Vec<u32>,
    ) -> Result<(), Error> {
        let mut offset: [u8; 3] = [0, 0, 0];
        for _ in 0..1024 {
            self.src.read_exact(&mut offset).await?;
            let size = self.src.read_u8().await?;
            locations.push(RegionLocation(BigEndian::read_u24(&offset), size));
        }
        for _ in 0..1024 {
            timestamps.push(self.src.read_u32().await?);
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use crate::region::reader::RegionReader;
    use crate::region::{ChunkHeader, RegionLocation};
    use axolotl_nbt::value::{NameLessValue, Value};
    use std::io::Error;
    use std::mem::size_of;
    use std::path::Path;

    #[tokio::test]
    pub async fn test() {
        let path =
            Path::new(r"C:\Users\wherk\Desktop\make_my_server\purpur\world\region\r.0.0.mca");
        let reader = tokio::fs::File::open(path).await.unwrap();
        let mut reader = RegionReader::new(reader);
        let header = reader.read_region_header_async().await.unwrap();
        //println!("{:#?}", header);
        assert_eq!(header.locations.len(), 1024);
        assert_eq!(header.timestamps.len(), 1024);
        for x in header.locations {
            println!("Chunk Location: {:?}", x);
            match reader.read_chunk_to_value_async(x).await {
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

impl<Read: AsyncReadExt + AsyncSeekExt + Unpin + Send + Debug> RegionReader<Read> {
    pub async fn read_chunk_header_async(
        &mut self,
        location: RegionLocation,
    ) -> Result<ChunkHeader, Error> {
        if location.0 == 0 {
            return Ok(ChunkHeader {
                length: 0,
                compression_type: CompressionType::Uncompressed,
            });
        }
        let calc_offset = location.calc_offset();
        self.src.seek(SeekFrom::Start(calc_offset as u64)).await?;

        let length = self.src.read_u32().await?;
        let compression_type = self.src.read_u8().await?;
        Ok(ChunkHeader {
            length,
            compression_type: compression_type.into(),
        })
    }
    pub async fn read_chunk_to_bytes_async(
        &mut self,
        location: RegionLocation,
    ) -> Result<(ChunkHeader, Vec<u8>), Error> {
        let result = self.read_chunk_header_async(location).await?;
        if result.length == 0 {
            return Ok((result, Vec::new()));
        }
        let mut data = Vec::with_capacity(result.length as usize);

        AsyncReadExt::take(&mut self.src, result.length as u64)
            .read_to_end(&mut data)
            .await?;
        Ok((result, data))
    }
    pub async fn read_chunk_to_value_async(
        &mut self,
        location: RegionLocation,
    ) -> Result<(ChunkHeader, Value), Error> {
        let result = self.read_chunk_header_async(location).await?;
        if result.length == 0 {
            return Ok((
                result,
                Value::Compound {
                    name: String::new(),
                    value: Vec::new(),
                },
            ));
        }
        let take = AsyncReadExt::take(&mut self.src, (result.length - 1) as u64);
        let value = match &result.compression_type {
            CompressionType::Gzip => {
                NBTReader::new(BufReader::new(GzipDecoder::new(BufReader::new(take))))
                    .async_read_value()
                    .await?
            }
            CompressionType::Zlib => {
                NBTReader::new(BufReader::new(ZlibDecoder::new(BufReader::new(take))))
                    .async_read_value()
                    .await?
            }
            CompressionType::Uncompressed => {
                NBTReader::new(BufReader::new(take))
                    .async_read_value()
                    .await?
            }
            CompressionType::Custom(value) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    format!("Unknown compression type {}", value),
                ));
            }
        };
        Ok((result, value))
    }
}
