use crate::serde_impl::deserializer::NBTDeserializer;
use crate::serde_impl::serialize::NBTSerializer;
use crate::sync::NBTWriter;
use crate::{NBTError, Tag};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::fmt::{Debug, Display};
use std::io::{BufRead, Read, Write};
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

pub trait SerdeReader: Read + BufRead + ReadBytesExt + Debug {}

impl<T: Read + BufRead + ReadBytesExt + Debug> SerdeReader for T {}

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
