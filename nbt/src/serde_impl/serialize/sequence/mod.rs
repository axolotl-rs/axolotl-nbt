mod sub_list;

use crate::serde_impl::serialize::macros::{gen_method_body, impossible, method_body};
use crate::serde_impl::serialize::named::{GetName, StringOrSerializer};
use crate::serde_impl::serialize::{cast_and_write, Compound};
use crate::serde_impl::Error;
use crate::{ListWriter, NBTDataType, NBTError, NBTType, Tag};
use serde::{ser, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt::Debug;
use std::io::Write;
use std::mem;

pub struct SerializeSeq<'writer, 'name: 'writer, W: Write, Type: NBTType, K: Serialize + ?Sized>
where
    i8: NBTDataType<Type>,
    i16: NBTDataType<Type>,
    i32: NBTDataType<Type>,
    i64: NBTDataType<Type>,
    f32: NBTDataType<Type>,
    f64: NBTDataType<Type>,
    String: NBTDataType<Type>,
    bool: NBTDataType<Type>,
{
    pub(crate) outer: &'writer mut W,
    pub(crate) name: &'name mut StringOrSerializer<'name, K>,
    pub(crate) wrote_header: bool,
    pub(crate) length: i32,
    pub(crate) phantom: std::marker::PhantomData<Type>,
}

impl<'writer, 'name: 'writer, W: Write, Type: NBTType, K: Serialize + ?Sized> ser::SerializeSeq
    for SerializeSeq<'writer, 'name, W, Type, K>
where
    i8: NBTDataType<Type>,
    i16: NBTDataType<Type>,
    i32: NBTDataType<Type>,
    i64: NBTDataType<Type>,
    f32: NBTDataType<Type>,
    f64: NBTDataType<Type>,
    String: NBTDataType<Type>,
    bool: NBTDataType<Type>,
{
    type Ok = ();
    type Error = super::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if !self.wrote_header {
            let mut inner: SerializeSeqInner<'_, '_, W, Type, K> = SerializeSeqInner {
                outer: self.outer,
                name: self.name,
                length: self.length,
                wrote_header: false,
                phantom: Default::default(),
            };
            value.serialize(&mut inner)?;
            self.wrote_header = true;

            *self.name = StringOrSerializer::None;
        } else {
            let mut inner: SerializeSeqInner<'_, '_, W, Type, K> = SerializeSeqInner {
                outer: self.outer,
                name: self.name,
                length: self.length,
                wrote_header: true,
                phantom: Default::default(),
            };
            value.serialize(&mut inner)?;
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct SerializeSeqInner<
    'writer,
    'name: 'writer,
    W: Write,
    Type: NBTType,
    K: Serialize + ?Sized,
> {
    pub(crate) outer: &'writer mut W,
    pub(crate) name: &'name StringOrSerializer<'name, K>,
    pub(crate) length: i32,
    pub(crate) wrote_header: bool,
    pub(crate) phantom: std::marker::PhantomData<Type>,
}

impl<'writer, 'name: 'writer, W: Write, Type: NBTType, K: Serialize + ?Sized>
    SerializeSeqInner<'writer, 'name, W, Type, K>
{
    #[inline]
    pub fn write<Data: NBTDataType<Type>>(&mut self, data: Data) -> Result<(), Error> {
        if !self.wrote_header {
            Type::ListWriter::<'writer, W>::write_sequence_header_name_callback(
                self.outer,
                Data::get_list_tag(),
                self.length,
                |w| {
                    match &self.name {
                        StringOrSerializer::String(name) => {
                            Type::write_tag_name(w, name)?;
                        }
                        StringOrSerializer::Serializer(name) => {
                            let name_getter: GetName<'_, W, Type> = GetName {
                                target: w,
                                phantom: Default::default(),
                            };
                            if let Err(error) = name.serialize(name_getter) {
                                match error {
                                    Error::IO(io) => {
                                        return Err(NBTError::IO(io));
                                    }
                                    Error::KeyMustBeString => {
                                        return Err(NBTError::KeyMustBeString);
                                    }
                                    Error::NBTErr(err) => {
                                        return Err(err);
                                    }
                                    Error::FromStrError(v) => {
                                        return Err(NBTError::NotAString(v));
                                    }
                                    _ => {
                                        // These errors should never happen
                                        panic!("Unexpected error: {:?}", error);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    Ok(())
                },
            )?;
        }
        data.write_alone(&mut self.outer)?;

        Ok(())
    }
    fn write_name(&mut self) -> Result<(), NBTError> {
        match &self.name {
            StringOrSerializer::String(name) => {
                Type::write_tag_name(&mut self.outer, name)?;
            }
            StringOrSerializer::Serializer(name) => {
                let name_getter: GetName<'_, W, Type> = GetName {
                    target: self.outer,
                    phantom: Default::default(),
                };
                if let Err(error) = name.serialize(name_getter) {
                    match error {
                        Error::IO(io) => {
                            return Err(NBTError::IO(io));
                        }
                        Error::KeyMustBeString => {
                            return Err(NBTError::KeyMustBeString);
                        }
                        Error::NBTErr(err) => {
                            return Err(err);
                        }
                        Error::FromStrError(v) => {
                            return Err(NBTError::NotAString(v));
                        }
                        _ => {
                            // These errors should never happen
                            panic!("Unexpected error: {:?}", error);
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl<'writer, 'name: 'writer, W: Write, Type: NBTType, K: Serialize + ?Sized> ser::Serializer
    for &'writer mut SerializeSeqInner<'writer, 'name, W, Type, K>
where
    i8: NBTDataType<Type>,
    i16: NBTDataType<Type>,
    i32: NBTDataType<Type>,
    i64: NBTDataType<Type>,
    f32: NBTDataType<Type>,
    f64: NBTDataType<Type>,
    String: NBTDataType<Type>,
    bool: NBTDataType<Type>,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = sub_list::SubList<'writer, W, Type>;
    type SerializeTuple = ser::Impossible<(), Self::Error>;
    type SerializeTupleStruct = ser::Impossible<(), Self::Error>;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
    type SerializeMap = Compound<'writer, W, Type>;
    type SerializeStruct = Compound<'writer, W, Type>;
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
        todo!("serialize_str")
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_seq(mut self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(len) = len {
            if !self.wrote_header {
                Tag::List.write_alone(&mut self.outer)?;
                self.write_name()?;
            }
            let list = sub_list::SubList {
                outer: self.outer,
                wrote_header: false,
                wrote_parent_header: self.wrote_header,
                parent_size: self.length,
                length: len as i32,
                phantom: Default::default(),
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
            Tag::List.write_alone(&mut self.outer)?;
            self.write_name()?;
            Tag::Compound.write_alone(&mut self.outer)?;
            self.length.write_alone(&mut self.outer)?;
        }

        Ok(Compound {
            writer: &mut self.outer,
            phantom: Default::default(),
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        if !self.wrote_header {
            Tag::List.write_alone(&mut self.outer)?;
            self.write_name()?;
            Tag::Compound.write_alone(&mut self.outer)?;
            self.length.write_alone(&mut self.outer)?;
        }

        Ok(Compound {
            writer: &mut self.outer,
            phantom: Default::default(),
        })
    }
}
