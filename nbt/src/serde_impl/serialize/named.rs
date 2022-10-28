use crate::serde_impl::serialize::macros::{
    gen_method_body, impossible, method_body, name_impossible,
};
use crate::serde_impl::serialize::sequence::SerializeSeq;
use crate::serde_impl::serialize::{cast_and_write, Compound};
use crate::serde_impl::Error;
use crate::{NBTDataType, NBTType, Tag};
use serde::{ser, Serialize, Serializer};
use std::borrow::Cow;
use std::io::Write;
use std::marker::PhantomData;

pub enum StringOrSerializer<'data, K: Serialize + ?Sized> {
    String(Cow<'data, [u8]>),
    Serializer(&'data K),
    None,
}

pub struct NamedValueSerializer<
    'writer,
    'name: 'writer,
    W: Write,
    Type: NBTType,
    K: Serialize + ?Sized,
> where
    &'name str: NBTDataType<Type>,
{
    pub target: &'writer mut W,
    pub name: StringOrSerializer<'name, K>,
    pub phantom: std::marker::PhantomData<Type>,
}

impl<'writer, 'name: 'writer, W: Write, Type: NBTType, K: Serialize + ?Sized>
    NamedValueSerializer<'writer, 'name, W, Type, K>
where
    'writer: 'name,
    &'name str: NBTDataType<Type>,
{
    pub fn new(target: &'writer mut W, name: StringOrSerializer<'name, K>) -> Self {
        Self {
            target,
            name,
            phantom: Default::default(),
        }
    }
    #[inline]
    pub fn write<Data: NBTDataType<Type>>(&mut self, value: Data) -> Result<(), Error> {
        Data::get_tag().write_alone(self.target)?;
        match &self.name {
            StringOrSerializer::String(name) => {
                Type::write_tag_name(self.target, name)?;
            }
            StringOrSerializer::Serializer(name) => {
                let get_name = GetName::<'_, W, Type> {
                    target: self.target,
                    phantom: Default::default(),
                };
                name.serialize(get_name)?;
            }
            StringOrSerializer::None => {}
        }
        value.write_alone(self.target)?;
        Ok(())
    }
    #[inline]
    pub fn write_tag(&mut self, value: Tag) -> Result<(), Error> {
        match &self.name {
            StringOrSerializer::String(name) => {
                value.write_alone(&mut self.target)?;
                Type::write_tag_name(&mut self.target, name.as_ref())?;
            }
            StringOrSerializer::Serializer(name) => {
                value.write_alone(&mut self.target)?;
                let name_getter: GetName<'_, W, Type> = GetName {
                    target: self.target,
                    phantom: Default::default(),
                };
                name.serialize(name_getter)?;
            }
            _ => {
                value.write_alone(&mut self.target)?;
                Type::write_tag_name(&mut self.target, b"")?;
            }
        }
        Ok(())
    }
}

impl<'writer, 'name: 'writer, W: Write, Type: NBTType, K: Serialize + ?Sized> Serializer
    for &'writer mut NamedValueSerializer<'writer, 'name, W, Type, K>
where
    'writer: 'name,
    i8: NBTDataType<Type>,
    i16: NBTDataType<Type>,
    i32: NBTDataType<Type>,
    i64: NBTDataType<Type>,
    f32: NBTDataType<Type>,
    f64: NBTDataType<Type>,
    String: NBTDataType<Type>,
    bool: NBTDataType<Type>,
    for<'str> &'str str: NBTDataType<Type>,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SerializeSeq<'writer, 'name, W, Type, K>;
    type SerializeTuple = ser::Impossible<(), Self::Error>;
    type SerializeTupleStruct = ser::Impossible<(), Self::Error>;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
    type SerializeMap = Compound<'writer, W, Type>;
    type SerializeStruct = Compound<'writer, W, Type>;
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

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        todo!("serialize_char")
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.write(v)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!("serialize_bytes")
    }
    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.write(variant)
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(len) = len {
            Ok(SerializeSeq {
                outer: self.target,
                name: &mut self.name,
                wrote_header: false,
                length: len as i32,
                phantom: Default::default(),
            })
        } else {
            Err(Error::UnrepresentableValueError(
                "Unrepresentable value: sequence with unknown length",
            ))
        }
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.write_tag(Tag::Compound)?;
        Ok(Compound {
            writer: self.target,
            phantom: Default::default(),
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.write_tag(Tag::Compound)?;
        Ok(Compound {
            writer: self.target,
            phantom: Default::default(),
        })
    }

    impossible!(
        none,
        some,
        unit,
        newtype_struct,
        newtype_variant,
        tuple,
        tuple_struct,
        tuple_variant,
        struct_variant
    );

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.write(name)
    }
}

#[derive(Debug)]
pub struct GetName<'writer, W: Write, Type: NBTType> {
    pub(crate) target: &'writer mut W,
    pub(crate) phantom: PhantomData<Type>,
}

impl<'writer, W: Write, Type: NBTType> Serializer for GetName<'writer, W, Type> {
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
        Type::write_tag_name(self.target, v)?;
        Ok(())
    }
    name_impossible!(
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
