use crate::bitvector::BitVector;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq)]
pub struct regidx(pub u8);

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct vregidx(pub u8);

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct fregidx(pub u8);

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct cfregidx(pub u8);

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct cregidx(pub u8);

#[allow(non_camel_case_types)]
pub type word_width = u8;

#[allow(non_camel_case_types)]
pub type word_width_wide = u8;

#[allow(non_camel_case_types)]
pub type nfields = u8;

#[allow(non_camel_case_types)]
pub type nfields_pow2 = u8;

#[allow(non_camel_case_types)]
pub type shamt_zba = BitVector<2>;

#[allow(non_camel_case_types)]
pub type csreg = BitVector<12>;

#[allow(non_upper_case_globals)]
pub const zreg: regidx = regidx(0);

#[allow(non_upper_case_globals)]
pub const x1: regidx = regidx(1);

#[allow(non_upper_case_globals)]
pub const ra: regidx = regidx(0b01);

#[allow(non_upper_case_globals)]
pub const sp: regidx = regidx(0b10);
