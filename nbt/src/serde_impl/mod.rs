

use crate::serde_impl::deserializer::NBTDeserializer;
use crate::{NBTError, NBTType, Tag};

use std::fmt::{Debug, Display};
use std::io::{BufRead, BufReader, Read, Write};
use std::string::FromUtf8Error;
use thiserror::Error;

mod deserializer;
mod serialize;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Expected tag {0:?} got {1:?}")]
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
/*
pub fn to_writer<Type: NBTType, W: Write, T: serde::Serialize>(
    writer: &mut W,
    value: &T,
) -> Result<(), Error> {
    let mut ser: NBTSerializer<'_, W, Type> = NBTSerializer {
        writer,
        phantom: Default::default(),
    };
    value.serialize(ser)
}*/

pub fn from_reader<'de, Type: NBTType, R: Read + Debug, T: serde::Deserialize<'de>>(
    reader: R,
) -> Result<T, Error> {
    let mut der = NBTDeserializer::<BufReader<R>, Type> {
        src: BufReader::new(reader),
        phantom: Default::default(),
    };
    T::deserialize(&mut der)
}

pub fn from_buf_reader<
    'de,
    Type: NBTType,
    R: Read + BufRead + Debug,
    T: serde::Deserialize<'de>,
>(
    reader: R,
) -> Result<T, Error> {
    let mut der = NBTDeserializer::<R, Type> {
        src: reader,
        phantom: Default::default(),
    };
    T::deserialize(&mut der)
}
