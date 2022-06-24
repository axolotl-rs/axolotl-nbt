use std::borrow::Cow;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt};
use crate::{Binary, NBTFormat, NBTReader, Tag};

impl<Read: AsyncBufReadExt + Unpin> NBTReader<Binary, Read> {
    pub async fn peak_tag_id(&mut self) -> Result<Tag, std::io::Error> {
        let result = self.src.fill_buf().await?[0];
        Tag::from_u8(result).ok_or(
            std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid tag id"))
    }
    pub async fn read_tag_id(&mut self) -> Result<Tag, std::io::Error> {
        let result = self.src.read_u8().await?;
        Tag::from_u8(result).ok_or(
            std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid tag id"))
    }
    /// Will be zero if the tag is an end tag.
    pub async fn read_tag_id_with_id_len(&mut self) -> Result<(Tag, u16), std::io::Error> {
        let result = self.src.read_u8().await?;
        let tag = Tag::from_u8(result).ok_or(
            std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid tag id"))?;
        if tag == Tag::End {
            return Ok((tag, 0));
        }
        let id_len = self.read_string_len().await?;
        Ok((tag, id_len))
    }
    pub async fn read_string_len(&mut self) -> Result<u16, std::io::Error> {
        let result = self.src.read_u16().await?;
        Ok(result)
    }
    /// You will need to convert this to a String.
    pub async fn read_str_as_bytes(&mut self, size: u16) -> Result<Vec<u8>, std::io::Error> {
        let mut result = Vec::with_capacity(size as usize);
        self.src.take(size as u64).read_to_end(&mut result).await?;
        Ok(result)
    }
    pub async fn read_str_lossy(&mut self, size: u16) -> Result<Cow<'_, String>, std::io::Error> {
        let result = self.read_str_as_bytes(size).await?;
        Ok(String::from_utf8_lossy(result.as_slice())?)
    }

    pub async fn read_byte(&mut self) -> Result<i8, std::io::Error> {
        let result = self.src.read_i8().await?;
        Ok(result)
    }
    pub async fn read_short(&mut self) -> Result<i16, std::io::Error> {
        let result = self.src.read_i16().await?;
        Ok(result)
    }
    pub async fn read_int(&mut self) -> Result<i32, std::io::Error> {
        let result = self.src.read_i32().await?;
        Ok(result)
    }
    pub async fn read_long(&mut self) -> Result<i64, std::io::Error> {
        let result = self.src.read_i64().await?;
        Ok(result)
    }
    pub async fn read_float(&mut self) -> Result<f32, std::io::Error> {
        let result = self.src.read_f32().await?;
        Ok(result)
    }
    pub async fn read_double(&mut self) -> Result<f64, std::io::Error> {
        let result = self.src.read_f64().await?;
        Ok(result)
    }
}