use crate::{Binary, NBTReader, Tag};
use std::fmt::Debug;
use std::io::Error;
use tokio::io::{AsyncBufReadExt, AsyncReadExt};

#[cfg(test)]
pub mod test {
    use crate::{Binary, NBTReader};
    use async_compression::tokio::bufread::{GzipDecoder};
    use std::marker::PhantomData;
    use std::path::Path;
    use tokio::io::BufReader;

    #[tokio::test]
    pub async fn test() {
        let path = Path::new("C:\\Users\\wherk\\AppData\\Roaming\\.minecraft\\saves\\New World\\playerdata\\d087006b-d72c-4cdf-924d-6f903704d05c.dat");
        let  reader = tokio::fs::File::open(path).await.unwrap();
        let  x = GzipDecoder::new(BufReader::new(reader));
        let mut reader = NBTReader {
            src: BufReader::new(x),
            phantom: PhantomData::<Binary>::default(),
        };
        let value = reader.read_value().await.unwrap();
        println!("{:#?}", value);
    }
}
impl<Read: AsyncBufReadExt + Unpin + Send + Debug> NBTReader<Binary, Read> {
    pub async fn peak_tag_id(&mut self) -> Result<Tag, Error> {
        let result = self.src.fill_buf().await?[0];
        Tag::from_u8(result).ok_or_else(|| Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid Tag Id {}", result),
        ))
    }
    /// You will need to convert this to a String.
    pub async fn read_str_as_bytes(&mut self, size: u16) -> Result<Vec<u8>, Error> {
        let result = self.src.fill_buf().await?[..size as usize].to_vec();
        self.src.consume(size as usize);
        Ok(result)
    }
}
impl<Read: AsyncReadExt + Unpin + Send + Debug> NBTReader<Binary, Read> {
    pub async fn read_tag_id(&mut self) -> Result<Tag, Error> {
        let result = self.src.read_u8().await?;
        Tag::from_u8(result).ok_or_else(|| Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid Tag Id {}", result),
        ))
    }
    /// Will be zero if the tag is an end tag.
    pub async fn read_tag_id_with_id_len(&mut self) -> Result<(Tag, u16), Error> {
        let result = self.src.read_u8().await?;
        let tag = Tag::from_u8(result).ok_or_else(|| Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid Tag Id {}", result),
        ))?;
        if tag == Tag::End {
            return Ok((tag, 0));
        }
        let id_len = self.read_string_len().await?;
        Ok((tag, id_len))
    }
    pub async fn read_string_len(&mut self) -> Result<u16, Error> {
        let result = self.src.read_u16().await?;
        Ok(result)
    }


    pub async fn read_byte(&mut self) -> Result<i8, Error> {
        let result = self.src.read_i8().await?;
        Ok(result)
    }
    pub async fn read_short(&mut self) -> Result<i16, Error> {
        let result = self.src.read_i16().await?;
        Ok(result)
    }
    pub async fn read_int(&mut self) -> Result<i32, Error> {
        let result = self.src.read_i32().await?;
        Ok(result)
    }
    pub async fn read_long(&mut self) -> Result<i64, Error> {
        let result = self.src.read_i64().await?;
        Ok(result)
    }
    pub async fn read_float(&mut self) -> Result<f32, Error> {
        let result = self.src.read_f32().await?;
        Ok(result)
    }
    pub async fn read_double(&mut self) -> Result<f64, Error> {
        let result = self.src.read_f64().await?;
        Ok(result)
    }
    pub async fn read_byte_array(&mut self, size: i32) -> Result<Vec<i8>, Error> {
        let mut result = Vec::with_capacity(size as usize);
        for _ in 0..size {
            result.push(self.src.read_i8().await?);
        }

        Ok(result)
    }
    pub async fn read_list_type_and_size(&mut self) -> Result<(Tag, u32), Error> {
        let tag = self.read_tag_id().await?;
        let size = self.read_int().await?;
        Ok((tag, size as u32))
    }
}
