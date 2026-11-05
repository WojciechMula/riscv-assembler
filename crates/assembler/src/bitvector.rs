#[derive(Eq, PartialEq)]
pub struct BitVector<const N: usize> {
    pub val: u32,
}

impl<const N: usize> BitVector<N> {
    pub const fn zero() -> Self {
        Self { val: 0 }
    }

    pub fn try_from(val: u32) -> crate::Result<Self> {
        let maxval = (1_u32 << N) - 1;
        if val > maxval {
            crate::err!("{val} exceedes 2^{N}")
        } else {
            Ok(Self { val })
        }
    }

    pub const fn from_subword(val: u32) -> Self {
        Self {
            val: val & Self::mask(),
        }
    }

    pub const fn new(val: u32) -> Self {
        Self {
            val: val & Self::mask(),
        }
    }

    pub const fn mask() -> u32 {
        (1_u32 << N) - 1
    }
}
