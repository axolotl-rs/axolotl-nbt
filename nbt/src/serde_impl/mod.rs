use crate::Tag;
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

#[cfg(test)]
pub mod tests {
    use std::path::Path;
    use serde::{Deserialize, Serialize};
    use crate::{NBTReader, NBTWriter};
    use crate::serde_impl::serialize::NBTSerializer;

    #[derive(Serialize, Deserialize)]
    pub struct SimplePlayer {
        level: i32,
        name: String,
        experience: f32,

    }

    #[test]
    pub fn test_write() {
        let player = SimplePlayer {
            level: 1,
            name: "KingTux".to_string(),
            experience: 0.0
        };
        let path = Path::new("test2.nbt");
        let file = if path.exists() {
            std::fs::remove_file(path).unwrap();
            std::fs::File::create(path).unwrap()
        } else {
            std::fs::File::create(path).unwrap()
        };
        let mut writer = NBTWriter::new(file);
        let serializer = NBTSerializer { writer: &mut writer };
        player.serialize(serializer).unwrap();

        let mut reader = NBTReader::new(std::fs::File::open(path).unwrap());
        println!("{:?}", reader.read_value());
    }
}