use std::fmt::Debug;

#[derive(Debug)]
pub struct RegionReader<Src: Debug> {
    pub(crate) src: Src,
}

impl<Src: Debug> RegionReader<Src> {
    pub fn new(src: Src) -> Self {
        RegionReader { src }
    }
    pub fn into_inner(self) -> Src {
        self.src
    }
}
