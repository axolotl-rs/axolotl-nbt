use crate::sync::{NBTData, NBTWriter};
use crate::{NBTError, Tag};
use std::fmt::Debug;
use std::io::Write;

impl<Writer: Write + Debug> NBTWriter<Writer> {
    pub fn write_tag_id(&mut self, tag: Tag) -> Result<(), NBTError> {
        tag.write_to(&mut self.target)?;
        Ok(())
    }
    pub fn write_tag<Data: NBTData>(
        &mut self,
        name: impl AsRef<[u8]>,
        tag: Data,
    ) -> Result<(), NBTError> {
        Data::tag().write_to(&mut self.target)?;
        self.write_string(name)?;
        tag.write_to(&mut self.target)?;
        Ok(())
    }
    #[inline(always)]
    pub fn write_string(&mut self, string: impl AsRef<[u8]>) -> Result<(), NBTError> {
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
    ) -> Result<(), NBTError> {
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
        tags: impl ExactSizeIterator<Item = Data>,
    ) -> Result<(), NBTError> {
        self.write_seq_header(Data::tag(), Some(name), tags.len() as i32)?;
        for tag in tags {
            tag.write_to(&mut self.target)?;
        }
        Ok(())
    }
    pub fn write_compound<Data: NBTData>(
        &mut self,
        name: impl AsRef<[u8]>,
        tags: impl Iterator<Item = Data>,
    ) -> Result<(), NBTError> {
        Tag::Compound.write_to(&mut self.target)?;
        self.write_string(name)?;
        for tag in tags {
            tag.write_to(&mut self.target)?;
        }
        Tag::End.write_to(&mut self.target)?;
        Ok(())
    }
}
