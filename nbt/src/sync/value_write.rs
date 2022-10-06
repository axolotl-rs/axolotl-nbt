use crate::sync::{NBTData, NBTWriter};
use crate::value::{NameLessValue, Value};
use crate::{NBTError, Tag};
use std::fmt::Debug;
use std::io::Write;

impl<Writer: Write + Debug> NBTWriter<Writer> {
    pub fn write_value(&mut self, value: Value) -> Result<(), NBTError> {
        match value {
            Value::Byte { name, value } => {
                self.write_tag(name, value)?;
            }
            Value::Short { name, value } => {
                self.write_tag(name, value)?;
            }
            Value::Int { name, value } => {
                self.write_tag(name, value)?;
            }
            Value::Long { name, value } => {
                self.write_tag(name, value)?;
            }
            Value::Float { name, value } => {
                self.write_tag(name, value)?;
            }
            Value::Double { name, value } => {
                self.write_tag(name, value)?;
            }
            Value::ByteArray { name, value } => {
                Tag::ByteArray.write_to(&mut self.target)?;
                self.write_string(name)?;
                (value.len() as i32).write_to(&mut self.target)?;
                for v in value {
                    v.write_to(&mut self.target)?;
                }
            }
            Value::IntArray { value, name } => {
                Tag::IntArray.write_to(&mut self.target)?;
                self.write_string(name)?;
                (value.len() as i32).write_to(&mut self.target)?;
                for v in value {
                    v.write_to(&mut self.target)?;
                }
            }
            Value::LongArray { value, name } => {
                Tag::LongArray.write_to(&mut self.target)?;
                self.write_string(name)?;
                (value.len() as i32).write_to(&mut self.target)?;
                for v in value {
                    v.write_to(&mut self.target)?;
                }
            }
            Value::String { name, value } => {
                self.write_tag(name, value)?;
            }
            Value::List { name, value } => {
                Tag::List.write_to(&mut self.target)?;
                self.write_string(name)?;
                if let Some(tag) = value.first() {
                    tag.tag().write_to(&mut self.target)?;
                    (value.len() as i32).write_to(&mut self.target)?;
                    for v in value {
                        self.write_nameless_value(v)?;
                    }
                }
            }
            Value::Compound { name, value } => {
                Tag::Compound.write_to(&mut self.target)?;
                self.write_string(name)?;
                for v in value {
                    self.write_value(v)?;
                }
                Tag::End.write_to(&mut self.target)?;
            }

            Value::End => {
                return Err(NBTError::UnexpectedEnd);
            }
        }
        Ok(())
    }
    pub(crate) fn write_nameless_value(&mut self, value: NameLessValue) -> Result<(), NBTError> {
        match value {
            NameLessValue::Byte(value) => {
                value.write_to(&mut self.target)?;
            }
            NameLessValue::Short(value) => {
                value.write_to(&mut self.target)?;
            }
            NameLessValue::Int(value) => {
                value.write_to(&mut self.target)?;
            }
            NameLessValue::Long(value) => {
                value.write_to(&mut self.target)?;
            }
            NameLessValue::Float(value) => {
                value.write_to(&mut self.target)?;
            }
            NameLessValue::Double(value) => {
                value.write_to(&mut self.target)?;
            }
            NameLessValue::ByteArray(value) => {
                (value.len() as i32).write_to(&mut self.target)?;
                for v in value {
                    v.write_to(&mut self.target)?;
                }
            }
            NameLessValue::IntArray(value) => {
                (value.len() as i32).write_to(&mut self.target)?;
                for v in value {
                    v.write_to(&mut self.target)?;
                }
            }
            NameLessValue::LongArray(value) => {
                (value.len() as i32).write_to(&mut self.target)?;
                for v in value {
                    v.write_to(&mut self.target)?;
                }
            }
            NameLessValue::String(value) => {
                self.write_string(value)?;
            }
            NameLessValue::List(value) => {
                if let Some(tag) = value.first() {
                    tag.tag().write_to(&mut self.target)?;
                    (value.len() as i32).write_to(&mut self.target)?;
                    for v in value {
                        self.write_nameless_value(v)?;
                    }
                }
            }
            NameLessValue::Compound(value) => {
                for v in value {
                    self.write_value(v)?;
                }
                Tag::End.write_to(&mut self.target)?;
            }
            NameLessValue::End => {
                return Err(NBTError::UnexpectedEnd);
            }
        }
        Ok(())
    }
}
