use std::fmt::Debug;
use std::marker::PhantomData;

pub mod sync;
#[cfg(feature = "tokio")]
pub mod tokio_impl;
#[cfg(feature = "value")]
pub mod value;

#[derive(PartialEq, Debug, Clone)]
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
}

pub trait NBTFormat {}

pub struct Binary;

impl NBTFormat for Binary {}

#[derive(Debug)]
pub struct NBTReader<Type: NBTFormat, Src: Debug> {
    src: Src,
    phantom: PhantomData<Type>,
}

impl<Type: NBTFormat, Src: Debug> NBTReader<Type, Src> {
    pub fn new(src: Src) -> Self {
        NBTReader {
            src,
            phantom: PhantomData,
        }
    }
    pub fn into_inner(self) -> Src {
        self.src
    }
}
