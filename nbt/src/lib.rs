

use std::fmt::{Debug};
use std::io::{Read, Write};

pub mod binary;
mod error;
#[cfg(feature = "serde")]
pub mod serde_impl;
pub mod snbt;
#[cfg(feature = "value")]
pub mod value;

use crate::value::Value;
pub use error::NBTError;

#[repr(i8)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Tag {
    End = 0,
    Byte = 1,
    Short = 2,
    Int = 3,
    Long = 4,
    Float = 5,
    Double = 6,
    ByteArray = 7,
    String = 8,
    List = 9,
    Compound = 10,
    IntArray = 11,
    LongArray = 12,
}

impl Tag {
    // Zero means based on other information
    pub fn get_size(&self) -> usize {
        match self {
            Tag::End | Tag::Byte => 1,
            Tag::Short => 2,
            Tag::Int | Tag::Float => 4,
            Tag::Long | Tag::Double => 8,
            _ => 0,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ListType {
    ByteArray,
    IntArray,
    LongArray,
    List(Tag),
}

/// Either implemented in Binary or String
pub trait NBTType: Debug + Sized {
    type ListReader<'reader, Reader: Read + 'reader>: ListReader<'reader, Self, Reader>;
    type ListWriter<'writer, Writer: Write + 'writer>: ListWriter<'writer, Self, Writer>;
    type CompoundWriter<'writer, Writer: Write + 'writer>: CompoundWriter<'writer, Self, Writer>;
    type CompoundReader<'reader, Reader: Read + 'reader>: CompoundReader<'reader, Self, Reader>;

    /// Will read the tag name and the tag
    fn read_tag_name<R: Read>(reader: &mut R) -> Result<String, NBTError>;
    /// Will read the tag name and the tag

    fn read_tag_name_raw<R: Read>(reader: &mut R, value: &mut [u8]) -> Result<(), NBTError>;

    /// Will Write the tag name then any separator
    /// Moving the cursor to the start of the value
    ///
    /// Name must be a valid NBT name
    fn write_tag_name<W: Write, Name: AsRef<[u8]>>(
        writer: &mut W,
        name: Name,
    ) -> Result<(), NBTError>;
}

pub trait NBTDataType<Type: NBTType>: Debug {
    /// Will read the tag name and the tag
    fn read_with_name<R: Read>(reader: &mut R) -> Result<(String, Self), NBTError>
    where
        Self: Sized,
    {
        let name = Type::read_tag_name(reader)?;
        let value = Self::read(reader)?;
        Ok((name, value))
    }

    /// Reads a value from the given reader.
    fn read<R: Read>(reader: &mut R) -> Result<Self, NBTError>
    where
        Self: Sized;

    /// Write with the tag
    fn write<W: Write, Name: AsRef<[u8]>>(self, name: Name, writer: &mut W)
        -> Result<(), NBTError>;
    /// Write without the tag
    fn write_alone<W: Write>(self, writer: &mut W) -> Result<(), NBTError>;

    // Returns the Tag to use when writing a sequence of this type
    fn get_list_tag() -> ListType {
        ListType::List(Self::get_tag())
    }

    /// Gets the tag for this type
    fn get_tag() -> Tag;
}

pub trait ListReader<'reader, Type: NBTType + ?Sized, Reader: Read + 'reader> {
    fn new(reader: &'reader mut Reader, list_type: ListType) -> Result<Self, NBTError>
    where
        Self: Sized;
    fn new_generic_list(reader: &'reader mut Reader) -> Result<Self, NBTError>
    where
        Self: Sized;
    fn size(&self) -> usize;

    fn list_type(&self) -> ListType;

    fn get_tag(&self) -> Tag;

    fn read_next_tag<DataType: NBTDataType<Type>>(&mut self) -> Result<DataType, NBTError>;

    fn read_all_bytes(&mut self) -> Result<Vec<u8>, NBTError>;
    #[cfg(feature = "value")]
    fn read_next(&mut self) -> Result<Value, NBTError>;
}

pub trait CompoundReader<'reader, Type: NBTType, Reader: Read + 'reader> {
    fn new(reader: &'reader mut Reader) -> Result<Self, NBTError>
    where
        Self: Sized;

    fn next_tag(&mut self) -> Result<Tag, NBTError>;

    fn read_next_tag_name(&mut self) -> Result<String, NBTError>;

    fn read_next_tag_value<DataType: NBTDataType<Type>>(&mut self) -> Result<DataType, NBTError>;

    fn read_next_tag<DataType: NBTDataType<Type>>(
        &mut self,
    ) -> Result<(String, DataType), NBTError>;

    #[cfg(feature = "value")]
    fn read_to_end(self) -> Result<Vec<Value>, NBTError>;

    #[cfg(feature = "value")]
    fn read_next(&mut self) -> Result<Value, NBTError>;
}

pub trait CompoundWriter<'writer, Type: NBTType, Writer: Write + 'writer> {
    fn new(reader: &'writer mut Writer) -> Result<Self, NBTError>
    where
        Self: Sized;

    fn write_start<Name: AsRef<[u8]>>(writer: &mut Writer, name: Name) -> Result<(), NBTError>;
    fn write_next_tag<DataType: NBTDataType<Type>>(
        &mut self,
        name: impl AsRef<[u8]>,
        value: DataType,
    ) -> Result<(), NBTError>;

    fn end(self) -> Result<(), NBTError>;
}

pub trait ListWriter<'writer, Type: NBTType, Writer: Write + 'writer> {
    fn new<Name: AsRef<[u8]>>(
        reader: &'writer mut Writer,
        size: i32,
        list_type: ListType,
        name: Name,
    ) -> Result<Self, NBTError>
    where
        Self: Sized;

    fn new_sub_sequence(
        reader: &'writer mut Writer,
        size: i32,
        list_type: ListType,
    ) -> Result<Self, NBTError>
    where
        Self: Sized;

    /// In SNBT the length is not needed
    fn write_sequence_header<W: Write, Name: AsRef<[u8]>>(
        writer: &mut W,
        list_type: ListType,
        name: Name,
        length_of_array: i32,
    ) -> Result<(), NBTError> {
        Self::write_sequence_header_name_callback(writer, list_type, length_of_array, |writer| {
            Type::write_tag_name(writer, name)
        })
    }

    /// In SNBT the length is not needed
    fn write_sequence_header_name_callback<W: Write, Name>(
        writer: &mut W,
        list_type: ListType,
        length_of_array: i32,
        name: Name,
    ) -> Result<(), NBTError>
    where
        Name: FnOnce(&mut W) -> Result<(), NBTError>;

    /// In SNBT the length is not needed
    fn write_sub_sequence_header<W: Write>(
        writer: &mut W,
        tag_of_data_within: ListType,
        length_of_array: i32,
    ) -> Result<(), NBTError>;

    fn write_next_tag<DataType: NBTDataType<Type>>(
        &mut self,
        value: DataType,
    ) -> Result<(), NBTError>;
}
