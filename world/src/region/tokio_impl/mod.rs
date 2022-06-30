use crate::region::reader::RegionReader;
use crate::region::{ChunkHeader, ChunkSection, CompressionType, RegionHeader};
use async_compression::tokio::bufread::{GzipDecoder, ZlibDecoder};
use axolotl_nbt::value::Value;
use axolotl_nbt::NBTReader;
use byteorder::{BigEndian, ByteOrder};
use std::fmt::Debug;
use std::io::{Error, SeekFrom};
use tokio::io::{AsyncReadExt, AsyncSeekExt, BufReader};

impl<Read: AsyncReadExt + Unpin + Send + Debug> RegionReader<Read> {
    pub async fn read_region_header(&mut self) -> Result<RegionHeader, Error> {
        let mut locations = Vec::with_capacity(1024);
        let mut timestamps = Vec::with_capacity(1024);
        self.read_chunk_header_to_location(&mut locations, &mut timestamps)
            .await?;
        Ok(RegionHeader {
            locations,
            timestamps,
        })
    }
    pub async fn read_chunk_header_to_location(
        &mut self,
        locations: &mut Vec<(u32, u8)>,
        timestamps: &mut Vec<u32>,
    ) -> Result<(), Error> {
        let mut offset: [u8; 3] = [0, 0, 0];
        for _ in 0..1024 {
            self.src.read_exact(&mut offset).await?;
            let size = self.src.read_u8().await?;
            locations.push((BigEndian::read_u24(&offset), size));
            timestamps.push(self.src.read_u32().await?);
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use crate::region::reader::RegionReader;
    use std::path::Path;

    #[tokio::test]
    pub async fn test() {
        let path = Path::new(
            "C:\\Users\\wherk\\AppData\\Roaming\\.minecraft\\saves\\New World\\region\\r.0.0.mca",
        );
        let reader = tokio::fs::File::open(path).await.unwrap();
        let mut reader = RegionReader::new(reader);
        let header = reader.read_region_header().await.unwrap();
        //println!("{:#?}", header);

        let location = *header.locations.get(2).unwrap();
        let (chunk, data) = reader.read_chunk_to_value(location).await.unwrap();
        println!("{:#?}", chunk);
        println!("{:?}", data);
    }
}

impl<Read: AsyncReadExt + AsyncSeekExt + Unpin + Send + Debug> RegionReader<Read> {
    pub async fn read_chunk_header<Location: ChunkSection>(
        &mut self,
        location: Location,
    ) -> Result<ChunkHeader, Error> {
        let (offset, _) = location.into();
        let calc_offset = 8192 + (offset * 4096);
        self.src.seek(SeekFrom::Start(calc_offset as u64)).await?;

        let length = self.src.read_u32().await?;
        let compression_type = self.src.read_u8().await?;
        Ok(ChunkHeader {
            length,
            compression_type: compression_type.into(),
        })
    }
    pub async fn read_chunk_to_bytes<Location: ChunkSection>(
        &mut self,
        location: Location,
    ) -> Result<(ChunkHeader, Vec<u8>), Error> {
        let result = self.read_chunk_header(location).await?;
        let mut data = Vec::with_capacity(result.length as usize);

        AsyncReadExt::take(&mut self.src, result.length as u64)
            .read_to_end(&mut data)
            .await?;
        Ok((result, data))
    }
    pub async fn read_chunk_to_value<Location: ChunkSection>(
        &mut self,
        location: Location,
    ) -> Result<(ChunkHeader, Value), Error> {
        let result = self.read_chunk_header(location).await?;

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
