use std::fmt::{Debug, Display, Formatter};

mod error;
#[cfg(feature = "serde")]
pub mod serde_impl;
pub mod sync;
#[cfg(feature = "value")]
pub mod value;

pub use error::NBTError;

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
