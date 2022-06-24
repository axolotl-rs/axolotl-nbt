use std::marker::PhantomData;


pub mod value;
mod tokio_impl;
#[derive(PartialEq)]
pub enum Tag {
    End,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    List,
    Compound,
    IntArray,
    LongArray,
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

pub struct NBTReader<Type: NBTFormat, Src> {
    src: Src,
    phantom: PhantomData<(Type)>,
}
