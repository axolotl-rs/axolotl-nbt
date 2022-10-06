mod sub_list;

use crate::serde_impl::serialize::macros::{gen_method_body, impossible, method_body};
use crate::serde_impl::serialize::named::{GetName, StringOrSerializer};
use crate::serde_impl::serialize::{cast_and_write, Compound};
use crate::serde_impl::{Error, SerdeWriter};
use crate::sync::{NBTData, NBTWriter};
use crate::Tag;
use serde::{ser, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt::Debug;
use std::mem;

/// Order:
///   - List, ByteList, IntList, LongList
///   - If List:
///         Name, Tag, Length, Elements wrapped in Tag
///   - Else:
///         Name, Length, Elements just written
pub struct SerializeSeq<'writer, 'name: 'writer, W: SerdeWriter, K: Serialize + ?Sized> {
    pub(crate) outer: &'writer mut NBTWriter<W>,
    pub(crate) name: &'name mut StringOrSerializer<'name, K>,
    pub(crate) wrote_header: bool,
    pub(crate) length: i32,
}

impl<'writer, 'name: 'writer, W: SerdeWriter, K: Serialize + ?Sized> ser::SerializeSeq
for SerializeSeq<'writer, 'name, W, K>
{
    type Ok = ();
    type Error = super::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
    {
        if !self.wrote_header {
            let mut inner = SerializeSeqInner {
                outer: self.outer,
                name: self.name,
                length: self.length,
                wrote_header: false,
            };
            value.serialize(&mut inner)?;
            self.wrote_header = true;

            *self.name = StringOrSerializer::None;
        } else {
            let mut inner = SerializeSeqInner {
                outer: self.outer,
                name: self.name,
                length: self.length,
                wrote_header: true,
            };
            value.serialize(&mut inner)?;
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
    pub(crate) length: i32,
    pub(crate) wrote_header: bool,
}

impl<'writer, 'name: 'writer, W: SerdeWriter, K: Serialize + ?Sized>
SerializeSeqInner<'writer, 'name, W, K>
{
    #[inline]
    pub fn write<Data: NBTData + Debug>(&mut self, data: Data) -> Result<(), Error> {
        if !self.wrote_header {
            match Data::tag() {
                Tag::Byte => {
                    Tag::ByteArray.write_to(&mut self.outer.target)?;
                    self.write_name()?;
                    self.length.write_to(&mut self.outer.target)?;
                }
                Tag::Int => {
                    Tag::IntArray.write_to(&mut self.outer.target)?;
                    self.write_name()?;
                    self.length.write_to(&mut self.outer.target)?;
                }
                Tag::Long => {
                    Tag::LongArray.write_to(&mut self.outer.target)?;
                    self.write_name()?;
                    self.length.write_to(&mut self.outer.target)?;
                }
                v => {
                    Tag::List.write_to(&mut self.outer.target)?;
                    self.write_name()?;
                    v.write_to(&mut self.outer.target)?;
                    self.length.write_to(&mut self.outer.target)?;
                }
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
                let name_getter = GetName { target: self.outer };
                name.serialize(name_getter)?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl<'writer, 'name: 'writer, W: SerdeWriter, K: Serialize + ?Sized> ser::Serializer
for &'writer mut SerializeSeqInner<'writer, 'name, W, K>
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = sub_list::SubList<'writer, W>;
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
        unit_variant
    );

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

    fn serialize_seq(mut self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(len) = len {
            if !self.wrote_header {
                Tag::List.write_to(&mut self.outer.target)?;
                self.write_name()?;
            }
            let list = sub_list::SubList {
                outer: self.outer,
                wrote_header: false,
                wrote_parent_header: self.wrote_header,
                parent_size: self.length,
                length: len as i32,
            };
            Ok(list)
        } else {
            Err(Error::UnrepresentableValueError(
                "Cannot serialize a sequence with unknown length",
            ))
        }
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        if !self.wrote_header {
            Tag::List.write_to(&mut self.outer.target)?;
            self.write_name()?;
            Tag::Compound.write_to(&mut self.outer.target)?;
            self.length.write_to(&mut self.outer.target)?;
        }

        Ok(Compound {
            writer: &mut self.outer,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        if !self.wrote_header {
            Tag::List.write_to(&mut self.outer.target)?;
            self.write_name()?;
            Tag::Compound.write_to(&mut self.outer.target)?;
            self.length.write_to(&mut self.outer.target)?;
        }

        Ok(Compound {
            writer: &mut self.outer,
        })
    }
}
