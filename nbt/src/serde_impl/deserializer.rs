use crate::serde_impl::{Error, SerdeReader};
use crate::sync::{NBTData, NBTReader};
use crate::Tag;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::{forward_to_deserialize_any, Deserializer};
use std::fmt::Debug;
use std::io::{BufRead, Read};
use std::mem;

pub struct NBTDeserializer<Reader: SerdeReader> {
    pub(crate) src: NBTReader<Reader>,
}

impl<'de, 'reader, Reader: SerdeReader> Deserializer<'de> for &'reader mut NBTDeserializer<Reader> {
    type Error = super::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        Err(Error::Custom("deserialize_any not implemented".to_string()))
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
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
        name: &'static str,
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
        let (tag, v) = self.src.read_tag_id_with_id_len()?;

        if Tag::Compound == tag {
            self.src.src.consume(v as usize);
            visitor.visit_map(CompoundMap {
                reader: &mut self.src,
                key: vec![],
                next_entry: None,
            })
        } else {
            return Err(Error::IncorrectTagError(Tag::Compound, tag));
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }
}

pub struct CompoundMap<'reader, Reader: SerdeReader> {
    pub reader: &'reader mut NBTReader<Reader>,
    pub key: Vec<u8>,
    pub next_entry: Option<Tag>,
}

impl<'de, 'reader, Reader: SerdeReader> MapAccess<'de>
for CompoundMap<'reader, Reader>
{
    type Error = super::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
    {
        let tag = Tag::read_from(&mut self.reader.src)?;
        if Tag::End == tag {
            return Ok(None);
        }
        let length = u16::read_from(&mut self.reader.src)?;
        self.key.clear();
        (&mut self.reader.src).take(length as u64).read_to_end(&mut self.key)?;

        self.next_entry = Some(tag);
        let mut inner = NameDeserializer {
            content: &mut self.key
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
                let mut inner = InnerDeserializer {
                    reader: &mut self.reader,
                    tag: value,
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
    fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        visitor.visit_str(std::str::from_utf8(self.content).unwrap())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        visitor.visit_string(String::from_utf8(mem::take(self.content))?)
    }
}


pub struct InnerDeserializer<'reader, Reader: SerdeReader> {
    pub reader: &'reader mut NBTReader<Reader>,
    pub tag: Tag,
}

impl<'de, 'reader, Reader: SerdeReader> Deserializer<'de> for InnerDeserializer<'reader, Reader> {
    type Error = super::Error;
    forward_to_deserialize_any! {
        u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string bytes byte_buf seq
        map tuple_struct struct tuple enum identifier ignored_any
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        match self.tag {
            Tag::Byte => {
                let value = self.reader.read_byte()?;
                visitor.visit_i8(value)
            }
            Tag::Short => {
                let value = self.reader.read_short()?;
                visitor.visit_i16(value)
            }
            Tag::Int => {
                let value = self.reader.read_int()?;
                visitor.visit_i32(value)
            }
            Tag::Long => {
                let value = self.reader.read_long()?;
                visitor.visit_i64(value)
            }
            Tag::Float => {
                let value = self.reader.read_float()?;
                visitor.visit_f32(value)
            }
            Tag::Double => {
                let value = self.reader.read_double()?;
                visitor.visit_f64(value)
            }
            Tag::ByteArray => {
                let length = self.reader.read_int()?;
                let deserializer = SequenceDeserializer {
                    reader: self.reader,
                    tag: Tag::Byte,
                    len: length as usize,
                    current: 0,
                };
                visitor.visit_seq(deserializer)
            }
            Tag::String => {
                let len = self.reader.read_string_len()?;
                let value = self.reader.read_str_as_bytes(len)?;
                let value = String::from_utf8(value)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                visitor.visit_string(value)
            }
            Tag::List => {
                let (tag, length) = self.reader.read_list_type_and_size()?;
                let deserializer = SequenceDeserializer {
                    reader: self.reader,
                    tag,
                    len: length as usize,
                    current: 0,
                };
                visitor.visit_seq(deserializer)
            }
            Tag::Compound => {
                let deserializer = CompoundMap {
                    reader: self.reader,
                    key: vec![],
                    next_entry: None,
                };
                visitor.visit_map(deserializer)
            }
            Tag::IntArray => {
                let length = self.reader.read_int()?;
                let deserializer = SequenceDeserializer {
                    reader: self.reader,
                    tag: Tag::Int,
                    len: length as usize,
                    current: 0,
                };
                visitor.visit_seq(deserializer)
            }
            Tag::LongArray => {
                let length = self.reader.read_int()?;
                let deserializer = SequenceDeserializer {
                    reader: self.reader,
                    tag: Tag::Long,
                    len: length as usize,
                    current: 0,
                };
                visitor.visit_seq(deserializer)
            }
            t => {
                return Err(Error::Custom(format!(
                    "deserialize_any not implemented for {:?}",
                    t
                )));
            }
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        match self.tag {
            Tag::Byte => {
                let value = self.reader.read_byte()?;
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
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }
}

pub struct SequenceDeserializer<'reader, Reader: SerdeReader> {
    pub reader: &'reader mut NBTReader<Reader>,
    pub tag: Tag,
    pub len: usize,
    pub current: usize,
}

impl<'de, 'reader, Reader: SerdeReader> SeqAccess<'de> for SequenceDeserializer<'reader, Reader> {
    type Error = super::Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
    {
        if self.current == self.len {
            return Ok(None);
        }
        let mut de = InnerDeserializer {
            reader: self.reader,
            tag: self.tag,
        };
        let value = seed.deserialize(de)?;

        self.current += 1;

        Ok(Some(value))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len as usize)
    }
}
