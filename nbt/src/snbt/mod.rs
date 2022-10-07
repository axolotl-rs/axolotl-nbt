mod error;

mod parse;

use std::io::{Read, Write};
use crate::snbt::error::Error;
use crate::{CompoundReader, CompoundWriter, ListReader, ListType, ListWriter, NBTDataType, NBTError, NBTType, Tag, Value};
use logos::Logos;
use crate::snbt::parse::lexer::Token;
use crate::snbt::parse::parser;

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

    fn read_tag_name<R: Read>(reader: &mut R) -> Result<String, NBTError> {
        todo!()
    }

    fn read_tag_name_raw<R: Read>(reader: &mut R, value: &mut [u8]) -> Result<(), NBTError> {
        todo!()
    }

    fn write_tag_name<W: Write, Name: AsRef<[u8]>>(writer: &mut W, name: Name) -> Result<(), NBTError> {
        todo!()
    }
}

pub struct SNBTListReader<'reader, Reader: Read + 'reader> {
    reader: &'reader mut Reader,
}

impl<'reader, Reader: Read + 'reader> ListReader<'reader, SNBT, Reader> for SNBTListReader<'reader, Reader> {
    fn new(reader: &'reader mut Reader, list_type: ListType) -> Result<Self, NBTError> where Self: Sized {
        todo!()
    }

    fn new_generic_list(reader: &'reader mut Reader) -> Result<Self, NBTError> where Self: Sized {
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

    fn read_all_bytes(&mut self) -> Result<Vec<u8>, NBTError> {
        todo!()
    }

    fn read_next(&mut self) -> Result<Value, NBTError> {
        todo!()
    }
}

pub struct SNBTListWriter<'writer, Writer: Write + 'writer> {
    writer: &'writer mut Writer,
}

impl<'reader, Reader: Read + 'reader> ListWriter<'reader, SNBT, Reader> for SNBTListWriter<'reader, Reader> {
    fn new<Name: AsRef<[u8]>>(reader: &'reader mut Reader, size: i32, list_type: ListType, name: Name) -> Result<Self, NBTError> where Self: Sized {
        todo!()
    }

    fn new_sub_sequence(reader: &'reader mut Reader, size: i32, list_type: ListType) -> Result<Self, NBTError> where Self: Sized {
        todo!()
    }

    fn write_sequence_header_name_callback<W: Write, Name>(writer: &mut W, list_type: ListType, length_of_array: i32, name: Name) -> Result<(), NBTError> where Name: FnOnce(&mut W) -> Result<(), NBTError> {
        todo!()
    }

    fn write_sub_sequence_header<W: Write>(writer: &mut W, tag_of_data_within: ListType, length_of_array: i32) -> Result<(), NBTError> {
        todo!()
    }

    fn write_next_tag<DataType: NBTDataType<SNBT>>(&mut self, value: DataType) -> Result<(), NBTError> {
        todo!()
    }
}

pub struct SNBTCompoundReader<'reader, Reader: Read + 'reader> {
    reader: &'reader mut Reader,
}

impl<'reader, Reader: Read + 'reader> CompoundReader<'reader, SNBT, Reader> for SNBTCompoundReader<'reader, Reader> {
    fn new(reader: &'reader mut Reader) -> Result<Self, NBTError> where Self: Sized {
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

    fn read_next_tag<DataType: NBTDataType<SNBT>>(&mut self) -> Result<(String, DataType), NBTError> {
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

impl<'reader, Reader: Read + 'reader> CompoundWriter<'reader, SNBT, Reader> for SNBTCompoundWriter<'reader, Reader> {
    fn new(reader: &'reader mut Reader) -> Result<Self, NBTError> where Self: Sized {
        todo!()
    }

    fn write_start<Name: AsRef<[u8]>>(writer: &mut Reader, name: Name) -> Result<(), NBTError> {
        todo!()
    }

    fn write_next_tag<DataType: NBTDataType<SNBT>>(&mut self, name: impl AsRef<[u8]>, value: DataType) -> Result<(), NBTError> {
        todo!()
    }

    fn end(self) -> Result<(), NBTError> {
        todo!()
    }
}
