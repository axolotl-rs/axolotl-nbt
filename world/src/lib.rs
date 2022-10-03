use crate::region::RegionLocation;
use std::mem::size_of;

pub mod region;

#[test]
pub fn size() {
    println!("{}", size_of::<RegionLocation>());
}
