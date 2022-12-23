use std::{iter::FromIterator, ops::BitAnd};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default, Debug)]
pub struct BitSet(u64);

impl BitSet {
    pub fn reduce_sum(self) -> u32 {
        // We can take advantage of the fact that the last set only has
        // one element.
        debug_assert_eq!(self.0.count_ones(), 1);
        self.0.trailing_zeros()
    }

    pub fn len(self) -> u32 {
        self.0.count_ones()
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    pub fn insert(&mut self, index: usize) {
        self.0 |= 1 << index;
    }

    pub fn contains(&self, index: usize) -> bool {
        self.0 & (1 << index) != 0
    }
}

impl From<BitSet> for u64 {
    fn from(value: BitSet) -> Self {
        value.0
    }
}

impl FromIterator<u32> for BitSet {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut bits = 0;
        for byte in iter {
            debug_assert!(byte < 64, "{byte} does not fit in 64-bit set");
            bits |= 1 << byte as usize;
        }
        Self(bits)
    }
}

impl FromIterator<u8> for BitSet {
    #[inline]
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self::from_iter(iter.into_iter().map(|n| n as u32))
    }
}

impl BitAnd for BitSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
