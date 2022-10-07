use crate::region::RegionLocation;
use std::mem::size_of;
use thiserror::Error;
use axolotl_nbt::{NBTError, serde_impl};

pub mod chunk;
pub mod region;

#[test]
pub fn size() {
    println!("{}", size_of::<RegionLocation>());
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from]std::io::Error),
    #[error(transparent)]
    Nbt(#[from]NBTError),
    #[error(transparent)]
    SerdeNBT(#[from]serde_impl::Error),
    #[error("Invalid chunk header: {0}")]
    InvalidChunkHeader(&'static str),
}