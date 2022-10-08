use crate::binary::{Binary, BinaryCompoundReader, BinaryCompoundWriter, BinaryListWriter};
use crate::value::NameLessValue;
use crate::{
    CompoundReader, CompoundWriter, ListType, ListWriter, NBTDataType, NBTError, NBTType, Tag,
    Value,
};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Write};

impl NBTDataType<Binary> for Value {
    fn read<R: Read>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        #[cfg(feature = "log_all")]
        log::trace!("Reading Tag");
        let tag = Tag::read(reader)?;
        if tag == Tag::End {
            return Ok(Value::End);
        }
        #[cfg(feature = "log_all")]
        log::trace!("Reading Name");
        let tag_name = Binary::read_tag_name(reader)?;
        #[cfg(feature = "log_all")]
        log::debug!("Reading tag: {:?} Name: {:?}", tag, tag_name);

        match tag {
            Tag::End => Err(NBTError::UnexpectedEnd),
            Tag::Byte => Ok(Value::Byte {
                name: tag_name,
                value: i8::read(reader)?,
            }),
            Tag::Short => Ok(Value::Short {
                name: tag_name,
                value: i16::read(reader)?,
            }),
            Tag::Int => Ok(Value::Int {
                name: tag_name,
                value: i32::read(reader)?,
            }),
            Tag::Long => Ok(Value::Long {
                name: tag_name,
                value: i64::read(reader)?,
            }),
            Tag::Float => Ok(Value::Float {
                name: tag_name,
                value: f32::read(reader)?,
            }),
            Tag::Double => Ok(Value::Double {
                name: tag_name,
                value: f64::read(reader)?,
            }),
            Tag::ByteArray => {
                let length = i32::read(reader)?;
                let mut bytes = vec![0; length as usize];
                reader.read_i8_into(&mut bytes).map_err(NBTError::IO)?;
                Ok(Value::ByteArray {
                    name: tag_name,
                    value: bytes,
                })
            }
            Tag::String => {
                let length = i16::read(reader)?;
                let mut bytes = Vec::with_capacity(length as usize);
                reader.take(length as u64).read_to_end(&mut bytes)?;
                Ok(Value::String {
                    name: tag_name,
                    value: String::from_utf8(bytes).map_err(NBTError::NotAString)?,
                })
            }
            Tag::IntArray => {
                let length = i32::read(reader)?;
                let mut ints = vec![0; length as usize];
                reader
                    .read_i32_into::<BigEndian>(&mut ints)
                    .map_err(NBTError::IO)?;
                Ok(Value::IntArray {
                    name: tag_name,
                    value: ints,
                })
            }
            Tag::LongArray => {
                let length = i32::read(reader)?;
                let mut longs = vec![0; length as usize];
                reader
                    .read_i64_into::<BigEndian>(&mut longs)
                    .map_err(NBTError::IO)?;
                Ok(Value::LongArray {
                    name: tag_name,
                    value: longs,
                })
            }
            Tag::List => {
                let list_type = Tag::read(reader)?;
                let length = i32::read(reader)?;
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    list.push(NameLessValue::read(list_type, reader)?);
                }
                Ok(Value::List {
                    name: tag_name,
                    value: list,
                })
            }
            Tag::Compound => {
                let result = BinaryCompoundReader::new(reader)?;
                Ok(Value::Compound {
                    name: tag_name,
                    value: result.read_to_end()?,
                })
            }
        }
    }

    fn write<W: Write, Name: AsRef<[u8]>>(self, _: Name, writer: &mut W) -> Result<(), NBTError> {
        self.write_alone(writer)
    }

    fn write_alone<W: Write>(self, writer: &mut W) -> Result<(), NBTError> {
        match self {
            Value::End => {
                return Err(NBTError::UnexpectedEnd);
            }
            Value::Byte { name, value } => {
                value.write(name, writer)?;
            }
            Value::Short { name, value } => {
                value.write(name, writer)?;
            }
            Value::Int { name, value } => {
                value.write(name, writer)?;
            }
            Value::Long { name, value } => {
                value.write(name, writer)?;
            }
            Value::Float { name, value } => {
                value.write(name, writer)?;
            }
            Value::Double { name, value } => {
                value.write(name, writer)?;
            }
            Value::ByteArray { name, value } => {
                let mut writer =
                    BinaryListWriter::new(writer, value.len() as i32, ListType::ByteArray, name)?;
                for i in value {
                    writer.write_next_tag(i)?;
                }
            }
            Value::String { name, value } => {
                value.write(name, writer)?;
            }
            Value::List { name, value } => {
                let tag = if let Some(first) = value.first() {
                    first.tag()
                } else {
                    return Ok(());
                };
                let mut writer =
                    BinaryListWriter::new(writer, value.len() as i32, ListType::List(tag), name)?;
                for i in value {
                    writer.write_next_tag(i)?;
                }
            }
            Value::Compound { name, value } => {
                BinaryCompoundWriter::write_start(writer, name)?;
                for i in value {
                    i.write_alone(writer)?;
                }
                Tag::End.write_alone(writer)?;
            }
            Value::IntArray { name, value } => {
                let mut writer =
                    BinaryListWriter::new(writer, value.len() as i32, ListType::IntArray, name)?;
                for i in value {
                    writer.write_next_tag(i)?;
                }
            }
            Value::LongArray { name, value } => {
                let mut writer =
                    BinaryListWriter::new(writer, value.len() as i32, ListType::LongArray, name)?;
                for i in value {
                    writer.write_next_tag(i)?;
                }
            }
            Value::Boolean { name, value } => {
                value.write(name, writer)?;
            }
        }
        Ok(())
    }

    fn get_tag() -> Tag {
        Tag::Compound
    }
}

impl NameLessValue {
    pub fn read<Reader: Read>(tag: Tag, reader: &mut Reader) -> Result<NameLessValue, NBTError> {
        match tag {
            Tag::End => Err(NBTError::UnexpectedEnd),
            Tag::Byte => Ok(NameLessValue::Byte(i8::read(reader)?)),
            Tag::Short => Ok(NameLessValue::Short(i16::read(reader)?)),
            Tag::Int => Ok(NameLessValue::Int(i32::read(reader)?)),
            Tag::Long => Ok(NameLessValue::Long(i64::read(reader)?)),
            Tag::Float => Ok(NameLessValue::Float(f32::read(reader)?)),
            Tag::Double => Ok(NameLessValue::Double(f64::read(reader)?)),
            Tag::ByteArray => {
                let length = i32::read(reader)?;
                let mut bytes = vec![0; length as usize];
                reader.read_i8_into(&mut bytes).map_err(NBTError::IO)?;
                Ok(NameLessValue::ByteArray(bytes))
            }
            Tag::String => {
                let length = i16::read(reader)?;
                let mut bytes = Vec::with_capacity(length as usize);
                reader.take(length as u64).read_to_end(&mut bytes)?;
                Ok(NameLessValue::String(
                    String::from_utf8(bytes).map_err(NBTError::NotAString)?,
                ))
            }
            Tag::List => {
                let list_type = Tag::read(reader)?;
                let length = i32::read(reader)?;
                let mut list = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    list.push(NameLessValue::read(list_type, reader)?);
                }
                Ok(NameLessValue::List(list))
            }
            Tag::Compound => {
                let result = BinaryCompoundReader::new(reader)?;
                Ok(NameLessValue::Compound(result.read_to_end()?))
            }
            Tag::IntArray => {
                let length = i32::read(reader)?;
                let mut ints = vec![0; length as usize];
                reader
                    .read_i32_into::<BigEndian>(&mut ints)
                    .map_err(NBTError::IO)?;
                Ok(NameLessValue::IntArray(ints))
            }
            Tag::LongArray => {
                let length = i32::read(reader)?;
                let mut longs = vec![0; length as usize];
                reader
                    .read_i64_into::<BigEndian>(&mut longs)
                    .map_err(NBTError::IO)?;
                Ok(NameLessValue::LongArray(longs))
            }
        }
    }
}

impl NBTDataType<Binary> for NameLessValue {
    fn read<R: Read>(_reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        unimplemented!("Please use read(tag, reader) instead")
    }

    fn write<W: Write, Name: AsRef<[u8]>>(
        self,
        _name: Name,
        _writer: &mut W,
    ) -> Result<(), NBTError> {
        unimplemented!("Please use write_alone(writer) instead")
    }

    fn write_alone<W: Write>(self, writer: &mut W) -> Result<(), NBTError> {
        match self {
            NameLessValue::End => {
                return Err(NBTError::UnexpectedEnd);
            }
            NameLessValue::Byte(data) => {
                data.write_alone(writer)?;
            }
            NameLessValue::Short(data) => {
                data.write_alone(writer)?;
            }
            NameLessValue::Int(data) => {
                data.write_alone(writer)?;
            }
            NameLessValue::Long(data) => {
                data.write_alone(writer)?;
            }
            NameLessValue::Float(data) => {
                data.write_alone(writer)?;
            }
            NameLessValue::Double(data) => {
                data.write_alone(writer)?;
            }
            NameLessValue::ByteArray(data) => {
                let mut writer = BinaryListWriter::new_sub_sequence(
                    writer,
                    data.len() as i32,
                    ListType::ByteArray,
                )?;
                for i in data {
                    writer.write_next_tag(i)?;
                }
            }
            NameLessValue::String(data) => {
                data.write_alone(writer)?;
            }
            NameLessValue::List(data) => {
                let tag = if let Some(first) = data.first() {
                    first.tag()
                } else {
                    return Ok(());
                };
                let mut writer = BinaryListWriter::new_sub_sequence(
                    writer,
                    data.len() as i32,
                    ListType::List(tag),
                )?;
                for i in data {
                    writer.write_next_tag(i)?;
                }
            }
            NameLessValue::Compound(data) => {
                for i in data {
                    i.write_alone(writer)?;
                }
                Tag::End.write_alone(writer)?;
            }
            NameLessValue::IntArray(data) => {
                let mut writer = BinaryListWriter::new_sub_sequence(
                    writer,
                    data.len() as i32,
                    ListType::IntArray,
                )?;
                for i in data {
                    writer.write_next_tag(i)?;
                }
            }
            NameLessValue::LongArray(data) => {
                let mut writer = BinaryListWriter::new_sub_sequence(
                    writer,
                    data.len() as i32,
                    ListType::LongArray,
                )?;
                for i in data {
                    writer.write_next_tag(i)?;
                }
            }
            NameLessValue::Boolean(bool) => {
                bool.write_alone(writer)?;
            }
        }
        Ok(())
    }

    fn get_tag() -> Tag {
        Tag::Compound
    }
}
