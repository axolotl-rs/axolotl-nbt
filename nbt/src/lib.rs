use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt::{Debug, Display, Formatter};
use std::io::{Error, Read, Write};
use std::marker::PhantomData;

#[cfg(feature = "serde")]
pub mod serde_impl;
pub mod sync;
#[cfg(feature = "async_io")]
pub mod tokio_impl;
#[cfg(feature = "value")]
pub mod value;

#[repr(i8)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Tag {
    End = 0,
    Byte = 1,
    Short = 2,
    Int = 3,
    Long = 4,
    Float = 5,
    Double = 6,
    ByteArray = 7,
    String = 8,
    List = 9,
    Compound = 10,
    IntArray = 11,
    LongArray = 12,
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Tag {
    pub fn from_u8(value: u8) -> Option<Self> {
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
    pub fn from_i8(value: i8) -> Option<Self> {
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
}

impl NBTData for Tag {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Tag::from_i8(i8::read_from(reader)?)
            .ok_or_else(|| Error::new(std::io::ErrorKind::InvalidData, "Invalid tag id"))
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), Error> {
        writer.write_u8(self as u8)
    }
    /// Technically, this is not a valid NBT tag
    fn tag() -> Tag {
        Tag::Byte
    }
}

#[derive(Debug)]
pub struct NBTReader<Src: Debug> {
    src: Src,
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
    target: Target,
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
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, std::io::Error>
    where
        Self: Sized;
    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), std::io::Error>;
    fn tag() -> Tag;
}

impl NBTData for &str {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        unimplemented!("Reading from a string is not supported")
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), Error> {
        writer.write_u16::<BigEndian>(self.len() as u16)?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }

    fn tag() -> Tag {
        Tag::String
    }
}

impl NBTData for &[u8] {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        unimplemented!("Reading from a string is not supported")
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), Error> {
        writer.write_u16::<BigEndian>(self.len() as u16)?;
        writer.write_all(self)?;
        Ok(())
    }

    fn tag() -> Tag {
        Tag::ByteArray
    }
}

impl NBTData for String {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let len = i16::read_from(reader)?;
        let mut buf = vec![0; len as usize];
        reader.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf)
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "Invalid utf8 string"))?)
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), Error> {
        writer.write_u16::<BigEndian>(self.len() as u16)?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }

    fn tag() -> Tag {
        Tag::String
    }
}

impl NBTData for i8 {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, std::io::Error> {
        reader.read_i8()
    }
    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_i8(self)
    }

    fn tag() -> Tag {
        Tag::Byte
    }
}

impl NBTData for u16 {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        reader.read_u16::<BigEndian>()
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), Error> {
        writer.write_u16::<BigEndian>(self)
    }

    fn tag() -> Tag {
        Tag::Short
    }
}

impl NBTData for i16 {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        reader.read_i16::<BigEndian>()
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), Error> {
        writer.write_i16::<BigEndian>(self)
    }

    fn tag() -> Tag {
        Tag::Short
    }
}

impl NBTData for i32 {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        reader.read_i32::<BigEndian>()
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), Error> {
        writer.write_i32::<BigEndian>(self)
    }

    fn tag() -> Tag {
        Tag::Int
    }
}

impl NBTData for i64 {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        reader.read_i64::<BigEndian>()
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), Error> {
        writer.write_i64::<BigEndian>(self)
    }

    fn tag() -> Tag {
        Tag::Long
    }
}

impl NBTData for f32 {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        reader.read_f32::<BigEndian>()
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), Error> {
        writer.write_f32::<BigEndian>(self)
    }

    fn tag() -> Tag {
        Tag::Float
    }
}

impl NBTData for f64 {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        reader.read_f64::<BigEndian>()
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), Error> {
        writer.write_f64::<BigEndian>(self)
    }

    fn tag() -> Tag {
        Tag::Float
    }
}

impl<Data: NBTData> NBTData for (String, Data) {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let name = String::read_from(reader)?;
        let data = Data::read_from(reader)?;
        Ok((name, data))
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), Error> {
        self.0.write_to(writer)?;
        self.1.write_to(writer)?;
        Ok(())
    }

    fn tag() -> Tag {
        Data::tag()
    }
}

impl NBTData for bool {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let val = i8::read_from(reader)?;
        Ok(val != 0)
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), Error> {
        (if self { 1i8 } else { 0i8 }).write_to(writer)
    }

    fn tag() -> Tag {
        Tag::Byte
    }
}

/// usize is treated as i32
impl NBTData for usize {
    fn read_from<R: Read + Debug>(reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        reader.read_i32::<BigEndian>().map(|x| x as usize)
    }

    fn write_to<W: Write + Debug>(self, writer: &mut W) -> Result<(), Error> {
        writer.write_i32::<BigEndian>(self as i32)
    }

    fn tag() -> Tag {
        Tag::Int
    }
}
