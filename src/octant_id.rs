use bit_vec::BitVec;

pub struct Octant {
    bits: BitVec<u32>,
}

impl Octant {
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
}
