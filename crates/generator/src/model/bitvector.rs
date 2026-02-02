use crate::err;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitVector {
    pub val: u64,
    pub bit_width: usize,
}

impl BitVector {
    pub fn try_new(val: u64, bit_width: usize) -> crate::Result<Self> {
        assert!(bit_width > 0);
        assert!(bit_width <= 64);
        let max = (1_u64 << bit_width) - 1;

        if val <= max {
            Ok(Self { val, bit_width })
        } else {
            err!("value {val} bigger than 2^{bit_width} - 1")
        }
    }

    pub fn from_signed(val: i64, bit_width: usize) -> crate::Result<Self> {
        assert!(bit_width > 0);
        assert!(bit_width <= 64);

        if val >= 0 {
            let max = (1_i64 << (bit_width - 1)) - 1;
            if val <= max {
                Ok(Self {
                    val: val as u64,
                    bit_width,
                })
            } else {
                err!("{} exceedes 2^{}-1)", val, bit_width)
            }
        } else {
            let min = -(1_i64 << (bit_width - 1));
            if val >= min {
                let mask = (1_u64 << bit_width) - 1;
                let val = (val as u64) & mask;
                Ok(Self { val, bit_width })
            } else {
                err!("{} exceedes 2^{})", val, bit_width)
            }
        }
    }

    pub const fn mask(&self) -> u64 {
        (1_u64 << self.bit_width) - 1
    }

    pub fn join(&self, other: &Self) -> Self {
        let val = (self.val << other.bit_width) | other.val;

        Self {
            val,
            bit_width: self.bit_width + other.bit_width,
        }
    }

    pub const fn shr(&mut self, n: usize) -> Self {
        if n <= self.bit_width {
            Self {
                val: self.val >> n,
                bit_width: self.bit_width - n,
            }
        } else {
            Self {
                val: 0,
                bit_width: 0,
            }
        }
    }
}
