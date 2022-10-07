use thiserror::Error;
use crate::snbt::parse::lexer::Token;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected Token {0:?}")]
    UnexpectedToken(Token),
    #[error("Name Token Missing")]
    MissingName,
}
