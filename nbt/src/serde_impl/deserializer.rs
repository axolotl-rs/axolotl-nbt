use crate::serde_impl::Error;
use crate::{NBTDataType, NBTType, Tag};

use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::{forward_to_deserialize_any, Deserializer};

use std::io::{BufRead, Read};
use std::mem;

pub struct NBTDeserializer<Reader: Read + BufRead, Type: NBTType> {
    pub(crate) src: Reader,
    pub(crate) phantom: std::marker::PhantomData<Type>,
}

impl<'de, 'reader, Reader: Read + BufRead, Type: NBTType> Deserializer<'de>
    for &'reader mut NBTDeserializer<Reader, Type>
{
    type Error = super::Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("deserialize_any not implemented".to_string()))
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string bytes byte_buf
        unit seq tuple_struct tuple option enum identifier ignored_any
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (_, tag) = Tag::read_with_name(&mut self.src)?;

        if Tag::Compound == tag {
            visitor.visit_map(CompoundMap::<'reader, Reader, Type> {
                reader: &mut self.src,
                key: vec![],
                next_entry: None,
                phantom: Default::default(),
            })
        } else {
            Err(Error::IncorrectTagError(Tag::Compound, tag))
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }
}

pub struct CompoundMap<'reader, Reader: Read + BufRead, Type: NBTType> {
    pub reader: &'reader mut Reader,
    pub key: Vec<u8>,
    pub next_entry: Option<Tag>,
    pub phantom: std::marker::PhantomData<Type>,
}

impl<'de, 'reader, Reader: Read + BufRead, Type: NBTType> MapAccess<'de>
    for CompoundMap<'reader, Reader, Type>
{
    type Error = super::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        let tag = Tag::read(&mut self.reader)?;
        if Tag::End == tag {
            return Ok(None);
        }
        self.key.clear();
        Type::read_tag_name_raw(&mut self.reader, &mut self.key)?;
        self.next_entry = Some(tag);
        let inner = NameDeserializer {
            content: &mut self.key,
        };
        seed.deserialize(inner).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.next_entry.take() {
            None => Err(Error::Custom(
                "next_value_seed called before next_key_seed".to_string(),
            )),
            Some(value) => {
                let inner = InnerDeserializer::<'_, Reader, Type> {
                    reader: self.reader,
                    tag: value,
                    phantom: Default::default(),
                };
                seed.deserialize(inner)
            }
        }
    }
}

pub struct NameDeserializer<'string> {
    pub content: &'string mut Vec<u8>,
}

impl<'de, 'string> Deserializer<'de> for NameDeserializer<'string> {
    type Error = super::Error;

    forward_to_deserialize_any! {
        u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char bytes byte_buf seq
        map tuple_struct struct tuple enum identifier ignored_any option unit newtype_struct bool unit_struct
    }
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(self.content)
    }
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(std::str::from_utf8(self.content).unwrap())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(String::from_utf8(mem::take(self.content))?)
    }
}

pub struct InnerDeserializer<'reader, Reader: Read + BufRead, Type: NBTType> {
    pub reader: &'reader mut Reader,
    pub tag: Tag,
    pub phantom: std::marker::PhantomData<Type>,
}

impl<'de, 'reader, Reader: Read + BufRead, Type: NBTType> Deserializer<'de>
    for InnerDeserializer<'reader, Reader, Type>
{
    type Error = super::Error;
    forward_to_deserialize_any! {
        u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string bytes byte_buf seq
        map tuple_struct struct tuple enum identifier ignored_any
    }

    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.tag {
            Tag::Byte => visitor.visit_i8(i8::read(&mut self.reader)?),
            Tag::Short => visitor.visit_i16(i16::read(&mut self.reader)?),
            Tag::Int => visitor.visit_i32(i32::read(&mut self.reader)?),
            Tag::Long => visitor.visit_i64(i64::read(&mut self.reader)?),
            Tag::Float => visitor.visit_f32(f32::read(&mut self.reader)?),
            Tag::Double => visitor.visit_f64(f64::read(&mut self.reader)?),
            Tag::ByteArray => {
                let length = i32::read(&mut self.reader)?;
                let deserializer = SequenceDeserializer::<'reader, Reader, Type> {
                    reader: self.reader,
                    tag: Tag::Byte,
                    len: length as usize,
                    current: 0,
                    phantom: Default::default(),
                };
                visitor.visit_seq(deserializer)
            }
            Tag::String => visitor.visit_string(String::read(&mut self.reader)?),
            Tag::List => {
                let tag = Tag::read(&mut self.reader)?;
                let length = i32::read(&mut self.reader)?;
                let deserializer = SequenceDeserializer::<'reader, Reader, Type> {
                    reader: self.reader,
                    tag,
                    len: length as usize,
                    current: 0,
                    phantom: Default::default(),
                };
                visitor.visit_seq(deserializer)
            }
            Tag::Compound => {
                let deserializer = CompoundMap::<'reader, Reader, Type> {
                    reader: self.reader,
                    key: vec![],
                    next_entry: None,
                    phantom: Default::default(),
                };
                visitor.visit_map(deserializer)
            }
            Tag::IntArray => {
                let length = i32::read(&mut self.reader)?;
                let deserializer = SequenceDeserializer::<'reader, Reader, Type> {
                    reader: self.reader,
                    tag: Tag::Int,
                    len: length as usize,
                    current: 0,
                    phantom: Default::default(),
                };
                visitor.visit_seq(deserializer)
            }
            Tag::LongArray => {
                let length = i32::read(&mut self.reader)?;
                let deserializer = SequenceDeserializer::<'reader, Reader, Type> {
                    reader: self.reader,
                    tag: Tag::Long,
                    len: length as usize,
                    current: 0,
                    phantom: Default::default(),
                };
                visitor.visit_seq(deserializer)
            }
            t => Err(Error::Custom(format!(
                "deserialize_any not implemented for {:?}",
                t
            ))),
        }
    }

    fn deserialize_bool<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.tag {
            Tag::Byte => {
                let value = i8::read(&mut self.reader)?;
                match value {
                    0 => visitor.visit_bool(false),
                    1 => visitor.visit_bool(true),
                    b => Err(Error::Custom(format!("Invalid byte value for bool: {}", b))),
                }
            }
            _ => Err(Error::IncorrectTagError(self.tag, Tag::Byte)),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }
}

pub struct SequenceDeserializer<'reader, Reader: Read + BufRead, Type: NBTType> {
    pub reader: &'reader mut Reader,
    pub tag: Tag,
    pub len: usize,
    pub current: usize,
    pub phantom: std::marker::PhantomData<Type>,
}

impl<'de, 'reader, Reader: Read + BufRead, Type: NBTType> SeqAccess<'de>
    for SequenceDeserializer<'reader, Reader, Type>
{
    type Error = super::Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.current == self.len {
            return Ok(None);
        }
        let de = InnerDeserializer::<'_, Reader, Type> {
            reader: self.reader,
            tag: self.tag,
            phantom: Default::default(),
        };
        let value = seed.deserialize(de)?;

        self.current += 1;

        Ok(Some(value))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len as usize)
    }
}
