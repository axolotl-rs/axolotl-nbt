use std::ops::{BitAndAssign, BitOr, Shl};

#[derive(Debug, Clone)]
pub struct CompactArray {
    pub bits_per_block: usize,
    pub data: Vec<u64>,
    pub length: usize,
    pub values_per_u64: usize,
    pub mask: u64,
}

impl CompactArray {
    pub fn new(bits_per_block: usize, length: usize) -> Self {
        let values_per_u64 = 64 / bits_per_block;
        let data = vec![0; (length + values_per_u64 as usize - 1) / values_per_u64 as usize];
        CompactArray {
            bits_per_block,
            data,
            length,
            values_per_u64,
            mask: (1 << bits_per_block as u64) - 1,
        }
    }
    pub fn get(&self, index: impl CompactArrayIndex) -> Option<u64> {
        let index = index.get();
        if index >= self.length {
            return None;
        }
        let (index, offset) = self.index_bit_value(index);
        Some((self.data[index] >> offset) & self.mask as u64)
    }
    pub fn set(&mut self, index: impl CompactArrayIndex, value: u64) {
        let index = index.get();
        let (index, offset) = self.index_bit_value(index);
        let re = &mut self.data[index];
        *re &= !(self.mask << offset);
        *re |= value << offset;
    }
    #[inline]
    fn index_bit_value(&self, index: usize) -> (usize, usize) {
        let list_index = (index / self.values_per_u64) as usize;
        let bit_offset = (index % self.values_per_u64) * self.bits_per_block;
        (list_index, bit_offset)
    }
}

pub trait CompactArrayIndex {
    fn get(self) -> usize;
}

impl CompactArrayIndex for u64 {
    #[inline(always)]
    fn get(self) -> usize {
        self as usize
    }
}

impl CompactArrayIndex for usize {
    #[inline(always)]
    fn get(self) -> usize {
        self
    }
}

impl CompactArrayIndex for (u32, u32, u32) {
    #[inline(always)]
    fn get(self) -> usize {
        let (x, y, z) = self;
        ((y << 8) | (z << 4) | x) as usize
    }
}

impl CompactArrayIndex for (u64, u64, u64) {
    #[inline(always)]
    fn get(self) -> usize {
        let (x, y, z) = self;
        ((y << 8) | (z << 4) | x) as usize
    }
}

#[cfg(test)]
pub mod tests {
    use crate::chunk::compact_array::CompactArrayIndex;

    #[test]
    pub fn number_tests() {
        println!("{:#066b}", (0u32, 1u32, 2u32).get())
    }
}
