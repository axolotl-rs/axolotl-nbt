use crate::serde_impl::serialize::macros::{gen_method_body, impossible, method_body};
use crate::serde_impl::serialize::sequence::SerializeSeq;
use crate::serde_impl::serialize::{cast_and_write, Compound};
use crate::serde_impl::{Error, SerdeWriter};
use crate::sync::{NBTData, NBTWriter};
use crate::Tag;
use serde::{ser, Serialize, Serializer};
use std::borrow::Cow;

pub enum StringOrSerializer<'data, K: Serialize + ?Sized> {
    String(Cow<'data, [u8]>),
    Serializer(&'data K),
    None,
}

pub struct NamedValueSerializer<'writer, 'name: 'writer, W: SerdeWriter, K: Serialize + ?Sized> {
    pub(crate) target: &'writer mut NBTWriter<W>,
    pub(crate) name: StringOrSerializer<'name, K>,
}

impl<'writer, 'name: 'writer, W: SerdeWriter, K: Serialize + ?Sized>
    NamedValueSerializer<'writer, 'name, W, K>
where
    'writer: 'name,
{
    #[inline]
    pub fn write<Data: NBTData>(&mut self, value: Data) -> Result<(), Error> {
        match &self.name {
            StringOrSerializer::String(name) => {
                self.target.write_tag(name.as_ref(), value)?;
            }
            StringOrSerializer::Serializer(name) => {
                Data::tag().write_to(&mut self.target.target)?;
                let name_getter = GetName {
                    target: self.target,
                };
                name.serialize(name_getter)?;
                value.write_to(&mut self.target.target)?;
            }
            _ => {
                Data::tag().write_to(&mut self.target.target)?;
                0u16.write_to(&mut self.target.target)?;
                value.write_to(&mut self.target.target)?;
            }
        }
        Ok(())
    }
    #[inline]
    pub fn write_tag(&mut self, value: Tag) -> Result<(), Error> {
        match &self.name {
            StringOrSerializer::String(name) => {
                value.write_to(&mut self.target.target)?;
                name.as_ref().write_to(&mut self.target.target)?;
            }
            StringOrSerializer::Serializer(name) => {
                value.write_to(&mut self.target.target)?;
                let name_getter = GetName {
                    target: self.target,
                };
                name.serialize(name_getter)?;
            }
            _ => {
                value.write_to(&mut self.target.target)?;
                0u16.write_to(&mut self.target.target)?;
            }
        }
        Ok(())
    }
}

impl<'writer, 'name: 'writer, W: SerdeWriter, K: Serialize + ?Sized> Serializer
    for &'writer mut NamedValueSerializer<'writer, 'name, W, K>
where
    'writer: 'name,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SerializeSeq<'writer, 'name, W, K>;
    type SerializeTuple = ser::Impossible<(), Self::Error>;
    type SerializeTupleStruct = ser::Impossible<(), Self::Error>;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
    type SerializeMap = Compound<'writer, W>;
    type SerializeStruct = Compound<'writer, W>;
    type SerializeStructVariant = ser::Impossible<(), Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.write(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.write(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.write(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.write(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.write(v)
    }

    cast_and_write!();

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.write(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.write(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.write(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!("serialize_bytes")
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(len) = len {
            Ok(SerializeSeq {
                outer: self.target,
                name: &mut self.name,
                wrote_header: false,
                length: len as i32,
            })
        } else {
            Err(Error::UnrepresentableValueError(
                "Unrepresentable value: sequence with unknown length",
            ))
        }
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.write_tag(Tag::Compound)?;
        Ok(Compound {
            writer: self.target,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.write_tag(Tag::Compound)?;
        Ok(Compound {
            writer: self.target,
        })
    }

    impossible!(
        none,
        some,
        unit,
        unit_struct,
        newtype_struct,
        newtype_variant,
        tuple,
        tuple_struct,
        tuple_variant,
        struct_variant,
        unit_variant
    );
}

#[derive(Debug)]
pub struct GetName<'writer, W: SerdeWriter> {
    pub(crate) target: &'writer mut NBTWriter<W>,
}

impl<'writer, W: SerdeWriter> Serializer for GetName<'writer, W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        v.write_to(&mut self.target.target)?;
        Ok(())
    }
    impossible!(
        bool,
        i8,
        i16,
        i32,
        i64,
        u8,
        u16,
        u32,
        u64,
        none,
        some,
        f32,
        f64,
        char,
        bytes,
        unit,
        unit_struct,
        newtype_struct,
        newtype_variant,
        seq,
        tuple,
        tuple_struct,
        tuple_variant,
        struct_variant,
        unit_variant,
        map,
        struct
    );
}
