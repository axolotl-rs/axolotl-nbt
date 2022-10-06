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

#[derive(Debug)]
pub struct SubList<'writer, W: SerdeWriter> {
    pub(crate) outer: &'writer mut NBTWriter<W>,
    pub(crate) wrote_header: bool,

    pub(crate) length: i32,
    pub(crate) wrote_parent_header: bool,
    pub(crate) parent_size: i32,
}

impl<'writer, W: SerdeWriter> ser::SerializeSeq for SubList<'writer, W> {
    type Ok = ();
    type Error = super::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: Serialize,
    {
        if !self.wrote_header {
            let mut inner = SerializeSeqInner {
                outer: self.outer,
                length: self.length,
                wrote_header: false,
                wrote_parent_header: self.wrote_parent_header,
                parent_size: self.parent_size,
            };
            value.serialize(&mut inner)?;
            self.wrote_header = true;
        } else {
            let mut inner = SerializeSeqInner {
                outer: self.outer,
                length: self.length,
                wrote_header: true,
                wrote_parent_header: self.wrote_parent_header,
                parent_size: self.parent_size,
            };
            value.serialize(&mut inner)?;
        }
        if !self.wrote_parent_header {
            self.wrote_parent_header = true;
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct SerializeSeqInner<'writer, W: SerdeWriter> {
    pub(crate) outer: &'writer mut NBTWriter<W>,
    pub(crate) length: i32,
    pub(crate) wrote_header: bool,
    pub(crate) wrote_parent_header: bool,
    pub(crate) parent_size: i32,
}

impl<'writer, W: SerdeWriter> SerializeSeqInner<'writer, W> {
    #[inline]
    fn parent(&mut self, tag: Tag) -> Result<(), Error> {
        if !self.wrote_parent_header {
            match tag {
                Tag::Byte => {
                    Tag::ByteArray.write_to(&mut self.outer.target)?;
                    self.parent_size.write_to(&mut self.outer.target)?;
                }
                Tag::Int => {
                    Tag::IntArray.write_to(&mut self.outer.target)?;
                    self.parent_size.write_to(&mut self.outer.target)?;
                }
                Tag::Long => {
                    Tag::LongArray.write_to(&mut self.outer.target)?;
                    self.parent_size.write_to(&mut self.outer.target)?;
                }
                _ => {
                    Tag::List.write_to(&mut self.outer.target)?;
                    self.parent_size.write_to(&mut self.outer.target)?;
                }
            }
        }
        Ok(())
    }
    #[inline]
    pub fn write<Data: NBTData + Debug>(&mut self, data: Data) -> Result<(), Error> {
        self.parent(Data::tag())?;
        if !self.wrote_header {
            match Data::tag() {
                Tag::Byte | Tag::Int | Tag::Long => {
                    self.length.write_to(&mut self.outer.target)?;
                }
                v => {
                    v.write_to(&mut self.outer.target)?;
                    self.length.write_to(&mut self.outer.target)?;
                }
            }
        }
        data.write_to(&mut self.outer.target)?;

        Ok(())
    }
}

impl<'writer, W: SerdeWriter> ser::Serializer for &'writer mut SerializeSeqInner<'writer, W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SubList<'writer, W>;
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
        self.parent(Tag::List)?;

        if let Some(len) = len {
            if !self.wrote_header {
                Tag::List.write_to(&mut self.outer.target)?;
            }
            let list = SubList {
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
        self.parent(Tag::Compound)?;
        if !self.wrote_header {
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
        self.parent(Tag::Compound)?;
        if !self.wrote_header {
            Tag::Compound.write_to(&mut self.outer.target)?;
            self.length.write_to(&mut self.outer.target)?;
        }

        Ok(Compound {
            writer: &mut self.outer,
        })
    }
}
