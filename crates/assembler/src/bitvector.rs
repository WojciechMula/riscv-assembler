#[derive(Eq, PartialEq)]
pub struct BitVector<const N: usize> {
    pub val: u32,
}

impl<const N: usize> BitVector<N> {
    pub const fn zero() -> Self {
        Self { val: 0 }
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
