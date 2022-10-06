use crate::Tag;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NBTError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("Invalid Tag ID {0}")]
    InvalidTag(i8),
    #[error("Expected Tag {0:?}, but got {1:?}")]
    ExpectedTag(Tag, Tag),
    #[error("Not a utf8 string: {0}")]
    NotAString(#[from] std::string::FromUtf8Error),
    #[error("Unexpected EOF")]
    UnexpectedEnd,
}
