use crate::sync::NBTReader;
use crate::value::{NameLessValue, Value};
use crate::{NBTError, Tag};
use byteorder::ByteOrder;
use std::fmt::Debug;
use std::io::Read;

impl<Reader: Read + Debug> NBTReader<Reader> {
    pub fn read_value(&mut self) -> Result<Value, NBTError> {
        let (tag, len) = self.read_tag_id_with_id_len()?;
        if let Tag::End = tag {
            return Ok(Value::End);
        }
        let name = self.read_str_as_bytes(len)?;
        let name = String::from_utf8(name)?;
        self.value_inner(tag, name)
    }
    pub(crate) fn value_inner(&mut self, tag: Tag, name: String) -> Result<Value, NBTError> {
        match tag {
            Tag::Byte => {
                let value = self.read_byte()?;
                Ok(Value::Byte { name, value })
            }
            Tag::Short => {
                let value = self.read_short()?;
                Ok(Value::Short { name, value })
            }
            Tag::Int => {
                let value = self.read_int()?;
                Ok(Value::Int { name, value })
            }
            Tag::Long => {
                let value = self.read_long()?;
                Ok(Value::Long { name, value })
            }
            Tag::Float => {
                let value = self.read_float()?;
                Ok(Value::Float { name, value })
            }
            Tag::Double => {
                let value = self.read_double()?;
                Ok(Value::Double { name, value })
            }
            Tag::String => {
                let len = self.read_string_len()?;
                let value = self.read_str_as_bytes(len)?;
                let value = String::from_utf8(value)?;
                Ok(Value::String { name, value })
            }
            Tag::Compound => {
                let mut values = Vec::new();
                loop {
                    let (tag, name_len) = self.read_tag_id_with_id_len()?;
                    if let Tag::End = tag {
                        return Ok(Value::Compound {
                            name,
                            value: values,
                        });
                    }

                    let name = self.read_str_as_bytes(name_len)?;

                    let name = String::from_utf8(name)?;
                    let value = self.value_inner(tag, name)?;
                    values.push(value);
                }
            }
            Tag::List => {
                let (tag, size) = self.read_list_type_and_size()?;
                let mut values = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    let value = self.read_nameless(&tag)?;
                    values.push(value);
                }
                Ok(Value::List {
                    name,
                    value: values,
                })
            }
            Tag::IntArray => {
                let size = self.read_int()?;
                let mut values = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    let value = self.read_int()?;
                    values.push(value);
                }
                Ok(Value::IntArray {
                    name,
                    value: values,
                })
            }
            Tag::ByteArray => {
                let value = self.read_int()?;
                let value = self.read_byte_array(value)?;
                Ok(Value::ByteArray { name, value })
            }
            Tag::LongArray => {
                let size = self.read_int()?;
                let mut values = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    let value = self.read_long()?;
                    values.push(value);
                }
                Ok(Value::LongArray {
                    name,
                    value: values,
                })
            }
            Tag::End => {
                return Err(NBTError::UnexpectedEnd);
            }
        }
    }
    fn read_nameless(&mut self, tag: &Tag) -> Result<NameLessValue, NBTError> {
        match &tag {
            Tag::End => Ok(NameLessValue::End),
            Tag::Byte => {
                let value = self.read_byte()?;
                Ok(NameLessValue::Byte(value))
            }
            Tag::Short => {
                let value = self.read_short()?;
                Ok(NameLessValue::Short(value))
            }
            Tag::Int => {
                let value = self.read_int()?;
                Ok(NameLessValue::Int(value))
            }
            Tag::Long => {
                let value = self.read_long()?;
                Ok(NameLessValue::Long(value))
            }
            Tag::Float => {
                let value = self.read_float()?;
                Ok(NameLessValue::Float(value))
            }
            Tag::Double => {
                let value = self.read_double()?;
                Ok(NameLessValue::Double(value))
            }
            Tag::ByteArray => {
                let value = self.read_int()?;
                let value = self.read_byte_array(value)?;
                Ok(NameLessValue::ByteArray(value))
            }
            Tag::String => {
                let len = self.read_string_len()?;
                let value = self.read_str_as_bytes(len)?;
                let value = String::from_utf8(value)?;
                Ok(NameLessValue::String(value))
            }
            Tag::List => {
                let (tag, size) = self.read_list_type_and_size()?;
                let mut values = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    let value = self.read_nameless(&tag)?;
                    values.push(value);
                }
                Ok(NameLessValue::List(values))
            }
            Tag::Compound => {
                let mut values = Vec::new();
                loop {
                    let (tag, name_len) = self.read_tag_id_with_id_len()?;
                    if let Tag::End = tag {
                        return Ok(NameLessValue::Compound(values));
                    }
                    let name = self.read_str_as_bytes(name_len)?;
                    let name = String::from_utf8(name)?;
                    let value = self.value_inner(tag, name)?;
                    values.push(value);
                }
            }
            Tag::IntArray => {
                let size = self.read_int()?;
                let mut values = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    let value = self.read_int()?;
                    values.push(value);
                }
                Ok(NameLessValue::IntArray(values))
            }
            Tag::LongArray => {
                let size = self.read_int()?;
                let mut values = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    let value = self.read_long()?;
                    values.push(value);
                }
                Ok(NameLessValue::LongArray(values))
            }
        }
    }
}
