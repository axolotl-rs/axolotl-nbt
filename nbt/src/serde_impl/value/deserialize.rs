use crate::serde_impl::Error;
use crate::value::{NameLessValue, Value};
use crate::Tag;
use serde::de::{DeserializeSeed, IntoDeserializer, MapAccess, SeqAccess, Visitor};
use serde::{forward_to_deserialize_any, Deserializer};

pub struct ValueDeserializer(pub Value);

macro_rules! impl_deserializer {
    ($name:ty, $self:ident, $visitor:ident, $map:block) => {
        impl<'de> Deserializer<'de> for $name {
            type Error = Error;
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

            serde::forward_to_deserialize_any! {
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

            fn deserialize_map<V>($self, $visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            $map
        }
    };
}
impl_deserializer!(ValueDeserializer, self, visitor, {
    match self.0 {
        Value::Compound { value, .. } => {
            let map = CompoundMap {
                value,
                next_value: None,
            };
            visitor.visit_map(map)
        }
        v => Err(Error::IncorrectTagError(Tag::Compound, v.tag())),
    }
});

pub struct NamelessValueDeserializer(pub NameLessValue);
impl_deserializer!(NamelessValueDeserializer, self, visitor, {
    match self.0 {
        NameLessValue::Compound(value) => {
            let map = CompoundMap {
                value,
                next_value: None,
            };
            visitor.visit_map(map)
        }
        v => Err(Error::IncorrectTagError(Tag::Compound, v.tag())),
    }
});

pub struct CompoundMap {
    pub value: Vec<Value>,
    pub next_value: Option<NameLessValue>,
}

impl<'de> MapAccess<'de> for CompoundMap {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
    {
        let key = match &self.next_value {
            None => {
                if let Some(v) = self.value.pop() {
                    let (value, name): (NameLessValue, String) = v.into();
                    self.next_value = Some(value);
                    name
                } else {
                    return Ok(None);
                }
            }
            Some(_) => {
                return Err(Error::Custom(
                    "next_key_seed called when next_value is not None".to_string(),
                ));
            }
        };
        seed.deserialize(key.into_deserializer()).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
    {
        let value = self.next_value.take();
        match value {
            None => Err(Error::Custom("No value found".to_string())),
            Some(v) => seed.deserialize(InnerValueDeserializer(v)),
        }
    }
}

struct InnerValueDeserializer(NameLessValue);

impl<'de> Deserializer<'de> for InnerValueDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        match self.0 {
            NameLessValue::End => Err(Error::Custom("End tag found".to_string())),
            NameLessValue::Byte(v) => visitor.visit_i8(v),
            NameLessValue::Short(v) => visitor.visit_i16(v),
            NameLessValue::Int(v) => visitor.visit_i32(v),
            NameLessValue::Long(v) => visitor.visit_i64(v),
            NameLessValue::Float(v) => visitor.visit_f32(v),
            NameLessValue::Double(v) => visitor.visit_f64(v),
            NameLessValue::ByteArray(v) => visitor.visit_seq(SequenceDeserializer(v)),
            NameLessValue::String(v) => visitor.visit_string(v),
            NameLessValue::List(v) => visitor.visit_seq(SequenceDeserializer(v)),
            NameLessValue::Compound(v) => visitor.visit_map(CompoundMap {
                value: v,
                next_value: None,
            }),
            NameLessValue::IntArray(v) => visitor.visit_seq(SequenceDeserializer(v)),
            NameLessValue::LongArray(v) => visitor.visit_seq(SequenceDeserializer(v)),
            NameLessValue::Boolean(v) => visitor.visit_bool(v),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        if let NameLessValue::Byte(value) = self.0 {
            visitor.visit_bool(value != 0)
        } else if let NameLessValue::Boolean(value) = self.0 {
            visitor.visit_bool(value)
        } else {
            Err(Error::IncorrectTagError(Tag::Byte, self.0.tag()))
        }
    }

    forward_to_deserialize_any! {
        u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string bytes byte_buf seq
        map tuple_struct struct tuple enum identifier ignored_any
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

impl NameLessValue {
    fn into_deserializer(self) -> InnerValueDeserializer {
        InnerValueDeserializer(self)
    }
}

struct SequenceDeserializer<Value>(Vec<Value>);

macro_rules! define_seq_access {
    ($p:ty) => {
        impl<'de> SeqAccess<'de> for SequenceDeserializer<$p> {
            type Error = Error;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
            where
                T: DeserializeSeed<'de>,
            {
                match self.0.pop() {
                    None => Ok(None),
                    Some(v) => seed.deserialize(v.into_deserializer()).map(Some),
                }
            }
        }
    };
}
define_seq_access!(i8);
define_seq_access!(i32);
define_seq_access!(i64);
define_seq_access!(NameLessValue);
