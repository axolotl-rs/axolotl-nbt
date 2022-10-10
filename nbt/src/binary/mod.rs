use crate::{
    CompoundReader, CompoundWriter, ListReader, ListType, ListWriter, NBTDataType, NBTError,
    NBTType, NameLessValue, Tag, Value,
};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt::Debug;
use std::io::{Read, Write};

pub mod binary_uuid;
pub mod sync;

#[derive(Debug)]
pub struct Binary;

impl Binary {
    pub fn tag_from_i8(value: i8) -> Option<Tag> {
        match value {
            0 => Some(Tag::End),
            1 => Some(Tag::Byte),
            2 => Some(Tag::Short),
            3 => Some(Tag::Int),
            4 => Some(Tag::Long),
            5 => Some(Tag::Float),
            6 => Some(Tag::Double),
            7 => Some(Tag::ByteArray),
            8 => Some(Tag::String),
            9 => Some(Tag::List),
            10 => Some(Tag::Compound),
            11 => Some(Tag::IntArray),
            12 => Some(Tag::LongArray),
            _ => None,
        }
    }
    pub fn get_list_type_id(typ: ListType) -> i8 {
        match typ {
            ListType::ByteArray => 7,
            ListType::IntArray => 11,
            ListType::LongArray => 12,
            ListType::List(_) => 9,
        }
    }
}

impl NBTType for Binary {
    type ListReader<'reader, Reader: Read + 'reader> = BinaryListReader<'reader, Reader>;
    type ListWriter<'writer, Writer: Write + 'writer> = BinaryListWriter<'writer, Writer>;
    type CompoundWriter<'writer, Writer: Write + 'writer> = BinaryCompoundWriter<'writer, Writer>;
    type CompoundReader<'reader, Reader: Read + 'reader> = BinaryCompoundReader<'reader, Reader>;

    #[inline]
    fn read_tag_name<R: Read>(reader: &mut R) -> Result<String, NBTError> {
        let length = reader.read_u16::<BigEndian>()?;
        #[cfg(feature = "log_all")]
        log::debug!("Reading tag name with length {}", length);
        let mut string = Vec::with_capacity(length as usize);
        reader.take(length as u64).read_to_end(&mut string)?;
        String::from_utf8(string).map_err(NBTError::NotAString)
    }

    fn read_tag_name_raw<R: Read>(reader: &mut R, value: &mut Vec<u8>) -> Result<(), NBTError> {
        let length = reader.read_u16::<BigEndian>()?;
        reader.take(length as u64).read_to_end(value)?;
        Ok(())
    }

    #[inline]
    fn write_tag_name<W: Write, Name: AsRef<[u8]>>(
        writer: &mut W,
        name: Name,
    ) -> Result<(), NBTError> {
        writer.write_u16::<BigEndian>(name.as_ref().len() as u16)?;
        writer.write_all(name.as_ref()).map_err(NBTError::from)
    }
}

/// Order:
///   - List, ByteList, IntList, LongList
///   - If List:
///         Name, Tag, Length, Elements of Tag
///   - Else:
///         Name, Length, Elements just written
pub struct BinaryListReader<'reader, R: Read> {
    reader: &'reader mut R,
    tag: ListType,
    length: i32,
}

impl<'reader, Reader: Read> ListReader<'reader, Binary, Reader>
    for BinaryListReader<'reader, Reader>
{
    fn new(reader: &'reader mut Reader, list_type: ListType) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        let length = reader.read_i32::<BigEndian>()?;
        Ok(Self {
            reader,
            tag: list_type,
            length,
        })
    }

    fn new_generic_list(reader: &'reader mut Reader) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        let tag = Tag::read(reader)?;
        let length = reader.read_i32::<BigEndian>()?;
        Ok(Self {
            reader,
            tag: ListType::List(tag),
            length,
        })
    }

    fn size(&self) -> usize {
        self.length as usize
    }

    fn list_type(&self) -> ListType {
        self.tag
    }

    fn get_tag(&self) -> Tag {
        match self.tag {
            ListType::ByteArray => Tag::Byte,
            ListType::IntArray => Tag::Int,
            ListType::LongArray => Tag::Long,
            ListType::List(v) => v,
        }
    }
    #[inline(always)]
    fn read_next_tag<DataType: NBTDataType<Binary>>(&mut self) -> Result<DataType, NBTError> {
        DataType::read(self.reader)
    }

    #[cfg(feature = "value")]
    fn read_next(&mut self) -> Result<NameLessValue, NBTError> {
        match self.tag {
            ListType::ByteArray => Ok(NameLessValue::Byte(self.read_next_tag()?)),
            ListType::IntArray => Ok(NameLessValue::Int(self.read_next_tag()?)),
            ListType::LongArray => Ok(NameLessValue::Long(self.read_next_tag()?)),
            ListType::List(v) => NameLessValue::read(v, self.reader),
        }
    }
}

pub struct BinaryCompoundReader<'reader, R: Read> {
    reader: &'reader mut R,
    next_tag: Option<Tag>,
}

impl<'reader, R: Read> CompoundReader<'reader, Binary, R> for BinaryCompoundReader<'reader, R> {
    fn new(reader: &'reader mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        Ok(Self {
            reader,
            next_tag: None,
        })
    }

    fn next_tag(&mut self) -> Result<Tag, NBTError> {
        if let Some(tag) = &self.next_tag {
            Ok(*tag)
        } else {
            let result = Tag::read(self.reader)?;
            self.next_tag = Some(result);
            Ok(result)
        }
    }

    fn read_next_tag_name(&mut self) -> Result<String, NBTError> {
        if let Some(ref tag) = self.next_tag {
            if *tag == Tag::End {
                return Err(NBTError::UnexpectedEnd);
            }
        } else {
            let tag = Tag::read(self.reader)?;
            if tag == Tag::End {
                return Err(NBTError::UnexpectedEnd);
            }
            self.next_tag = Some(tag);
        }
        assert_ne!(self.next_tag, Some(Tag::End));
        Binary::read_tag_name(self.reader)
    }

    fn read_next_tag_value<DataType: NBTDataType<Binary>>(&mut self) -> Result<DataType, NBTError> {
        if let Some(tag) = self.next_tag.take() {
            if tag == Tag::End {
                return Err(NBTError::UnexpectedEnd);
            }
            if tag != DataType::get_tag() {
                self.next_tag = Some(tag);
                return Err(NBTError::ExpectedTag(DataType::get_tag(), tag));
            }

            let data = DataType::read(self.reader)?;
            Ok(data)
        } else {
            let tag = Tag::read(self.reader)?;
            if tag == Tag::End {
                return Err(NBTError::UnexpectedEnd);
            }
            if tag != DataType::get_tag() {
                self.next_tag = Some(tag);
                return Err(NBTError::ExpectedTag(DataType::get_tag(), tag));
            }
            let data = DataType::read(self.reader)?;
            Ok(data)
        }
    }

    fn read_next_tag<DataType: NBTDataType<Binary>>(
        &mut self,
    ) -> Result<(String, DataType), NBTError> {
        if let Some(tag) = self.next_tag.take() {
            if tag == Tag::End {
                return Err(NBTError::UnexpectedEnd);
            }
            if tag != DataType::get_tag() {
                self.next_tag = Some(tag);
                return Err(NBTError::ExpectedTag(DataType::get_tag(), tag));
            }
            let string = Binary::read_tag_name(self.reader)?;
            let data = DataType::read(self.reader)?;
            Ok((string, data))
        } else {
            let tag = Tag::read(self.reader)?;
            if tag == Tag::End {
                return Err(NBTError::UnexpectedEnd);
            }
            if tag != DataType::get_tag() {
                self.next_tag = Some(tag);
                return Err(NBTError::ExpectedTag(DataType::get_tag(), tag));
            }
            let string = Binary::read_tag_name(self.reader)?;
            let data = DataType::read(self.reader)?;
            Ok((string, data))
        }
    }
    #[cfg(feature = "value")]
    fn read_to_end(self) -> Result<Vec<Value>, NBTError> {
        let mut result = Vec::new();
        loop {
            let value = Value::read(self.reader);
            match value {
                Ok(ok) => {
                    if ok == Value::End {
                        return Ok(result);
                    } else {
                        result.push(ok);
                    }
                }
                Err(err) => {
                    return match err {
                        NBTError::UnexpectedEnd => Ok(result),
                        err => Err(err),
                    };
                }
            }
        }
    }
    #[cfg(feature = "value")]
    fn read_next(&mut self) -> Result<Value, NBTError> {
        Value::read(self.reader)
    }
}

pub struct BinaryListWriter<'writer, W: Write> {
    writer: &'writer mut W,
}

pub struct BinaryCompoundWriter<'writer, W: Write> {
    writer: &'writer mut W,
}

impl<'writer, W: Write> CompoundWriter<'writer, Binary, W> for BinaryCompoundWriter<'writer, W> {
    fn new(reader: &'writer mut W) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        Ok(Self { writer: reader })
    }

    fn write_start<Name: AsRef<[u8]>>(writer: &mut W, name: Name) -> Result<(), NBTError> {
        Tag::Compound.write_alone(writer)?;
        Binary::write_tag_name(writer, name)?;
        Ok(())
    }

    fn write_next_tag<DataType: NBTDataType<Binary>>(
        &mut self,
        name: impl AsRef<[u8]>,
        value: DataType,
    ) -> Result<(), NBTError> {
        value.write(name, self.writer)
    }

    fn end(self) -> Result<(), NBTError> {
        Tag::End.write_alone(self.writer)
    }
}

impl<'writer, Writer: Write> ListWriter<'writer, Binary, Writer>
    for BinaryListWriter<'writer, Writer>
{
    fn new<Name: AsRef<[u8]>>(
        reader: &'writer mut Writer,
        size: i32,
        list_type: ListType,
        name: Name,
    ) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        Self::write_sequence_header(reader, list_type, name, size)?;
        Ok(Self { writer: reader })
    }

    fn new_sub_sequence(
        reader: &'writer mut Writer,
        size: i32,
        list_type: ListType,
    ) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        Self::write_sub_sequence_header(reader, list_type, size)?;
        Ok(Self { writer: reader })
    }

    fn write_sequence_header_name_callback<W: Write, Name>(
        writer: &mut W,
        list_type: ListType,
        length_of_array: i32,
        name: Name,
    ) -> Result<(), NBTError>
    where
        Name: FnOnce(&mut W) -> Result<(), NBTError>,
    {
        match list_type {
            ListType::List(v) => {
                Tag::List.write_alone(writer)?;
                name(writer)?;
                v.write_alone(writer)?;
                length_of_array.write_alone(writer)?;
            }
            v => {
                Binary::get_list_type_id(v).write_alone(writer)?;
                name(writer)?;
                length_of_array.write_alone(writer)?;
            }
        }
        Ok(())
    }

    fn write_sub_sequence_header<W: Write>(
        writer: &mut W,
        list_type: ListType,
        length_of_array: i32,
    ) -> Result<(), NBTError> {
        match list_type {
            ListType::List(v) => {
                v.write_alone(writer)?;
                length_of_array.write_alone(writer)?;
            }
            _ => {
                length_of_array.write_alone(writer)?;
            }
        }
        Ok(())
    }
    #[inline]
    fn write_next_tag<DataType: NBTDataType<Binary>>(
        &mut self,
        value: DataType,
    ) -> Result<(), NBTError> {
        value.write_alone(self.writer)
    }
}
