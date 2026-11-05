// Sail types
use crate::assembler::Signedness;
use crate::assembler::VectorHalf;
use crate::assembler::mul_op;
use crate::assembler::rounding_mode;
//

use crate::LabelResolverTrait;
use crate::bitvector::BitVector;
use crate::err;
use crate::parser::Parser;
use crate::parser::parse_i64;
use crate::sail_types::*;

pub fn encdec_reg(reg: regidx) -> BitVector<5> {
    BitVector::<5>::new(reg.0 as u32)
}

pub fn encdec_vreg(reg: vregidx) -> BitVector<5> {
    BitVector::<5>::new(reg.0 as u32)
}

pub fn encdec_freg(reg: fregidx) -> BitVector<5> {
    BitVector::<5>::new(reg.0 as u32)
}

pub fn bool_bit(f: bool) -> BitVector<1> {
    if f {
        BitVector::<1>::new(1)
    } else {
        BitVector::<1>::new(0)
    }
}

pub fn width_enc(v: word_width) -> BitVector<2> {
    match v {
        1 => BitVector::<2>::new(0b00),
        2 => BitVector::<2>::new(0b01),
        4 => BitVector::<2>::new(0b10),
        8 => BitVector::<2>::new(0b11),
        _ => unreachable!(),
    }
}

pub fn width_enc_wide(v: word_width_wide) -> BitVector<3> {
    match v {
        1 => BitVector::<3>::new(0b000),
        2 => BitVector::<3>::new(0b001),
        4 => BitVector::<3>::new(0b010),
        8 => BitVector::<3>::new(0b011),
        16 => BitVector::<3>::new(0b100),
        _ => unreachable!(),
    }
}

pub fn encdec_mul_op(val: mul_op) -> BitVector<3> {
    match val {
        mul_op {
            result_part: VectorHalf::Low,
            signed_rs1: Signedness::Signed,
            signed_rs2: Signedness::Signed,
        } => BitVector::<3>::new(0b000),
        mul_op {
            result_part: VectorHalf::High,
            signed_rs1: Signedness::Signed,
            signed_rs2: Signedness::Signed,
        } => BitVector::<3>::new(0b001),
        mul_op {
            result_part: VectorHalf::High,
            signed_rs1: Signedness::Signed,
            signed_rs2: Signedness::Unsigned,
        } => BitVector::<3>::new(0b010),
        mul_op {
            result_part: VectorHalf::High,
            signed_rs1: Signedness::Unsigned,
            signed_rs2: Signedness::Unsigned,
        } => BitVector::<3>::new(0b011),
        _ => unreachable!(),
    }
}

pub fn encdec_nfields(v: nfields) -> BitVector<3> {
    match v {
        1 => BitVector::<3>::new(0),
        2 => BitVector::<3>::new(1),
        3 => BitVector::<3>::new(2),
        4 => BitVector::<3>::new(3),
        5 => BitVector::<3>::new(4),
        6 => BitVector::<3>::new(5),
        7 => BitVector::<3>::new(6),
        8 => BitVector::<3>::new(7),
        _ => unreachable!(),
    }
}

pub fn encdec_nfields_pow2(v: nfields_pow2) -> BitVector<4> {
    match v {
        1 => BitVector::<4>::new(0b000),
        2 => BitVector::<4>::new(0b001),
        4 => BitVector::<4>::new(0b011),
        8 => BitVector::<4>::new(0b111),
        _ => unreachable!(),
    }
}

pub fn encdec_creg(reg: cregidx) -> BitVector<5> {
    BitVector::new(reg.0 as u32 | 0b01_000)
}

pub fn encdec_nreg(nreg: usize) -> BitVector<5> {
    match nreg {
        1 => BitVector::new(0b0_0000),
        2 => BitVector::new(0b0_0001),
        4 => BitVector::new(0b0_0011),
        8 => BitVector::new(0b0_0111),
        _ => unreachable!(),
    }
}

pub fn encdec_cfreg(reg: cfregidx) -> BitVector<5> {
    BitVector::new(reg.0 as u32 | 0b01_000)
}

fn reg_name_abi(s: &str) -> Option<regidx> {
    match s {
        "zero" => Some(regidx(0b00000)),
        "ra" => Some(regidx(0b00001)),
        "sp" => Some(regidx(0b00010)),
        "gp" => Some(regidx(0b00011)),
        "tp" => Some(regidx(0b00100)),
        "t0" => Some(regidx(0b00101)),
        "t1" => Some(regidx(0b00110)),
        "t2" => Some(regidx(0b00111)),
        "s0" => Some(regidx(0b01000)),
        "fp" => Some(regidx(0b01000)),
        "s1" => Some(regidx(0b01001)),
        "a0" => Some(regidx(0b01010)),
        "a1" => Some(regidx(0b01011)),
        "a2" => Some(regidx(0b01100)),
        "a3" => Some(regidx(0b01101)),
        "a4" => Some(regidx(0b01110)),
        "a5" => Some(regidx(0b01111)),
        "a6" => Some(regidx(0b10000)),
        "a7" => Some(regidx(0b10001)),
        "s2" => Some(regidx(0b10010)),
        "s3" => Some(regidx(0b10011)),
        "s4" => Some(regidx(0b10100)),
        "s5" => Some(regidx(0b10101)),
        "s6" => Some(regidx(0b10110)),
        "s7" => Some(regidx(0b10111)),
        "s8" => Some(regidx(0b11000)),
        "s9" => Some(regidx(0b11001)),
        "s10" => Some(regidx(0b11010)),
        "s11" => Some(regidx(0b11011)),
        "t3" => Some(regidx(0b11100)),
        "t4" => Some(regidx(0b11101)),
        "t5" => Some(regidx(0b11110)),
        "t6" => Some(regidx(0b11111)),
        _ => None,
    }
}

fn reg_name_arch(s: &str) -> Option<regidx> {
    match s {
        "x0" => Some(regidx(0b00000)),
        "x1" => Some(regidx(0b00001)),
        "x2" => Some(regidx(0b00010)),
        "x3" => Some(regidx(0b00011)),
        "x4" => Some(regidx(0b00100)),
        "x5" => Some(regidx(0b00101)),
        "x6" => Some(regidx(0b00110)),
        "x7" => Some(regidx(0b00111)),
        "x8" => Some(regidx(0b01000)),
        "x9" => Some(regidx(0b01001)),
        "x10" => Some(regidx(0b01010)),
        "x11" => Some(regidx(0b01011)),
        "x12" => Some(regidx(0b01100)),
        "x13" => Some(regidx(0b01101)),
        "x14" => Some(regidx(0b01110)),
        "x15" => Some(regidx(0b01111)),
        "x16" => Some(regidx(0b10000)),
        "x17" => Some(regidx(0b10001)),
        "x18" => Some(regidx(0b10010)),
        "x19" => Some(regidx(0b10011)),
        "x20" => Some(regidx(0b10100)),
        "x21" => Some(regidx(0b10101)),
        "x22" => Some(regidx(0b10110)),
        "x23" => Some(regidx(0b10111)),
        "x24" => Some(regidx(0b11000)),
        "x25" => Some(regidx(0b11001)),
        "x26" => Some(regidx(0b11010)),
        "x27" => Some(regidx(0b11011)),
        "x28" => Some(regidx(0b11100)),
        "x29" => Some(regidx(0b11101)),
        "x30" => Some(regidx(0b11110)),
        "x31" => Some(regidx(0b11111)),
        _ => None,
    }
}

pub fn reg_name(parser: &mut Parser) -> crate::Result<regidx> {
    let ident = parser.expect_alpha_num()?;

    if let Some(reg) = reg_name_abi(ident) {
        Ok(reg)
    } else if let Some(reg) = reg_name_arch(ident) {
        Ok(reg)
    } else {
        err!("expected a GPR register, got `{ident}`")
    }
}

pub fn freg_name_abi(s: &str) -> Option<fregidx> {
    match s {
        "ft0" => Some(fregidx(0b00000)),
        "ft1" => Some(fregidx(0b00001)),
        "ft2" => Some(fregidx(0b00010)),
        "ft3" => Some(fregidx(0b00011)),
        "ft4" => Some(fregidx(0b00100)),
        "ft5" => Some(fregidx(0b00101)),
        "ft6" => Some(fregidx(0b00110)),
        "ft7" => Some(fregidx(0b00111)),
        "fs0" => Some(fregidx(0b01000)),
        "fs1" => Some(fregidx(0b01001)),
        "fa0" => Some(fregidx(0b01010)),
        "fa1" => Some(fregidx(0b01011)),
        "fa2" => Some(fregidx(0b01100)),
        "fa3" => Some(fregidx(0b01101)),
        "fa4" => Some(fregidx(0b01110)),
        "fa5" => Some(fregidx(0b01111)),
        "fa6" => Some(fregidx(0b10000)),
        "fa7" => Some(fregidx(0b10001)),
        "fs2" => Some(fregidx(0b10010)),
        "fs3" => Some(fregidx(0b10011)),
        "fs4" => Some(fregidx(0b10100)),
        "fs5" => Some(fregidx(0b10101)),
        "fs6" => Some(fregidx(0b10110)),
        "fs7" => Some(fregidx(0b10111)),
        "fs8" => Some(fregidx(0b11000)),
        "fs9" => Some(fregidx(0b11001)),
        "fs10" => Some(fregidx(0b11010)),
        "fs11" => Some(fregidx(0b11011)),
        "ft8" => Some(fregidx(0b11100)),
        "ft9" => Some(fregidx(0b11101)),
        "ft10" => Some(fregidx(0b11110)),
        "ft11" => Some(fregidx(0b11111)),
        _ => None,
    }
}

pub fn freg_name_arch(s: &str) -> Option<fregidx> {
    match s {
        "f0" => Some(fregidx(0b00000)),
        "f1" => Some(fregidx(0b00001)),
        "f2" => Some(fregidx(0b00010)),
        "f3" => Some(fregidx(0b00011)),
        "f4" => Some(fregidx(0b00100)),
        "f5" => Some(fregidx(0b00101)),
        "f6" => Some(fregidx(0b00110)),
        "f7" => Some(fregidx(0b00111)),
        "f8" => Some(fregidx(0b01000)),
        "f9" => Some(fregidx(0b01001)),
        "f10" => Some(fregidx(0b01010)),
        "f11" => Some(fregidx(0b01011)),
        "f12" => Some(fregidx(0b01100)),
        "f13" => Some(fregidx(0b01101)),
        "f14" => Some(fregidx(0b01110)),
        "f15" => Some(fregidx(0b01111)),
        "f16" => Some(fregidx(0b10000)),
        "f17" => Some(fregidx(0b10001)),
        "f18" => Some(fregidx(0b10010)),
        "f19" => Some(fregidx(0b10011)),
        "f20" => Some(fregidx(0b10100)),
        "f21" => Some(fregidx(0b10101)),
        "f22" => Some(fregidx(0b10110)),
        "f23" => Some(fregidx(0b10111)),
        "f24" => Some(fregidx(0b11000)),
        "f25" => Some(fregidx(0b11001)),
        "f26" => Some(fregidx(0b11010)),
        "f27" => Some(fregidx(0b11011)),
        "f28" => Some(fregidx(0b11100)),
        "f29" => Some(fregidx(0b11101)),
        "f30" => Some(fregidx(0b11110)),
        "f31" => Some(fregidx(0b11111)),
        _ => None,
    }
}

pub fn freg_name(parser: &mut Parser) -> crate::Result<fregidx> {
    let ident = parser.expect_alpha_num()?;

    if let Some(reg) = freg_name_abi(ident) {
        Ok(reg)
    } else if let Some(reg) = freg_name_arch(ident) {
        Ok(reg)
    } else {
        err!("expected an FP register, got `{ident}`")
    }
}

pub fn cfreg_name(p: &mut Parser) -> crate::Result<cfregidx> {
    let fregidx(id) = freg_name(p)?;

    if (id & 0b11_000) != 0b01_000 {
        return err!("instruction does not support this register");
    }

    Ok(cfregidx(id))
}

pub fn creg_name(parser: &mut Parser) -> crate::Result<cregidx> {
    let regidx(id) = reg_name(parser)?;

    if (id & 0b11_000) != 0b01_000 {
        return err!("instruction does not support this register");
    }

    Ok(cregidx(id))
}

pub fn vreg_name(parser: &mut Parser) -> crate::Result<vregidx> {
    let ident = parser.expect_alpha_num()?;

    match ident {
        "v0" => Ok(vregidx(0b00000)),
        "v1" => Ok(vregidx(0b00001)),
        "v2" => Ok(vregidx(0b00010)),
        "v3" => Ok(vregidx(0b00011)),
        "v4" => Ok(vregidx(0b00100)),
        "v5" => Ok(vregidx(0b00101)),
        "v6" => Ok(vregidx(0b00110)),
        "v7" => Ok(vregidx(0b00111)),
        "v8" => Ok(vregidx(0b01000)),
        "v9" => Ok(vregidx(0b01001)),
        "v10" => Ok(vregidx(0b01010)),
        "v11" => Ok(vregidx(0b01011)),
        "v12" => Ok(vregidx(0b01100)),
        "v13" => Ok(vregidx(0b01101)),
        "v14" => Ok(vregidx(0b01110)),
        "v15" => Ok(vregidx(0b01111)),
        "v16" => Ok(vregidx(0b10000)),
        "v17" => Ok(vregidx(0b10001)),
        "v18" => Ok(vregidx(0b10010)),
        "v19" => Ok(vregidx(0b10011)),
        "v20" => Ok(vregidx(0b10100)),
        "v21" => Ok(vregidx(0b10101)),
        "v22" => Ok(vregidx(0b10110)),
        "v23" => Ok(vregidx(0b10111)),
        "v24" => Ok(vregidx(0b11000)),
        "v25" => Ok(vregidx(0b11001)),
        "v26" => Ok(vregidx(0b11010)),
        "v27" => Ok(vregidx(0b11011)),
        "v28" => Ok(vregidx(0b11100)),
        "v29" => Ok(vregidx(0b11101)),
        "v30" => Ok(vregidx(0b11110)),
        "v31" => Ok(vregidx(0b11111)),
        _ => err!("expected a vector register, got `{ident}`"),
    }
}

pub fn sp_reg_name(parser: &mut Parser) -> crate::Result<()> {
    let reg = reg_name(parser)?;

    if reg != sp {
        return err!("expected stack pointer (sp or x2)");
    }

    Ok(())
}

pub fn freg_or_reg_name(parser: &mut Parser) -> crate::Result<fregidx> {
    let ident = parser.expect_alpha_num()?;

    if let Some(reg) = reg_name_abi(ident) {
        Ok(fregidx(reg.0))
    } else if let Some(reg) = reg_name_arch(ident) {
        Ok(fregidx(reg.0))
    } else if let Some(reg) = freg_name_abi(ident) {
        Ok(reg)
    } else if let Some(reg) = freg_name_arch(ident) {
        Ok(reg)
    } else {
        err!("expected a GPR or FP register, got `{ident}`")
    }
}

pub fn maybe_vmask(p: &mut Parser) -> crate::Result<BitVector<1>> {
    if p.try_consume(",") {
        p.skip_ws();
        p.expect("v0.t")?;

        Ok(BitVector::<1>::new(0))
    } else {
        Ok(BitVector::<1>::new(1))
    }
}

pub fn frm_mnemonic(p: &mut Parser) -> crate::Result<rounding_mode> {
    let ident = p.expect_alpha_num()?;
    match ident {
        "rne" => Ok(rounding_mode::RM_RNE),
        "rtz" => Ok(rounding_mode::RM_RTZ),
        "rdn" => Ok(rounding_mode::RM_RDN),
        "rup" => Ok(rounding_mode::RM_RUP),
        "rmm" => Ok(rounding_mode::RM_RMM),
        "dyn" => Ok(rounding_mode::RM_DYN),
        _ => err!("`{ident}` is not a valid rouding mode"),
    }
}

pub fn resolve_label<const N: usize>(
    p: &mut Parser,
    l: &mut dyn LabelResolverTrait,
) -> crate::Result<BitVector<N>> {
    let ident = p.expect_label()?;

    let offset = if let Ok(offset) = parse_i64(ident) {
        offset
    } else {
        l.lookup(ident)
    };

    BitVector::<N>::try_from(offset as u32) // XXX
}

pub fn vtype_assembly(p: &mut Parser) -> crate::Result<u32> {
    p.skip_ws();
    let ident = p.expect_alpha_num()?;
    let sew = match ident {
        "e8" => 0b000,
        "e16" => 0b001,
        "e32" => 0b010,
        "e64" => 0b011,
        _ => return err!("`{ident}` is not a valid SEW"),
    };

    let mut lmul: Option<u32> = None;
    let mut mask: Option<u32> = None;
    let mut tail: Option<u32> = None;
    for _ in 0..3 {
        p.skip_ws();
        if !p.try_consume(",") {
            break;
        }
        p.skip_ws();

        match vtype_assembly_aux(p)? {
            (VTypeField::Lmul, val) => {
                if lmul.is_none() {
                    lmul = Some(val);
                } else {
                    return err!(
                        "unexpected LMUL; fields have be in the following order: SEW, LMUL, tail, mask"
                    );
                }
            }
            (VTypeField::Tail, val) => {
                if mask.is_none() {
                    tail = Some(val);
                    if lmul.is_none() {
                        lmul = Some(0);
                    }
                } else {
                    return err!(
                        "unexpected tail policy; fields have be in the follwing order: SEW, LMUL, tail, mask"
                    );
                }
            }
            (VTypeField::Mask, val) => {
                if mask.is_none() {
                    mask = Some(val);
                    if lmul.is_none() {
                        lmul = Some(0);
                    }
                    if tail.is_none() {
                        tail = Some(0);
                    }
                } else {
                    return err!(
                        "unexpected mask policy; fields have be in the follwing order: SEW, LMUL, tail, mask"
                    );
                }
            }
        }
    }

    let mask = mask.unwrap_or(0);
    let tail = tail.unwrap_or(0);
    let lmul = lmul.unwrap_or(0);

    Ok((mask << 7) | (tail << 6) | (sew << 3) | lmul)
}

enum VTypeField {
    Lmul,
    Mask,
    Tail,
}

fn vtype_assembly_aux(p: &mut Parser) -> crate::Result<(VTypeField, u32)> {
    let ident = p.expect_alpha_num()?;
    match ident {
        "mf8" => Ok((VTypeField::Lmul, 0b101)),
        "mf4" => Ok((VTypeField::Lmul, 0b110)),
        "mf2" => Ok((VTypeField::Lmul, 0b111)),
        "m1" => Ok((VTypeField::Lmul, 0b000)),
        "m2" => Ok((VTypeField::Lmul, 0b001)),
        "m4" => Ok((VTypeField::Lmul, 0b010)),
        "m8" => Ok((VTypeField::Lmul, 0b011)),

        "ma" => Ok((VTypeField::Mask, 1)),
        "mu" => Ok((VTypeField::Mask, 0)),

        "ta" => Ok((VTypeField::Tail, 1)),
        "tu" => Ok((VTypeField::Tail, 0)),

        _ => err!("`{ident}` is not a valid LMUL, mask or tail agnostic symbol"),
    }
}
