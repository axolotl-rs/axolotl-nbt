mod macros;
pub mod named;
pub mod sequence;

use crate::serde_impl::Error;
use crate::{NBTDataType, NBTType, Tag};

use serde::{ser, Serialize, Serializer};
use std::borrow::Cow;
use std::io::Write;

#[derive(Debug)]
pub struct NBTSerializer<'writer, W: Write, Type: NBTType>
where
    i8: NBTDataType<Type>,
    i16: NBTDataType<Type>,
    i32: NBTDataType<Type>,
    i64: NBTDataType<Type>,
    f32: NBTDataType<Type>,
    f64: NBTDataType<Type>,
    String: NBTDataType<Type>,
    for<'str> &'str str: NBTDataType<Type>,
{
    pub(crate) writer: &'writer mut W,
    pub(crate) phantom: std::marker::PhantomData<Type>,
}
macro_rules! cast_and_write {
    () => {
        fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
            self.serialize_i8(v as i8)
        }

        fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
            self.serialize_i16(v as i16)
        }

        fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
            self.serialize_i32(v as i32)
        }

        fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
            self.serialize_i64(v as i64)
        }
    };
}
use crate::serde_impl::serialize::macros::{gen_method_body, impossible, method_body};
use crate::serde_impl::serialize::named::{NamedValueSerializer, StringOrSerializer};
pub(crate) use cast_and_write;

impl<'writer, W: Write, Type: NBTType> Serializer for NBTSerializer<'writer, W, Type>
where
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
    type Error = super::Error;
    type SerializeSeq = ser::Impossible<(), Self::Error>;
    type SerializeTuple = ser::Impossible<(), Self::Error>;
    type SerializeTupleStruct = ser::Impossible<(), Self::Error>;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
    type SerializeMap = Compound<'writer, W, Type>;
    type SerializeStruct = Compound<'writer, W, Type>;
    type SerializeStructVariant = ser::Impossible<(), Self::Error>;
    fn serialize_map(mut self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Tag::Compound.write_alone(&mut self.writer)?;
        Type::write_tag_name(&mut self.writer, b"")?;
        Ok(Compound {
            writer: self.writer,
            phantom: Default::default(),
        })
    }

    fn serialize_struct(
        mut self,
        name: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Tag::Compound.write_alone(&mut self.writer)?;
        Type::write_tag_name(&mut self.writer, name)?;
        Ok(Compound {
            writer: self.writer,
            phantom: Default::default(),
        })
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
        str,
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
        unit_variant
    );
}

pub struct Compound<'writer, W: Write, Type: NBTType>
where
    i8: NBTDataType<Type>,
    i16: NBTDataType<Type>,
    i32: NBTDataType<Type>,
    i64: NBTDataType<Type>,
    f32: NBTDataType<Type>,
    f64: NBTDataType<Type>,
    String: NBTDataType<Type>,
    for<'str> &'str str: NBTDataType<Type>,
{
    pub writer: &'writer mut W,
    pub phantom: std::marker::PhantomData<Type>,
}

impl<'writer, W: Write, Type: NBTType> ser::SerializeMap for Compound<'writer, W, Type>
where
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

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        unimplemented!()
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        let serializer = StringOrSerializer::Serializer(key);
        let mut serializer1: NamedValueSerializer<'_, '_, W, Type, K> =
            NamedValueSerializer::new(self.writer, serializer);
        value.serialize(&mut serializer1)
    }

    fn end(mut self) -> Result<(), Self::Error> {
        Tag::End.write_alone(&mut self.writer)?;
        Ok(())
    }
}

impl<'writer, W: Write, Type: NBTType> ser::SerializeStruct for Compound<'writer, W, Type>
where
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
    type Error = super::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let serializer: StringOrSerializer<'static, &str> =
            StringOrSerializer::String(Cow::Borrowed(key.as_bytes()));
        let mut serializer1: NamedValueSerializer<'_, '_, W, Type, &str> =
            NamedValueSerializer::new(self.writer, serializer);
        value.serialize(&mut serializer1)
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        Tag::End.write_alone(&mut self.writer)?;
        Ok(())
    }
}

impl<'writer, W: Write, Type: NBTType> ser::SerializeStructVariant for Compound<'writer, W, Type>
where
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
    type Error = super::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let serializer: StringOrSerializer<'static, &str> =
            StringOrSerializer::String(Cow::Borrowed(key.as_bytes()));
        let mut serializer1: NamedValueSerializer<'_, '_, W, Type, &str> =
            NamedValueSerializer::new(self.writer, serializer);
        value.serialize(&mut serializer1)
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        Tag::End.write_alone(&mut self.writer)?;
        Ok(())
    }
}
