mod error;

mod parse;

use crate::snbt::error::Error;
use crate::snbt::parse::lexer::Token;
use crate::snbt::parse::parser;
use crate::{
    CompoundReader, CompoundWriter, ListReader, ListType, ListWriter, NBTDataType, NBTError,
    NBTType, NameLessValue, Tag, Value,
};
use logos::Logos;
use std::io::{Read, Write};

#[cfg(test)]
mod tests {
    use crate::snbt::to_value;
    use crate::to_value;

    #[test]
    fn it_works() {
        let value =
            to_value(r#"{name1:123,name2:"sometext1",name3:{subname1:456,subname2:"sometext2"}}"#)
                .unwrap();
        println!("{:?}", value);
    }
}

pub fn to_value(str: &str) -> Result<Value, Error> {
    let lex = Token::lexer(str);
    let value = parser::parse(lex)?;
    Ok(value)
}

#[derive(Debug)]
pub struct SNBT;

impl NBTType for SNBT {
    type ListReader<'reader, Reader: Read + 'reader> = SNBTListReader<'reader, Reader>;
    type ListWriter<'writer, Writer: Write + 'writer> = SNBTListWriter<'writer, Writer>;
    type CompoundWriter<'writer, Writer: Write + 'writer> = SNBTCompoundWriter<'writer, Writer>;
    type CompoundReader<'reader, Reader: Read + 'reader> = SNBTCompoundReader<'reader, Reader>;

    fn read_tag_name<R: Read>(_reader: &mut R) -> Result<String, NBTError> {
        todo!()
    }

    fn read_tag_name_raw<R: Read>(reader: &mut R, value: &mut Vec<u8>) -> Result<(), NBTError> {
        todo!()
    }

    fn write_tag_name<W: Write, Name: AsRef<[u8]>>(
        _writer: &mut W,
        _name: Name,
    ) -> Result<(), NBTError> {
        todo!()
    }
}

pub struct SNBTListReader<'reader, Reader: Read + 'reader> {
    reader: &'reader mut Reader,
}

impl<'reader, Reader: Read + 'reader> ListReader<'reader, SNBT, Reader>
    for SNBTListReader<'reader, Reader>
{
    fn new(_reader: &'reader mut Reader, _list_type: ListType) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn new_generic_list(_reader: &'reader mut Reader) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn list_type(&self) -> ListType {
        todo!()
    }

    fn get_tag(&self) -> Tag {
        todo!()
    }

    fn read_next_tag<DataType: NBTDataType<SNBT>>(&mut self) -> Result<DataType, NBTError> {
        todo!()
    }

    fn read_next(&mut self) -> Result<NameLessValue, NBTError> {
        todo!()
    }
}

pub struct SNBTListWriter<'writer, Writer: Write + 'writer> {
    writer: &'writer mut Writer,
}
impl<'writer, Writer: Write> ListWriter<'writer, SNBT, Writer> for SNBTListWriter<'writer, Writer> {
    fn new<Name: AsRef<[u8]>>(
        _reader: &'writer mut Writer,
        _size: i32,
        _list_type: ListType,
        _name: Name,
    ) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn new_sub_sequence(
        _reader: &'writer mut Writer,
        _size: i32,
        _list_type: ListType,
    ) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn write_sequence_header_name_callback<W: Write, Name>(
        _writer: &mut W,
        _list_type: ListType,
        _length_of_array: i32,
        _name: Name,
    ) -> Result<(), NBTError>
    where
        Name: FnOnce(&mut W) -> Result<(), NBTError>,
    {
        todo!()
    }

    fn write_sub_sequence_header<W: Write>(
        _writer: &mut W,
        _tag_of_data_within: ListType,
        _length_of_array: i32,
    ) -> Result<(), NBTError> {
        todo!()
    }

    fn write_next_tag<DataType: NBTDataType<SNBT>>(
        &mut self,
        _value: DataType,
    ) -> Result<(), NBTError> {
        todo!()
    }
}

pub struct SNBTCompoundReader<'reader, Reader: Read + 'reader> {
    reader: &'reader mut Reader,
}

impl<'reader, Reader: Read + 'reader> CompoundReader<'reader, SNBT, Reader>
    for SNBTCompoundReader<'reader, Reader>
{
    fn new(_reader: &'reader mut Reader) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn next_tag(&mut self) -> Result<Tag, NBTError> {
        todo!()
    }

    fn read_next_tag_name(&mut self) -> Result<String, NBTError> {
        todo!()
    }

    fn read_next_tag_value<DataType: NBTDataType<SNBT>>(&mut self) -> Result<DataType, NBTError> {
        todo!()
    }

    fn read_next_tag<DataType: NBTDataType<SNBT>>(
        &mut self,
    ) -> Result<(String, DataType), NBTError> {
        todo!()
    }

    fn read_to_end(self) -> Result<Vec<Value>, NBTError> {
        todo!()
    }

    fn read_next(&mut self) -> Result<Value, NBTError> {
        todo!()
    }
}

pub struct SNBTCompoundWriter<'writer, Writer: Write + 'writer> {
    writer: &'writer mut Writer,
}

impl<'writer, Writer: Write + 'writer> CompoundWriter<'writer, SNBT, Writer>
    for SNBTCompoundWriter<'writer, Writer>
{
    fn new(_reader: &'writer mut Writer) -> Result<Self, NBTError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn write_start<Name: AsRef<[u8]>>(_writer: &mut Writer, _name: Name) -> Result<(), NBTError> {
        todo!()
    }

    fn write_next_tag<DataType: NBTDataType<SNBT>>(
        &mut self,
        _name: impl AsRef<[u8]>,
        _value: DataType,
    ) -> Result<(), NBTError> {
        todo!()
    }

    fn end(self) -> Result<(), NBTError> {
        todo!()
    }
}
