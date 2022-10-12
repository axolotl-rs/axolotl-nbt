pub mod deserialize;

use crate::{NameLessValue, Value};
use serde::de::{DeserializeSeed, Error, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            Value::Compound { value, .. } => {
                let mut map = serializer.serialize_map(Some(value.len()))?;
                for v in value {
                    match v {
                        Value::Byte { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::Short { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::Int { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::Long { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::Float { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::Double { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::ByteArray { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::String { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::List { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::IntArray { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::LongArray { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::Boolean { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::End => {
                            // I think
                            continue;
                        }
                        v => {
                            let name = if let Value::Compound { name, .. } = v {
                                name.as_str()
                            } else {
                                ""
                            };
                            map.serialize_entry(name, v)?;
                        }
                    }
                }
                map.end()
            }
            _ => Err(serde::ser::Error::custom("Value is not a compound")),
        }
    }
}
macro_rules! visit_map {
    ($self:ident $ident:ident $ok:block) => {
        fn visit_map<A>($self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut $ident = Vec::with_capacity(map.size_hint().unwrap_or(0));
            let mut key = map.next_key::<String>()?;
            while let Some(value) = key {
                let value = map.next_value_seed(InnerValueDeserializer(value))?;
                $ident.push(value);
                key = map.next_key::<String>()?;
            }
            $ok
        }
    };
}
pub struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a compound")
    }
    visit_map!(self value {
        Ok(Value::Compound {
            name: String::new(),
            value,
        })
    });
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ValueVisitor)
    }
}

struct InnerValueDeserializer(String);

impl<'de> Visitor<'de> for InnerValueDeserializer {
    type Value = Value;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("A Value")
    }
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: Error,
    {
        Ok(Value::Boolean {
            name: self.0,
            value: v,
        })
    }
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(Value::Byte {
            name: self.0,
            value: v,
        })
    }
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(Value::Short {
            name: self.0,
            value: v,
        })
    }
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(Value::Int {
            name: self.0,
            value: v,
        })
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(Value::Long {
            name: self.0,
            value: v,
        })
    }
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(Value::Float {
            name: self.0,
            value: v,
        })
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(Value::Double {
            name: self.0,
            value: v,
        })
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(Value::String {
            name: self.0,
            value: v.to_string(),
        })
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(Value::String {
            name: self.0,
            value: v,
        })
    }
    visit_map!(self value {
        Ok(Value::Compound {
            name: self.0,
            value,
        })
    });
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
    {
        let mut values = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(value) = seq.next_element_seed(NamelessValueVisitor)? {
            values.push(value);
        }
        Ok(Value::List {
            name: self.0,
            value: values,
        })
    }
}

impl<'de> DeserializeSeed<'de> for InnerValueDeserializer {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
    {
        println!("Deserializing {}", self.0);
        deserializer.deserialize_any(self)
    }
}

pub struct NamelessValueVisitor;

impl<'de> Visitor<'de> for NamelessValueVisitor {
    type Value = NameLessValue;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("A valid NBT value")
    }
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(NameLessValue::Byte(v))
    }
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(NameLessValue::Short(v))
    }
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(NameLessValue::Int(v))
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(NameLessValue::Long(v))
    }
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(NameLessValue::Float(v))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(NameLessValue::Double(v))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(NameLessValue::String(v.to_string()))
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(NameLessValue::String(v))
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
    {
        let mut values = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(value) = seq.next_element_seed(NamelessValueVisitor)? {
            values.push(value);
        }
        Ok(NameLessValue::List(values))
    }
    visit_map!(self values {
        Ok(NameLessValue::Compound(values))
    });
}

impl<'de> DeserializeSeed<'de> for NamelessValueVisitor {
    type Value = NameLessValue;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}

impl<'de> Deserialize<'de> for NameLessValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NamelessValueVisitor)
    }
}

impl Serialize for NameLessValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            NameLessValue::Byte(v) => serializer.serialize_i8(*v),
            NameLessValue::Short(v) => serializer.serialize_i16(*v),
            NameLessValue::Int(v) => serializer.serialize_i32(*v),
            NameLessValue::Long(v) => serializer.serialize_i64(*v),
            NameLessValue::Float(v) => serializer.serialize_f32(*v),
            NameLessValue::Double(v) => serializer.serialize_f64(*v),
            NameLessValue::String(v) => serializer.serialize_str(v),
            NameLessValue::List(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for value in v {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
            NameLessValue::Compound(v) => {
                let mut map = serializer.serialize_map(Some(v.len()))?;
                for value in v {
                    match value {
                        Value::End => {}
                        Value::Byte { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::Short { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::Int { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::Long { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::Float { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::Double { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::ByteArray { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::String { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::List { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::Compound { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::IntArray { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::LongArray { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                        Value::Boolean { name, value } => {
                            map.serialize_entry(name, value)?;
                        }
                    }
                }
                map.end()
            }
            NameLessValue::ByteArray(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for value in v {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
            NameLessValue::IntArray(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for value in v {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
            NameLessValue::LongArray(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for value in v {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
            NameLessValue::Boolean(v) => serializer.serialize_bool(*v),
            _ => Err(serde::ser::Error::custom("Unsupported type")),
        }
    }
}
