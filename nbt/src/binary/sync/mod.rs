use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::binary::Binary;
use crate::NBTError::InvalidTag;
use crate::{ListType, NBTDataType, NBTError, NBTType, Tag};
use byteorder::BigEndian;
use std::io::{Read, Write};

#[cfg(feature = "value")]
pub mod value;

impl NBTDataType<Binary> for bool {
    fn read<R: Read>(reader: &mut R) -> Result<Self, NBTError>
        where
            Self: Sized,
    {
        Ok(reader.read_u8()? != 0)
    }

    fn write<W: Write, Name: AsRef<[u8]>>(
        self,
        name: Name,
        writer: &mut W,
    ) -> Result<(), NBTError> {
        Tag::Byte.write_alone(writer)?;
        Binary::write_tag_name(writer, name)?;
        writer.write_i8(i8::from(self))?;
        Ok(())
    }

    #[inline]
    fn write_alone<W: Write>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_i8(i8::from(self)).map_err(NBTError::from)
    }

    fn get_tag() -> Tag {
        Tag::Byte
    }
}

impl NBTDataType<Binary> for Tag {
    fn read_with_name<R: Read>(reader: &mut R) -> Result<(String, Self), NBTError>
        where
            Self: Sized,
    {
        let tag = reader.read_u8()? as i8;
        let tag = Binary::tag_from_i8(tag).ok_or(InvalidTag(tag))?;
        if tag == Tag::End {
            Ok((String::new(), tag))
        } else {
            let name = Binary::read_tag_name(reader)?;
            Ok((name, tag))
        }
    }
    #[inline(always)]
    fn read<R: Read>(reader: &mut R) -> Result<Self, NBTError>
        where
            Self: Sized,
    {
        let tag_id = reader.read_i8()?;
        Binary::tag_from_i8(tag_id).ok_or(InvalidTag(tag_id))
    }
    #[inline(always)]
    fn write<W: Write, Name: AsRef<[u8]>>(self, _: Name, writer: &mut W) -> Result<(), NBTError> {
        writer.write_i8(self as i8).map_err(NBTError::IO)
    }
    #[inline(always)]
    fn write_alone<W: Write>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_i8(self as i8).map_err(NBTError::IO)
    }

    fn get_list_tag() -> ListType {
        ListType::ByteArray
    }

    fn get_tag() -> Tag {
        Tag::Byte
    }
}

impl NBTDataType<Binary> for &str {
    fn read<R: Read>(_: &mut R) -> Result<Self, NBTError>
        where
            Self: Sized,
    {
        unimplemented!("read str ref")
    }

    fn write<W: Write, Name: AsRef<[u8]>>(
        self,
        name: Name,
        writer: &mut W,
    ) -> Result<(), NBTError> {
        Tag::String.write_alone(writer)?;
        Binary::write_tag_name(writer, name)?;
        writer.write_u16::<BigEndian>(self.len() as u16)?;
        writer.write_all(self.as_ref()).map_err(NBTError::IO)
    }

    fn write_alone<W: Write>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_u16::<BigEndian>(self.len() as u16)?;
        writer.write_all(self.as_ref()).map_err(NBTError::IO)
    }

    fn get_tag() -> Tag {
        Tag::String
    }
}

impl NBTDataType<Binary> for String {
    fn read<R: Read>(reader: &mut R) -> Result<Self, NBTError>
        where
            Self: Sized,
    {
        let len = reader.read_u16::<BigEndian>()?;
        let mut buf = vec![0; len as usize];
        reader.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(NBTError::NotAString)
    }

    fn write<W: Write, Name: AsRef<[u8]>>(
        self,
        name: Name,
        writer: &mut W,
    ) -> Result<(), NBTError> {
        self.as_str().write(name, writer)
    }

    fn write_alone<W: Write>(self, writer: &mut W) -> Result<(), NBTError> {
        self.as_str().write_alone(writer)
    }

    fn get_tag() -> Tag {
        Tag::String
    }
}

impl NBTDataType<Binary> for i8 {
    fn read<R: Read>(reader: &mut R) -> Result<Self, NBTError>
        where
            Self: Sized,
    {
        reader.read_i8().map_err(NBTError::IO)
    }

    fn write<W: Write, Name: AsRef<[u8]>>(
        self,
        name: Name,
        writer: &mut W,
    ) -> Result<(), NBTError> {
        Tag::Byte.write_alone(writer)?;
        Binary::write_tag_name(writer, name)?;
        writer.write_i8(self).map_err(NBTError::IO)
    }

    fn write_alone<W: Write>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_i8(self).map_err(NBTError::IO)
    }
    fn get_list_tag() -> ListType {
        ListType::ByteArray
    }
    fn get_tag() -> Tag {
        Tag::Byte
    }
}

impl NBTDataType<Binary> for i16 {
    fn read<R: Read>(reader: &mut R) -> Result<Self, NBTError>
        where
            Self: Sized,
    {
        reader.read_i16::<BigEndian>().map_err(NBTError::IO)
    }

    fn write<W: Write, Name: AsRef<[u8]>>(
        self,
        name: Name,
        writer: &mut W,
    ) -> Result<(), NBTError> {
        Tag::Short.write_alone(writer)?;
        Binary::write_tag_name(writer, name)?;
        writer.write_i16::<BigEndian>(self).map_err(NBTError::IO)
    }

    fn write_alone<W: Write>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_i16::<BigEndian>(self).map_err(NBTError::IO)
    }

    fn get_tag() -> Tag {
        Tag::Short
    }
}
impl NBTDataType<Binary> for i32 {
    fn read<R: Read>(reader: &mut R) -> Result<Self, NBTError>
        where
            Self: Sized,
    {
        reader.read_i32::<BigEndian>().map_err(NBTError::IO)
    }

    fn write<W: Write, Name: AsRef<[u8]>>(
        self,
        name: Name,
        writer: &mut W,
    ) -> Result<(), NBTError> {
        Tag::Int.write_alone(writer)?;
        Binary::write_tag_name(writer, name)?;
        writer.write_i32::<BigEndian>(self).map_err(NBTError::IO)
    }

    fn write_alone<W: Write>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_i32::<BigEndian>(self).map_err(NBTError::IO)
    }
    fn get_list_tag() -> ListType {
        ListType::IntArray
    }
    fn get_tag() -> Tag {
        Tag::Int
    }
}

impl NBTDataType<Binary> for i64 {
    fn read<R: Read>(reader: &mut R) -> Result<Self, NBTError>
        where
            Self: Sized,
    {
        reader.read_i64::<BigEndian>().map_err(NBTError::IO)
    }

    fn write<W: Write, Name: AsRef<[u8]>>(
        self,
        name: Name,
        writer: &mut W,
    ) -> Result<(), NBTError> {
        Tag::Long.write_alone(writer)?;
        Binary::write_tag_name(writer, name)?;
        writer.write_i64::<BigEndian>(self).map_err(NBTError::IO)
    }

    fn write_alone<W: Write>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_i64::<BigEndian>(self).map_err(NBTError::IO)
    }

    fn get_list_tag() -> ListType {
        ListType::LongArray
    }
    fn get_tag() -> Tag {
        Tag::Long
    }
}

impl NBTDataType<Binary> for f32 {
    fn read<R: Read>(reader: &mut R) -> Result<Self, NBTError>
        where
            Self: Sized,
    {
        reader.read_f32::<BigEndian>().map_err(NBTError::IO)
    }

    fn write<W: Write, Name: AsRef<[u8]>>(
        self,
        name: Name,
        writer: &mut W,
    ) -> Result<(), NBTError> {
        Tag::Float.write_alone(writer)?;
        Binary::write_tag_name(writer, name)?;
        writer.write_f32::<BigEndian>(self).map_err(NBTError::IO)
    }

    fn write_alone<W: Write>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_f32::<BigEndian>(self).map_err(NBTError::IO)
    }

    fn get_tag() -> Tag {
        Tag::Float
    }
}

impl NBTDataType<Binary> for f64 {
    fn read<R: Read>(reader: &mut R) -> Result<Self, NBTError>
        where
            Self: Sized,
    {
        reader.read_f64::<BigEndian>().map_err(NBTError::IO)
    }

    fn write<W: Write, Name: AsRef<[u8]>>(
        self,
        name: Name,
        writer: &mut W,
    ) -> Result<(), NBTError> {
        Tag::Double.write_alone(writer)?;
        Binary::write_tag_name(writer, name)?;
        writer.write_f64::<BigEndian>(self).map_err(NBTError::IO)
    }

    fn write_alone<W: Write>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_f64::<BigEndian>(self).map_err(NBTError::IO)
    }

    fn get_tag() -> Tag {
        Tag::Double
    }
}

/// usize is treated as i32
impl NBTDataType<Binary> for usize {
    fn read<R: Read>(reader: &mut R) -> Result<Self, NBTError>
        where
            Self: Sized,
    {
        reader
            .read_i32::<BigEndian>()
            .map(|x| x as usize)
            .map_err(NBTError::IO)
    }

    fn write<W: Write, Name: AsRef<[u8]>>(
        self,
        name: Name,
        writer: &mut W,
    ) -> Result<(), NBTError> {
        Tag::Int.write_alone(writer)?;
        Binary::write_tag_name(writer, name)?;
        writer
            .write_i32::<BigEndian>(self as i32)
            .map_err(NBTError::IO)
    }

    fn write_alone<W: Write>(self, writer: &mut W) -> Result<(), NBTError> {
        writer
            .write_i32::<BigEndian>(self as i32)
            .map_err(NBTError::IO)
    }

    fn get_tag() -> Tag {
        Tag::Int
    }
}
