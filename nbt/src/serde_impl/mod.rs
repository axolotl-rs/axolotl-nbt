use crate::serde_impl::deserializer::NBTDeserializer;
use crate::serde_impl::serialize::NBTSerializer;
use crate::sync::{NBTReader, NBTWriter};
use crate::{NBTError, Tag};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, Read, Write};
use std::string::FromUtf8Error;
use thiserror::Error;

mod deserializer;
mod serialize;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Expected tag {0} got {1}")]
    IncorrectTagError(Tag, Tag),
    #[error("{0}")]
    Custom(String),
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    UnrepresentableValueError(&'static str),
    #[error("Key must be a string")]
    KeyMustBeString,
    #[error(transparent)]
    NBTErr(#[from] NBTError),
    #[error(transparent)]
    FromStrError(#[from] FromUtf8Error),
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
        where
            T: Display,
    {
        Self::Custom(format!("{}", msg))
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
        where
            T: Display,
    {
        Self::Custom(format!("{}", msg))
    }
}

pub trait SerdeReader: Read + ReadBytesExt + Debug + BufRead {}

impl<T: Read + ReadBytesExt + Debug + BufRead> SerdeReader for T {}

pub trait SerdeWriter: Write + WriteBytesExt + Debug {}

impl<T: Write + WriteBytesExt + Debug> SerdeWriter for T {}

pub fn to_writer<W: SerdeWriter, T: serde::Serialize>(
    writer: &mut W,
    value: &T,
) -> Result<(), Error> {
    let mut nbt_writer = NBTWriter::new(writer);
    let mut ser = NBTSerializer {
        writer: &mut nbt_writer,
    };
    value.serialize(ser)
}

pub fn from_reader<'de, R: Read + Debug, T: serde::Deserialize<'de>>(
    reader: &mut R,
) -> Result<T, Error> {
    let mut nbt_writer = NBTReader::new(BufReader::new(reader));
    let mut der = NBTDeserializer {
        src: nbt_writer,
    };
    T::deserialize(&mut der)
}

pub fn from_buf_reader<'de, R: SerdeReader, T: serde::Deserialize<'de>>(
    reader: &mut BufReader<R>,
) -> Result<T, Error> {
    let mut nbt_writer = NBTReader::new(reader);
    let mut der = NBTDeserializer {
        src: nbt_writer,
    };
    T::deserialize(&mut der)
}
