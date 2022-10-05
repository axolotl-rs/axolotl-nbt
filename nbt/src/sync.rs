use crate::{NBTData, NBTReader, NBTWriter, Tag};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt::Debug;
use std::io::{BufRead, Error, Read, Write};

impl<R: BufRead + Debug> NBTReader<R> {
    /// Uses a Fill Buf to read the next tag id without moving the cursor.
    pub fn peak_tag_id(&mut self) -> Result<Tag, Error> {
        let result = self.src.fill_buf()?[0];
        Tag::from_u8(result).ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid Tag Id {}", result),
            )
        })
    }
}

impl<R: Read + Debug> NBTReader<R> {
    /// You will need to convert this to a String.
    pub fn read_str_as_bytes(&mut self, size: u16) -> Result<Vec<u8>, Error> {
        let mut result = Vec::with_capacity(size as usize);
        Read::take(&mut self.src, size as u64).read_to_end(&mut result)?;
        Ok(result)
    }
    pub fn read_tag_id(&mut self) -> Result<Tag, Error> {
        let result = self.src.read_u8()?;
        Tag::from_u8(result).ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid Tag Id {}", result),
            )
        })
    }
    /// Will be zero if the tag is an end tag.
    pub fn read_tag_id_with_id_len(&mut self) -> Result<(Tag, u16), Error> {
        let result = self.src.read_u8()?;
        let tag = Tag::from_u8(result).ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid Tag Id {}", result),
            )
        })?;
        if tag == Tag::End {
            return Ok((tag, 0));
        }
        let id_len = self.read_string_len()?;
        Ok((tag, id_len))
    }
    pub fn read_string_len(&mut self) -> Result<u16, Error> {
        let result = self.src.read_u16::<BigEndian>()?;
        Ok(result)
    }

    pub fn read_byte(&mut self) -> Result<i8, Error> {
        let result = self.src.read_i8()?;
        Ok(result)
    }
    pub fn read_short(&mut self) -> Result<i16, Error> {
        let result = self.src.read_i16::<BigEndian>()?;
        Ok(result)
    }
    pub fn read_int(&mut self) -> Result<i32, Error> {
        let result = self.src.read_i32::<BigEndian>()?;
        Ok(result)
    }
    pub fn read_long(&mut self) -> Result<i64, Error> {
        let result = self.src.read_i64::<BigEndian>()?;
        Ok(result)
    }
    pub fn read_float(&mut self) -> Result<f32, Error> {
        let result = self.src.read_f32::<BigEndian>()?;
        Ok(result)
    }
    pub fn read_double(&mut self) -> Result<f64, Error> {
        let result = self.src.read_f64::<BigEndian>()?;
        Ok(result)
    }
    pub fn read_byte_array(&mut self, size: i32) -> Result<Vec<i8>, Error> {
        let mut result = Vec::with_capacity(size as usize);
        for _ in 0..size {
            result.push(self.src.read_i8()?);
        }

        Ok(result)
    }
    pub fn read_list_type_and_size(&mut self) -> Result<(Tag, u32), Error> {
        let tag = self.read_tag_id()?;
        let size = self.read_int()?;
        Ok((tag, size as u32))
    }
}

impl<Writer: Write + Debug> NBTWriter<Writer> {
    pub fn write_tag_id(&mut self, tag: Tag) -> Result<(), Error> {
        tag.write_to(&mut self.target)?;
        Ok(())
    }
    pub fn write_tag<Data: NBTData>(
        &mut self,
        name: impl AsRef<[u8]>,
        tag: Data,
    ) -> Result<(), Error> {
        Data::tag().write_to(&mut self.target)?;
        self.write_string(name)?;
        tag.write_to(&mut self.target)?;
        Ok(())
    }
    #[inline(always)]
    pub fn write_string(&mut self, string: impl AsRef<[u8]>) -> Result<(), Error> {
        (string.as_ref().len() as u16).write_to(&mut self.target)?;
        self.target.write_all(string.as_ref())?;
        Ok(())
    }

    #[inline(always)]
    pub fn write_seq_header<Name: AsRef<[u8]>>(
        &mut self,
        tag: Tag,
        name: Option<Name>,
        len: i32,
    ) -> Result<(), Error> {
        match tag {
            Tag::Byte => {
                self.write_tag_id(Tag::ByteArray)?;
                if let Some(name) = name {
                    self.write_string(name)?;
                }
            }
            Tag::Int => {
                self.write_tag_id(Tag::IntArray)?;
                if let Some(name) = name {
                    self.write_string(name)?;
                }
            }
            Tag::Long => {
                self.write_tag_id(Tag::LongArray)?;
                if let Some(name) = name {
                    self.write_string(name)?;
                }
            }
            tag => {
                Tag::List.write_to(&mut self.target)?;
                if let Some(name) = name {
                    self.write_string(name)?;
                }
                tag.write_to(&mut self.target)?;
            }
        }
        len.write_to(&mut self.target)
    }
    pub fn write_seq<Data: NBTData>(
        &mut self,
        name: impl AsRef<[u8]>,
        tags: impl ExactSizeIterator<Item=Data>,
    ) -> Result<(), Error> {
        self.write_seq_header(Data::tag(), Some(name), tags.len() as i32)?;
        for tag in tags {
            tag.write_to(&mut self.target)?;
        }
        Ok(())
    }
    pub fn write_compound<Data: NBTData>(
        &mut self,
        name: impl AsRef<[u8]>,
        tags: impl Iterator<Item=Data>,
    ) -> Result<(), Error> {
        Tag::Compound.write_to(&mut self.target)?;
        self.write_string(name)?;
        for tag in tags {
            tag.write_to(&mut self.target)?;
        }
        Tag::End.write_to(&mut self.target)?;
        Ok(())
    }
}
