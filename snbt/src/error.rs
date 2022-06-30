use crate::Token;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected Token {0:?}")]
    UnexpectedToken(Token),
    #[error("Name Token Missing")]
    MissingName,
}
