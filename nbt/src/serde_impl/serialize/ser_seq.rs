use crate::serde_impl::{Error, SerdeWriter};
use crate::{NBTData, NBTWriter, Tag};
use serde::{ser, Serialize};
use std::borrow::Cow;
use std::mem;
use crate::serde_impl::serialize::{cast_and_write, Compound, SimpleWrite};
use crate::serde_impl::serialize::named::{GetName, NamedValueSerializer, StringOrSerializer};
use crate::serde_impl::serialize::macros::{gen_method_body, impossible, method_body};

pub struct SerializeSeq<'writer, 'name: 'writer, W: SerdeWriter, K: Serialize + ?Sized> {
    pub(crate) outer: &'writer mut NBTWriter<W>,
    pub(crate) name: &'name mut StringOrSerializer<'name, K>,
    pub(crate) wrote_header: bool,
    pub(crate) length: i32,

}


impl<'writer, 'name: 'writer, W: SerdeWriter, K: Serialize + ?Sized> ser::SerializeSeq for SerializeSeq<'writer, 'name, W, K> {
    type Ok = ();
    type Error = super::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
    {
        if !self.wrote_header {
            let mut inner = SerializeSeqInner { outer: self.outer, name: self.name };
            value.serialize(&mut inner)?;
            self.wrote_header = true;
            *self.name = StringOrSerializer::None;
        } else {
            let inner = SimpleWrite { writer: self.outer };
            value.serialize(inner)?;
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct SerializeSeqInner<'writer, 'name: 'writer, W: SerdeWriter, K: Serialize + ?Sized> {
    pub(crate) outer: &'writer mut NBTWriter<W>,
    pub(crate) name: &'name StringOrSerializer<'name, K>,
}

impl<'writer, 'name: 'writer, W: SerdeWriter, K: Serialize + ?Sized> SerializeSeqInner<'writer, 'name, W, K> {
    #[inline]
    pub fn write<Data: NBTData>(&mut self, data: Data) -> Result<(), Error> {
        match Data::tag() {
            Tag::Byte => {
                self.outer.write_tag_id(Tag::ByteArray)?;
                self.write_name()?;
            }
            Tag::Int => {
                self.outer.write_tag_id(Tag::ByteArray)?;
                self.write_name()?;
            }
            Tag::Long => {
                self.outer.write_tag_id(Tag::ByteArray)?;
                self.write_name()?;
            }
            v => {
                Tag::List.write_to(&mut self.outer.target)?;
                self.write_name()?;
                v.write_to(&mut self.outer.target)?;
            }
        }
        data.write_to(&mut self.outer.target)?;

        Ok(())
    }
    fn write_name(&mut self) -> Result<(), Error> {
        match &self.name {
            StringOrSerializer::String(name) => {
                name.as_ref().write_to(&mut self.outer.target)?;
            }
            StringOrSerializer::Serializer(name) => {
                let name_getter = GetName {
                    target: self.outer,
                };
                name.serialize(name_getter)?;
            }
            _ => {}
        }
        Ok(())
    }
}


impl<'writer, 'name: 'writer, W: SerdeWriter, K: Serialize + ?Sized> ser::Serializer for &'writer mut SerializeSeqInner<'writer, 'name, W, K> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SerializeSeq<'writer, 'name, W, K>;
    type SerializeTuple = ser::Impossible<(), Self::Error>;
    type SerializeTupleStruct = ser::Impossible<(), Self::Error>;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
    type SerializeMap = Compound<'writer, W>;
    type SerializeStruct = Compound<'writer, W>;
    type SerializeStructVariant = ser::Impossible<(), Self::Error>;



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
        unit_variant);

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
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }
}
