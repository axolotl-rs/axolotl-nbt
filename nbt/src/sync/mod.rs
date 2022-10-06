use byteorder::{ReadBytesExt, WriteBytesExt};
use std::fmt::Debug;

use crate::{NBTError, Tag};
use byteorder::BigEndian;
use std::hash::Hasher;
use std::io::{Read, Write};

pub mod read;
#[cfg(feature = "value")]
pub mod value_read;
#[cfg(feature = "value")]
pub mod value_write;
pub mod write;

#[derive(Debug)]
pub struct NBTReader<Src: Debug> {
    pub(crate) src: Src,
}

impl<Src: Debug> NBTReader<Src> {
    pub fn new(src: Src) -> Self {
        NBTReader { src }
    }
    pub fn into_inner(self) -> Src {
        self.src
    }
}

#[derive(Debug)]
pub struct NBTWriter<Target: Debug> {
    pub(crate) target: Target,
}

impl<Target: Debug> NBTWriter<Target> {
    pub fn new(target: Target) -> Self {
        NBTWriter { target }
    }
    pub fn into_inner(self) -> Target {
        self.target
    }
}

pub trait NBTData {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized;
    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), NBTError>;
    fn tag() -> Tag;
}

impl NBTData for Tag {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        let tag = reader.read_u8()? as i8;
        Tag::from_i8(tag).ok_or(NBTError::InvalidTag(tag))
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_i8(self as i8)?;
        Ok(())
    }

    fn tag() -> Tag {
        Tag::Byte
    }
}

impl NBTData for &str {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        unimplemented!("Reading read to str ref is not supported")
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_u16::<BigEndian>(self.len() as u16)?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }

    fn tag() -> Tag {
        Tag::String
    }
}

impl NBTData for &[u8] {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        unimplemented!("Reading from a string is not supported")
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_u16::<BigEndian>(self.len() as u16)?;
        writer.write_all(self)?;
        Ok(())
    }

    fn tag() -> Tag {
        Tag::ByteArray
    }
}

impl NBTData for String {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        let len = i16::read_from(reader)?;
        let mut buf = vec![0; len as usize];
        reader.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf).map_err(|v| NBTError::NotAString(v))?)
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_u16::<BigEndian>(self.len() as u16)?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }

    fn tag() -> Tag {
        Tag::String
    }
}

impl NBTData for i8 {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, NBTError> {
        reader.read_i8().map_err(NBTError::IO)
    }
    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_i8(self).map_err(NBTError::IO)
    }

    fn tag() -> Tag {
        Tag::Byte
    }
}

impl NBTData for u16 {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        reader.read_u16::<BigEndian>().map_err(NBTError::IO)
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_u16::<BigEndian>(self).map_err(NBTError::IO)
    }

    fn tag() -> Tag {
        Tag::Short
    }
}

impl NBTData for i16 {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        reader.read_i16::<BigEndian>().map_err(NBTError::IO)
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_i16::<BigEndian>(self).map_err(NBTError::IO)
    }

    fn tag() -> Tag {
        Tag::Short
    }
}

impl NBTData for i32 {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        reader.read_i32::<BigEndian>().map_err(NBTError::IO)
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_i32::<BigEndian>(self).map_err(NBTError::IO)
    }

    fn tag() -> Tag {
        Tag::Int
    }
}

impl NBTData for i64 {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        reader.read_i64::<BigEndian>().map_err(NBTError::IO)
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_i64::<BigEndian>(self).map_err(NBTError::IO)
    }

    fn tag() -> Tag {
        Tag::Long
    }
}

impl NBTData for f32 {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        reader.read_f32::<BigEndian>().map_err(NBTError::IO)
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_f32::<BigEndian>(self).map_err(NBTError::IO)
    }

    fn tag() -> Tag {
        Tag::Float
    }
}

impl NBTData for f64 {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        reader.read_f64::<BigEndian>().map_err(NBTError::IO)
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), NBTError> {
        writer.write_f64::<BigEndian>(self).map_err(NBTError::IO)
    }

    fn tag() -> Tag {
        Tag::Float
    }
}

impl<Data: NBTData> NBTData for (String, Data) {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        let name = String::read_from(reader)?;
        let data = Data::read_from(reader)?;
        Ok((name, data))
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), NBTError> {
        self.0.write_to(writer)?;
        self.1.write_to(writer)?;
        Ok(())
    }

    fn tag() -> Tag {
        Data::tag()
    }
}

impl NBTData for bool {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        let val = i8::read_from(reader)?;
        Ok(val != 0)
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), NBTError> {
        (if self { 1i8 } else { 0i8 }).write_to(writer)
    }

    fn tag() -> Tag {
        Tag::Byte
    }
}

/// usize is treated as i32
impl NBTData for usize {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        reader
            .read_i32::<BigEndian>()
            .map(|x| x as usize)
            .map_err(NBTError::IO)
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), NBTError> {
        writer
            .write_i32::<BigEndian>(self as i32)
            .map_err(NBTError::IO)
    }

    fn tag() -> Tag {
        Tag::Int
    }
}
