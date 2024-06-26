use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OctantId {
    bits: BitVec<u32>,
}

impl Default for OctantId {
    fn default() -> Self {
        Self::new()
    }
}

impl OctantId {
    pub fn new() -> Self {
        Self {
            bits: BitVec::from_elem(3, false),
        }
    }

    pub fn set_true(&mut self, index: usize) {
        self.bits.set(index, true)
    }

    pub fn flip_at(&mut self, index: usize) {
        self.bits.set(index, !self.bits[index])
    }

    pub fn with_index(index: usize) -> Self {
        let mut bits = BitVec::from_elem(3, false);
        for i in 0..3 {
            bits.set(i, (index & (1 << i)) != 0);
        }
        Self { bits }
    }

    pub fn x(&self) -> u8 {
        if self.bits[0] {
            1
        } else {
            0
        }
    }

    pub fn y(&self) -> u8 {
        if self.bits[1] {
            1
        } else {
            0
        }
    }

    pub fn z(&self) -> u8 {
        if self.bits[2] {
            1
        } else {
            0
        }
    }

    pub fn inverse(&self) -> Self {
        let mut inverse = BitVec::from_elem(3, false);
        inverse.set(0, !self.bits[0]);
        inverse.set(1, !self.bits[1]);
        inverse.set(2, !self.bits[2]);

        Self { bits: inverse }
    }

    pub fn to_numeral(&self) -> usize {
        let mut result = 0;
        for i in 0..3 {
            if self.bits[i] {
                result |= 1 << i;
            }
        }
        result
    }
}
