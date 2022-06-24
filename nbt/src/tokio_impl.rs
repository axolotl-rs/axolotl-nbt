use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Error;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt};
use crate::{Binary, NBTFormat, NBTReader, Tag};
use crate::value::{NameLessValue, Value};


#[cfg(test)]
pub mod test {
    use std::marker::PhantomData;
    use std::path::Path;
    use async_compression::tokio::bufread::{GzipDecoder, GzipEncoder};
    use tokio::io::BufReader;
    use crate::{Binary, NBTReader};

    #[tokio::test]
    pub async fn test() {
        let path = Path::new("C:\\Users\\wherk\\AppData\\Roaming\\.minecraft\\saves\\New World\\playerdata\\d087006b-d72c-4cdf-924d-6f903704d05c.dat");
        let mut reader = tokio::fs::File::open(path).await.unwrap();
        let mut x = GzipDecoder::new(BufReader::new(reader));
        let mut reader = NBTReader {
            src: BufReader::new(x),
            phantom: PhantomData::<Binary>::default(),
        };
        let value = reader.read_value().await.unwrap();
        println!("{:#?}", value);
    }
}

impl<Read: AsyncBufReadExt + Unpin + Send> NBTReader<Binary, Read> {
    pub async fn peak_tag_id(&mut self) -> Result<Tag, std::io::Error> {
        let result = self.src.fill_buf().await?[0];
        Tag::from_u8(result).ok_or(
            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid Tag Id {}", result)))
    }
    pub async fn read_tag_id(&mut self) -> Result<Tag, std::io::Error> {
        let result = self.src.read_u8().await?;
        Tag::from_u8(result).ok_or(
            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid Tag Id {}", result)))
    }
    /// Will be zero if the tag is an end tag.
    pub async fn read_tag_id_with_id_len(&mut self) -> Result<(Tag, u16), std::io::Error> {
        let result = self.src.read_u8().await?;
        let tag = Tag::from_u8(result).ok_or(
            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid Tag Id {}", result)))?;
        let id_len = self.read_string_len().await?;
        Ok((tag, id_len))
    }
    pub async fn read_string_len(&mut self) -> Result<u16, std::io::Error> {
        let result = self.src.read_u16().await?;
        Ok(result)
    }
    /// You will need to convert this to a String.
    pub async fn read_str_as_bytes(&mut self, size: u16) -> Result<Vec<u8>, std::io::Error> {
        let mut result = self.src.fill_buf().await?[..size as usize].to_vec();
        self.src.consume(size as usize);
        Ok(result)
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
    pub async fn read_byte_array(&mut self, size: u32) -> Result<Vec<i8>, std::io::Error> {
        let mut result = Vec::with_capacity(size as usize);
        for _ in 0..size {
            result.push(self.src.read_i8().await?);
        }

        Ok(result)
    }
    pub async fn read_list_type_and_size(&mut self) -> Result<(Tag, u32), std::io::Error> {
        let tag = self.read_tag_id().await?;
        let size = self.read_int().await?;
        Ok((tag, size as u32))
    }
    pub async fn read_value(&mut self) -> Result<Value, std::io::Error> {
        let (tag, len) = self.read_tag_id_with_id_len().await?;
        if let Tag::End = tag {
            return Ok(Value::End);
        }
        let name = self.read_str_as_bytes(len).await?;
        let name = String::from_utf8(name).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        self.value_inner(tag, name).await
    }
    #[async_recursion::async_recursion]
    async fn value_inner(&mut self, tag: Tag, name: String) -> Result<Value, Error> {
        println!("{:?}", tag);
        match tag {
            Tag::Byte => {
                let value = self.read_byte().await?;
                Ok(Value::Byte { name, value })
            }
            Tag::Short => {
                let value = self.read_short().await?;
                Ok(Value::Short { name, value })
            }
            Tag::Int => {
                let value = self.read_int().await?;
                Ok(Value::Int { name, value })
            }
            Tag::Long => {
                let value = self.read_long().await?;
                Ok(Value::Long { name, value })
            }
            Tag::Float => {
                let value = self.read_float().await?;
                Ok(Value::Float { name, value })
            }
            Tag::Double => {
                let value = self.read_double().await?;
                Ok(Value::Double { name, value })
            }
            Tag::String => {
                let len = self.read_string_len().await?;
                let value = self.read_str_as_bytes(len).await?;
                let value = String::from_utf8(value).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                Ok(Value::String { name, value })
            }
            Tag::Compound => {
                let mut values = Vec::new();
                while let (tag, name_len) = self.read_tag_id_with_id_len().await? {
                    println!("{:?} Len {}", tag, name_len);
                    if let Tag::End = tag {
                        return Ok(Value::Compound { name, value: values });
                    }
                    let name = self.read_str_as_bytes(name_len).await?;

                    let name = String::from_utf8(name).map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
                    println!("{}", name);
                    let value = self.value_inner(tag, name).await?;
                    println!("{:?}", value);
                    values.push(value);
                }
                Err(Error::new(std::io::ErrorKind::InvalidData, "end tag not found"))
            }
            Tag::List =>{
                let (tag, size) = self.read_list_type_and_size().await?;
                let mut values = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    match &tag {
                        Tag::End => {}
                        Tag::Byte => {
                            let value = self.read_byte().await?;
                            values.push(NameLessValue::Byte(value));
                        }
                        Tag::Short => {
                            let value = self.read_short().await?;
                            values.push(NameLessValue::Short(value));
                        }
                        Tag::Int => {
                            let value = self.read_int().await?;
                            values.push(NameLessValue::Int(value));
                        }
                        Tag::Long => {
                            let value = self.read_long().await?;
                            values.push(NameLessValue::Long(value));
                        }
                        Tag::Float => {
                            let value = self.read_float().await?;
                            values.push(NameLessValue::Float(value));
                        }
                        Tag::Double => {
                            let value = self.read_double().await?;
                            values.push(NameLessValue::Double(value));
                        }
                        Tag::ByteArray => {}
                        Tag::String => {}
                        Tag::List => {}
                        Tag::Compound => {}
                        Tag::IntArray => {}
                        Tag::LongArray => {}
                    }
                }
                return Ok(Value::List { name, value: values });
            }
            v => {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("invalid tag id {:?}",v)));
            }
        }
    }
}

