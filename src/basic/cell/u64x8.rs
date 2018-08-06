use std::fmt;
use std::fmt::Display;

use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

#[derive(Copy, Clone)]
pub struct U64x8 {
    pub value: [u64; 8],
}

impl U64x8 {
    pub fn new(x: [u64; 8]) -> Self {
        Self { value: x }
    }
}

impl Display for U64x8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:016X} {:016X} {:016X} {:016X} {:016X} {:016X} {:016X} {:016X}",
            self.value[7],
            self.value[6],
            self.value[5],
            self.value[4],
            self.value[3],
            self.value[2],
            self.value[1],
            self.value[0]
        )
    }
}

impl Not for U64x8 {
    type Output = Self;

    fn not(self) -> Self {
        Self {
            value: [
                !self.value[0],
                !self.value[1],
                !self.value[2],
                !self.value[3],
                !self.value[4],
                !self.value[5],
                !self.value[6],
                !self.value[7],
            ],
        }
    }
}

impl BitAnd for U64x8 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self {
            value: [
                self.value[0] & rhs.value[0],
                self.value[1] & rhs.value[1],
                self.value[2] & rhs.value[2],
                self.value[3] & rhs.value[3],
                self.value[4] & rhs.value[4],
                self.value[5] & rhs.value[5],
                self.value[6] & rhs.value[6],
                self.value[7] & rhs.value[7],
            ],
        }
    }
}

impl BitOr for U64x8 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self {
            value: [
                self.value[0] | rhs.value[0],
                self.value[1] | rhs.value[1],
                self.value[2] | rhs.value[2],
                self.value[3] | rhs.value[3],
                self.value[4] | rhs.value[4],
                self.value[5] | rhs.value[5],
                self.value[6] | rhs.value[6],
                self.value[7] | rhs.value[7],
            ],
        }
    }
}

impl BitXor for U64x8 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Self {
            value: [
                self.value[0] ^ rhs.value[0],
                self.value[1] ^ rhs.value[1],
                self.value[2] ^ rhs.value[2],
                self.value[3] ^ rhs.value[3],
                self.value[4] ^ rhs.value[4],
                self.value[5] ^ rhs.value[5],
                self.value[6] ^ rhs.value[6],
                self.value[7] ^ rhs.value[7],
            ],
        }
    }
}

impl BitAndAssign for U64x8 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.value[0] &= rhs.value[0];
        self.value[1] &= rhs.value[1];
        self.value[2] &= rhs.value[2];
        self.value[3] &= rhs.value[3];
        self.value[4] &= rhs.value[4];
        self.value[5] &= rhs.value[5];
        self.value[6] &= rhs.value[6];
        self.value[7] &= rhs.value[7];
    }
}

impl BitOrAssign for U64x8 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.value[0] |= rhs.value[0];
        self.value[1] |= rhs.value[1];
        self.value[2] |= rhs.value[2];
        self.value[3] |= rhs.value[3];
        self.value[4] |= rhs.value[4];
        self.value[5] |= rhs.value[5];
        self.value[6] |= rhs.value[6];
        self.value[7] |= rhs.value[7];
    }
}

impl BitXorAssign for U64x8 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.value[0] ^= rhs.value[0];
        self.value[1] ^= rhs.value[1];
        self.value[2] ^= rhs.value[2];
        self.value[3] ^= rhs.value[3];
        self.value[4] ^= rhs.value[4];
        self.value[5] ^= rhs.value[5];
        self.value[6] ^= rhs.value[6];
        self.value[7] ^= rhs.value[7];
    }
}
