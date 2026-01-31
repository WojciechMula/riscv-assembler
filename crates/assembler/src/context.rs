use crate::assembler::Extensions;

#[derive(Debug, Clone)]
pub struct Context {
    pub xlen: Xlen,
    pub extensions: Extensions,
}

#[derive(Debug, Clone)]
pub enum Xlen {
    Xlen32,
    Xlen64,
}

impl Context {
    pub const fn xlen32() -> Self {
        Self {
            xlen: Xlen::Xlen32,
            extensions: Extensions::all(),
        }
    }

    pub const fn xlen64() -> Self {
        Self {
            xlen: Xlen::Xlen64,
            extensions: Extensions::all(),
        }
    }

    pub const fn is_32bit(&self) -> bool {
        matches!(&self.xlen, Xlen::Xlen32)
    }

    pub const fn is_64bit(&self) -> bool {
        matches!(&self.xlen, Xlen::Xlen64)
    }
}
