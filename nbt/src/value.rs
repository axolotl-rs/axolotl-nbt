use crate::{Binary, NBTReader, Tag};
use std::fmt::Debug;
use std::io::Error;

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    End,
    Byte {
        name: String,
        value: i8,
    },
    Short {
        name: String,
        value: i16,
    },
    Int {
        name: String,
        value: i32,
    },
    Long {
        name: String,
        value: i64,
    },
    Float {
        name: String,
        value: f32,
    },
    Double {
        name: String,
        value: f64,
    },
    ByteArray {
        name: String,
        value: Vec<i8>,
    },
    String {
        name: String,
        value: String,
    },
    List {
        name: String,
        value: Vec<NameLessValue>,
    },
    Compound {
        name: String,
        value: Vec<Value>,
    },
    IntArray {
        name: String,
        value: Vec<i32>,
    },
    LongArray {
        name: String,
        value: Vec<i64>,
    },
}

#[derive(PartialEq, Clone, Debug)]
pub enum NameLessValue {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<NameLessValue>),
    Compound(Vec<Value>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[cfg(feature = "tokio")]
impl<Read: tokio::io::AsyncReadExt + Unpin + Send + Debug> NBTReader<Binary, Read> {
    pub async fn async_read_value(&mut self) -> Result<Value, Error> {
        let (tag, len) = self.async_read_tag_id_with_id_len().await?;
        if let Tag::End = tag {
            return Ok(Value::End);
        }
        let name = self.async_read_str_as_bytes(len).await?;
        let name = String::from_utf8(name)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        self.value_inner(tag, name).await
    }
    #[cfg_attr(feature = "value", async_recursion::async_recursion)]
    async fn value_inner(&mut self, tag: Tag, name: String) -> Result<Value, Error> {
        match tag {
            Tag::Byte => {
                let value = self.async_read_byte().await?;
                Ok(Value::Byte { name, value })
            }
            Tag::Short => {
                let value = self.async_read_short().await?;
                Ok(Value::Short { name, value })
            }
            Tag::Int => {
                let value = self.async_read_int().await?;
                Ok(Value::Int { name, value })
            }
            Tag::Long => {
                let value = self.async_read_long().await?;
                Ok(Value::Long { name, value })
            }
            Tag::Float => {
                let value = self.async_read_float().await?;
                Ok(Value::Float { name, value })
            }
            Tag::Double => {
                let value = self.async_read_double().await?;
                Ok(Value::Double { name, value })
            }
            Tag::String => {
                let len = self.async_read_string_len().await?;
                let value = self.async_read_str_as_bytes(len).await?;
                let value = String::from_utf8(value)
                    .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
                Ok(Value::String { name, value })
            }
            Tag::Compound => {
                let mut values = Vec::new();
                loop {
                    let (tag, name_len) = self.async_read_tag_id_with_id_len().await?;
                    if let Tag::End = tag {
                        return Ok(Value::Compound {
                            name,
                            value: values,
                        });
                    }

                    let name = self.async_read_str_as_bytes(name_len).await?;

                    let name = String::from_utf8(name)
                        .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
                    let value = self.value_inner(tag, name).await?;
                    values.push(value);
                }
            }
            Tag::List => {
                let (tag, size) = self.async_read_list_type_and_size().await?;
                let mut values = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    let value = self.read_nameless(&tag).await?;
                    values.push(value);
                }
                Ok(Value::List {
                    name,
                    value: values,
                })
            }
            Tag::IntArray => {
                let size = self.async_read_int().await?;
                let mut values = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    let value = self.async_read_int().await?;
                    values.push(value);
                }
                Ok(Value::IntArray {
                    name,
                    value: values,
                })
            }
            Tag::ByteArray => {
                let value = self.async_read_int().await?;
                let value = self.async_read_byte_array(value).await?;
                Ok(Value::ByteArray { name, value })
            }
            Tag::LongArray => {
                let size = self.async_read_int().await?;
                let mut values = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    let value = self.async_read_long().await?;
                    values.push(value);
                }
                Ok(Value::LongArray {
                    name,
                    value: values,
                })
            }
            Tag::End => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "end tag not allowed",
            )),
        }
    }
    #[cfg_attr(feature = "value", async_recursion::async_recursion)]
    async fn read_nameless(&mut self, tag: &Tag) -> Result<NameLessValue, Error> {
        match &tag {
            Tag::End => Ok(NameLessValue::End),
            Tag::Byte => {
                let value = self.async_read_byte().await?;
                Ok(NameLessValue::Byte(value))
            }
            Tag::Short => {
                let value = self.async_read_short().await?;
                Ok(NameLessValue::Short(value))
            }
            Tag::Int => {
                let value = self.async_read_int().await?;
                Ok(NameLessValue::Int(value))
            }
            Tag::Long => {
                let value = self.async_read_long().await?;
                Ok(NameLessValue::Long(value))
            }
            Tag::Float => {
                let value = self.async_read_float().await?;
                Ok(NameLessValue::Float(value))
            }
            Tag::Double => {
                let value = self.async_read_double().await?;
                Ok(NameLessValue::Double(value))
            }
            Tag::ByteArray => {
                let value = self.async_read_int().await?;
                let value = self.async_read_byte_array(value).await?;
                Ok(NameLessValue::ByteArray(value))
            }
            Tag::String => {
                let len = self.async_read_string_len().await?;
                let value = self.async_read_str_as_bytes(len).await?;
                let value = String::from_utf8(value)
                    .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
                Ok(NameLessValue::String(value))
            }
            Tag::List => {
                let (tag, size) = self.async_read_list_type_and_size().await?;
                let mut values = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    let value = self.read_nameless(&tag).await?;
                    values.push(value);
                }
                Ok(NameLessValue::List(values))
            }
            Tag::Compound => {
                let mut values = Vec::new();
                loop {
                    let (tag, name_len) = self.async_read_tag_id_with_id_len().await?;
                    if let Tag::End = tag {
                        return Ok(NameLessValue::Compound(values));
                    }
                    let name = self.async_read_str_as_bytes(name_len).await?;
                    let name = String::from_utf8(name)
                        .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
                    let value = self.value_inner(tag, name).await?;
                    values.push(value);
                }
            }
            Tag::IntArray => {
                let size = self.async_read_int().await?;
                let mut values = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    let value = self.async_read_int().await?;
                    values.push(value);
                }
                Ok(NameLessValue::IntArray(values))
            }
            Tag::LongArray => {
                let size = self.async_read_int().await?;
                let mut values = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    let value = self.async_read_long().await?;
                    values.push(value);
                }
                Ok(NameLessValue::LongArray(values))
            }
        }
    }
}
