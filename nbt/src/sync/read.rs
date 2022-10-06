use crate::sync::{NBTData, NBTReader};
use crate::{NBTError, Tag};
use byteorder::{BigEndian, ReadBytesExt};
use std::fmt::Debug;
use std::io::{BufRead, Read, Write};

impl<R: BufRead + Debug> NBTReader<R> {
    /// Uses a Fill Buf to read the next tag id without moving the cursor.
    pub fn peak_tag_id(&mut self) -> Result<Tag, NBTError> {
        let result = self.src.fill_buf()?[0] as i8;
        Tag::from_i8(result).ok_or_else(|| NBTError::InvalidTag(result))
    }
}

impl<R: Read + Debug> NBTReader<R> {
    /// You will need to convert this to a String.
    pub fn read_str_as_bytes(&mut self, size: u16) -> Result<Vec<u8>, NBTError> {
        let mut result = Vec::with_capacity(size as usize);
        Read::take(&mut self.src, size as u64).read_to_end(&mut result)?;
        Ok(result)
    }
    pub fn read_tag_id(&mut self) -> Result<Tag, NBTError> {
        Tag::read_from(&mut self.src)
    }
    /// Will be zero if the tag is an end tag.
    pub fn read_tag_id_with_id_len(&mut self) -> Result<(Tag, u16), NBTError> {
        let result = self.src.read_i8()?;
        let tag = Tag::from_i8(result).ok_or_else(|| NBTError::InvalidTag(result))?;
        if tag == Tag::End {
            return Ok((tag, 0));
        }
        let id_len = self.read_string_len()?;
        Ok((tag, id_len))
    }
    pub fn read_string_len(&mut self) -> Result<u16, NBTError> {
        let result = self.src.read_u16::<BigEndian>()?;
        Ok(result)
    }

    pub fn read_byte(&mut self) -> Result<i8, NBTError> {
        let result = self.src.read_i8()?;
        Ok(result)
    }
    pub fn read_short(&mut self) -> Result<i16, NBTError> {
        let result = self.src.read_i16::<BigEndian>()?;
        Ok(result)
    }
    pub fn read_int(&mut self) -> Result<i32, NBTError> {
        let result = self.src.read_i32::<BigEndian>()?;
        Ok(result)
    }
    pub fn read_long(&mut self) -> Result<i64, NBTError> {
        let result = self.src.read_i64::<BigEndian>()?;
        Ok(result)
    }
    pub fn read_float(&mut self) -> Result<f32, NBTError> {
        let result = self.src.read_f32::<BigEndian>()?;
        Ok(result)
    }
    pub fn read_double(&mut self) -> Result<f64, NBTError> {
        let result = self.src.read_f64::<BigEndian>()?;
        Ok(result)
    }
    pub fn read_byte_array(&mut self, size: i32) -> Result<Vec<i8>, NBTError> {
        let mut result = Vec::with_capacity(size as usize);
        for _ in 0..size {
            result.push(self.src.read_i8()?);
        }
        Ok(result)
    }
    pub fn read_list_type_and_size(&mut self) -> Result<(Tag, u32), NBTError> {
        let tag = self.read_tag_id()?;
        let size = self.read_int()?;
        Ok((tag, size as u32))
    }
}
