// Code generated automatically; DO NOT EDIT
use crate::err;
use crate::helpers::*;
pub fn assemble(s: &str, ctx: &Context, l: &mut dyn LabelResolverTrait) -> crate::Result<u32> {
    let mut p = Parser::new(s);
    let ident = p.consume_instruction();
    match ident {
        "add" => parse_add(&mut p),
        "add.uw" => parse_add_uw(&mut p, ctx),
        "addi" => parse_addi(&mut p),
        "addiw" => parse_addiw(&mut p, ctx),
        "addw" => parse_addw(&mut p, ctx),
        "aes32dsi" => parse_aes32dsi(&mut p),
        "aes32dsmi" => parse_aes32dsmi(&mut p),
        "aes32esi" => parse_aes32esi(&mut p),
        "aes32esmi" => parse_aes32esmi(&mut p),
        "aes64ds" => parse_aes64ds(&mut p),
        "aes64dsm" => parse_aes64dsm(&mut p),
        "aes64es" => parse_aes64es(&mut p),
        "aes64esm" => parse_aes64esm(&mut p),
        "aes64im" => parse_aes64im(&mut p),
        "aes64ks1i" => parse_aes64ks1i(&mut p),
        "aes64ks2" => parse_aes64ks2(&mut p),
        "amoadd.b" => parse_amoadd_b(&mut p),
        "amoadd.b.aq" => parse_amoadd_b_aq(&mut p),
        "amoadd.b.aqrl" => parse_amoadd_b_aqrl(&mut p),
        "amoadd.b.rl" => parse_amoadd_b_rl(&mut p),
        "amoadd.d" => parse_amoadd_d(&mut p),
        "amoadd.d.aq" => parse_amoadd_d_aq(&mut p),
        "amoadd.d.aqrl" => parse_amoadd_d_aqrl(&mut p),
        "amoadd.d.rl" => parse_amoadd_d_rl(&mut p),
        "amoadd.h" => parse_amoadd_h(&mut p),
        "amoadd.h.aq" => parse_amoadd_h_aq(&mut p),
        "amoadd.h.aqrl" => parse_amoadd_h_aqrl(&mut p),
        "amoadd.h.rl" => parse_amoadd_h_rl(&mut p),
        "amoadd.q" => parse_amoadd_q(&mut p),
        "amoadd.q.aq" => parse_amoadd_q_aq(&mut p),
        "amoadd.q.aqrl" => parse_amoadd_q_aqrl(&mut p),
        "amoadd.q.rl" => parse_amoadd_q_rl(&mut p),
        "amoadd.w" => parse_amoadd_w(&mut p),
        "amoadd.w.aq" => parse_amoadd_w_aq(&mut p),
        "amoadd.w.aqrl" => parse_amoadd_w_aqrl(&mut p),
        "amoadd.w.rl" => parse_amoadd_w_rl(&mut p),
        "amoand.b" => parse_amoand_b(&mut p),
        "amoand.b.aq" => parse_amoand_b_aq(&mut p),
        "amoand.b.aqrl" => parse_amoand_b_aqrl(&mut p),
        "amoand.b.rl" => parse_amoand_b_rl(&mut p),
        "amoand.d" => parse_amoand_d(&mut p),
        "amoand.d.aq" => parse_amoand_d_aq(&mut p),
        "amoand.d.aqrl" => parse_amoand_d_aqrl(&mut p),
        "amoand.d.rl" => parse_amoand_d_rl(&mut p),
        "amoand.h" => parse_amoand_h(&mut p),
        "amoand.h.aq" => parse_amoand_h_aq(&mut p),
        "amoand.h.aqrl" => parse_amoand_h_aqrl(&mut p),
        "amoand.h.rl" => parse_amoand_h_rl(&mut p),
        "amoand.q" => parse_amoand_q(&mut p),
        "amoand.q.aq" => parse_amoand_q_aq(&mut p),
        "amoand.q.aqrl" => parse_amoand_q_aqrl(&mut p),
        "amoand.q.rl" => parse_amoand_q_rl(&mut p),
        "amoand.w" => parse_amoand_w(&mut p),
        "amoand.w.aq" => parse_amoand_w_aq(&mut p),
        "amoand.w.aqrl" => parse_amoand_w_aqrl(&mut p),
        "amoand.w.rl" => parse_amoand_w_rl(&mut p),
        "amocas.b" => parse_amocas_b(&mut p),
        "amocas.b.aq" => parse_amocas_b_aq(&mut p),
        "amocas.b.aqrl" => parse_amocas_b_aqrl(&mut p),
        "amocas.b.rl" => parse_amocas_b_rl(&mut p),
        "amocas.d" => parse_amocas_d(&mut p),
        "amocas.d.aq" => parse_amocas_d_aq(&mut p),
        "amocas.d.aqrl" => parse_amocas_d_aqrl(&mut p),
        "amocas.d.rl" => parse_amocas_d_rl(&mut p),
        "amocas.h" => parse_amocas_h(&mut p),
        "amocas.h.aq" => parse_amocas_h_aq(&mut p),
        "amocas.h.aqrl" => parse_amocas_h_aqrl(&mut p),
        "amocas.h.rl" => parse_amocas_h_rl(&mut p),
        "amocas.q" => parse_amocas_q(&mut p),
        "amocas.q.aq" => parse_amocas_q_aq(&mut p),
        "amocas.q.aqrl" => parse_amocas_q_aqrl(&mut p),
        "amocas.q.rl" => parse_amocas_q_rl(&mut p),
        "amocas.w" => parse_amocas_w(&mut p),
        "amocas.w.aq" => parse_amocas_w_aq(&mut p),
        "amocas.w.aqrl" => parse_amocas_w_aqrl(&mut p),
        "amocas.w.rl" => parse_amocas_w_rl(&mut p),
        "amomax.b" => parse_amomax_b(&mut p),
        "amomax.b.aq" => parse_amomax_b_aq(&mut p),
        "amomax.b.aqrl" => parse_amomax_b_aqrl(&mut p),
        "amomax.b.rl" => parse_amomax_b_rl(&mut p),
        "amomax.d" => parse_amomax_d(&mut p),
        "amomax.d.aq" => parse_amomax_d_aq(&mut p),
        "amomax.d.aqrl" => parse_amomax_d_aqrl(&mut p),
        "amomax.d.rl" => parse_amomax_d_rl(&mut p),
        "amomax.h" => parse_amomax_h(&mut p),
        "amomax.h.aq" => parse_amomax_h_aq(&mut p),
        "amomax.h.aqrl" => parse_amomax_h_aqrl(&mut p),
        "amomax.h.rl" => parse_amomax_h_rl(&mut p),
        "amomax.q" => parse_amomax_q(&mut p),
        "amomax.q.aq" => parse_amomax_q_aq(&mut p),
        "amomax.q.aqrl" => parse_amomax_q_aqrl(&mut p),
        "amomax.q.rl" => parse_amomax_q_rl(&mut p),
        "amomax.w" => parse_amomax_w(&mut p),
        "amomax.w.aq" => parse_amomax_w_aq(&mut p),
        "amomax.w.aqrl" => parse_amomax_w_aqrl(&mut p),
        "amomax.w.rl" => parse_amomax_w_rl(&mut p),
        "amomaxu.b" => parse_amomaxu_b(&mut p),
        "amomaxu.b.aq" => parse_amomaxu_b_aq(&mut p),
        "amomaxu.b.aqrl" => parse_amomaxu_b_aqrl(&mut p),
        "amomaxu.b.rl" => parse_amomaxu_b_rl(&mut p),
        "amomaxu.d" => parse_amomaxu_d(&mut p),
        "amomaxu.d.aq" => parse_amomaxu_d_aq(&mut p),
        "amomaxu.d.aqrl" => parse_amomaxu_d_aqrl(&mut p),
        "amomaxu.d.rl" => parse_amomaxu_d_rl(&mut p),
        "amomaxu.h" => parse_amomaxu_h(&mut p),
        "amomaxu.h.aq" => parse_amomaxu_h_aq(&mut p),
        "amomaxu.h.aqrl" => parse_amomaxu_h_aqrl(&mut p),
        "amomaxu.h.rl" => parse_amomaxu_h_rl(&mut p),
        "amomaxu.q" => parse_amomaxu_q(&mut p),
        "amomaxu.q.aq" => parse_amomaxu_q_aq(&mut p),
        "amomaxu.q.aqrl" => parse_amomaxu_q_aqrl(&mut p),
        "amomaxu.q.rl" => parse_amomaxu_q_rl(&mut p),
        "amomaxu.w" => parse_amomaxu_w(&mut p),
        "amomaxu.w.aq" => parse_amomaxu_w_aq(&mut p),
        "amomaxu.w.aqrl" => parse_amomaxu_w_aqrl(&mut p),
        "amomaxu.w.rl" => parse_amomaxu_w_rl(&mut p),
        "amomin.b" => parse_amomin_b(&mut p),
        "amomin.b.aq" => parse_amomin_b_aq(&mut p),
        "amomin.b.aqrl" => parse_amomin_b_aqrl(&mut p),
        "amomin.b.rl" => parse_amomin_b_rl(&mut p),
        "amomin.d" => parse_amomin_d(&mut p),
        "amomin.d.aq" => parse_amomin_d_aq(&mut p),
        "amomin.d.aqrl" => parse_amomin_d_aqrl(&mut p),
        "amomin.d.rl" => parse_amomin_d_rl(&mut p),
        "amomin.h" => parse_amomin_h(&mut p),
        "amomin.h.aq" => parse_amomin_h_aq(&mut p),
        "amomin.h.aqrl" => parse_amomin_h_aqrl(&mut p),
        "amomin.h.rl" => parse_amomin_h_rl(&mut p),
        "amomin.q" => parse_amomin_q(&mut p),
        "amomin.q.aq" => parse_amomin_q_aq(&mut p),
        "amomin.q.aqrl" => parse_amomin_q_aqrl(&mut p),
        "amomin.q.rl" => parse_amomin_q_rl(&mut p),
        "amomin.w" => parse_amomin_w(&mut p),
        "amomin.w.aq" => parse_amomin_w_aq(&mut p),
        "amomin.w.aqrl" => parse_amomin_w_aqrl(&mut p),
        "amomin.w.rl" => parse_amomin_w_rl(&mut p),
        "amominu.b" => parse_amominu_b(&mut p),
        "amominu.b.aq" => parse_amominu_b_aq(&mut p),
        "amominu.b.aqrl" => parse_amominu_b_aqrl(&mut p),
        "amominu.b.rl" => parse_amominu_b_rl(&mut p),
        "amominu.d" => parse_amominu_d(&mut p),
        "amominu.d.aq" => parse_amominu_d_aq(&mut p),
        "amominu.d.aqrl" => parse_amominu_d_aqrl(&mut p),
        "amominu.d.rl" => parse_amominu_d_rl(&mut p),
        "amominu.h" => parse_amominu_h(&mut p),
        "amominu.h.aq" => parse_amominu_h_aq(&mut p),
        "amominu.h.aqrl" => parse_amominu_h_aqrl(&mut p),
        "amominu.h.rl" => parse_amominu_h_rl(&mut p),
        "amominu.q" => parse_amominu_q(&mut p),
        "amominu.q.aq" => parse_amominu_q_aq(&mut p),
        "amominu.q.aqrl" => parse_amominu_q_aqrl(&mut p),
        "amominu.q.rl" => parse_amominu_q_rl(&mut p),
        "amominu.w" => parse_amominu_w(&mut p),
        "amominu.w.aq" => parse_amominu_w_aq(&mut p),
        "amominu.w.aqrl" => parse_amominu_w_aqrl(&mut p),
        "amominu.w.rl" => parse_amominu_w_rl(&mut p),
        "amoor.b" => parse_amoor_b(&mut p),
        "amoor.b.aq" => parse_amoor_b_aq(&mut p),
        "amoor.b.aqrl" => parse_amoor_b_aqrl(&mut p),
        "amoor.b.rl" => parse_amoor_b_rl(&mut p),
        "amoor.d" => parse_amoor_d(&mut p),
        "amoor.d.aq" => parse_amoor_d_aq(&mut p),
        "amoor.d.aqrl" => parse_amoor_d_aqrl(&mut p),
        "amoor.d.rl" => parse_amoor_d_rl(&mut p),
        "amoor.h" => parse_amoor_h(&mut p),
        "amoor.h.aq" => parse_amoor_h_aq(&mut p),
        "amoor.h.aqrl" => parse_amoor_h_aqrl(&mut p),
        "amoor.h.rl" => parse_amoor_h_rl(&mut p),
        "amoor.q" => parse_amoor_q(&mut p),
        "amoor.q.aq" => parse_amoor_q_aq(&mut p),
        "amoor.q.aqrl" => parse_amoor_q_aqrl(&mut p),
        "amoor.q.rl" => parse_amoor_q_rl(&mut p),
        "amoor.w" => parse_amoor_w(&mut p),
        "amoor.w.aq" => parse_amoor_w_aq(&mut p),
        "amoor.w.aqrl" => parse_amoor_w_aqrl(&mut p),
        "amoor.w.rl" => parse_amoor_w_rl(&mut p),
        "amoswap.b" => parse_amoswap_b(&mut p),
        "amoswap.b.aq" => parse_amoswap_b_aq(&mut p),
        "amoswap.b.aqrl" => parse_amoswap_b_aqrl(&mut p),
        "amoswap.b.rl" => parse_amoswap_b_rl(&mut p),
        "amoswap.d" => parse_amoswap_d(&mut p),
        "amoswap.d.aq" => parse_amoswap_d_aq(&mut p),
        "amoswap.d.aqrl" => parse_amoswap_d_aqrl(&mut p),
        "amoswap.d.rl" => parse_amoswap_d_rl(&mut p),
        "amoswap.h" => parse_amoswap_h(&mut p),
        "amoswap.h.aq" => parse_amoswap_h_aq(&mut p),
        "amoswap.h.aqrl" => parse_amoswap_h_aqrl(&mut p),
        "amoswap.h.rl" => parse_amoswap_h_rl(&mut p),
        "amoswap.q" => parse_amoswap_q(&mut p),
        "amoswap.q.aq" => parse_amoswap_q_aq(&mut p),
        "amoswap.q.aqrl" => parse_amoswap_q_aqrl(&mut p),
        "amoswap.q.rl" => parse_amoswap_q_rl(&mut p),
        "amoswap.w" => parse_amoswap_w(&mut p),
        "amoswap.w.aq" => parse_amoswap_w_aq(&mut p),
        "amoswap.w.aqrl" => parse_amoswap_w_aqrl(&mut p),
        "amoswap.w.rl" => parse_amoswap_w_rl(&mut p),
        "amoxor.b" => parse_amoxor_b(&mut p),
        "amoxor.b.aq" => parse_amoxor_b_aq(&mut p),
        "amoxor.b.aqrl" => parse_amoxor_b_aqrl(&mut p),
        "amoxor.b.rl" => parse_amoxor_b_rl(&mut p),
        "amoxor.d" => parse_amoxor_d(&mut p),
        "amoxor.d.aq" => parse_amoxor_d_aq(&mut p),
        "amoxor.d.aqrl" => parse_amoxor_d_aqrl(&mut p),
        "amoxor.d.rl" => parse_amoxor_d_rl(&mut p),
        "amoxor.h" => parse_amoxor_h(&mut p),
        "amoxor.h.aq" => parse_amoxor_h_aq(&mut p),
        "amoxor.h.aqrl" => parse_amoxor_h_aqrl(&mut p),
        "amoxor.h.rl" => parse_amoxor_h_rl(&mut p),
        "amoxor.q" => parse_amoxor_q(&mut p),
        "amoxor.q.aq" => parse_amoxor_q_aq(&mut p),
        "amoxor.q.aqrl" => parse_amoxor_q_aqrl(&mut p),
        "amoxor.q.rl" => parse_amoxor_q_rl(&mut p),
        "amoxor.w" => parse_amoxor_w(&mut p),
        "amoxor.w.aq" => parse_amoxor_w_aq(&mut p),
        "amoxor.w.aqrl" => parse_amoxor_w_aqrl(&mut p),
        "amoxor.w.rl" => parse_amoxor_w_rl(&mut p),
        "and" => parse_and(&mut p),
        "andi" => parse_andi(&mut p),
        "andn" => parse_andn(&mut p, ctx),
        "auipc" => parse_auipc(&mut p),
        "bclr" => parse_bclr(&mut p, ctx),
        "bclri" => parse_bclri(&mut p, ctx),
        "bext" => parse_bext(&mut p, ctx),
        "bexti" => parse_bexti(&mut p, ctx),
        "binv" => parse_binv(&mut p, ctx),
        "binvi" => parse_binvi(&mut p, ctx),
        "brev8" => parse_brev8(&mut p),
        "bset" => parse_bset(&mut p, ctx),
        "bseti" => parse_bseti(&mut p, ctx),
        "c.add" => parse_c_add(&mut p),
        "c.addi" => parse_c_addi(&mut p),
        "c.addi16sp" => parse_c_addi16sp(&mut p),
        "c.addi4spn" => parse_c_addi4spn(&mut p),
        "c.addiw" => parse_c_addiw(&mut p, ctx),
        "c.addw" => parse_c_addw(&mut p, ctx),
        "c.and" => parse_c_and(&mut p),
        "c.andi" => parse_c_andi(&mut p),
        "c.beqz" => parse_c_beqz(&mut p),
        "c.bnez" => parse_c_bnez(&mut p),
        "c.ebreak" => parse_c_ebreak(),
        "c.fld" => parse_c_fld(&mut p),
        "c.fldsp" => parse_c_fldsp(&mut p),
        "c.flw" => parse_c_flw(&mut p, ctx),
        "c.flwsp" => parse_c_flwsp(&mut p, ctx),
        "c.fsd" => parse_c_fsd(&mut p),
        "c.fsdsp" => parse_c_fsdsp(&mut p),
        "c.fsw" => parse_c_fsw(&mut p, ctx),
        "c.fswsp" => parse_c_fswsp(&mut p, ctx),
        "c.illegal" => parse_c_illegal(&mut p),
        "c.j" => parse_c_j(&mut p),
        "c.jal" => parse_c_jal(&mut p, ctx),
        "c.jalr" => parse_c_jalr(&mut p),
        "c.jr" => parse_c_jr(&mut p),
        "c.lbu" => parse_c_lbu(&mut p),
        "c.ld" => parse_c_ld(&mut p, ctx),
        "c.ldsp" => parse_c_ldsp(&mut p, ctx),
        "c.lh" => parse_c_lh(&mut p),
        "c.lhu" => parse_c_lhu(&mut p),
        "c.li" => parse_c_li(&mut p),
        "c.lui" => parse_c_lui(&mut p),
        "c.lw" => parse_c_lw(&mut p),
        "c.lwsp" => parse_c_lwsp(&mut p),
        "c.mop.1" => parse_c_mop_1(),
        "c.mop.11" => parse_c_mop_11(),
        "c.mop.13" => parse_c_mop_13(),
        "c.mop.15" => parse_c_mop_15(),
        "c.mop.3" => parse_c_mop_3(),
        "c.mop.5" => parse_c_mop_5(),
        "c.mop.7" => parse_c_mop_7(),
        "c.mop.9" => parse_c_mop_9(),
        "c.mul" => parse_c_mul(&mut p),
        "c.mv" => parse_c_mv(&mut p),
        "c.nop" => parse_c_nop(&mut p),
        "c.not" => parse_c_not(&mut p),
        "c.ntl.all" => parse_c_ntl_all(),
        "c.ntl.p1" => parse_c_ntl_p1(),
        "c.ntl.pall" => parse_c_ntl_pall(),
        "c.ntl.s1" => parse_c_ntl_s1(),
        "c.or" => parse_c_or(&mut p),
        "c.sb" => parse_c_sb(&mut p),
        "c.sd" => parse_c_sd(&mut p, ctx),
        "c.sdsp" => parse_c_sdsp(&mut p, ctx),
        "c.sext.b" => parse_c_sext_b(&mut p),
        "c.sext.h" => parse_c_sext_h(&mut p),
        "c.sh" => parse_c_sh(&mut p),
        "c.slli" => parse_c_slli(&mut p),
        "c.srai" => parse_c_srai(&mut p),
        "c.srli" => parse_c_srli(&mut p),
        "c.sub" => parse_c_sub(&mut p),
        "c.subw" => parse_c_subw(&mut p, ctx),
        "c.sw" => parse_c_sw(&mut p),
        "c.swsp" => parse_c_swsp(&mut p),
        "c.xor" => parse_c_xor(&mut p),
        "c.zext.b" => parse_c_zext_b(&mut p),
        "c.zext.h" => parse_c_zext_h(&mut p),
        "c.zext.w" => parse_c_zext_w(&mut p),
        "cbo.clean" => parse_cbo_clean(&mut p),
        "cbo.flush" => parse_cbo_flush(&mut p),
        "cbo.inval" => parse_cbo_inval(&mut p),
        "cbo.zero" => parse_cbo_zero(&mut p),
        "clmul" => parse_clmul(&mut p),
        "clmulh" => parse_clmulh(&mut p),
        "clmulr" => parse_clmulr(&mut p),
        "clz" => parse_clz(&mut p),
        "clzw" => parse_clzw(&mut p),
        "cpop" => parse_cpop(&mut p),
        "cpopw" => parse_cpopw(&mut p),
        "csrrc" => parse_csrrc(&mut p),
        "csrrci" => parse_csrrci(&mut p),
        "csrrs" => parse_csrrs(&mut p),
        "csrrsi" => parse_csrrsi(&mut p),
        "csrrw" => parse_csrrw(&mut p),
        "csrrwi" => parse_csrrwi(&mut p),
        "ctz" => parse_ctz(&mut p),
        "ctzw" => parse_ctzw(&mut p),
        "czero.eqz" => parse_czero_eqz(&mut p),
        "czero.nez" => parse_czero_nez(&mut p),
        "div" => parse_div(&mut p),
        "divu" => parse_divu(&mut p),
        "divuw" => parse_divuw(&mut p, ctx),
        "divw" => parse_divw(&mut p, ctx),
        "ebreak" => parse_ebreak(),
        "ecall" => parse_ecall(),
        "fadd.d" => parse_fadd_d(&mut p, ctx),
        "fadd.h" => parse_fadd_h(&mut p, ctx),
        "fadd.s" => parse_fadd_s(&mut p, ctx),
        "fclass.d" => parse_fclass_d(&mut p, ctx),
        "fclass.h" => parse_fclass_h(&mut p, ctx),
        "fclass.s" => parse_fclass_s(&mut p, ctx),
        "fcvt.bf16.s" => parse_fcvt_bf16_s(&mut p),
        "fcvt.d.h" => parse_fcvt_d_h(&mut p, ctx),
        "fcvt.d.l" => parse_fcvt_d_l(&mut p, ctx),
        "fcvt.d.lu" => parse_fcvt_d_lu(&mut p, ctx),
        "fcvt.d.s" => parse_fcvt_d_s(&mut p, ctx),
        "fcvt.d.w" => parse_fcvt_d_w(&mut p, ctx),
        "fcvt.d.wu" => parse_fcvt_d_wu(&mut p, ctx),
        "fcvt.h.d" => parse_fcvt_h_d(&mut p, ctx),
        "fcvt.h.l" => parse_fcvt_h_l(&mut p, ctx),
        "fcvt.h.lu" => parse_fcvt_h_lu(&mut p, ctx),
        "fcvt.h.s" => parse_fcvt_h_s(&mut p, ctx),
        "fcvt.h.w" => parse_fcvt_h_w(&mut p, ctx),
        "fcvt.h.wu" => parse_fcvt_h_wu(&mut p, ctx),
        "fcvt.l.d" => parse_fcvt_l_d(&mut p, ctx),
        "fcvt.l.h" => parse_fcvt_l_h(&mut p, ctx),
        "fcvt.l.s" => parse_fcvt_l_s(&mut p, ctx),
        "fcvt.lu.d" => parse_fcvt_lu_d(&mut p, ctx),
        "fcvt.lu.h" => parse_fcvt_lu_h(&mut p, ctx),
        "fcvt.lu.s" => parse_fcvt_lu_s(&mut p, ctx),
        "fcvt.s.bf16" => parse_fcvt_s_bf16(&mut p),
        "fcvt.s.d" => parse_fcvt_s_d(&mut p, ctx),
        "fcvt.s.h" => parse_fcvt_s_h(&mut p, ctx),
        "fcvt.s.l" => parse_fcvt_s_l(&mut p, ctx),
        "fcvt.s.lu" => parse_fcvt_s_lu(&mut p, ctx),
        "fcvt.s.w" => parse_fcvt_s_w(&mut p, ctx),
        "fcvt.s.wu" => parse_fcvt_s_wu(&mut p, ctx),
        "fcvt.w.d" => parse_fcvt_w_d(&mut p, ctx),
        "fcvt.w.h" => parse_fcvt_w_h(&mut p, ctx),
        "fcvt.w.s" => parse_fcvt_w_s(&mut p, ctx),
        "fcvt.wu.d" => parse_fcvt_wu_d(&mut p, ctx),
        "fcvt.wu.h" => parse_fcvt_wu_h(&mut p, ctx),
        "fcvt.wu.s" => parse_fcvt_wu_s(&mut p, ctx),
        "fcvtmod.w.d" => parse_fcvtmod_w_d(&mut p),
        "fdiv.d" => parse_fdiv_d(&mut p, ctx),
        "fdiv.h" => parse_fdiv_h(&mut p, ctx),
        "fdiv.s" => parse_fdiv_s(&mut p, ctx),
        "fence.tso" => parse_fence_tso(),
        "feq.d" => parse_feq_d(&mut p, ctx),
        "feq.h" => parse_feq_h(&mut p, ctx),
        "feq.s" => parse_feq_s(&mut p, ctx),
        "flb" => parse_flb(&mut p),
        "fld" => parse_fld(&mut p),
        "fle.d" => parse_fle_d(&mut p, ctx),
        "fle.h" => parse_fle_h(&mut p, ctx),
        "fle.s" => parse_fle_s(&mut p, ctx),
        "fleq.d" => parse_fleq_d(&mut p),
        "fleq.h" => parse_fleq_h(&mut p),
        "fleq.s" => parse_fleq_s(&mut p),
        "flh" => parse_flh(&mut p),
        "fli.d" => parse_fli_d(&mut p),
        "fli.h" => parse_fli_h(&mut p),
        "fli.s" => parse_fli_s(&mut p),
        "flt.d" => parse_flt_d(&mut p, ctx),
        "flt.h" => parse_flt_h(&mut p, ctx),
        "flt.s" => parse_flt_s(&mut p, ctx),
        "fltq.d" => parse_fltq_d(&mut p),
        "fltq.h" => parse_fltq_h(&mut p),
        "fltq.s" => parse_fltq_s(&mut p),
        "flw" => parse_flw(&mut p),
        "fmadd.d" => parse_fmadd_d(&mut p, ctx),
        "fmadd.h" => parse_fmadd_h(&mut p, ctx),
        "fmadd.s" => parse_fmadd_s(&mut p, ctx),
        "fmax.d" => parse_fmax_d(&mut p, ctx),
        "fmax.h" => parse_fmax_h(&mut p, ctx),
        "fmax.s" => parse_fmax_s(&mut p, ctx),
        "fmaxm.d" => parse_fmaxm_d(&mut p),
        "fmaxm.h" => parse_fmaxm_h(&mut p),
        "fmaxm.s" => parse_fmaxm_s(&mut p),
        "fmin.d" => parse_fmin_d(&mut p, ctx),
        "fmin.h" => parse_fmin_h(&mut p, ctx),
        "fmin.s" => parse_fmin_s(&mut p, ctx),
        "fminm.d" => parse_fminm_d(&mut p),
        "fminm.h" => parse_fminm_h(&mut p),
        "fminm.s" => parse_fminm_s(&mut p),
        "fmsub.d" => parse_fmsub_d(&mut p, ctx),
        "fmsub.h" => parse_fmsub_h(&mut p, ctx),
        "fmsub.s" => parse_fmsub_s(&mut p, ctx),
        "fmul.d" => parse_fmul_d(&mut p, ctx),
        "fmul.h" => parse_fmul_h(&mut p, ctx),
        "fmul.s" => parse_fmul_s(&mut p, ctx),
        "fmv.d.x" => parse_fmv_d_x(&mut p),
        "fmv.h.x" => parse_fmv_h_x(&mut p),
        "fmv.w.x" => parse_fmv_w_x(&mut p),
        "fmv.x.d" => parse_fmv_x_d(&mut p, ctx),
        "fmv.x.h" => parse_fmv_x_h(&mut p, ctx),
        "fmv.x.w" => parse_fmv_x_w(&mut p, ctx),
        "fmvh.x.d" => parse_fmvh_x_d(&mut p),
        "fmvp.d.x" => parse_fmvp_d_x(&mut p),
        "fnmadd.d" => parse_fnmadd_d(&mut p, ctx),
        "fnmadd.h" => parse_fnmadd_h(&mut p, ctx),
        "fnmadd.s" => parse_fnmadd_s(&mut p, ctx),
        "fnmsub.d" => parse_fnmsub_d(&mut p, ctx),
        "fnmsub.h" => parse_fnmsub_h(&mut p, ctx),
        "fnmsub.s" => parse_fnmsub_s(&mut p, ctx),
        "fround.d" => parse_fround_d(&mut p),
        "fround.h" => parse_fround_h(&mut p),
        "fround.s" => parse_fround_s(&mut p),
        "froundnx.d" => parse_froundnx_d(&mut p),
        "froundnx.h" => parse_froundnx_h(&mut p),
        "froundnx.s" => parse_froundnx_s(&mut p),
        "fsb" => parse_fsb(&mut p),
        "fsd" => parse_fsd(&mut p),
        "fsgnj.d" => parse_fsgnj_d(&mut p, ctx),
        "fsgnj.h" => parse_fsgnj_h(&mut p, ctx),
        "fsgnj.s" => parse_fsgnj_s(&mut p, ctx),
        "fsgnjn.d" => parse_fsgnjn_d(&mut p, ctx),
        "fsgnjn.h" => parse_fsgnjn_h(&mut p, ctx),
        "fsgnjn.s" => parse_fsgnjn_s(&mut p, ctx),
        "fsgnjx.d" => parse_fsgnjx_d(&mut p, ctx),
        "fsgnjx.h" => parse_fsgnjx_h(&mut p, ctx),
        "fsgnjx.s" => parse_fsgnjx_s(&mut p, ctx),
        "fsh" => parse_fsh(&mut p),
        "fsqrt.d" => parse_fsqrt_d(&mut p, ctx),
        "fsqrt.h" => parse_fsqrt_h(&mut p, ctx),
        "fsqrt.s" => parse_fsqrt_s(&mut p),
        "fsub.d" => parse_fsub_d(&mut p, ctx),
        "fsub.h" => parse_fsub_h(&mut p, ctx),
        "fsub.s" => parse_fsub_s(&mut p, ctx),
        "fsw" => parse_fsw(&mut p),
        "illegal" => parse_illegal(&mut p),
        "jalr" => parse_jalr(&mut p),
        "lb" => parse_lb(&mut p),
        "lbu" => parse_lbu(&mut p),
        "ld" => parse_ld(&mut p),
        "ldu" => parse_ldu(&mut p),
        "lh" => parse_lh(&mut p),
        "lhu" => parse_lhu(&mut p),
        "lpad" => parse_lpad(&mut p),
        "lr.b" => parse_lr_b(&mut p),
        "lr.b.aq" => parse_lr_b_aq(&mut p),
        "lr.b.aqrl" => parse_lr_b_aqrl(&mut p),
        "lr.b.rl" => parse_lr_b_rl(&mut p),
        "lr.d" => parse_lr_d(&mut p),
        "lr.d.aq" => parse_lr_d_aq(&mut p),
        "lr.d.aqrl" => parse_lr_d_aqrl(&mut p),
        "lr.d.rl" => parse_lr_d_rl(&mut p),
        "lr.h" => parse_lr_h(&mut p),
        "lr.h.aq" => parse_lr_h_aq(&mut p),
        "lr.h.aqrl" => parse_lr_h_aqrl(&mut p),
        "lr.h.rl" => parse_lr_h_rl(&mut p),
        "lr.w" => parse_lr_w(&mut p),
        "lr.w.aq" => parse_lr_w_aq(&mut p),
        "lr.w.aqrl" => parse_lr_w_aqrl(&mut p),
        "lr.w.rl" => parse_lr_w_rl(&mut p),
        "lui" => parse_lui(&mut p),
        "lw" => parse_lw(&mut p),
        "lwu" => parse_lwu(&mut p),
        "max" => parse_max(&mut p, ctx),
        "maxu" => parse_maxu(&mut p, ctx),
        "min" => parse_min(&mut p, ctx),
        "minu" => parse_minu(&mut p, ctx),
        "mop.r.0" => parse_mop_r_0(&mut p),
        "mop.r.1" => parse_mop_r_1(&mut p),
        "mop.r.10" => parse_mop_r_10(&mut p),
        "mop.r.11" => parse_mop_r_11(&mut p),
        "mop.r.12" => parse_mop_r_12(&mut p),
        "mop.r.13" => parse_mop_r_13(&mut p),
        "mop.r.14" => parse_mop_r_14(&mut p),
        "mop.r.15" => parse_mop_r_15(&mut p),
        "mop.r.16" => parse_mop_r_16(&mut p),
        "mop.r.17" => parse_mop_r_17(&mut p),
        "mop.r.18" => parse_mop_r_18(&mut p),
        "mop.r.19" => parse_mop_r_19(&mut p),
        "mop.r.2" => parse_mop_r_2(&mut p),
        "mop.r.20" => parse_mop_r_20(&mut p),
        "mop.r.21" => parse_mop_r_21(&mut p),
        "mop.r.22" => parse_mop_r_22(&mut p),
        "mop.r.23" => parse_mop_r_23(&mut p),
        "mop.r.24" => parse_mop_r_24(&mut p),
        "mop.r.25" => parse_mop_r_25(&mut p),
        "mop.r.26" => parse_mop_r_26(&mut p),
        "mop.r.27" => parse_mop_r_27(&mut p),
        "mop.r.28" => parse_mop_r_28(&mut p),
        "mop.r.29" => parse_mop_r_29(&mut p),
        "mop.r.3" => parse_mop_r_3(&mut p),
        "mop.r.30" => parse_mop_r_30(&mut p),
        "mop.r.31" => parse_mop_r_31(&mut p),
        "mop.r.4" => parse_mop_r_4(&mut p),
        "mop.r.5" => parse_mop_r_5(&mut p),
        "mop.r.6" => parse_mop_r_6(&mut p),
        "mop.r.7" => parse_mop_r_7(&mut p),
        "mop.r.8" => parse_mop_r_8(&mut p),
        "mop.r.9" => parse_mop_r_9(&mut p),
        "mop.rr.0" => parse_mop_rr_0(&mut p),
        "mop.rr.1" => parse_mop_rr_1(&mut p),
        "mop.rr.2" => parse_mop_rr_2(&mut p),
        "mop.rr.3" => parse_mop_rr_3(&mut p),
        "mop.rr.4" => parse_mop_rr_4(&mut p),
        "mop.rr.5" => parse_mop_rr_5(&mut p),
        "mop.rr.6" => parse_mop_rr_6(&mut p),
        "mop.rr.7" => parse_mop_rr_7(&mut p),
        "mret" => parse_mret(),
        "mul" => parse_mul(&mut p),
        "mulh" => parse_mulh(&mut p),
        "mulhsu" => parse_mulhsu(&mut p),
        "mulhu" => parse_mulhu(&mut p),
        "mulw" => parse_mulw(&mut p, ctx),
        "mv" => parse_mv(&mut p),
        "ntl.all" => parse_ntl_all(),
        "ntl.p1" => parse_ntl_p1(),
        "ntl.pall" => parse_ntl_pall(),
        "ntl.s1" => parse_ntl_s1(),
        "or" => parse_or(&mut p),
        "orc.b" => parse_orc_b(&mut p),
        "ori" => parse_ori(&mut p),
        "orn" => parse_orn(&mut p, ctx),
        "pack" => parse_pack(&mut p, ctx),
        "packh" => parse_packh(&mut p, ctx),
        "packw" => parse_packw(&mut p),
        "pause" => parse_pause(),
        "prefetch.i" => parse_prefetch_i(&mut p),
        "prefetch.r" => parse_prefetch_r(&mut p),
        "prefetch.w" => parse_prefetch_w(&mut p),
        "rem" => parse_rem(&mut p),
        "remu" => parse_remu(&mut p),
        "remuw" => parse_remuw(&mut p, ctx),
        "remw" => parse_remw(&mut p, ctx),
        "rev8" => parse_rev8(&mut p, ctx),
        "rol" => parse_rol(&mut p, ctx),
        "rolw" => parse_rolw(&mut p, ctx),
        "ror" => parse_ror(&mut p, ctx),
        "rori" => parse_rori(&mut p),
        "roriw" => parse_roriw(&mut p),
        "rorw" => parse_rorw(&mut p, ctx),
        "sb" => parse_sb(&mut p),
        "sc.b" => parse_sc_b(&mut p),
        "sc.b.aq" => parse_sc_b_aq(&mut p),
        "sc.b.aqrl" => parse_sc_b_aqrl(&mut p),
        "sc.b.rl" => parse_sc_b_rl(&mut p),
        "sc.d" => parse_sc_d(&mut p),
        "sc.d.aq" => parse_sc_d_aq(&mut p),
        "sc.d.aqrl" => parse_sc_d_aqrl(&mut p),
        "sc.d.rl" => parse_sc_d_rl(&mut p),
        "sc.h" => parse_sc_h(&mut p),
        "sc.h.aq" => parse_sc_h_aq(&mut p),
        "sc.h.aqrl" => parse_sc_h_aqrl(&mut p),
        "sc.h.rl" => parse_sc_h_rl(&mut p),
        "sc.w" => parse_sc_w(&mut p),
        "sc.w.aq" => parse_sc_w_aq(&mut p),
        "sc.w.aqrl" => parse_sc_w_aqrl(&mut p),
        "sc.w.rl" => parse_sc_w_rl(&mut p),
        "sd" => parse_sd(&mut p),
        "sext.b" => parse_sext_b(&mut p, ctx),
        "sext.h" => parse_sext_h(&mut p, ctx),
        "sfence.inval.ir" => parse_sfence_inval_ir(),
        "sfence.vma" => parse_sfence_vma(&mut p),
        "sfence.w.inval" => parse_sfence_w_inval(),
        "sh" => parse_sh(&mut p),
        "sh1add" => parse_sh1add(&mut p),
        "sh1add.uw" => parse_sh1add_uw(&mut p, ctx),
        "sh2add" => parse_sh2add(&mut p),
        "sh2add.uw" => parse_sh2add_uw(&mut p, ctx),
        "sh3add" => parse_sh3add(&mut p),
        "sh3add.uw" => parse_sh3add_uw(&mut p, ctx),
        "sha256sig0" => parse_sha256sig0(&mut p),
        "sha256sig1" => parse_sha256sig1(&mut p),
        "sha256sum0" => parse_sha256sum0(&mut p),
        "sha256sum1" => parse_sha256sum1(&mut p),
        "sha512sig0" => parse_sha512sig0(&mut p),
        "sha512sig0h" => parse_sha512sig0h(&mut p),
        "sha512sig0l" => parse_sha512sig0l(&mut p),
        "sha512sig1" => parse_sha512sig1(&mut p),
        "sha512sig1h" => parse_sha512sig1h(&mut p),
        "sha512sig1l" => parse_sha512sig1l(&mut p),
        "sha512sum0" => parse_sha512sum0(&mut p),
        "sha512sum0r" => parse_sha512sum0r(&mut p),
        "sha512sum1" => parse_sha512sum1(&mut p),
        "sha512sum1r" => parse_sha512sum1r(&mut p),
        "sinval.vma" => parse_sinval_vma(&mut p),
        "sll" => parse_sll(&mut p),
        "slli" => parse_slli(&mut p),
        "slli.uw" => parse_slli_uw(&mut p),
        "slliw" => parse_slliw(&mut p, ctx),
        "sllw" => parse_sllw(&mut p, ctx),
        "slt" => parse_slt(&mut p),
        "slti" => parse_slti(&mut p),
        "sltiu" => parse_sltiu(&mut p),
        "sltu" => parse_sltu(&mut p),
        "sm3p0" => parse_sm3p0(&mut p),
        "sm3p1" => parse_sm3p1(&mut p),
        "sm4ed" => parse_sm4ed(&mut p),
        "sm4ks" => parse_sm4ks(&mut p),
        "sra" => parse_sra(&mut p),
        "srai" => parse_srai(&mut p),
        "sraiw" => parse_sraiw(&mut p, ctx),
        "sraw" => parse_sraw(&mut p, ctx),
        "sret" => parse_sret(),
        "srl" => parse_srl(&mut p),
        "srli" => parse_srli(&mut p),
        "srliw" => parse_srliw(&mut p, ctx),
        "srlw" => parse_srlw(&mut p, ctx),
        "sub" => parse_sub(&mut p),
        "subw" => parse_subw(&mut p, ctx),
        "sw" => parse_sw(&mut p),
        "unzip" => parse_unzip(&mut p),
        "vaadd.vv" => parse_vaadd_vv(&mut p),
        "vaadd.vx" => parse_vaadd_vx(&mut p),
        "vaaddu.vv" => parse_vaaddu_vv(&mut p),
        "vaaddu.vx" => parse_vaaddu_vx(&mut p),
        "vadc.vim" => parse_vadc_vim(&mut p),
        "vadc.vvm" => parse_vadc_vvm(&mut p),
        "vadc.vxm" => parse_vadc_vxm(&mut p),
        "vadd.vi" => parse_vadd_vi(&mut p),
        "vadd.vv" => parse_vadd_vv(&mut p),
        "vadd.vx" => parse_vadd_vx(&mut p),
        "vaesdf.vs" => parse_vaesdf_vs(&mut p),
        "vaesdf.vv" => parse_vaesdf_vv(&mut p),
        "vaesdm.vs" => parse_vaesdm_vs(&mut p),
        "vaesdm.vv" => parse_vaesdm_vv(&mut p),
        "vaesef.vs" => parse_vaesef_vs(&mut p),
        "vaesef.vv" => parse_vaesef_vv(&mut p),
        "vaesem.vs" => parse_vaesem_vs(&mut p),
        "vaesem.vv" => parse_vaesem_vv(&mut p),
        "vaeskf1.vi" => parse_vaeskf1_vi(&mut p),
        "vaeskf2.vi" => parse_vaeskf2_vi(&mut p),
        "vaesz.vs" => parse_vaesz_vs(&mut p),
        "vand.vi" => parse_vand_vi(&mut p),
        "vand.vv" => parse_vand_vv(&mut p),
        "vand.vx" => parse_vand_vx(&mut p),
        "vandn.vv" => parse_vandn_vv(&mut p),
        "vandn.vx" => parse_vandn_vx(&mut p),
        "vasub.vv" => parse_vasub_vv(&mut p),
        "vasub.vx" => parse_vasub_vx(&mut p),
        "vasubu.vv" => parse_vasubu_vv(&mut p),
        "vasubu.vx" => parse_vasubu_vx(&mut p),
        "vbrev.v" => parse_vbrev_v(&mut p),
        "vbrev8.v" => parse_vbrev8_v(&mut p),
        "vclmul.vv" => parse_vclmul_vv(&mut p),
        "vclmul.vx" => parse_vclmul_vx(&mut p),
        "vclmulh.vv" => parse_vclmulh_vv(&mut p),
        "vclmulh.vx" => parse_vclmulh_vx(&mut p),
        "vclz.v" => parse_vclz_v(&mut p),
        "vcompress.vm" => parse_vcompress_vm(&mut p),
        "vcpop.m" => parse_vcpop_m(&mut p),
        "vcpop.v" => parse_vcpop_v(&mut p),
        "vctz.v" => parse_vctz_v(&mut p),
        "vdiv.vv" => parse_vdiv_vv(&mut p),
        "vdiv.vx" => parse_vdiv_vx(&mut p),
        "vdivu.vv" => parse_vdivu_vv(&mut p),
        "vdivu.vx" => parse_vdivu_vx(&mut p),
        "vfadd.vf" => parse_vfadd_vf(&mut p),
        "vfadd.vv" => parse_vfadd_vv(&mut p),
        "vfclass.v" => parse_vfclass_v(&mut p),
        "vfcvt.f.x.v" => parse_vfcvt_f_x_v(&mut p),
        "vfcvt.f.xu.v" => parse_vfcvt_f_xu_v(&mut p),
        "vfcvt.rtz.x.f.v" => parse_vfcvt_rtz_x_f_v(&mut p),
        "vfcvt.rtz.xu.f.v" => parse_vfcvt_rtz_xu_f_v(&mut p),
        "vfcvt.x.f.v" => parse_vfcvt_x_f_v(&mut p),
        "vfcvt.xu.f.v" => parse_vfcvt_xu_f_v(&mut p),
        "vfdiv.vf" => parse_vfdiv_vf(&mut p),
        "vfdiv.vv" => parse_vfdiv_vv(&mut p),
        "vfirst.m" => parse_vfirst_m(&mut p),
        "vfmacc.vf" => parse_vfmacc_vf(&mut p),
        "vfmacc.vv" => parse_vfmacc_vv(&mut p),
        "vfmadd.vf" => parse_vfmadd_vf(&mut p),
        "vfmadd.vv" => parse_vfmadd_vv(&mut p),
        "vfmax.vf" => parse_vfmax_vf(&mut p),
        "vfmax.vv" => parse_vfmax_vv(&mut p),
        "vfmerge.vfm" => parse_vfmerge_vfm(&mut p),
        "vfmin.vf" => parse_vfmin_vf(&mut p),
        "vfmin.vv" => parse_vfmin_vv(&mut p),
        "vfmsac.vf" => parse_vfmsac_vf(&mut p),
        "vfmsac.vv" => parse_vfmsac_vv(&mut p),
        "vfmsub.vf" => parse_vfmsub_vf(&mut p),
        "vfmsub.vv" => parse_vfmsub_vv(&mut p),
        "vfmul.vf" => parse_vfmul_vf(&mut p),
        "vfmul.vv" => parse_vfmul_vv(&mut p),
        "vfmv.f.s" => parse_vfmv_f_s(&mut p),
        "vfmv.s.f" => parse_vfmv_s_f(&mut p),
        "vfmv.v.f" => parse_vfmv_v_f(&mut p),
        "vfncvt.f.f.w" => parse_vfncvt_f_f_w(&mut p),
        "vfncvt.f.x.w" => parse_vfncvt_f_x_w(&mut p),
        "vfncvt.f.xu.w" => parse_vfncvt_f_xu_w(&mut p),
        "vfncvt.rod.f.f.w" => parse_vfncvt_rod_f_f_w(&mut p),
        "vfncvt.rtz.x.f.w" => parse_vfncvt_rtz_x_f_w(&mut p),
        "vfncvt.rtz.xu.f.w" => parse_vfncvt_rtz_xu_f_w(&mut p),
        "vfncvt.x.f.w" => parse_vfncvt_x_f_w(&mut p),
        "vfncvt.xu.f.w" => parse_vfncvt_xu_f_w(&mut p),
        "vfncvtbf16.f.f.w" => parse_vfncvtbf16_f_f_w(&mut p),
        "vfnmacc.vf" => parse_vfnmacc_vf(&mut p),
        "vfnmacc.vv" => parse_vfnmacc_vv(&mut p),
        "vfnmadd.vf" => parse_vfnmadd_vf(&mut p),
        "vfnmadd.vv" => parse_vfnmadd_vv(&mut p),
        "vfnmsac.vf" => parse_vfnmsac_vf(&mut p),
        "vfnmsac.vv" => parse_vfnmsac_vv(&mut p),
        "vfnmsub.vf" => parse_vfnmsub_vf(&mut p),
        "vfnmsub.vv" => parse_vfnmsub_vv(&mut p),
        "vfrdiv.vf" => parse_vfrdiv_vf(&mut p),
        "vfrec7.v" => parse_vfrec7_v(&mut p),
        "vfredmax.vs" => parse_vfredmax_vs(&mut p),
        "vfredmin.vs" => parse_vfredmin_vs(&mut p),
        "vfredosum.vs" => parse_vfredosum_vs(&mut p),
        "vfredusum.vs" => parse_vfredusum_vs(&mut p),
        "vfrsqrt7.v" => parse_vfrsqrt7_v(&mut p),
        "vfrsub.vf" => parse_vfrsub_vf(&mut p),
        "vfsgnj.vf" => parse_vfsgnj_vf(&mut p),
        "vfsgnj.vv" => parse_vfsgnj_vv(&mut p),
        "vfsgnjn.vf" => parse_vfsgnjn_vf(&mut p),
        "vfsgnjn.vv" => parse_vfsgnjn_vv(&mut p),
        "vfsgnjx.vf" => parse_vfsgnjx_vf(&mut p),
        "vfsgnjx.vv" => parse_vfsgnjx_vv(&mut p),
        "vfslide1down.vf" => parse_vfslide1down_vf(&mut p),
        "vfslide1up.vf" => parse_vfslide1up_vf(&mut p),
        "vfsqrt.v" => parse_vfsqrt_v(&mut p),
        "vfsub.vf" => parse_vfsub_vf(&mut p),
        "vfsub.vv" => parse_vfsub_vv(&mut p),
        "vfwadd.vf" => parse_vfwadd_vf(&mut p),
        "vfwadd.vv" => parse_vfwadd_vv(&mut p),
        "vfwadd.wf" => parse_vfwadd_wf(&mut p),
        "vfwadd.wv" => parse_vfwadd_wv(&mut p),
        "vfwcvt.f.f.v" => parse_vfwcvt_f_f_v(&mut p),
        "vfwcvt.f.x.v" => parse_vfwcvt_f_x_v(&mut p),
        "vfwcvt.f.xu.v" => parse_vfwcvt_f_xu_v(&mut p),
        "vfwcvt.rtz.x.f.v" => parse_vfwcvt_rtz_x_f_v(&mut p),
        "vfwcvt.rtz.xu.f.v" => parse_vfwcvt_rtz_xu_f_v(&mut p),
        "vfwcvt.x.f.v" => parse_vfwcvt_x_f_v(&mut p),
        "vfwcvt.xu.f.v" => parse_vfwcvt_xu_f_v(&mut p),
        "vfwcvtbf16.f.f.v" => parse_vfwcvtbf16_f_f_v(&mut p),
        "vfwmacc.vf" => parse_vfwmacc_vf(&mut p),
        "vfwmacc.vv" => parse_vfwmacc_vv(&mut p),
        "vfwmaccbf16.vf" => parse_vfwmaccbf16_vf(&mut p),
        "vfwmaccbf16.vv" => parse_vfwmaccbf16_vv(&mut p),
        "vfwmsac.vf" => parse_vfwmsac_vf(&mut p),
        "vfwmsac.vv" => parse_vfwmsac_vv(&mut p),
        "vfwmul.vf" => parse_vfwmul_vf(&mut p),
        "vfwmul.vv" => parse_vfwmul_vv(&mut p),
        "vfwnmacc.vf" => parse_vfwnmacc_vf(&mut p),
        "vfwnmacc.vv" => parse_vfwnmacc_vv(&mut p),
        "vfwnmsac.vf" => parse_vfwnmsac_vf(&mut p),
        "vfwnmsac.vv" => parse_vfwnmsac_vv(&mut p),
        "vfwredosum.vs" => parse_vfwredosum_vs(&mut p),
        "vfwredusum.vs" => parse_vfwredusum_vs(&mut p),
        "vfwsub.vf" => parse_vfwsub_vf(&mut p),
        "vfwsub.vv" => parse_vfwsub_vv(&mut p),
        "vfwsub.wf" => parse_vfwsub_wf(&mut p),
        "vfwsub.wv" => parse_vfwsub_wv(&mut p),
        "vghsh.vv" => parse_vghsh_vv(&mut p),
        "vgmul.vv" => parse_vgmul_vv(&mut p),
        "vid.v" => parse_vid_v(&mut p),
        "viota.m" => parse_viota_m(&mut p),
        "vl1re16.v" => parse_vl1re16_v(&mut p),
        "vl1re32.v" => parse_vl1re32_v(&mut p),
        "vl1re64.v" => parse_vl1re64_v(&mut p),
        "vl1re8.v" => parse_vl1re8_v(&mut p),
        "vl2re16.v" => parse_vl2re16_v(&mut p),
        "vl2re32.v" => parse_vl2re32_v(&mut p),
        "vl2re64.v" => parse_vl2re64_v(&mut p),
        "vl2re8.v" => parse_vl2re8_v(&mut p),
        "vl4re16.v" => parse_vl4re16_v(&mut p),
        "vl4re32.v" => parse_vl4re32_v(&mut p),
        "vl4re64.v" => parse_vl4re64_v(&mut p),
        "vl4re8.v" => parse_vl4re8_v(&mut p),
        "vl8re16.v" => parse_vl8re16_v(&mut p),
        "vl8re32.v" => parse_vl8re32_v(&mut p),
        "vl8re64.v" => parse_vl8re64_v(&mut p),
        "vl8re8.v" => parse_vl8re8_v(&mut p),
        "vle16.v" => parse_vle16_v(&mut p),
        "vle16ff.v" => parse_vle16ff_v(&mut p),
        "vle32.v" => parse_vle32_v(&mut p),
        "vle32ff.v" => parse_vle32ff_v(&mut p),
        "vle64.v" => parse_vle64_v(&mut p),
        "vle64ff.v" => parse_vle64ff_v(&mut p),
        "vle8.v" => parse_vle8_v(&mut p),
        "vle8ff.v" => parse_vle8ff_v(&mut p),
        "vlm.v" => parse_vlm_v(&mut p),
        "vloxei16.v" => parse_vloxei16_v(&mut p),
        "vloxei32.v" => parse_vloxei32_v(&mut p),
        "vloxei64.v" => parse_vloxei64_v(&mut p),
        "vloxei8.v" => parse_vloxei8_v(&mut p),
        "vloxseg2ei16.v" => parse_vloxseg2ei16_v(&mut p),
        "vloxseg2ei32.v" => parse_vloxseg2ei32_v(&mut p),
        "vloxseg2ei64.v" => parse_vloxseg2ei64_v(&mut p),
        "vloxseg2ei8.v" => parse_vloxseg2ei8_v(&mut p),
        "vloxseg3ei16.v" => parse_vloxseg3ei16_v(&mut p),
        "vloxseg3ei32.v" => parse_vloxseg3ei32_v(&mut p),
        "vloxseg3ei64.v" => parse_vloxseg3ei64_v(&mut p),
        "vloxseg3ei8.v" => parse_vloxseg3ei8_v(&mut p),
        "vloxseg4ei16.v" => parse_vloxseg4ei16_v(&mut p),
        "vloxseg4ei32.v" => parse_vloxseg4ei32_v(&mut p),
        "vloxseg4ei64.v" => parse_vloxseg4ei64_v(&mut p),
        "vloxseg4ei8.v" => parse_vloxseg4ei8_v(&mut p),
        "vloxseg5ei16.v" => parse_vloxseg5ei16_v(&mut p),
        "vloxseg5ei32.v" => parse_vloxseg5ei32_v(&mut p),
        "vloxseg5ei64.v" => parse_vloxseg5ei64_v(&mut p),
        "vloxseg5ei8.v" => parse_vloxseg5ei8_v(&mut p),
        "vloxseg6ei16.v" => parse_vloxseg6ei16_v(&mut p),
        "vloxseg6ei32.v" => parse_vloxseg6ei32_v(&mut p),
        "vloxseg6ei64.v" => parse_vloxseg6ei64_v(&mut p),
        "vloxseg6ei8.v" => parse_vloxseg6ei8_v(&mut p),
        "vloxseg7ei16.v" => parse_vloxseg7ei16_v(&mut p),
        "vloxseg7ei32.v" => parse_vloxseg7ei32_v(&mut p),
        "vloxseg7ei64.v" => parse_vloxseg7ei64_v(&mut p),
        "vloxseg7ei8.v" => parse_vloxseg7ei8_v(&mut p),
        "vloxseg8ei16.v" => parse_vloxseg8ei16_v(&mut p),
        "vloxseg8ei32.v" => parse_vloxseg8ei32_v(&mut p),
        "vloxseg8ei64.v" => parse_vloxseg8ei64_v(&mut p),
        "vloxseg8ei8.v" => parse_vloxseg8ei8_v(&mut p),
        "vlse16.v" => parse_vlse16_v(&mut p),
        "vlse32.v" => parse_vlse32_v(&mut p),
        "vlse64.v" => parse_vlse64_v(&mut p),
        "vlse8.v" => parse_vlse8_v(&mut p),
        "vlseg2e16.v" => parse_vlseg2e16_v(&mut p),
        "vlseg2e16ff.v" => parse_vlseg2e16ff_v(&mut p),
        "vlseg2e32.v" => parse_vlseg2e32_v(&mut p),
        "vlseg2e32ff.v" => parse_vlseg2e32ff_v(&mut p),
        "vlseg2e64.v" => parse_vlseg2e64_v(&mut p),
        "vlseg2e64ff.v" => parse_vlseg2e64ff_v(&mut p),
        "vlseg2e8.v" => parse_vlseg2e8_v(&mut p),
        "vlseg2e8ff.v" => parse_vlseg2e8ff_v(&mut p),
        "vlseg3e16.v" => parse_vlseg3e16_v(&mut p),
        "vlseg3e16ff.v" => parse_vlseg3e16ff_v(&mut p),
        "vlseg3e32.v" => parse_vlseg3e32_v(&mut p),
        "vlseg3e32ff.v" => parse_vlseg3e32ff_v(&mut p),
        "vlseg3e64.v" => parse_vlseg3e64_v(&mut p),
        "vlseg3e64ff.v" => parse_vlseg3e64ff_v(&mut p),
        "vlseg3e8.v" => parse_vlseg3e8_v(&mut p),
        "vlseg3e8ff.v" => parse_vlseg3e8ff_v(&mut p),
        "vlseg4e16.v" => parse_vlseg4e16_v(&mut p),
        "vlseg4e16ff.v" => parse_vlseg4e16ff_v(&mut p),
        "vlseg4e32.v" => parse_vlseg4e32_v(&mut p),
        "vlseg4e32ff.v" => parse_vlseg4e32ff_v(&mut p),
        "vlseg4e64.v" => parse_vlseg4e64_v(&mut p),
        "vlseg4e64ff.v" => parse_vlseg4e64ff_v(&mut p),
        "vlseg4e8.v" => parse_vlseg4e8_v(&mut p),
        "vlseg4e8ff.v" => parse_vlseg4e8ff_v(&mut p),
        "vlseg5e16.v" => parse_vlseg5e16_v(&mut p),
        "vlseg5e16ff.v" => parse_vlseg5e16ff_v(&mut p),
        "vlseg5e32.v" => parse_vlseg5e32_v(&mut p),
        "vlseg5e32ff.v" => parse_vlseg5e32ff_v(&mut p),
        "vlseg5e64.v" => parse_vlseg5e64_v(&mut p),
        "vlseg5e64ff.v" => parse_vlseg5e64ff_v(&mut p),
        "vlseg5e8.v" => parse_vlseg5e8_v(&mut p),
        "vlseg5e8ff.v" => parse_vlseg5e8ff_v(&mut p),
        "vlseg6e16.v" => parse_vlseg6e16_v(&mut p),
        "vlseg6e16ff.v" => parse_vlseg6e16ff_v(&mut p),
        "vlseg6e32.v" => parse_vlseg6e32_v(&mut p),
        "vlseg6e32ff.v" => parse_vlseg6e32ff_v(&mut p),
        "vlseg6e64.v" => parse_vlseg6e64_v(&mut p),
        "vlseg6e64ff.v" => parse_vlseg6e64ff_v(&mut p),
        "vlseg6e8.v" => parse_vlseg6e8_v(&mut p),
        "vlseg6e8ff.v" => parse_vlseg6e8ff_v(&mut p),
        "vlseg7e16.v" => parse_vlseg7e16_v(&mut p),
        "vlseg7e16ff.v" => parse_vlseg7e16ff_v(&mut p),
        "vlseg7e32.v" => parse_vlseg7e32_v(&mut p),
        "vlseg7e32ff.v" => parse_vlseg7e32ff_v(&mut p),
        "vlseg7e64.v" => parse_vlseg7e64_v(&mut p),
        "vlseg7e64ff.v" => parse_vlseg7e64ff_v(&mut p),
        "vlseg7e8.v" => parse_vlseg7e8_v(&mut p),
        "vlseg7e8ff.v" => parse_vlseg7e8ff_v(&mut p),
        "vlseg8e16.v" => parse_vlseg8e16_v(&mut p),
        "vlseg8e16ff.v" => parse_vlseg8e16ff_v(&mut p),
        "vlseg8e32.v" => parse_vlseg8e32_v(&mut p),
        "vlseg8e32ff.v" => parse_vlseg8e32ff_v(&mut p),
        "vlseg8e64.v" => parse_vlseg8e64_v(&mut p),
        "vlseg8e64ff.v" => parse_vlseg8e64ff_v(&mut p),
        "vlseg8e8.v" => parse_vlseg8e8_v(&mut p),
        "vlseg8e8ff.v" => parse_vlseg8e8ff_v(&mut p),
        "vlsseg2e16.v" => parse_vlsseg2e16_v(&mut p),
        "vlsseg2e32.v" => parse_vlsseg2e32_v(&mut p),
        "vlsseg2e64.v" => parse_vlsseg2e64_v(&mut p),
        "vlsseg2e8.v" => parse_vlsseg2e8_v(&mut p),
        "vlsseg3e16.v" => parse_vlsseg3e16_v(&mut p),
        "vlsseg3e32.v" => parse_vlsseg3e32_v(&mut p),
        "vlsseg3e64.v" => parse_vlsseg3e64_v(&mut p),
        "vlsseg3e8.v" => parse_vlsseg3e8_v(&mut p),
        "vlsseg4e16.v" => parse_vlsseg4e16_v(&mut p),
        "vlsseg4e32.v" => parse_vlsseg4e32_v(&mut p),
        "vlsseg4e64.v" => parse_vlsseg4e64_v(&mut p),
        "vlsseg4e8.v" => parse_vlsseg4e8_v(&mut p),
        "vlsseg5e16.v" => parse_vlsseg5e16_v(&mut p),
        "vlsseg5e32.v" => parse_vlsseg5e32_v(&mut p),
        "vlsseg5e64.v" => parse_vlsseg5e64_v(&mut p),
        "vlsseg5e8.v" => parse_vlsseg5e8_v(&mut p),
        "vlsseg6e16.v" => parse_vlsseg6e16_v(&mut p),
        "vlsseg6e32.v" => parse_vlsseg6e32_v(&mut p),
        "vlsseg6e64.v" => parse_vlsseg6e64_v(&mut p),
        "vlsseg6e8.v" => parse_vlsseg6e8_v(&mut p),
        "vlsseg7e16.v" => parse_vlsseg7e16_v(&mut p),
        "vlsseg7e32.v" => parse_vlsseg7e32_v(&mut p),
        "vlsseg7e64.v" => parse_vlsseg7e64_v(&mut p),
        "vlsseg7e8.v" => parse_vlsseg7e8_v(&mut p),
        "vlsseg8e16.v" => parse_vlsseg8e16_v(&mut p),
        "vlsseg8e32.v" => parse_vlsseg8e32_v(&mut p),
        "vlsseg8e64.v" => parse_vlsseg8e64_v(&mut p),
        "vlsseg8e8.v" => parse_vlsseg8e8_v(&mut p),
        "vluxei16.v" => parse_vluxei16_v(&mut p),
        "vluxei32.v" => parse_vluxei32_v(&mut p),
        "vluxei64.v" => parse_vluxei64_v(&mut p),
        "vluxei8.v" => parse_vluxei8_v(&mut p),
        "vluxseg2ei16.v" => parse_vluxseg2ei16_v(&mut p),
        "vluxseg2ei32.v" => parse_vluxseg2ei32_v(&mut p),
        "vluxseg2ei64.v" => parse_vluxseg2ei64_v(&mut p),
        "vluxseg2ei8.v" => parse_vluxseg2ei8_v(&mut p),
        "vluxseg3ei16.v" => parse_vluxseg3ei16_v(&mut p),
        "vluxseg3ei32.v" => parse_vluxseg3ei32_v(&mut p),
        "vluxseg3ei64.v" => parse_vluxseg3ei64_v(&mut p),
        "vluxseg3ei8.v" => parse_vluxseg3ei8_v(&mut p),
        "vluxseg4ei16.v" => parse_vluxseg4ei16_v(&mut p),
        "vluxseg4ei32.v" => parse_vluxseg4ei32_v(&mut p),
        "vluxseg4ei64.v" => parse_vluxseg4ei64_v(&mut p),
        "vluxseg4ei8.v" => parse_vluxseg4ei8_v(&mut p),
        "vluxseg5ei16.v" => parse_vluxseg5ei16_v(&mut p),
        "vluxseg5ei32.v" => parse_vluxseg5ei32_v(&mut p),
        "vluxseg5ei64.v" => parse_vluxseg5ei64_v(&mut p),
        "vluxseg5ei8.v" => parse_vluxseg5ei8_v(&mut p),
        "vluxseg6ei16.v" => parse_vluxseg6ei16_v(&mut p),
        "vluxseg6ei32.v" => parse_vluxseg6ei32_v(&mut p),
        "vluxseg6ei64.v" => parse_vluxseg6ei64_v(&mut p),
        "vluxseg6ei8.v" => parse_vluxseg6ei8_v(&mut p),
        "vluxseg7ei16.v" => parse_vluxseg7ei16_v(&mut p),
        "vluxseg7ei32.v" => parse_vluxseg7ei32_v(&mut p),
        "vluxseg7ei64.v" => parse_vluxseg7ei64_v(&mut p),
        "vluxseg7ei8.v" => parse_vluxseg7ei8_v(&mut p),
        "vluxseg8ei16.v" => parse_vluxseg8ei16_v(&mut p),
        "vluxseg8ei32.v" => parse_vluxseg8ei32_v(&mut p),
        "vluxseg8ei64.v" => parse_vluxseg8ei64_v(&mut p),
        "vluxseg8ei8.v" => parse_vluxseg8ei8_v(&mut p),
        "vmacc.vv" => parse_vmacc_vv(&mut p),
        "vmacc.vx" => parse_vmacc_vx(&mut p),
        "vmadc.vi" => parse_vmadc_vi(&mut p),
        "vmadc.vim" => parse_vmadc_vim(&mut p),
        "vmadc.vv" => parse_vmadc_vv(&mut p),
        "vmadc.vvm" => parse_vmadc_vvm(&mut p),
        "vmadc.vx" => parse_vmadc_vx(&mut p),
        "vmadc.vxm" => parse_vmadc_vxm(&mut p),
        "vmadd.vv" => parse_vmadd_vv(&mut p),
        "vmadd.vx" => parse_vmadd_vx(&mut p),
        "vmand.mm" => parse_vmand_mm(&mut p),
        "vmandn.mm" => parse_vmandn_mm(&mut p),
        "vmax.vv" => parse_vmax_vv(&mut p),
        "vmax.vx" => parse_vmax_vx(&mut p),
        "vmaxu.vv" => parse_vmaxu_vv(&mut p),
        "vmaxu.vx" => parse_vmaxu_vx(&mut p),
        "vmerge.vim" => parse_vmerge_vim(&mut p),
        "vmerge.vvm" => parse_vmerge_vvm(&mut p),
        "vmerge.vxm" => parse_vmerge_vxm(&mut p),
        "vmfeq.vf" => parse_vmfeq_vf(&mut p),
        "vmfeq.vv" => parse_vmfeq_vv(&mut p),
        "vmfge.vf" => parse_vmfge_vf(&mut p),
        "vmfgt.vf" => parse_vmfgt_vf(&mut p),
        "vmfle.vf" => parse_vmfle_vf(&mut p),
        "vmfle.vv" => parse_vmfle_vv(&mut p),
        "vmflt.vf" => parse_vmflt_vf(&mut p),
        "vmflt.vv" => parse_vmflt_vv(&mut p),
        "vmfne.vf" => parse_vmfne_vf(&mut p),
        "vmfne.vv" => parse_vmfne_vv(&mut p),
        "vmin.vv" => parse_vmin_vv(&mut p),
        "vmin.vx" => parse_vmin_vx(&mut p),
        "vminu.vv" => parse_vminu_vv(&mut p),
        "vminu.vx" => parse_vminu_vx(&mut p),
        "vmnand.mm" => parse_vmnand_mm(&mut p),
        "vmnor.mm" => parse_vmnor_mm(&mut p),
        "vmor.mm" => parse_vmor_mm(&mut p),
        "vmorn.mm" => parse_vmorn_mm(&mut p),
        "vmsbc.vv" => parse_vmsbc_vv(&mut p),
        "vmsbc.vvm" => parse_vmsbc_vvm(&mut p),
        "vmsbc.vx" => parse_vmsbc_vx(&mut p),
        "vmsbc.vxm" => parse_vmsbc_vxm(&mut p),
        "vmsbf.m" => parse_vmsbf_m(&mut p),
        "vmseq.vi" => parse_vmseq_vi(&mut p),
        "vmseq.vv" => parse_vmseq_vv(&mut p),
        "vmseq.vx" => parse_vmseq_vx(&mut p),
        "vmsgt.vi" => parse_vmsgt_vi(&mut p),
        "vmsgt.vx" => parse_vmsgt_vx(&mut p),
        "vmsgtu.vi" => parse_vmsgtu_vi(&mut p),
        "vmsgtu.vx" => parse_vmsgtu_vx(&mut p),
        "vmsif.m" => parse_vmsif_m(&mut p),
        "vmsle.vi" => parse_vmsle_vi(&mut p),
        "vmsle.vv" => parse_vmsle_vv(&mut p),
        "vmsle.vx" => parse_vmsle_vx(&mut p),
        "vmsleu.vi" => parse_vmsleu_vi(&mut p),
        "vmsleu.vv" => parse_vmsleu_vv(&mut p),
        "vmsleu.vx" => parse_vmsleu_vx(&mut p),
        "vmslt.vv" => parse_vmslt_vv(&mut p),
        "vmslt.vx" => parse_vmslt_vx(&mut p),
        "vmsltu.vv" => parse_vmsltu_vv(&mut p),
        "vmsltu.vx" => parse_vmsltu_vx(&mut p),
        "vmsne.vi" => parse_vmsne_vi(&mut p),
        "vmsne.vv" => parse_vmsne_vv(&mut p),
        "vmsne.vx" => parse_vmsne_vx(&mut p),
        "vmsof.m" => parse_vmsof_m(&mut p),
        "vmul.vv" => parse_vmul_vv(&mut p),
        "vmul.vx" => parse_vmul_vx(&mut p),
        "vmulh.vv" => parse_vmulh_vv(&mut p),
        "vmulh.vx" => parse_vmulh_vx(&mut p),
        "vmulhsu.vv" => parse_vmulhsu_vv(&mut p),
        "vmulhsu.vx" => parse_vmulhsu_vx(&mut p),
        "vmulhu.vv" => parse_vmulhu_vv(&mut p),
        "vmulhu.vx" => parse_vmulhu_vx(&mut p),
        "vmv.s.x" => parse_vmv_s_x(&mut p),
        "vmv.v.i" => parse_vmv_v_i(&mut p),
        "vmv.v.v" => parse_vmv_v_v(&mut p),
        "vmv.v.x" => parse_vmv_v_x(&mut p),
        "vmv.x.s" => parse_vmv_x_s(&mut p),
        "vmv1r.v" => parse_vmv1r_v(&mut p),
        "vmv2r.v" => parse_vmv2r_v(&mut p),
        "vmv4r.v" => parse_vmv4r_v(&mut p),
        "vmv8r.v" => parse_vmv8r_v(&mut p),
        "vmxnor.mm" => parse_vmxnor_mm(&mut p),
        "vmxor.mm" => parse_vmxor_mm(&mut p),
        "vnclip.wi" => parse_vnclip_wi(&mut p),
        "vnclip.wv" => parse_vnclip_wv(&mut p),
        "vnclip.wx" => parse_vnclip_wx(&mut p),
        "vnclipu.wi" => parse_vnclipu_wi(&mut p),
        "vnclipu.wv" => parse_vnclipu_wv(&mut p),
        "vnclipu.wx" => parse_vnclipu_wx(&mut p),
        "vnmsac.vv" => parse_vnmsac_vv(&mut p),
        "vnmsac.vx" => parse_vnmsac_vx(&mut p),
        "vnmsub.vv" => parse_vnmsub_vv(&mut p),
        "vnmsub.vx" => parse_vnmsub_vx(&mut p),
        "vnsra.wi" => parse_vnsra_wi(&mut p),
        "vnsra.wv" => parse_vnsra_wv(&mut p),
        "vnsra.wx" => parse_vnsra_wx(&mut p),
        "vnsrl.wi" => parse_vnsrl_wi(&mut p),
        "vnsrl.wv" => parse_vnsrl_wv(&mut p),
        "vnsrl.wx" => parse_vnsrl_wx(&mut p),
        "vor.vi" => parse_vor_vi(&mut p),
        "vor.vv" => parse_vor_vv(&mut p),
        "vor.vx" => parse_vor_vx(&mut p),
        "vredand.vs" => parse_vredand_vs(&mut p),
        "vredmax.vs" => parse_vredmax_vs(&mut p),
        "vredmaxu.vs" => parse_vredmaxu_vs(&mut p),
        "vredmin.vs" => parse_vredmin_vs(&mut p),
        "vredminu.vs" => parse_vredminu_vs(&mut p),
        "vredor.vs" => parse_vredor_vs(&mut p),
        "vredsum.vs" => parse_vredsum_vs(&mut p),
        "vredxor.vs" => parse_vredxor_vs(&mut p),
        "vrem.vv" => parse_vrem_vv(&mut p),
        "vrem.vx" => parse_vrem_vx(&mut p),
        "vremu.vv" => parse_vremu_vv(&mut p),
        "vremu.vx" => parse_vremu_vx(&mut p),
        "vrev8.v" => parse_vrev8_v(&mut p),
        "vrgather.vi" => parse_vrgather_vi(&mut p),
        "vrgather.vv" => parse_vrgather_vv(&mut p),
        "vrgather.vx" => parse_vrgather_vx(&mut p),
        "vrgatherei16.vv" => parse_vrgatherei16_vv(&mut p),
        "vrol.vv" => parse_vrol_vv(&mut p),
        "vrol.vx" => parse_vrol_vx(&mut p),
        "vror.vi" => parse_vror_vi(&mut p),
        "vror.vv" => parse_vror_vv(&mut p),
        "vror.vx" => parse_vror_vx(&mut p),
        "vrsub.vi" => parse_vrsub_vi(&mut p),
        "vrsub.vx" => parse_vrsub_vx(&mut p),
        "vs1r.v" => parse_vs1r_v(&mut p),
        "vs2r.v" => parse_vs2r_v(&mut p),
        "vs4r.v" => parse_vs4r_v(&mut p),
        "vs8r.v" => parse_vs8r_v(&mut p),
        "vsadd.vi" => parse_vsadd_vi(&mut p),
        "vsadd.vv" => parse_vsadd_vv(&mut p),
        "vsadd.vx" => parse_vsadd_vx(&mut p),
        "vsaddu.vi" => parse_vsaddu_vi(&mut p),
        "vsaddu.vv" => parse_vsaddu_vv(&mut p),
        "vsaddu.vx" => parse_vsaddu_vx(&mut p),
        "vsbc.vvm" => parse_vsbc_vvm(&mut p),
        "vsbc.vxm" => parse_vsbc_vxm(&mut p),
        "vse16.v" => parse_vse16_v(&mut p),
        "vse32.v" => parse_vse32_v(&mut p),
        "vse64.v" => parse_vse64_v(&mut p),
        "vse8.v" => parse_vse8_v(&mut p),
        "vsetivli" => parse_vsetivli(&mut p),
        "vsetvl" => parse_vsetvl(&mut p),
        "vsetvli" => parse_vsetvli(&mut p),
        "vsext.vf2" => parse_vsext_vf2(&mut p),
        "vsext.vf4" => parse_vsext_vf4(&mut p),
        "vsext.vf8" => parse_vsext_vf8(&mut p),
        "vsha2ch.vv" => parse_vsha2ch_vv(&mut p),
        "vsha2cl.vv" => parse_vsha2cl_vv(&mut p),
        "vsha2ms.vv" => parse_vsha2ms_vv(&mut p),
        "vslide1down.vx" => parse_vslide1down_vx(&mut p),
        "vslide1up.vx" => parse_vslide1up_vx(&mut p),
        "vslidedown.vi" => parse_vslidedown_vi(&mut p),
        "vslidedown.vx" => parse_vslidedown_vx(&mut p),
        "vslideup.vi" => parse_vslideup_vi(&mut p),
        "vslideup.vx" => parse_vslideup_vx(&mut p),
        "vsll.vi" => parse_vsll_vi(&mut p),
        "vsll.vv" => parse_vsll_vv(&mut p),
        "vsll.vx" => parse_vsll_vx(&mut p),
        "vsm.v" => parse_vsm_v(&mut p),
        "vsm3c.vi" => parse_vsm3c_vi(&mut p),
        "vsm3me.vv" => parse_vsm3me_vv(&mut p),
        "vsm4k.vi" => parse_vsm4k_vi(&mut p),
        "vsm4r.vs" => parse_vsm4r_vs(&mut p, ctx),
        "vsm4r.vv" => parse_vsm4r_vv(&mut p, ctx),
        "vsmul.vv" => parse_vsmul_vv(&mut p),
        "vsmul.vx" => parse_vsmul_vx(&mut p),
        "vsoxei16.v" => parse_vsoxei16_v(&mut p),
        "vsoxei32.v" => parse_vsoxei32_v(&mut p),
        "vsoxei64.v" => parse_vsoxei64_v(&mut p),
        "vsoxei8.v" => parse_vsoxei8_v(&mut p),
        "vsoxseg2ei16.v" => parse_vsoxseg2ei16_v(&mut p),
        "vsoxseg2ei32.v" => parse_vsoxseg2ei32_v(&mut p),
        "vsoxseg2ei64.v" => parse_vsoxseg2ei64_v(&mut p),
        "vsoxseg2ei8.v" => parse_vsoxseg2ei8_v(&mut p),
        "vsoxseg3ei16.v" => parse_vsoxseg3ei16_v(&mut p),
        "vsoxseg3ei32.v" => parse_vsoxseg3ei32_v(&mut p),
        "vsoxseg3ei64.v" => parse_vsoxseg3ei64_v(&mut p),
        "vsoxseg3ei8.v" => parse_vsoxseg3ei8_v(&mut p),
        "vsoxseg4ei16.v" => parse_vsoxseg4ei16_v(&mut p),
        "vsoxseg4ei32.v" => parse_vsoxseg4ei32_v(&mut p),
        "vsoxseg4ei64.v" => parse_vsoxseg4ei64_v(&mut p),
        "vsoxseg4ei8.v" => parse_vsoxseg4ei8_v(&mut p),
        "vsoxseg5ei16.v" => parse_vsoxseg5ei16_v(&mut p),
        "vsoxseg5ei32.v" => parse_vsoxseg5ei32_v(&mut p),
        "vsoxseg5ei64.v" => parse_vsoxseg5ei64_v(&mut p),
        "vsoxseg5ei8.v" => parse_vsoxseg5ei8_v(&mut p),
        "vsoxseg6ei16.v" => parse_vsoxseg6ei16_v(&mut p),
        "vsoxseg6ei32.v" => parse_vsoxseg6ei32_v(&mut p),
        "vsoxseg6ei64.v" => parse_vsoxseg6ei64_v(&mut p),
        "vsoxseg6ei8.v" => parse_vsoxseg6ei8_v(&mut p),
        "vsoxseg7ei16.v" => parse_vsoxseg7ei16_v(&mut p),
        "vsoxseg7ei32.v" => parse_vsoxseg7ei32_v(&mut p),
        "vsoxseg7ei64.v" => parse_vsoxseg7ei64_v(&mut p),
        "vsoxseg7ei8.v" => parse_vsoxseg7ei8_v(&mut p),
        "vsoxseg8ei16.v" => parse_vsoxseg8ei16_v(&mut p),
        "vsoxseg8ei32.v" => parse_vsoxseg8ei32_v(&mut p),
        "vsoxseg8ei64.v" => parse_vsoxseg8ei64_v(&mut p),
        "vsoxseg8ei8.v" => parse_vsoxseg8ei8_v(&mut p),
        "vsra.vi" => parse_vsra_vi(&mut p),
        "vsra.vv" => parse_vsra_vv(&mut p),
        "vsra.vx" => parse_vsra_vx(&mut p),
        "vsrl.vi" => parse_vsrl_vi(&mut p),
        "vsrl.vv" => parse_vsrl_vv(&mut p),
        "vsrl.vx" => parse_vsrl_vx(&mut p),
        "vsse16.v" => parse_vsse16_v(&mut p),
        "vsse32.v" => parse_vsse32_v(&mut p),
        "vsse64.v" => parse_vsse64_v(&mut p),
        "vsse8.v" => parse_vsse8_v(&mut p),
        "vsseg2e16.v" => parse_vsseg2e16_v(&mut p),
        "vsseg2e32.v" => parse_vsseg2e32_v(&mut p),
        "vsseg2e64.v" => parse_vsseg2e64_v(&mut p),
        "vsseg2e8.v" => parse_vsseg2e8_v(&mut p),
        "vsseg3e16.v" => parse_vsseg3e16_v(&mut p),
        "vsseg3e32.v" => parse_vsseg3e32_v(&mut p),
        "vsseg3e64.v" => parse_vsseg3e64_v(&mut p),
        "vsseg3e8.v" => parse_vsseg3e8_v(&mut p),
        "vsseg4e16.v" => parse_vsseg4e16_v(&mut p),
        "vsseg4e32.v" => parse_vsseg4e32_v(&mut p),
        "vsseg4e64.v" => parse_vsseg4e64_v(&mut p),
        "vsseg4e8.v" => parse_vsseg4e8_v(&mut p),
        "vsseg5e16.v" => parse_vsseg5e16_v(&mut p),
        "vsseg5e32.v" => parse_vsseg5e32_v(&mut p),
        "vsseg5e64.v" => parse_vsseg5e64_v(&mut p),
        "vsseg5e8.v" => parse_vsseg5e8_v(&mut p),
        "vsseg6e16.v" => parse_vsseg6e16_v(&mut p),
        "vsseg6e32.v" => parse_vsseg6e32_v(&mut p),
        "vsseg6e64.v" => parse_vsseg6e64_v(&mut p),
        "vsseg6e8.v" => parse_vsseg6e8_v(&mut p),
        "vsseg7e16.v" => parse_vsseg7e16_v(&mut p),
        "vsseg7e32.v" => parse_vsseg7e32_v(&mut p),
        "vsseg7e64.v" => parse_vsseg7e64_v(&mut p),
        "vsseg7e8.v" => parse_vsseg7e8_v(&mut p),
        "vsseg8e16.v" => parse_vsseg8e16_v(&mut p),
        "vsseg8e32.v" => parse_vsseg8e32_v(&mut p),
        "vsseg8e64.v" => parse_vsseg8e64_v(&mut p),
        "vsseg8e8.v" => parse_vsseg8e8_v(&mut p),
        "vssra.vi" => parse_vssra_vi(&mut p),
        "vssra.vv" => parse_vssra_vv(&mut p),
        "vssra.vx" => parse_vssra_vx(&mut p),
        "vssrl.vi" => parse_vssrl_vi(&mut p),
        "vssrl.vv" => parse_vssrl_vv(&mut p),
        "vssrl.vx" => parse_vssrl_vx(&mut p),
        "vssseg2e16.v" => parse_vssseg2e16_v(&mut p),
        "vssseg2e32.v" => parse_vssseg2e32_v(&mut p),
        "vssseg2e64.v" => parse_vssseg2e64_v(&mut p),
        "vssseg2e8.v" => parse_vssseg2e8_v(&mut p),
        "vssseg3e16.v" => parse_vssseg3e16_v(&mut p),
        "vssseg3e32.v" => parse_vssseg3e32_v(&mut p),
        "vssseg3e64.v" => parse_vssseg3e64_v(&mut p),
        "vssseg3e8.v" => parse_vssseg3e8_v(&mut p),
        "vssseg4e16.v" => parse_vssseg4e16_v(&mut p),
        "vssseg4e32.v" => parse_vssseg4e32_v(&mut p),
        "vssseg4e64.v" => parse_vssseg4e64_v(&mut p),
        "vssseg4e8.v" => parse_vssseg4e8_v(&mut p),
        "vssseg5e16.v" => parse_vssseg5e16_v(&mut p),
        "vssseg5e32.v" => parse_vssseg5e32_v(&mut p),
        "vssseg5e64.v" => parse_vssseg5e64_v(&mut p),
        "vssseg5e8.v" => parse_vssseg5e8_v(&mut p),
        "vssseg6e16.v" => parse_vssseg6e16_v(&mut p),
        "vssseg6e32.v" => parse_vssseg6e32_v(&mut p),
        "vssseg6e64.v" => parse_vssseg6e64_v(&mut p),
        "vssseg6e8.v" => parse_vssseg6e8_v(&mut p),
        "vssseg7e16.v" => parse_vssseg7e16_v(&mut p),
        "vssseg7e32.v" => parse_vssseg7e32_v(&mut p),
        "vssseg7e64.v" => parse_vssseg7e64_v(&mut p),
        "vssseg7e8.v" => parse_vssseg7e8_v(&mut p),
        "vssseg8e16.v" => parse_vssseg8e16_v(&mut p),
        "vssseg8e32.v" => parse_vssseg8e32_v(&mut p),
        "vssseg8e64.v" => parse_vssseg8e64_v(&mut p),
        "vssseg8e8.v" => parse_vssseg8e8_v(&mut p),
        "vssub.vv" => parse_vssub_vv(&mut p),
        "vssub.vx" => parse_vssub_vx(&mut p),
        "vssubu.vv" => parse_vssubu_vv(&mut p),
        "vssubu.vx" => parse_vssubu_vx(&mut p),
        "vsub.vv" => parse_vsub_vv(&mut p),
        "vsub.vx" => parse_vsub_vx(&mut p),
        "vsuxei16.v" => parse_vsuxei16_v(&mut p),
        "vsuxei32.v" => parse_vsuxei32_v(&mut p),
        "vsuxei64.v" => parse_vsuxei64_v(&mut p),
        "vsuxei8.v" => parse_vsuxei8_v(&mut p),
        "vsuxseg2ei16.v" => parse_vsuxseg2ei16_v(&mut p),
        "vsuxseg2ei32.v" => parse_vsuxseg2ei32_v(&mut p),
        "vsuxseg2ei64.v" => parse_vsuxseg2ei64_v(&mut p),
        "vsuxseg2ei8.v" => parse_vsuxseg2ei8_v(&mut p),
        "vsuxseg3ei16.v" => parse_vsuxseg3ei16_v(&mut p),
        "vsuxseg3ei32.v" => parse_vsuxseg3ei32_v(&mut p),
        "vsuxseg3ei64.v" => parse_vsuxseg3ei64_v(&mut p),
        "vsuxseg3ei8.v" => parse_vsuxseg3ei8_v(&mut p),
        "vsuxseg4ei16.v" => parse_vsuxseg4ei16_v(&mut p),
        "vsuxseg4ei32.v" => parse_vsuxseg4ei32_v(&mut p),
        "vsuxseg4ei64.v" => parse_vsuxseg4ei64_v(&mut p),
        "vsuxseg4ei8.v" => parse_vsuxseg4ei8_v(&mut p),
        "vsuxseg5ei16.v" => parse_vsuxseg5ei16_v(&mut p),
        "vsuxseg5ei32.v" => parse_vsuxseg5ei32_v(&mut p),
        "vsuxseg5ei64.v" => parse_vsuxseg5ei64_v(&mut p),
        "vsuxseg5ei8.v" => parse_vsuxseg5ei8_v(&mut p),
        "vsuxseg6ei16.v" => parse_vsuxseg6ei16_v(&mut p),
        "vsuxseg6ei32.v" => parse_vsuxseg6ei32_v(&mut p),
        "vsuxseg6ei64.v" => parse_vsuxseg6ei64_v(&mut p),
        "vsuxseg6ei8.v" => parse_vsuxseg6ei8_v(&mut p),
        "vsuxseg7ei16.v" => parse_vsuxseg7ei16_v(&mut p),
        "vsuxseg7ei32.v" => parse_vsuxseg7ei32_v(&mut p),
        "vsuxseg7ei64.v" => parse_vsuxseg7ei64_v(&mut p),
        "vsuxseg7ei8.v" => parse_vsuxseg7ei8_v(&mut p),
        "vsuxseg8ei16.v" => parse_vsuxseg8ei16_v(&mut p),
        "vsuxseg8ei32.v" => parse_vsuxseg8ei32_v(&mut p),
        "vsuxseg8ei64.v" => parse_vsuxseg8ei64_v(&mut p),
        "vsuxseg8ei8.v" => parse_vsuxseg8ei8_v(&mut p),
        "vwadd.vv" => parse_vwadd_vv(&mut p),
        "vwadd.vx" => parse_vwadd_vx(&mut p),
        "vwadd.wv" => parse_vwadd_wv(&mut p),
        "vwadd.wx" => parse_vwadd_wx(&mut p),
        "vwaddu.vv" => parse_vwaddu_vv(&mut p),
        "vwaddu.vx" => parse_vwaddu_vx(&mut p),
        "vwaddu.wv" => parse_vwaddu_wv(&mut p),
        "vwaddu.wx" => parse_vwaddu_wx(&mut p),
        "vwmacc.vv" => parse_vwmacc_vv(&mut p),
        "vwmacc.vx" => parse_vwmacc_vx(&mut p),
        "vwmaccsu.vv" => parse_vwmaccsu_vv(&mut p),
        "vwmaccsu.vx" => parse_vwmaccsu_vx(&mut p),
        "vwmaccu.vv" => parse_vwmaccu_vv(&mut p),
        "vwmaccu.vx" => parse_vwmaccu_vx(&mut p),
        "vwmaccus.vx" => parse_vwmaccus_vx(&mut p),
        "vwmul.vv" => parse_vwmul_vv(&mut p),
        "vwmul.vx" => parse_vwmul_vx(&mut p),
        "vwmulsu.vv" => parse_vwmulsu_vv(&mut p),
        "vwmulsu.vx" => parse_vwmulsu_vx(&mut p),
        "vwmulu.vv" => parse_vwmulu_vv(&mut p),
        "vwmulu.vx" => parse_vwmulu_vx(&mut p),
        "vwredsum.vs" => parse_vwredsum_vs(&mut p),
        "vwredsumu.vs" => parse_vwredsumu_vs(&mut p),
        "vwsll.vi" => parse_vwsll_vi(&mut p),
        "vwsll.vv" => parse_vwsll_vv(&mut p),
        "vwsll.vx" => parse_vwsll_vx(&mut p),
        "vwsub.vv" => parse_vwsub_vv(&mut p),
        "vwsub.vx" => parse_vwsub_vx(&mut p),
        "vwsub.wv" => parse_vwsub_wv(&mut p),
        "vwsub.wx" => parse_vwsub_wx(&mut p),
        "vwsubu.vv" => parse_vwsubu_vv(&mut p),
        "vwsubu.vx" => parse_vwsubu_vx(&mut p),
        "vwsubu.wv" => parse_vwsubu_wv(&mut p),
        "vwsubu.wx" => parse_vwsubu_wx(&mut p),
        "vxor.vi" => parse_vxor_vi(&mut p),
        "vxor.vv" => parse_vxor_vv(&mut p),
        "vxor.vx" => parse_vxor_vx(&mut p),
        "vzext.vf2" => parse_vzext_vf2(&mut p),
        "vzext.vf4" => parse_vzext_vf4(&mut p),
        "vzext.vf8" => parse_vzext_vf8(&mut p),
        "wfi" => parse_wfi(),
        "wrs.nto" => parse_wrs_nto(),
        "wrs.sto" => parse_wrs_sto(),
        "xnor" => parse_xnor(&mut p, ctx),
        "xor" => parse_xor(&mut p),
        "xori" => parse_xori(&mut p),
        "xperm4" => parse_xperm4(&mut p),
        "xperm8" => parse_xperm8(&mut p),
        "zext.h" => parse_zext_h(&mut p, ctx),
        "zip" => parse_zip(&mut p),
        _ => err!("`{ident}` is not a recognized instruction"),
    }
}
pub fn csr_code(s: &str) -> Option<BitVector<12>> {
    match s {
        "cycle" => Some(BitVector::<12> {
            val: 0b110000000000,
        }),
        "cycleh" => Some(BitVector::<12> {
            val: 0b110010000000,
        }),
        "fcsr" => Some(BitVector::<12> {
            val: 0b000000000011,
        }),
        "fflags" => Some(BitVector::<12> {
            val: 0b000000000001,
        }),
        "frm" => Some(BitVector::<12> {
            val: 0b000000000010,
        }),
        "hpmcounter10" => Some(BitVector::<12> {
            val: 0b110000001010,
        }),
        "hpmcounter10h" => Some(BitVector::<12> {
            val: 0b110010001010,
        }),
        "hpmcounter11" => Some(BitVector::<12> {
            val: 0b110000001011,
        }),
        "hpmcounter11h" => Some(BitVector::<12> {
            val: 0b110010001011,
        }),
        "hpmcounter12" => Some(BitVector::<12> {
            val: 0b110000001100,
        }),
        "hpmcounter12h" => Some(BitVector::<12> {
            val: 0b110010001100,
        }),
        "hpmcounter13" => Some(BitVector::<12> {
            val: 0b110000001101,
        }),
        "hpmcounter13h" => Some(BitVector::<12> {
            val: 0b110010001101,
        }),
        "hpmcounter14" => Some(BitVector::<12> {
            val: 0b110000001110,
        }),
        "hpmcounter14h" => Some(BitVector::<12> {
            val: 0b110010001110,
        }),
        "hpmcounter15" => Some(BitVector::<12> {
            val: 0b110000001111,
        }),
        "hpmcounter15h" => Some(BitVector::<12> {
            val: 0b110010001111,
        }),
        "hpmcounter16" => Some(BitVector::<12> {
            val: 0b110000010000,
        }),
        "hpmcounter16h" => Some(BitVector::<12> {
            val: 0b110010010000,
        }),
        "hpmcounter17" => Some(BitVector::<12> {
            val: 0b110000010001,
        }),
        "hpmcounter17h" => Some(BitVector::<12> {
            val: 0b110010010001,
        }),
        "hpmcounter18" => Some(BitVector::<12> {
            val: 0b110000010010,
        }),
        "hpmcounter18h" => Some(BitVector::<12> {
            val: 0b110010010010,
        }),
        "hpmcounter19" => Some(BitVector::<12> {
            val: 0b110000010011,
        }),
        "hpmcounter19h" => Some(BitVector::<12> {
            val: 0b110010010011,
        }),
        "hpmcounter20" => Some(BitVector::<12> {
            val: 0b110000010100,
        }),
        "hpmcounter20h" => Some(BitVector::<12> {
            val: 0b110010010100,
        }),
        "hpmcounter21" => Some(BitVector::<12> {
            val: 0b110000010101,
        }),
        "hpmcounter21h" => Some(BitVector::<12> {
            val: 0b110010010101,
        }),
        "hpmcounter22" => Some(BitVector::<12> {
            val: 0b110000010110,
        }),
        "hpmcounter22h" => Some(BitVector::<12> {
            val: 0b110010010110,
        }),
        "hpmcounter23" => Some(BitVector::<12> {
            val: 0b110000010111,
        }),
        "hpmcounter23h" => Some(BitVector::<12> {
            val: 0b110010010111,
        }),
        "hpmcounter24" => Some(BitVector::<12> {
            val: 0b110000011000,
        }),
        "hpmcounter24h" => Some(BitVector::<12> {
            val: 0b110010011000,
        }),
        "hpmcounter25" => Some(BitVector::<12> {
            val: 0b110000011001,
        }),
        "hpmcounter25h" => Some(BitVector::<12> {
            val: 0b110010011001,
        }),
        "hpmcounter26" => Some(BitVector::<12> {
            val: 0b110000011010,
        }),
        "hpmcounter26h" => Some(BitVector::<12> {
            val: 0b110010011010,
        }),
        "hpmcounter27" => Some(BitVector::<12> {
            val: 0b110000011011,
        }),
        "hpmcounter27h" => Some(BitVector::<12> {
            val: 0b110010011011,
        }),
        "hpmcounter28" => Some(BitVector::<12> {
            val: 0b110000011100,
        }),
        "hpmcounter28h" => Some(BitVector::<12> {
            val: 0b110010011100,
        }),
        "hpmcounter29" => Some(BitVector::<12> {
            val: 0b110000011101,
        }),
        "hpmcounter29h" => Some(BitVector::<12> {
            val: 0b110010011101,
        }),
        "hpmcounter3" => Some(BitVector::<12> {
            val: 0b110000000011,
        }),
        "hpmcounter30" => Some(BitVector::<12> {
            val: 0b110000011110,
        }),
        "hpmcounter30h" => Some(BitVector::<12> {
            val: 0b110010011110,
        }),
        "hpmcounter31" => Some(BitVector::<12> {
            val: 0b110000011111,
        }),
        "hpmcounter31h" => Some(BitVector::<12> {
            val: 0b110010011111,
        }),
        "hpmcounter3h" => Some(BitVector::<12> {
            val: 0b110010000011,
        }),
        "hpmcounter4" => Some(BitVector::<12> {
            val: 0b110000000100,
        }),
        "hpmcounter4h" => Some(BitVector::<12> {
            val: 0b110010000100,
        }),
        "hpmcounter5" => Some(BitVector::<12> {
            val: 0b110000000101,
        }),
        "hpmcounter5h" => Some(BitVector::<12> {
            val: 0b110010000101,
        }),
        "hpmcounter6" => Some(BitVector::<12> {
            val: 0b110000000110,
        }),
        "hpmcounter6h" => Some(BitVector::<12> {
            val: 0b110010000110,
        }),
        "hpmcounter7" => Some(BitVector::<12> {
            val: 0b110000000111,
        }),
        "hpmcounter7h" => Some(BitVector::<12> {
            val: 0b110010000111,
        }),
        "hpmcounter8" => Some(BitVector::<12> {
            val: 0b110000001000,
        }),
        "hpmcounter8h" => Some(BitVector::<12> {
            val: 0b110010001000,
        }),
        "hpmcounter9" => Some(BitVector::<12> {
            val: 0b110000001001,
        }),
        "hpmcounter9h" => Some(BitVector::<12> {
            val: 0b110010001001,
        }),
        "hstateen0" => Some(BitVector::<12> {
            val: 0b011000001100,
        }),
        "hstateen0h" => Some(BitVector::<12> {
            val: 0b011000011100,
        }),
        "hstateen1" => Some(BitVector::<12> {
            val: 0b011000001101,
        }),
        "hstateen1h" => Some(BitVector::<12> {
            val: 0b011000011101,
        }),
        "hstateen2" => Some(BitVector::<12> {
            val: 0b011000001110,
        }),
        "hstateen2h" => Some(BitVector::<12> {
            val: 0b011000011110,
        }),
        "hstateen3" => Some(BitVector::<12> {
            val: 0b011000001111,
        }),
        "hstateen3h" => Some(BitVector::<12> {
            val: 0b011000011111,
        }),
        "instret" => Some(BitVector::<12> {
            val: 0b110000000010,
        }),
        "instreth" => Some(BitVector::<12> {
            val: 0b110010000010,
        }),
        "marchid" => Some(BitVector::<12> {
            val: 0b111100010010,
        }),
        "mcause" => Some(BitVector::<12> {
            val: 0b001101000010,
        }),
        "mconfigptr" => Some(BitVector::<12> {
            val: 0b111100010101,
        }),
        "mcounteren" => Some(BitVector::<12> {
            val: 0b001100000110,
        }),
        "mcountinhibit" => Some(BitVector::<12> {
            val: 0b001100100000,
        }),
        "mcycle" => Some(BitVector::<12> {
            val: 0b101100000000,
        }),
        "mcyclecfg" => Some(BitVector::<12> {
            val: 0b001100100001,
        }),
        "mcyclecfgh" => Some(BitVector::<12> {
            val: 0b011100100001,
        }),
        "mcycleh" => Some(BitVector::<12> {
            val: 0b101110000000,
        }),
        "medeleg" => Some(BitVector::<12> {
            val: 0b001100000010,
        }),
        "medelegh" => Some(BitVector::<12> {
            val: 0b001100010010,
        }),
        "menvcfg" => Some(BitVector::<12> {
            val: 0b001100001010,
        }),
        "menvcfgh" => Some(BitVector::<12> {
            val: 0b001100011010,
        }),
        "mepc" => Some(BitVector::<12> {
            val: 0b001101000001,
        }),
        "mhartid" => Some(BitVector::<12> {
            val: 0b111100010100,
        }),
        "mhpmcounter10" => Some(BitVector::<12> {
            val: 0b101100001010,
        }),
        "mhpmcounter10h" => Some(BitVector::<12> {
            val: 0b101110001010,
        }),
        "mhpmcounter11" => Some(BitVector::<12> {
            val: 0b101100001011,
        }),
        "mhpmcounter11h" => Some(BitVector::<12> {
            val: 0b101110001011,
        }),
        "mhpmcounter12" => Some(BitVector::<12> {
            val: 0b101100001100,
        }),
        "mhpmcounter12h" => Some(BitVector::<12> {
            val: 0b101110001100,
        }),
        "mhpmcounter13" => Some(BitVector::<12> {
            val: 0b101100001101,
        }),
        "mhpmcounter13h" => Some(BitVector::<12> {
            val: 0b101110001101,
        }),
        "mhpmcounter14" => Some(BitVector::<12> {
            val: 0b101100001110,
        }),
        "mhpmcounter14h" => Some(BitVector::<12> {
            val: 0b101110001110,
        }),
        "mhpmcounter15" => Some(BitVector::<12> {
            val: 0b101100001111,
        }),
        "mhpmcounter15h" => Some(BitVector::<12> {
            val: 0b101110001111,
        }),
        "mhpmcounter16" => Some(BitVector::<12> {
            val: 0b101100010000,
        }),
        "mhpmcounter16h" => Some(BitVector::<12> {
            val: 0b101110010000,
        }),
        "mhpmcounter17" => Some(BitVector::<12> {
            val: 0b101100010001,
        }),
        "mhpmcounter17h" => Some(BitVector::<12> {
            val: 0b101110010001,
        }),
        "mhpmcounter18" => Some(BitVector::<12> {
            val: 0b101100010010,
        }),
        "mhpmcounter18h" => Some(BitVector::<12> {
            val: 0b101110010010,
        }),
        "mhpmcounter19" => Some(BitVector::<12> {
            val: 0b101100010011,
        }),
        "mhpmcounter19h" => Some(BitVector::<12> {
            val: 0b101110010011,
        }),
        "mhpmcounter20" => Some(BitVector::<12> {
            val: 0b101100010100,
        }),
        "mhpmcounter20h" => Some(BitVector::<12> {
            val: 0b101110010100,
        }),
        "mhpmcounter21" => Some(BitVector::<12> {
            val: 0b101100010101,
        }),
        "mhpmcounter21h" => Some(BitVector::<12> {
            val: 0b101110010101,
        }),
        "mhpmcounter22" => Some(BitVector::<12> {
            val: 0b101100010110,
        }),
        "mhpmcounter22h" => Some(BitVector::<12> {
            val: 0b101110010110,
        }),
        "mhpmcounter23" => Some(BitVector::<12> {
            val: 0b101100010111,
        }),
        "mhpmcounter23h" => Some(BitVector::<12> {
            val: 0b101110010111,
        }),
        "mhpmcounter24" => Some(BitVector::<12> {
            val: 0b101100011000,
        }),
        "mhpmcounter24h" => Some(BitVector::<12> {
            val: 0b101110011000,
        }),
        "mhpmcounter25" => Some(BitVector::<12> {
            val: 0b101100011001,
        }),
        "mhpmcounter25h" => Some(BitVector::<12> {
            val: 0b101110011001,
        }),
        "mhpmcounter26" => Some(BitVector::<12> {
            val: 0b101100011010,
        }),
        "mhpmcounter26h" => Some(BitVector::<12> {
            val: 0b101110011010,
        }),
        "mhpmcounter27" => Some(BitVector::<12> {
            val: 0b101100011011,
        }),
        "mhpmcounter27h" => Some(BitVector::<12> {
            val: 0b101110011011,
        }),
        "mhpmcounter28" => Some(BitVector::<12> {
            val: 0b101100011100,
        }),
        "mhpmcounter28h" => Some(BitVector::<12> {
            val: 0b101110011100,
        }),
        "mhpmcounter29" => Some(BitVector::<12> {
            val: 0b101100011101,
        }),
        "mhpmcounter29h" => Some(BitVector::<12> {
            val: 0b101110011101,
        }),
        "mhpmcounter3" => Some(BitVector::<12> {
            val: 0b101100000011,
        }),
        "mhpmcounter30" => Some(BitVector::<12> {
            val: 0b101100011110,
        }),
        "mhpmcounter30h" => Some(BitVector::<12> {
            val: 0b101110011110,
        }),
        "mhpmcounter31" => Some(BitVector::<12> {
            val: 0b101100011111,
        }),
        "mhpmcounter31h" => Some(BitVector::<12> {
            val: 0b101110011111,
        }),
        "mhpmcounter3h" => Some(BitVector::<12> {
            val: 0b101110000011,
        }),
        "mhpmcounter4" => Some(BitVector::<12> {
            val: 0b101100000100,
        }),
        "mhpmcounter4h" => Some(BitVector::<12> {
            val: 0b101110000100,
        }),
        "mhpmcounter5" => Some(BitVector::<12> {
            val: 0b101100000101,
        }),
        "mhpmcounter5h" => Some(BitVector::<12> {
            val: 0b101110000101,
        }),
        "mhpmcounter6" => Some(BitVector::<12> {
            val: 0b101100000110,
        }),
        "mhpmcounter6h" => Some(BitVector::<12> {
            val: 0b101110000110,
        }),
        "mhpmcounter7" => Some(BitVector::<12> {
            val: 0b101100000111,
        }),
        "mhpmcounter7h" => Some(BitVector::<12> {
            val: 0b101110000111,
        }),
        "mhpmcounter8" => Some(BitVector::<12> {
            val: 0b101100001000,
        }),
        "mhpmcounter8h" => Some(BitVector::<12> {
            val: 0b101110001000,
        }),
        "mhpmcounter9" => Some(BitVector::<12> {
            val: 0b101100001001,
        }),
        "mhpmcounter9h" => Some(BitVector::<12> {
            val: 0b101110001001,
        }),
        "mhpmevent10" => Some(BitVector::<12> {
            val: 0b001100101010,
        }),
        "mhpmevent10h" => Some(BitVector::<12> {
            val: 0b011100101010,
        }),
        "mhpmevent11" => Some(BitVector::<12> {
            val: 0b001100101011,
        }),
        "mhpmevent11h" => Some(BitVector::<12> {
            val: 0b011100101011,
        }),
        "mhpmevent12" => Some(BitVector::<12> {
            val: 0b001100101100,
        }),
        "mhpmevent12h" => Some(BitVector::<12> {
            val: 0b011100101100,
        }),
        "mhpmevent13" => Some(BitVector::<12> {
            val: 0b001100101101,
        }),
        "mhpmevent13h" => Some(BitVector::<12> {
            val: 0b011100101101,
        }),
        "mhpmevent14" => Some(BitVector::<12> {
            val: 0b001100101110,
        }),
        "mhpmevent14h" => Some(BitVector::<12> {
            val: 0b011100101110,
        }),
        "mhpmevent15" => Some(BitVector::<12> {
            val: 0b001100101111,
        }),
        "mhpmevent15h" => Some(BitVector::<12> {
            val: 0b011100101111,
        }),
        "mhpmevent16" => Some(BitVector::<12> {
            val: 0b001100110000,
        }),
        "mhpmevent16h" => Some(BitVector::<12> {
            val: 0b011100110000,
        }),
        "mhpmevent17" => Some(BitVector::<12> {
            val: 0b001100110001,
        }),
        "mhpmevent17h" => Some(BitVector::<12> {
            val: 0b011100110001,
        }),
        "mhpmevent18" => Some(BitVector::<12> {
            val: 0b001100110010,
        }),
        "mhpmevent18h" => Some(BitVector::<12> {
            val: 0b011100110010,
        }),
        "mhpmevent19" => Some(BitVector::<12> {
            val: 0b001100110011,
        }),
        "mhpmevent19h" => Some(BitVector::<12> {
            val: 0b011100110011,
        }),
        "mhpmevent20" => Some(BitVector::<12> {
            val: 0b001100110100,
        }),
        "mhpmevent20h" => Some(BitVector::<12> {
            val: 0b011100110100,
        }),
        "mhpmevent21" => Some(BitVector::<12> {
            val: 0b001100110101,
        }),
        "mhpmevent21h" => Some(BitVector::<12> {
            val: 0b011100110101,
        }),
        "mhpmevent22" => Some(BitVector::<12> {
            val: 0b001100110110,
        }),
        "mhpmevent22h" => Some(BitVector::<12> {
            val: 0b011100110110,
        }),
        "mhpmevent23" => Some(BitVector::<12> {
            val: 0b001100110111,
        }),
        "mhpmevent23h" => Some(BitVector::<12> {
            val: 0b011100110111,
        }),
        "mhpmevent24" => Some(BitVector::<12> {
            val: 0b001100111000,
        }),
        "mhpmevent24h" => Some(BitVector::<12> {
            val: 0b011100111000,
        }),
        "mhpmevent25" => Some(BitVector::<12> {
            val: 0b001100111001,
        }),
        "mhpmevent25h" => Some(BitVector::<12> {
            val: 0b011100111001,
        }),
        "mhpmevent26" => Some(BitVector::<12> {
            val: 0b001100111010,
        }),
        "mhpmevent26h" => Some(BitVector::<12> {
            val: 0b011100111010,
        }),
        "mhpmevent27" => Some(BitVector::<12> {
            val: 0b001100111011,
        }),
        "mhpmevent27h" => Some(BitVector::<12> {
            val: 0b011100111011,
        }),
        "mhpmevent28" => Some(BitVector::<12> {
            val: 0b001100111100,
        }),
        "mhpmevent28h" => Some(BitVector::<12> {
            val: 0b011100111100,
        }),
        "mhpmevent29" => Some(BitVector::<12> {
            val: 0b001100111101,
        }),
        "mhpmevent29h" => Some(BitVector::<12> {
            val: 0b011100111101,
        }),
        "mhpmevent3" => Some(BitVector::<12> {
            val: 0b001100100011,
        }),
        "mhpmevent30" => Some(BitVector::<12> {
            val: 0b001100111110,
        }),
        "mhpmevent30h" => Some(BitVector::<12> {
            val: 0b011100111110,
        }),
        "mhpmevent31" => Some(BitVector::<12> {
            val: 0b001100111111,
        }),
        "mhpmevent31h" => Some(BitVector::<12> {
            val: 0b011100111111,
        }),
        "mhpmevent3h" => Some(BitVector::<12> {
            val: 0b011100100011,
        }),
        "mhpmevent4" => Some(BitVector::<12> {
            val: 0b001100100100,
        }),
        "mhpmevent4h" => Some(BitVector::<12> {
            val: 0b011100100100,
        }),
        "mhpmevent5" => Some(BitVector::<12> {
            val: 0b001100100101,
        }),
        "mhpmevent5h" => Some(BitVector::<12> {
            val: 0b011100100101,
        }),
        "mhpmevent6" => Some(BitVector::<12> {
            val: 0b001100100110,
        }),
        "mhpmevent6h" => Some(BitVector::<12> {
            val: 0b011100100110,
        }),
        "mhpmevent7" => Some(BitVector::<12> {
            val: 0b001100100111,
        }),
        "mhpmevent7h" => Some(BitVector::<12> {
            val: 0b011100100111,
        }),
        "mhpmevent8" => Some(BitVector::<12> {
            val: 0b001100101000,
        }),
        "mhpmevent8h" => Some(BitVector::<12> {
            val: 0b011100101000,
        }),
        "mhpmevent9" => Some(BitVector::<12> {
            val: 0b001100101001,
        }),
        "mhpmevent9h" => Some(BitVector::<12> {
            val: 0b011100101001,
        }),
        "mideleg" => Some(BitVector::<12> {
            val: 0b001100000011,
        }),
        "mie" => Some(BitVector::<12> {
            val: 0b001100000100,
        }),
        "mimpid" => Some(BitVector::<12> {
            val: 0b111100010011,
        }),
        "minstret" => Some(BitVector::<12> {
            val: 0b101100000010,
        }),
        "minstretcfg" => Some(BitVector::<12> {
            val: 0b001100100010,
        }),
        "minstretcfgh" => Some(BitVector::<12> {
            val: 0b011100100010,
        }),
        "minstreth" => Some(BitVector::<12> {
            val: 0b101110000010,
        }),
        "mip" => Some(BitVector::<12> {
            val: 0b001101000100,
        }),
        "misa" => Some(BitVector::<12> {
            val: 0b001100000001,
        }),
        "mscratch" => Some(BitVector::<12> {
            val: 0b001101000000,
        }),
        "mseccfg" => Some(BitVector::<12> {
            val: 0b011101000111,
        }),
        "mseccfgh" => Some(BitVector::<12> {
            val: 0b011101010111,
        }),
        "mstateen0" => Some(BitVector::<12> {
            val: 0b001100001100,
        }),
        "mstateen0h" => Some(BitVector::<12> {
            val: 0b001100011100,
        }),
        "mstateen1" => Some(BitVector::<12> {
            val: 0b001100001101,
        }),
        "mstateen1h" => Some(BitVector::<12> {
            val: 0b001100011101,
        }),
        "mstateen2" => Some(BitVector::<12> {
            val: 0b001100001110,
        }),
        "mstateen2h" => Some(BitVector::<12> {
            val: 0b001100011110,
        }),
        "mstateen3" => Some(BitVector::<12> {
            val: 0b001100001111,
        }),
        "mstateen3h" => Some(BitVector::<12> {
            val: 0b001100011111,
        }),
        "mstatus" => Some(BitVector::<12> {
            val: 0b001100000000,
        }),
        "mstatush" => Some(BitVector::<12> {
            val: 0b001100010000,
        }),
        "mtval" => Some(BitVector::<12> {
            val: 0b001101000011,
        }),
        "mtvec" => Some(BitVector::<12> {
            val: 0b001100000101,
        }),
        "mvendorid" => Some(BitVector::<12> {
            val: 0b111100010001,
        }),
        "pmpaddr0" => Some(BitVector::<12> {
            val: 0b001110110000,
        }),
        "pmpaddr1" => Some(BitVector::<12> {
            val: 0b001110110001,
        }),
        "pmpaddr10" => Some(BitVector::<12> {
            val: 0b001110111010,
        }),
        "pmpaddr11" => Some(BitVector::<12> {
            val: 0b001110111011,
        }),
        "pmpaddr12" => Some(BitVector::<12> {
            val: 0b001110111100,
        }),
        "pmpaddr13" => Some(BitVector::<12> {
            val: 0b001110111101,
        }),
        "pmpaddr14" => Some(BitVector::<12> {
            val: 0b001110111110,
        }),
        "pmpaddr15" => Some(BitVector::<12> {
            val: 0b001110111111,
        }),
        "pmpaddr16" => Some(BitVector::<12> {
            val: 0b001111000000,
        }),
        "pmpaddr17" => Some(BitVector::<12> {
            val: 0b001111000001,
        }),
        "pmpaddr18" => Some(BitVector::<12> {
            val: 0b001111000010,
        }),
        "pmpaddr19" => Some(BitVector::<12> {
            val: 0b001111000011,
        }),
        "pmpaddr2" => Some(BitVector::<12> {
            val: 0b001110110010,
        }),
        "pmpaddr20" => Some(BitVector::<12> {
            val: 0b001111000100,
        }),
        "pmpaddr21" => Some(BitVector::<12> {
            val: 0b001111000101,
        }),
        "pmpaddr22" => Some(BitVector::<12> {
            val: 0b001111000110,
        }),
        "pmpaddr23" => Some(BitVector::<12> {
            val: 0b001111000111,
        }),
        "pmpaddr24" => Some(BitVector::<12> {
            val: 0b001111001000,
        }),
        "pmpaddr25" => Some(BitVector::<12> {
            val: 0b001111001001,
        }),
        "pmpaddr26" => Some(BitVector::<12> {
            val: 0b001111001010,
        }),
        "pmpaddr27" => Some(BitVector::<12> {
            val: 0b001111001011,
        }),
        "pmpaddr28" => Some(BitVector::<12> {
            val: 0b001111001100,
        }),
        "pmpaddr29" => Some(BitVector::<12> {
            val: 0b001111001101,
        }),
        "pmpaddr3" => Some(BitVector::<12> {
            val: 0b001110110011,
        }),
        "pmpaddr30" => Some(BitVector::<12> {
            val: 0b001111001110,
        }),
        "pmpaddr31" => Some(BitVector::<12> {
            val: 0b001111001111,
        }),
        "pmpaddr32" => Some(BitVector::<12> {
            val: 0b001111010000,
        }),
        "pmpaddr33" => Some(BitVector::<12> {
            val: 0b001111010001,
        }),
        "pmpaddr34" => Some(BitVector::<12> {
            val: 0b001111010010,
        }),
        "pmpaddr35" => Some(BitVector::<12> {
            val: 0b001111010011,
        }),
        "pmpaddr36" => Some(BitVector::<12> {
            val: 0b001111010100,
        }),
        "pmpaddr37" => Some(BitVector::<12> {
            val: 0b001111010101,
        }),
        "pmpaddr38" => Some(BitVector::<12> {
            val: 0b001111010110,
        }),
        "pmpaddr39" => Some(BitVector::<12> {
            val: 0b001111010111,
        }),
        "pmpaddr4" => Some(BitVector::<12> {
            val: 0b001110110100,
        }),
        "pmpaddr40" => Some(BitVector::<12> {
            val: 0b001111011000,
        }),
        "pmpaddr41" => Some(BitVector::<12> {
            val: 0b001111011001,
        }),
        "pmpaddr42" => Some(BitVector::<12> {
            val: 0b001111011010,
        }),
        "pmpaddr43" => Some(BitVector::<12> {
            val: 0b001111011011,
        }),
        "pmpaddr44" => Some(BitVector::<12> {
            val: 0b001111011100,
        }),
        "pmpaddr45" => Some(BitVector::<12> {
            val: 0b001111011101,
        }),
        "pmpaddr46" => Some(BitVector::<12> {
            val: 0b001111011110,
        }),
        "pmpaddr47" => Some(BitVector::<12> {
            val: 0b001111011111,
        }),
        "pmpaddr48" => Some(BitVector::<12> {
            val: 0b001111100000,
        }),
        "pmpaddr49" => Some(BitVector::<12> {
            val: 0b001111100001,
        }),
        "pmpaddr5" => Some(BitVector::<12> {
            val: 0b001110110101,
        }),
        "pmpaddr50" => Some(BitVector::<12> {
            val: 0b001111100010,
        }),
        "pmpaddr51" => Some(BitVector::<12> {
            val: 0b001111100011,
        }),
        "pmpaddr52" => Some(BitVector::<12> {
            val: 0b001111100100,
        }),
        "pmpaddr53" => Some(BitVector::<12> {
            val: 0b001111100101,
        }),
        "pmpaddr54" => Some(BitVector::<12> {
            val: 0b001111100110,
        }),
        "pmpaddr55" => Some(BitVector::<12> {
            val: 0b001111100111,
        }),
        "pmpaddr56" => Some(BitVector::<12> {
            val: 0b001111101000,
        }),
        "pmpaddr57" => Some(BitVector::<12> {
            val: 0b001111101001,
        }),
        "pmpaddr58" => Some(BitVector::<12> {
            val: 0b001111101010,
        }),
        "pmpaddr59" => Some(BitVector::<12> {
            val: 0b001111101011,
        }),
        "pmpaddr6" => Some(BitVector::<12> {
            val: 0b001110110110,
        }),
        "pmpaddr60" => Some(BitVector::<12> {
            val: 0b001111101100,
        }),
        "pmpaddr61" => Some(BitVector::<12> {
            val: 0b001111101101,
        }),
        "pmpaddr62" => Some(BitVector::<12> {
            val: 0b001111101110,
        }),
        "pmpaddr63" => Some(BitVector::<12> {
            val: 0b001111101111,
        }),
        "pmpaddr7" => Some(BitVector::<12> {
            val: 0b001110110111,
        }),
        "pmpaddr8" => Some(BitVector::<12> {
            val: 0b001110111000,
        }),
        "pmpaddr9" => Some(BitVector::<12> {
            val: 0b001110111001,
        }),
        "pmpcfg0" => Some(BitVector::<12> {
            val: 0b001110100000,
        }),
        "pmpcfg1" => Some(BitVector::<12> {
            val: 0b001110100001,
        }),
        "pmpcfg10" => Some(BitVector::<12> {
            val: 0b001110101010,
        }),
        "pmpcfg11" => Some(BitVector::<12> {
            val: 0b001110101011,
        }),
        "pmpcfg12" => Some(BitVector::<12> {
            val: 0b001110101100,
        }),
        "pmpcfg13" => Some(BitVector::<12> {
            val: 0b001110101101,
        }),
        "pmpcfg14" => Some(BitVector::<12> {
            val: 0b001110101110,
        }),
        "pmpcfg15" => Some(BitVector::<12> {
            val: 0b001110101111,
        }),
        "pmpcfg2" => Some(BitVector::<12> {
            val: 0b001110100010,
        }),
        "pmpcfg3" => Some(BitVector::<12> {
            val: 0b001110100011,
        }),
        "pmpcfg4" => Some(BitVector::<12> {
            val: 0b001110100100,
        }),
        "pmpcfg5" => Some(BitVector::<12> {
            val: 0b001110100101,
        }),
        "pmpcfg6" => Some(BitVector::<12> {
            val: 0b001110100110,
        }),
        "pmpcfg7" => Some(BitVector::<12> {
            val: 0b001110100111,
        }),
        "pmpcfg8" => Some(BitVector::<12> {
            val: 0b001110101000,
        }),
        "pmpcfg9" => Some(BitVector::<12> {
            val: 0b001110101001,
        }),
        "satp" => Some(BitVector::<12> {
            val: 0b000110000000,
        }),
        "scause" => Some(BitVector::<12> {
            val: 0b000101000010,
        }),
        "scounteren" => Some(BitVector::<12> {
            val: 0b000100000110,
        }),
        "scountovf" => Some(BitVector::<12> {
            val: 0b110110100000,
        }),
        "seed" => Some(BitVector::<12> {
            val: 0b000000010101,
        }),
        "senvcfg" => Some(BitVector::<12> {
            val: 0b000100001010,
        }),
        "sepc" => Some(BitVector::<12> {
            val: 0b000101000001,
        }),
        "sie" => Some(BitVector::<12> {
            val: 0b000100000100,
        }),
        "sip" => Some(BitVector::<12> {
            val: 0b000101000100,
        }),
        "srmcfg" => Some(BitVector::<12> {
            val: 0b000110000001,
        }),
        "sscratch" => Some(BitVector::<12> {
            val: 0b000101000000,
        }),
        "sstateen0" => Some(BitVector::<12> {
            val: 0b000100001100,
        }),
        "sstateen1" => Some(BitVector::<12> {
            val: 0b000100001101,
        }),
        "sstateen2" => Some(BitVector::<12> {
            val: 0b000100001110,
        }),
        "sstateen3" => Some(BitVector::<12> {
            val: 0b000100001111,
        }),
        "sstatus" => Some(BitVector::<12> {
            val: 0b000100000000,
        }),
        "stimecmp" => Some(BitVector::<12> {
            val: 0b000101001101,
        }),
        "stimecmph" => Some(BitVector::<12> {
            val: 0b000101011101,
        }),
        "stval" => Some(BitVector::<12> {
            val: 0b000101000011,
        }),
        "stvec" => Some(BitVector::<12> {
            val: 0b000100000101,
        }),
        "tdata1" => Some(BitVector::<12> {
            val: 0b011110100001,
        }),
        "tdata2" => Some(BitVector::<12> {
            val: 0b011110100010,
        }),
        "tdata3" => Some(BitVector::<12> {
            val: 0b011110100011,
        }),
        "time" => Some(BitVector::<12> {
            val: 0b110000000001,
        }),
        "timeh" => Some(BitVector::<12> {
            val: 0b110010000001,
        }),
        "tselect" => Some(BitVector::<12> {
            val: 0b011110100000,
        }),
        "vcsr" => Some(BitVector::<12> {
            val: 0b000000001111,
        }),
        "vl" => Some(BitVector::<12> {
            val: 0b110000100000,
        }),
        "vlenb" => Some(BitVector::<12> {
            val: 0b110000100010,
        }),
        "vstart" => Some(BitVector::<12> {
            val: 0b000000001000,
        }),
        "vtype" => Some(BitVector::<12> {
            val: 0b110000100001,
        }),
        "vxrm" => Some(BitVector::<12> {
            val: 0b000000001010,
        }),
        "vxsat" => Some(BitVector::<12> {
            val: 0b000000001001,
        }),
        _ => None,
    }
}
pub fn csr_name_map(p: &mut Parser) -> crate::Result<BitVector<12>> {
    let ident = p.expect_alpha_num()?;
    match csr_code(ident) {
        Some(code) => Ok(code),
        None => {
            let v = crate::parser::parse_u64(ident)?;
            Ok(BitVector::<12>::new(v as u32))
        }
    }
}
#[derive(Default, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Extensions {
    pub Ext_M: bool,
    pub Ext_A: bool,
    pub Ext_F: bool,
    pub Ext_D: bool,
    pub Ext_B: bool,
    pub Ext_V: bool,
    pub Ext_S: bool,
    pub Ext_U: bool,
    pub Ext_H: bool,
    pub Ext_Zic64b: bool,
    pub Ext_Zicbom: bool,
    pub Ext_Zicbop: bool,
    pub Ext_Zicboz: bool,
    pub Ext_Zicfilp: bool,
    pub Ext_Zicntr: bool,
    pub Ext_Zicond: bool,
    pub Ext_Zicsr: bool,
    pub Ext_Zifencei: bool,
    pub Ext_Zihintntl: bool,
    pub Ext_Zihintpause: bool,
    pub Ext_Zihpm: bool,
    pub Ext_Zimop: bool,
    pub Ext_Zmmul: bool,
    pub Ext_Zaamo: bool,
    pub Ext_Zabha: bool,
    pub Ext_Zacas: bool,
    pub Ext_Zalrsc: bool,
    pub Ext_Zawrs: bool,
    pub Ext_Za64rs: bool,
    pub Ext_Za128rs: bool,
    pub Ext_Zfa: bool,
    pub Ext_Zfbfmin: bool,
    pub Ext_Zfh: bool,
    pub Ext_Zfhmin: bool,
    pub Ext_Zfinx: bool,
    pub Ext_Zdinx: bool,
    pub Ext_Zca: bool,
    pub Ext_Zcb: bool,
    pub Ext_Zcd: bool,
    pub Ext_Zcf: bool,
    pub Ext_Zcmop: bool,
    pub Ext_C: bool,
    pub Ext_Zba: bool,
    pub Ext_Zbb: bool,
    pub Ext_Zbc: bool,
    pub Ext_Zbkb: bool,
    pub Ext_Zbkc: bool,
    pub Ext_Zbkx: bool,
    pub Ext_Zbs: bool,
    pub Ext_Zknd: bool,
    pub Ext_Zkne: bool,
    pub Ext_Zknh: bool,
    pub Ext_Zkr: bool,
    pub Ext_Zksed: bool,
    pub Ext_Zksh: bool,
    pub Ext_Zkt: bool,
    pub Ext_Zhinx: bool,
    pub Ext_Zhinxmin: bool,
    pub Ext_Zvl32b: bool,
    pub Ext_Zvl64b: bool,
    pub Ext_Zvl128b: bool,
    pub Ext_Zvl256b: bool,
    pub Ext_Zvl512b: bool,
    pub Ext_Zvl1024b: bool,
    pub Ext_Zve32f: bool,
    pub Ext_Zve32x: bool,
    pub Ext_Zve64d: bool,
    pub Ext_Zve64f: bool,
    pub Ext_Zve64x: bool,
    pub Ext_Zvfbfmin: bool,
    pub Ext_Zvfbfwma: bool,
    pub Ext_Zvfh: bool,
    pub Ext_Zvfhmin: bool,
    pub Ext_Zvbb: bool,
    pub Ext_Zvbc: bool,
    pub Ext_Zvkb: bool,
    pub Ext_Zvkg: bool,
    pub Ext_Zvkned: bool,
    pub Ext_Zvknha: bool,
    pub Ext_Zvknhb: bool,
    pub Ext_Zvksed: bool,
    pub Ext_Zvksh: bool,
    pub Ext_Zvkt: bool,
    pub Ext_Zvkn: bool,
    pub Ext_Zvknc: bool,
    pub Ext_Zvkng: bool,
    pub Ext_Zvks: bool,
    pub Ext_Zvksc: bool,
    pub Ext_Zvksg: bool,
    pub Ext_Sscofpmf: bool,
    pub Ext_Ssstateen: bool,
    pub Ext_Sstc: bool,
    pub Ext_Sstvala: bool,
    pub Ext_Sstvecd: bool,
    pub Ext_Ssu64xl: bool,
    pub Ext_Svbare: bool,
    pub Ext_Sv32: bool,
    pub Ext_Sv39: bool,
    pub Ext_Sv48: bool,
    pub Ext_Sv57: bool,
    pub Ext_Svinval: bool,
    pub Ext_Svnapot: bool,
    pub Ext_Svpbmt: bool,
    pub Ext_Svrsw60t59b: bool,
    pub Ext_Smcntrpmf: bool,
    pub Ext_Smstateen: bool,
    pub Ext_Ssqosid: bool,
}
impl Extensions {
    pub fn new(extensions: &[&String]) -> crate::Result<Self> {
        let mut res = Self::default();
        for ext in extensions {
            match ext.to_lowercase().as_str() {
                "m" => res.Ext_M = true,
                "a" => res.Ext_A = true,
                "f" => res.Ext_F = true,
                "d" => res.Ext_D = true,
                "b" => res.Ext_B = true,
                "v" => res.Ext_V = true,
                "s" => res.Ext_S = true,
                "u" => res.Ext_U = true,
                "h" => res.Ext_H = true,
                "zic64b" => res.Ext_Zic64b = true,
                "zicbom" => res.Ext_Zicbom = true,
                "zicbop" => res.Ext_Zicbop = true,
                "zicboz" => res.Ext_Zicboz = true,
                "zicfilp" => res.Ext_Zicfilp = true,
                "zicntr" => res.Ext_Zicntr = true,
                "zicond" => res.Ext_Zicond = true,
                "zicsr" => res.Ext_Zicsr = true,
                "zifencei" => res.Ext_Zifencei = true,
                "zihintntl" => res.Ext_Zihintntl = true,
                "zihintpause" => res.Ext_Zihintpause = true,
                "zihpm" => res.Ext_Zihpm = true,
                "zimop" => res.Ext_Zimop = true,
                "zmmul" => res.Ext_Zmmul = true,
                "zaamo" => res.Ext_Zaamo = true,
                "zabha" => res.Ext_Zabha = true,
                "zacas" => res.Ext_Zacas = true,
                "zalrsc" => res.Ext_Zalrsc = true,
                "zawrs" => res.Ext_Zawrs = true,
                "za64rs" => res.Ext_Za64rs = true,
                "za128rs" => res.Ext_Za128rs = true,
                "zfa" => res.Ext_Zfa = true,
                "zfbfmin" => res.Ext_Zfbfmin = true,
                "zfh" => res.Ext_Zfh = true,
                "zfhmin" => res.Ext_Zfhmin = true,
                "zfinx" => res.Ext_Zfinx = true,
                "zdinx" => res.Ext_Zdinx = true,
                "zca" => res.Ext_Zca = true,
                "zcb" => res.Ext_Zcb = true,
                "zcd" => res.Ext_Zcd = true,
                "zcf" => res.Ext_Zcf = true,
                "zcmop" => res.Ext_Zcmop = true,
                "c" => res.Ext_C = true,
                "zba" => res.Ext_Zba = true,
                "zbb" => res.Ext_Zbb = true,
                "zbc" => res.Ext_Zbc = true,
                "zbkb" => res.Ext_Zbkb = true,
                "zbkc" => res.Ext_Zbkc = true,
                "zbkx" => res.Ext_Zbkx = true,
                "zbs" => res.Ext_Zbs = true,
                "zknd" => res.Ext_Zknd = true,
                "zkne" => res.Ext_Zkne = true,
                "zknh" => res.Ext_Zknh = true,
                "zkr" => res.Ext_Zkr = true,
                "zksed" => res.Ext_Zksed = true,
                "zksh" => res.Ext_Zksh = true,
                "zkt" => res.Ext_Zkt = true,
                "zhinx" => res.Ext_Zhinx = true,
                "zhinxmin" => res.Ext_Zhinxmin = true,
                "zvl32b" => res.Ext_Zvl32b = true,
                "zvl64b" => res.Ext_Zvl64b = true,
                "zvl128b" => res.Ext_Zvl128b = true,
                "zvl256b" => res.Ext_Zvl256b = true,
                "zvl512b" => res.Ext_Zvl512b = true,
                "zvl1024b" => res.Ext_Zvl1024b = true,
                "zve32f" => res.Ext_Zve32f = true,
                "zve32x" => res.Ext_Zve32x = true,
                "zve64d" => res.Ext_Zve64d = true,
                "zve64f" => res.Ext_Zve64f = true,
                "zve64x" => res.Ext_Zve64x = true,
                "zvfbfmin" => res.Ext_Zvfbfmin = true,
                "zvfbfwma" => res.Ext_Zvfbfwma = true,
                "zvfh" => res.Ext_Zvfh = true,
                "zvfhmin" => res.Ext_Zvfhmin = true,
                "zvbb" => res.Ext_Zvbb = true,
                "zvbc" => res.Ext_Zvbc = true,
                "zvkb" => res.Ext_Zvkb = true,
                "zvkg" => res.Ext_Zvkg = true,
                "zvkned" => res.Ext_Zvkned = true,
                "zvknha" => res.Ext_Zvknha = true,
                "zvknhb" => res.Ext_Zvknhb = true,
                "zvksed" => res.Ext_Zvksed = true,
                "zvksh" => res.Ext_Zvksh = true,
                "zvkt" => res.Ext_Zvkt = true,
                "zvkn" => res.Ext_Zvkn = true,
                "zvknc" => res.Ext_Zvknc = true,
                "zvkng" => res.Ext_Zvkng = true,
                "zvks" => res.Ext_Zvks = true,
                "zvksc" => res.Ext_Zvksc = true,
                "zvksg" => res.Ext_Zvksg = true,
                "sscofpmf" => res.Ext_Sscofpmf = true,
                "ssstateen" => res.Ext_Ssstateen = true,
                "sstc" => res.Ext_Sstc = true,
                "sstvala" => res.Ext_Sstvala = true,
                "sstvecd" => res.Ext_Sstvecd = true,
                "ssu64xl" => res.Ext_Ssu64xl = true,
                "svbare" => res.Ext_Svbare = true,
                "sv32" => res.Ext_Sv32 = true,
                "sv39" => res.Ext_Sv39 = true,
                "sv48" => res.Ext_Sv48 = true,
                "sv57" => res.Ext_Sv57 = true,
                "svinval" => res.Ext_Svinval = true,
                "svnapot" => res.Ext_Svnapot = true,
                "svpbmt" => res.Ext_Svpbmt = true,
                "svrsw60t59b" => res.Ext_Svrsw60t59b = true,
                "smcntrpmf" => res.Ext_Smcntrpmf = true,
                "smstateen" => res.Ext_Smstateen = true,
                "ssqosid" => res.Ext_Ssqosid = true,
                _ => return err!("unknown extension '{ext}'"),
            }
        }
        Ok(res)
    }
    pub const fn all() -> Self {
        Self {
            Ext_M: true,
            Ext_A: true,
            Ext_F: true,
            Ext_D: true,
            Ext_B: true,
            Ext_V: true,
            Ext_S: true,
            Ext_U: true,
            Ext_H: true,
            Ext_Zic64b: true,
            Ext_Zicbom: true,
            Ext_Zicbop: true,
            Ext_Zicboz: true,
            Ext_Zicfilp: true,
            Ext_Zicntr: true,
            Ext_Zicond: true,
            Ext_Zicsr: true,
            Ext_Zifencei: true,
            Ext_Zihintntl: true,
            Ext_Zihintpause: true,
            Ext_Zihpm: true,
            Ext_Zimop: true,
            Ext_Zmmul: true,
            Ext_Zaamo: true,
            Ext_Zabha: true,
            Ext_Zacas: true,
            Ext_Zalrsc: true,
            Ext_Zawrs: true,
            Ext_Za64rs: true,
            Ext_Za128rs: true,
            Ext_Zfa: true,
            Ext_Zfbfmin: true,
            Ext_Zfh: true,
            Ext_Zfhmin: true,
            Ext_Zfinx: true,
            Ext_Zdinx: true,
            Ext_Zca: true,
            Ext_Zcb: true,
            Ext_Zcd: true,
            Ext_Zcf: true,
            Ext_Zcmop: true,
            Ext_C: true,
            Ext_Zba: true,
            Ext_Zbb: true,
            Ext_Zbc: true,
            Ext_Zbkb: true,
            Ext_Zbkc: true,
            Ext_Zbkx: true,
            Ext_Zbs: true,
            Ext_Zknd: true,
            Ext_Zkne: true,
            Ext_Zknh: true,
            Ext_Zkr: true,
            Ext_Zksed: true,
            Ext_Zksh: true,
            Ext_Zkt: true,
            Ext_Zhinx: true,
            Ext_Zhinxmin: true,
            Ext_Zvl32b: true,
            Ext_Zvl64b: true,
            Ext_Zvl128b: true,
            Ext_Zvl256b: true,
            Ext_Zvl512b: true,
            Ext_Zvl1024b: true,
            Ext_Zve32f: true,
            Ext_Zve32x: true,
            Ext_Zve64d: true,
            Ext_Zve64f: true,
            Ext_Zve64x: true,
            Ext_Zvfbfmin: true,
            Ext_Zvfbfwma: true,
            Ext_Zvfh: true,
            Ext_Zvfhmin: true,
            Ext_Zvbb: true,
            Ext_Zvbc: true,
            Ext_Zvkb: true,
            Ext_Zvkg: true,
            Ext_Zvkned: true,
            Ext_Zvknha: true,
            Ext_Zvknhb: true,
            Ext_Zvksed: true,
            Ext_Zvksh: true,
            Ext_Zvkt: true,
            Ext_Zvkn: true,
            Ext_Zvknc: true,
            Ext_Zvkng: true,
            Ext_Zvks: true,
            Ext_Zvksc: true,
            Ext_Zvksg: true,
            Ext_Sscofpmf: true,
            Ext_Ssstateen: true,
            Ext_Sstc: true,
            Ext_Sstvala: true,
            Ext_Sstvecd: true,
            Ext_Ssu64xl: true,
            Ext_Svbare: true,
            Ext_Sv32: true,
            Ext_Sv39: true,
            Ext_Sv48: true,
            Ext_Sv57: true,
            Ext_Svinval: true,
            Ext_Svnapot: true,
            Ext_Svpbmt: true,
            Ext_Svrsw60t59b: true,
            Ext_Smcntrpmf: true,
            Ext_Smstateen: true,
            Ext_Ssqosid: true,
        }
    }
}
#[allow(non_camel_case_types)]
pub(crate) enum amoop {
    AMOSWAP,
    AMOADD,
    AMOXOR,
    AMOAND,
    AMOOR,
    AMOMIN,
    AMOMAX,
    AMOMINU,
    AMOMAXU,
    AMOCAS,
}
#[allow(non_camel_case_types)]
pub(crate) enum biop_zbs {
    BCLRI,
    BEXTI,
    BINVI,
    BSETI,
}
#[allow(non_camel_case_types)]
pub(crate) enum bop {
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
}
#[allow(non_camel_case_types)]
pub(crate) enum brop_zbb {
    ANDN,
    ORN,
    XNOR,
    MAX,
    MAXU,
    MIN,
    MINU,
    ROL,
    ROR,
}
#[allow(non_camel_case_types)]
pub(crate) enum brop_zbkb {
    PACK,
    PACKH,
}
#[allow(non_camel_case_types)]
pub(crate) enum brop_zbs {
    BCLR,
    BEXT,
    BINV,
    BSET,
}
#[allow(non_camel_case_types)]
pub(crate) enum bropw_zbb {
    ROLW,
    RORW,
}
#[allow(non_camel_case_types)]
pub(crate) enum cbop_zicbom {
    CBO_CLEAN,
    CBO_FLUSH,
    CBO_INVAL,
}
#[allow(non_camel_case_types)]
pub(crate) enum cbop_zicbop {
    PREFETCH_I,
    PREFETCH_R,
    PREFETCH_W,
}
#[allow(non_camel_case_types)]
pub(crate) enum csrop {
    CSRRW,
    CSRRS,
    CSRRC,
}
#[allow(non_camel_case_types)]
pub(crate) enum extop_zbb {
    SEXTB,
    SEXTH,
    ZEXTH,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_bin_f_op_D {
    FSGNJ_D,
    FSGNJN_D,
    FSGNJX_D,
    FMIN_D,
    FMAX_D,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_bin_f_op_H {
    FSGNJ_H,
    FSGNJN_H,
    FSGNJX_H,
    FMIN_H,
    FMAX_H,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_bin_op_f_S {
    FSGNJ_S,
    FSGNJN_S,
    FSGNJX_S,
    FMIN_S,
    FMAX_S,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_bin_op_x_S {
    FEQ_S,
    FLT_S,
    FLE_S,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_bin_rm_op_D {
    FADD_D,
    FSUB_D,
    FMUL_D,
    FDIV_D,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_bin_rm_op_H {
    FADD_H,
    FSUB_H,
    FMUL_H,
    FDIV_H,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_bin_rm_op_S {
    FADD_S,
    FSUB_S,
    FMUL_S,
    FDIV_S,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_bin_x_op_D {
    FEQ_D,
    FLT_D,
    FLE_D,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_bin_x_op_H {
    FEQ_H,
    FLT_H,
    FLE_H,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_madd_op_D {
    FMADD_D,
    FMSUB_D,
    FNMSUB_D,
    FNMADD_D,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_madd_op_H {
    FMADD_H,
    FMSUB_H,
    FNMSUB_H,
    FNMADD_H,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_madd_op_S {
    FMADD_S,
    FMSUB_S,
    FNMSUB_S,
    FNMADD_S,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_un_f_op_D {
    FMV_D_X,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_un_f_op_H {
    FMV_H_X,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_un_op_f_S {
    FMV_W_X,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_un_op_x_S {
    FCLASS_S,
    FMV_X_W,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_un_rm_ff_op_D {
    FSQRT_D,
    FCVT_S_D,
    FCVT_D_S,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_un_rm_ff_op_H {
    FSQRT_H,
    FCVT_H_S,
    FCVT_H_D,
    FCVT_S_H,
    FCVT_D_H,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_un_rm_ff_op_S {
    FSQRT_S,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_un_rm_fx_op_D {
    FCVT_W_D,
    FCVT_WU_D,
    FCVT_L_D,
    FCVT_LU_D,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_un_rm_fx_op_H {
    FCVT_W_H,
    FCVT_WU_H,
    FCVT_L_H,
    FCVT_LU_H,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_un_rm_fx_op_S {
    FCVT_W_S,
    FCVT_WU_S,
    FCVT_L_S,
    FCVT_LU_S,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_un_rm_xf_op_D {
    FCVT_D_W,
    FCVT_D_WU,
    FCVT_D_L,
    FCVT_D_LU,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_un_rm_xf_op_H {
    FCVT_H_W,
    FCVT_H_WU,
    FCVT_H_L,
    FCVT_H_LU,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_un_rm_xf_op_S {
    FCVT_S_W,
    FCVT_S_WU,
    FCVT_S_L,
    FCVT_S_LU,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_un_x_op_D {
    FCLASS_D,
    FMV_X_D,
}
#[allow(non_camel_case_types)]
pub(crate) enum f_un_x_op_H {
    FCLASS_H,
    FMV_X_H,
}
#[allow(non_camel_case_types)]
pub(crate) enum fvffunct6 {
    VF_VADD,
    VF_VSUB,
    VF_VMIN,
    VF_VMAX,
    VF_VSGNJ,
    VF_VSGNJN,
    VF_VSGNJX,
    VF_VDIV,
    VF_VRDIV,
    VF_VMUL,
    VF_VRSUB,
    VF_VSLIDE1UP,
    VF_VSLIDE1DOWN,
}
#[allow(non_camel_case_types)]
pub(crate) enum fvfmafunct6 {
    VF_VMADD,
    VF_VNMADD,
    VF_VMSUB,
    VF_VNMSUB,
    VF_VMACC,
    VF_VNMACC,
    VF_VMSAC,
    VF_VNMSAC,
}
#[allow(non_camel_case_types)]
pub(crate) enum fvfmfunct6 {
    VFM_VMFEQ,
    VFM_VMFLE,
    VFM_VMFLT,
    VFM_VMFNE,
    VFM_VMFGT,
    VFM_VMFGE,
}
#[allow(non_camel_case_types)]
pub(crate) enum fvvfunct6 {
    FVV_VADD,
    FVV_VSUB,
    FVV_VMIN,
    FVV_VMAX,
    FVV_VSGNJ,
    FVV_VSGNJN,
    FVV_VSGNJX,
    FVV_VDIV,
    FVV_VMUL,
}
#[allow(non_camel_case_types)]
pub(crate) enum fvvmafunct6 {
    FVV_VMADD,
    FVV_VNMADD,
    FVV_VMSUB,
    FVV_VNMSUB,
    FVV_VMACC,
    FVV_VNMACC,
    FVV_VMSAC,
    FVV_VNMSAC,
}
#[allow(non_camel_case_types)]
pub(crate) enum fvvmfunct6 {
    FVVM_VMFEQ,
    FVVM_VMFLE,
    FVVM_VMFLT,
    FVVM_VMFNE,
}
#[allow(non_camel_case_types)]
pub(crate) enum fwffunct6 {
    FWF_VADD,
    FWF_VSUB,
}
#[allow(non_camel_case_types)]
pub(crate) enum fwvffunct6 {
    FWVF_VADD,
    FWVF_VSUB,
    FWVF_VMUL,
}
#[allow(non_camel_case_types)]
pub(crate) enum fwvfmafunct6 {
    FWVF_VMACC,
    FWVF_VNMACC,
    FWVF_VMSAC,
    FWVF_VNMSAC,
}
#[allow(non_camel_case_types)]
pub(crate) enum fwvfunct6 {
    FWV_VADD,
    FWV_VSUB,
}
#[allow(non_camel_case_types)]
pub(crate) enum fwvvfunct6 {
    FWVV_VADD,
    FWVV_VSUB,
    FWVV_VMUL,
}
#[allow(non_camel_case_types)]
pub(crate) enum fwvvmafunct6 {
    FWVV_VMACC,
    FWVV_VNMACC,
    FWVV_VMSAC,
    FWVV_VNMSAC,
}
#[allow(non_camel_case_types)]
pub(crate) enum indexed_mop {
    INDEXED_UNORDERED,
    INDEXED_ORDERED,
}
#[allow(non_camel_case_types)]
pub(crate) enum iop {
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
}
#[allow(non_camel_case_types)]
pub(crate) enum mmfunct6 {
    MM_VMAND,
    MM_VMNAND,
    MM_VMANDN,
    MM_VMXOR,
    MM_VMOR,
    MM_VMNOR,
    MM_VMORN,
    MM_VMXNOR,
}
#[allow(non_camel_case_types)]
pub(crate) enum mvvfunct6 {
    MVV_VAADDU,
    MVV_VAADD,
    MVV_VASUBU,
    MVV_VASUB,
    MVV_VMUL,
    MVV_VMULH,
    MVV_VMULHU,
    MVV_VMULHSU,
    MVV_VDIVU,
    MVV_VDIV,
    MVV_VREMU,
    MVV_VREM,
}
#[allow(non_camel_case_types)]
pub(crate) enum mvvmafunct6 {
    MVV_VMACC,
    MVV_VNMSAC,
    MVV_VMADD,
    MVV_VNMSUB,
}
#[allow(non_camel_case_types)]
pub(crate) enum mvxfunct6 {
    MVX_VAADDU,
    MVX_VAADD,
    MVX_VASUBU,
    MVX_VASUB,
    MVX_VSLIDE1UP,
    MVX_VSLIDE1DOWN,
    MVX_VMUL,
    MVX_VMULH,
    MVX_VMULHU,
    MVX_VMULHSU,
    MVX_VDIVU,
    MVX_VDIV,
    MVX_VREMU,
    MVX_VREM,
}
#[allow(non_camel_case_types)]
pub(crate) enum mvxmafunct6 {
    MVX_VMACC,
    MVX_VNMSAC,
    MVX_VMADD,
    MVX_VNMSUB,
}
#[allow(non_camel_case_types)]
pub(crate) enum nifunct6 {
    NI_VNCLIPU,
    NI_VNCLIP,
}
#[allow(non_camel_case_types)]
pub(crate) enum nisfunct6 {
    NIS_VNSRL,
    NIS_VNSRA,
}
#[allow(non_camel_case_types)]
pub(crate) enum ntl_type {
    NTL_P1,
    NTL_PALL,
    NTL_S1,
    NTL_ALL,
}
#[allow(non_camel_case_types)]
pub(crate) enum nvfunct6 {
    NV_VNCLIPU,
    NV_VNCLIP,
}
#[allow(non_camel_case_types)]
pub(crate) enum nvsfunct6 {
    NVS_VNSRL,
    NVS_VNSRA,
}
#[allow(non_camel_case_types)]
pub(crate) enum nxfunct6 {
    NX_VNCLIPU,
    NX_VNCLIP,
}
#[allow(non_camel_case_types)]
pub(crate) enum nxsfunct6 {
    NXS_VNSRL,
    NXS_VNSRA,
}
#[allow(non_camel_case_types)]
pub(crate) enum rfvvfunct6 {
    FVV_VFREDOSUM,
    FVV_VFREDUSUM,
    FVV_VFREDMAX,
    FVV_VFREDMIN,
}
#[allow(non_camel_case_types)]
pub(crate) enum rfwvvfunct6 {
    FVV_VFWREDOSUM,
    FVV_VFWREDUSUM,
}
#[allow(non_camel_case_types)]
pub(crate) enum rivvfunct6 {
    IVV_VWREDSUMU,
    IVV_VWREDSUM,
}
#[allow(non_camel_case_types)]
pub(crate) enum rmvvfunct6 {
    MVV_VREDSUM,
    MVV_VREDAND,
    MVV_VREDOR,
    MVV_VREDXOR,
    MVV_VREDMINU,
    MVV_VREDMIN,
    MVV_VREDMAXU,
    MVV_VREDMAX,
}
#[allow(non_camel_case_types)]
pub(crate) enum rop {
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
}
#[allow(non_camel_case_types)]
pub(crate) enum ropw {
    ADDW,
    SUBW,
    SLLW,
    SRLW,
    SRAW,
}
#[allow(non_camel_case_types)]
pub(crate) enum rounding_mode {
    RM_RNE,
    RM_RTZ,
    RM_RDN,
    RM_RUP,
    RM_RMM,
    RM_DYN,
}
#[allow(non_camel_case_types)]
pub(crate) enum sop {
    SLLI,
    SRLI,
    SRAI,
}
#[allow(non_camel_case_types)]
pub(crate) enum sopw {
    SLLIW,
    SRLIW,
    SRAIW,
}
#[allow(non_camel_case_types)]
pub(crate) enum uop {
    LUI,
    AUIPC,
}
#[allow(non_camel_case_types)]
pub(crate) enum vextfunct6 {
    VEXT2_ZVF2,
    VEXT2_SVF2,
    VEXT4_ZVF4,
    VEXT4_SVF4,
    VEXT8_ZVF8,
    VEXT8_SVF8,
}
#[allow(non_camel_case_types)]
pub(crate) enum vfnunary0 {
    FNV_CVT_XU_F,
    FNV_CVT_X_F,
    FNV_CVT_F_XU,
    FNV_CVT_F_X,
    FNV_CVT_F_F,
    FNV_CVT_ROD_F_F,
    FNV_CVT_RTZ_XU_F,
    FNV_CVT_RTZ_X_F,
}
#[allow(non_camel_case_types)]
pub(crate) enum vfunary0 {
    FV_CVT_XU_F,
    FV_CVT_X_F,
    FV_CVT_F_XU,
    FV_CVT_F_X,
    FV_CVT_RTZ_XU_F,
    FV_CVT_RTZ_X_F,
}
#[allow(non_camel_case_types)]
pub(crate) enum vfunary1 {
    FVV_VSQRT,
    FVV_VRSQRT7,
    FVV_VREC7,
    FVV_VCLASS,
}
#[allow(non_camel_case_types)]
pub(crate) enum vfwunary0 {
    FWV_CVT_XU_F,
    FWV_CVT_X_F,
    FWV_CVT_F_XU,
    FWV_CVT_F_X,
    FWV_CVT_F_F,
    FWV_CVT_RTZ_XU_F,
    FWV_CVT_RTZ_X_F,
}
#[allow(non_camel_case_types)]
pub(crate) enum vicmpfunct6 {
    VICMP_VMSEQ,
    VICMP_VMSNE,
    VICMP_VMSLEU,
    VICMP_VMSLE,
    VICMP_VMSGTU,
    VICMP_VMSGT,
}
#[allow(non_camel_case_types)]
pub(crate) enum vifunct6 {
    VI_VADD,
    VI_VRSUB,
    VI_VAND,
    VI_VOR,
    VI_VXOR,
    VI_VSADDU,
    VI_VSADD,
    VI_VSLL,
    VI_VSRL,
    VI_VSRA,
    VI_VSSRL,
    VI_VSSRA,
}
#[allow(non_camel_case_types)]
pub(crate) enum vimcfunct6 {
    VIMC_VMADC,
}
#[allow(non_camel_case_types)]
pub(crate) enum vimfunct6 {
    VIM_VMADC,
}
#[allow(non_camel_case_types)]
pub(crate) enum vimsfunct6 {
    VIMS_VADC,
}
#[allow(non_camel_case_types)]
pub(crate) enum visgfunct6 {
    VI_VSLIDEUP,
    VI_VSLIDEDOWN,
    VI_VRGATHER,
}
#[allow(non_camel_case_types)]
pub(crate) enum vlewidth {
    VLE8,
    VLE16,
    VLE32,
    VLE64,
}
#[allow(non_camel_case_types)]
pub(crate) enum vmlsop {
    VLM,
    VSM,
}
#[allow(non_camel_case_types)]
pub(crate) enum vvcmpfunct6 {
    VVCMP_VMSEQ,
    VVCMP_VMSNE,
    VVCMP_VMSLTU,
    VVCMP_VMSLT,
    VVCMP_VMSLEU,
    VVCMP_VMSLE,
}
#[allow(non_camel_case_types)]
pub(crate) enum vvfunct6 {
    VV_VADD,
    VV_VSUB,
    VV_VMINU,
    VV_VMIN,
    VV_VMAXU,
    VV_VMAX,
    VV_VAND,
    VV_VOR,
    VV_VXOR,
    VV_VRGATHER,
    VV_VRGATHEREI16,
    VV_VSADDU,
    VV_VSADD,
    VV_VSSUBU,
    VV_VSSUB,
    VV_VSLL,
    VV_VSMUL,
    VV_VSRL,
    VV_VSRA,
    VV_VSSRL,
    VV_VSSRA,
}
#[allow(non_camel_case_types)]
pub(crate) enum vvmcfunct6 {
    VVMC_VMADC,
    VVMC_VMSBC,
}
#[allow(non_camel_case_types)]
pub(crate) enum vvmfunct6 {
    VVM_VMADC,
    VVM_VMSBC,
}
#[allow(non_camel_case_types)]
pub(crate) enum vvmsfunct6 {
    VVMS_VADC,
    VVMS_VSBC,
}
#[allow(non_camel_case_types)]
pub(crate) enum vxcmpfunct6 {
    VXCMP_VMSEQ,
    VXCMP_VMSNE,
    VXCMP_VMSLTU,
    VXCMP_VMSLT,
    VXCMP_VMSLEU,
    VXCMP_VMSLE,
    VXCMP_VMSGTU,
    VXCMP_VMSGT,
}
#[allow(non_camel_case_types)]
pub(crate) enum vxfunct6 {
    VX_VADD,
    VX_VSUB,
    VX_VRSUB,
    VX_VMINU,
    VX_VMIN,
    VX_VMAXU,
    VX_VMAX,
    VX_VAND,
    VX_VOR,
    VX_VXOR,
    VX_VSADDU,
    VX_VSADD,
    VX_VSSUBU,
    VX_VSSUB,
    VX_VSLL,
    VX_VSMUL,
    VX_VSRL,
    VX_VSRA,
    VX_VSSRL,
    VX_VSSRA,
}
#[allow(non_camel_case_types)]
pub(crate) enum vxmcfunct6 {
    VXMC_VMADC,
    VXMC_VMSBC,
}
#[allow(non_camel_case_types)]
pub(crate) enum vxmfunct6 {
    VXM_VMADC,
    VXM_VMSBC,
}
#[allow(non_camel_case_types)]
pub(crate) enum vxmsfunct6 {
    VXMS_VADC,
    VXMS_VSBC,
}
#[allow(non_camel_case_types)]
pub(crate) enum vxsgfunct6 {
    VX_VSLIDEUP,
    VX_VSLIDEDOWN,
    VX_VRGATHER,
}
#[allow(non_camel_case_types)]
pub(crate) enum wmvvfunct6 {
    WMVV_VWMACCU,
    WMVV_VWMACC,
    WMVV_VWMACCSU,
}
#[allow(non_camel_case_types)]
pub(crate) enum wmvxfunct6 {
    WMVX_VWMACCU,
    WMVX_VWMACC,
    WMVX_VWMACCUS,
    WMVX_VWMACCSU,
}
#[allow(non_camel_case_types)]
pub(crate) enum wrsop {
    WRS_STO,
    WRS_NTO,
}
#[allow(non_camel_case_types)]
pub(crate) enum wvfunct6 {
    WV_VADD,
    WV_VSUB,
    WV_VADDU,
    WV_VSUBU,
}
#[allow(non_camel_case_types)]
pub(crate) enum wvvfunct6 {
    WVV_VADD,
    WVV_VSUB,
    WVV_VADDU,
    WVV_VSUBU,
    WVV_VWMUL,
    WVV_VWMULU,
    WVV_VWMULSU,
}
#[allow(non_camel_case_types)]
pub(crate) enum wvxfunct6 {
    WVX_VADD,
    WVX_VSUB,
    WVX_VADDU,
    WVX_VSUBU,
    WVX_VWMUL,
    WVX_VWMULU,
    WVX_VWMULSU,
}
#[allow(non_camel_case_types)]
pub(crate) enum wxfunct6 {
    WX_VADD,
    WX_VSUB,
    WX_VADDU,
    WX_VSUBU,
}
#[allow(non_camel_case_types)]
pub(crate) enum zicondop {
    CZERO_EQZ,
    CZERO_NEZ,
}
#[allow(non_camel_case_types)]
pub(crate) enum zvk_vaesdf_funct6 {
    ZVK_VAESDF_VV,
    ZVK_VAESDF_VS,
}
#[allow(non_camel_case_types)]
pub(crate) enum zvk_vaesdm_funct6 {
    ZVK_VAESDM_VV,
    ZVK_VAESDM_VS,
}
#[allow(non_camel_case_types)]
pub(crate) enum zvk_vaesef_funct6 {
    ZVK_VAESEF_VV,
    ZVK_VAESEF_VS,
}
#[allow(non_camel_case_types)]
pub(crate) enum zvk_vaesem_funct6 {
    ZVK_VAESEM_VV,
    ZVK_VAESEM_VS,
}
#[allow(non_camel_case_types)]
pub(crate) enum zvk_vsha2_funct6 {
    ZVK_VSHA2CH_VV,
    ZVK_VSHA2CL_VV,
}
#[allow(non_camel_case_types)]
pub(crate) enum zvk_vsm4r_funct6 {
    ZVK_VSM4R_VV,
    ZVK_VSM4R_VS,
}
#[allow(non_camel_case_types)]
pub(crate) struct mul_op {
    pub(crate) result_part: VectorHalf,
    pub(crate) signed_rs1: Signedness,
    pub(crate) signed_rs2: Signedness,
}
fn parse_prefetch_i(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let cbop = cbop_zicbop::PREFETCH_I;
    // parse arguments
    let offset = parser.expect_unsigned_immediate::<12>()?;
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_zicbop(cbop, rs1, offset)
}
fn parse_prefetch_r(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let cbop = cbop_zicbop::PREFETCH_R;
    // parse arguments
    let offset = parser.expect_unsigned_immediate::<12>()?;
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_zicbop(cbop, rs1, offset)
}
fn parse_prefetch_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let cbop = cbop_zicbop::PREFETCH_W;
    // parse arguments
    let offset = parser.expect_unsigned_immediate::<12>()?;
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_zicbop(cbop, rs1, offset)
}
fn parse_ntl_p1() -> crate::Result<u32> {
    // predefined constants
    let op = ntl_type::NTL_P1;
    encode_ntl(op)
}
fn parse_ntl_pall() -> crate::Result<u32> {
    // predefined constants
    let op = ntl_type::NTL_PALL;
    encode_ntl(op)
}
fn parse_ntl_s1() -> crate::Result<u32> {
    // predefined constants
    let op = ntl_type::NTL_S1;
    encode_ntl(op)
}
fn parse_ntl_all() -> crate::Result<u32> {
    // predefined constants
    let op = ntl_type::NTL_ALL;
    encode_ntl(op)
}
fn parse_c_ntl_p1() -> crate::Result<u32> {
    // predefined constants
    let op = ntl_type::NTL_P1;
    encode_c_ntl(op)
}
fn parse_c_ntl_pall() -> crate::Result<u32> {
    // predefined constants
    let op = ntl_type::NTL_PALL;
    encode_c_ntl(op)
}
fn parse_c_ntl_s1() -> crate::Result<u32> {
    // predefined constants
    let op = ntl_type::NTL_S1;
    encode_c_ntl(op)
}
fn parse_c_ntl_all() -> crate::Result<u32> {
    // predefined constants
    let op = ntl_type::NTL_ALL;
    encode_c_ntl(op)
}
fn parse_pause() -> crate::Result<u32> {
    encode_pause()
}
fn parse_lpad(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let lpl = parser.expect_unsigned_immediate::<20>()?;
    encode_lpad(lpl)
}
fn parse_lui(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = uop::LUI;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<20>()?;
    encode_utype(imm, rd, op)
}
fn parse_auipc(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = uop::AUIPC;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<20>()?;
    encode_utype(imm, rd, op)
}
fn parse_jalr(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_jalr(imm, rs1, rd)
}
fn parse_addi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = iop::ADDI;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    encode_itype(imm, rs1, rd, op)
}
fn parse_slti(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = iop::SLTI;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    encode_itype(imm, rs1, rd, op)
}
fn parse_sltiu(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = iop::SLTIU;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    encode_itype(imm, rs1, rd, op)
}
fn parse_xori(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = iop::XORI;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    encode_itype(imm, rs1, rd, op)
}
fn parse_ori(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = iop::ORI;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    encode_itype(imm, rs1, rd, op)
}
fn parse_andi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = iop::ANDI;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    encode_itype(imm, rs1, rd, op)
}
fn parse_slli(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = sop::SLLI;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<6>()?;
    encode_shiftiop(shamt, rs1, rd, op)
}
fn parse_srli(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = sop::SRLI;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<6>()?;
    encode_shiftiop(shamt, rs1, rd, op)
}
fn parse_srai(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = sop::SRAI;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<6>()?;
    encode_shiftiop(shamt, rs1, rd, op)
}
fn parse_add(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = rop::ADD;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_rtype(rs2, rs1, rd, op)
}
fn parse_slt(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = rop::SLT;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_rtype(rs2, rs1, rd, op)
}
fn parse_sltu(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = rop::SLTU;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_rtype(rs2, rs1, rd, op)
}
fn parse_and(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = rop::AND;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_rtype(rs2, rs1, rd, op)
}
fn parse_or(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = rop::OR;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_rtype(rs2, rs1, rd, op)
}
fn parse_xor(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = rop::XOR;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_rtype(rs2, rs1, rd, op)
}
fn parse_sll(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = rop::SLL;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_rtype(rs2, rs1, rd, op)
}
fn parse_srl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = rop::SRL;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_rtype(rs2, rs1, rd, op)
}
fn parse_sub(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = rop::SUB;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_rtype(rs2, rs1, rd, op)
}
fn parse_sra(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = rop::SRA;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_rtype(rs2, rs1, rd, op)
}
fn parse_lbu(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = optional_signed_12(parser)?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_load(imm, rs1, rd, is_unsigned, width)
}
fn parse_lhu(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = optional_signed_12(parser)?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_load(imm, rs1, rd, is_unsigned, width)
}
fn parse_lwu(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = optional_signed_12(parser)?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_load(imm, rs1, rd, is_unsigned, width)
}
fn parse_ldu(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = optional_signed_12(parser)?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_load(imm, rs1, rd, is_unsigned, width)
}
fn parse_lb(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = optional_signed_12(parser)?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_load(imm, rs1, rd, is_unsigned, width)
}
fn parse_lh(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = optional_signed_12(parser)?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_load(imm, rs1, rd, is_unsigned, width)
}
fn parse_lw(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = optional_signed_12(parser)?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_load(imm, rs1, rd, is_unsigned, width)
}
fn parse_ld(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = optional_signed_12(parser)?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_load(imm, rs1, rd, is_unsigned, width)
}
fn parse_sb(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let width = 1;
    // parse arguments
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_store(imm, rs2, rs1, width)
}
fn parse_sh(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let width = 2;
    // parse arguments
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_store(imm, rs2, rs1, width)
}
fn parse_sw(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let width = 4;
    // parse arguments
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_store(imm, rs2, rs1, width)
}
fn parse_sd(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let width = 8;
    // parse arguments
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_store(imm, rs2, rs1, width)
}
fn parse_addiw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_addiw(imm, rs1, rd)
}
fn parse_addw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = ropw::ADDW;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_rtypew(rs2, rs1, rd, op, ctx)
}
fn parse_subw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = ropw::SUBW;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_rtypew(rs2, rs1, rd, op, ctx)
}
fn parse_sllw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = ropw::SLLW;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_rtypew(rs2, rs1, rd, op, ctx)
}
fn parse_srlw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = ropw::SRLW;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_rtypew(rs2, rs1, rd, op, ctx)
}
fn parse_sraw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = ropw::SRAW;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_rtypew(rs2, rs1, rd, op, ctx)
}
fn parse_slliw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = sopw::SLLIW;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<5>()?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_shiftiwop(shamt, rs1, rd, op, ctx)
}
fn parse_srliw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = sopw::SRLIW;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<5>()?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_shiftiwop(shamt, rs1, rd, op, ctx)
}
fn parse_sraiw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = sopw::SRAIW;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<5>()?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_shiftiwop(shamt, rs1, rd, op, ctx)
}
fn parse_fence_tso() -> crate::Result<u32> {
    encode_fence_tso()
}
fn parse_ecall() -> crate::Result<u32> {
    encode_ecall()
}
fn parse_mret() -> crate::Result<u32> {
    encode_mret()
}
fn parse_sret() -> crate::Result<u32> {
    encode_sret()
}
fn parse_ebreak() -> crate::Result<u32> {
    encode_ebreak()
}
fn parse_wfi() -> crate::Result<u32> {
    encode_wfi()
}
fn parse_sfence_vma(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_sfence_vma(rs1, rs2)
}
fn parse_amoswap_b_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_b_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_b_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_b_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_b_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_b_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_b_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_b_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_b_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_b_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_h_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_h_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_h_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_h_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_h_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_h_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_h_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_h_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_h_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_h_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_w_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_w_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_w_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_w_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_w_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_w_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_w_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_w_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_w_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_w_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_d_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_d_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_d_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_d_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_d_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_d_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_d_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_d_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_d_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_d_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_q_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_q_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_q_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_q_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_q_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_q_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_q_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_q_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_q_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_q_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_b_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_b_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_b_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_b_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_b_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_b_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_b_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_b_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_b_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_b_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_h_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_h_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_h_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_h_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_h_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_h_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_h_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_h_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_h_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_h_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_w_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_w_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_w_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_w_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_w_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_w_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_w_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_w_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_w_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_w_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_d_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_d_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_d_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_d_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_d_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_d_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_d_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_d_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_d_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_d_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_q_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_q_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_q_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_q_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_q_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_q_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_q_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_q_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_q_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_q_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_b_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_b_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_b_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_b_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_b_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_b_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_b_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_b_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_b_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_b_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_h_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_h_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_h_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_h_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_h_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_h_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_h_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_h_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_h_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_h_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_w_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_w_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_w_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_w_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_w_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_w_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_w_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_w_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_w_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_w_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_d_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_d_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_d_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_d_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_d_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_d_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_d_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_d_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_d_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_d_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_q_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_q_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_q_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_q_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_q_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_q_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_q_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_q_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_q_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_q_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_b(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_b(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_b(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_b(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_b(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_b(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_b(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_b(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_b(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_b(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_h(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_h(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_h(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_h(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_h(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_h(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_h(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_h(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_h(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_h(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_d(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_d(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_d(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_d(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_d(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_d(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_d(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_d(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_d(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_d(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoswap_q(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOSWAP;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoadd_q(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOADD;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoxor_q(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOXOR;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoand_q(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOAND;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amoor_q(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOOR;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomin_q(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMIN;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomax_q(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAX;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amominu_q(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMINU;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amomaxu_q(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOMAXU;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_amocas_q(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let op = amoop::AMOCAS;
    let rl = true;
    let width = 16;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_amo(op, aq, rl, rs2, rs1, width, rd)
}
fn parse_lr_b_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_lr_h_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_lr_w_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_lr_d_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_lr_b_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_lr_h_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_lr_w_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_lr_d_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_lr_b_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_lr_h_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_lr_w_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_lr_d_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_lr_b(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_lr_h(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_lr_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_lr_d(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_loadres(aq, rl, rs1, width, rd)
}
fn parse_sc_b_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_sc_h_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_sc_w_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_sc_d_aqrl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_sc_b_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_sc_h_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_sc_w_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_sc_d_aq(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_sc_b_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_sc_h_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_sc_w_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_sc_d_rl(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_sc_b(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 1;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_sc_h(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 2;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_sc_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 4;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_sc_d(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let aq = true;
    let rl = true;
    let width = 8;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_storecon(aq, rl, rs2, rs1, width, rd)
}
fn parse_mul(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mul_op = mul_op {
        result_part: Low,
        signed_rs1: Signed,
        signed_rs2: Signed,
    };
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_mul(rs2, rs1, rd, mul_op)
}
fn parse_mulh(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mul_op = mul_op {
        result_part: High,
        signed_rs1: Signed,
        signed_rs2: Signed,
    };
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_mul(rs2, rs1, rd, mul_op)
}
fn parse_mulhsu(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mul_op = mul_op {
        result_part: High,
        signed_rs1: Signed,
        signed_rs2: Unsigned,
    };
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_mul(rs2, rs1, rd, mul_op)
}
fn parse_mulhu(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mul_op = mul_op {
        result_part: High,
        signed_rs1: Unsigned,
        signed_rs2: Unsigned,
    };
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_mul(rs2, rs1, rd, mul_op)
}
fn parse_divu(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_div(rs2, rs1, rd, is_unsigned)
}
fn parse_div(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_div(rs2, rs1, rd, is_unsigned)
}
fn parse_remu(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_rem(rs2, rs1, rd, is_unsigned)
}
fn parse_rem(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_rem(rs2, rs1, rd, is_unsigned)
}
fn parse_mulw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_mulw(rs2, rs1, rd)
}
fn parse_divuw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_divw(rs2, rs1, rd, is_unsigned)
}
fn parse_divw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_divw(rs2, rs1, rd, is_unsigned)
}
fn parse_remuw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_remw(rs2, rs1, rd, is_unsigned)
}
fn parse_remw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let is_unsigned = true;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_remw(rs2, rs1, rd, is_unsigned)
}
fn parse_slli_uw(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<6>()?;
    encode_slliuw(shamt, rs1, rd)
}
fn parse_add_uw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let shamt = BitVector::<2>::new(0b00);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zba_rtypeuw(rs2, rs1, rd, shamt, ctx)
}
fn parse_sh1add_uw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let shamt = BitVector::<2>::new(0b01);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zba_rtypeuw(rs2, rs1, rd, shamt, ctx)
}
fn parse_sh2add_uw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let shamt = BitVector::<2>::new(0b10);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zba_rtypeuw(rs2, rs1, rd, shamt, ctx)
}
fn parse_sh3add_uw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let shamt = BitVector::<2>::new(0b11);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zba_rtypeuw(rs2, rs1, rd, shamt, ctx)
}
fn parse_sh1add(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let shamt = BitVector::<2>::new(0b01);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zba_rtype(rs2, rs1, rd, shamt)
}
fn parse_sh2add(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let shamt = BitVector::<2>::new(0b10);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zba_rtype(rs2, rs1, rd, shamt)
}
fn parse_sh3add(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let shamt = BitVector::<2>::new(0b11);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zba_rtype(rs2, rs1, rd, shamt)
}
fn parse_roriw(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<5>()?;
    encode_roriw(shamt, rs1, rd)
}
fn parse_rori(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<6>()?;
    encode_rori(shamt, rs1, rd)
}
fn parse_rolw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = bropw_zbb::ROLW;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbb_rtypew(rs2, rs1, rd, op, ctx)
}
fn parse_rorw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = bropw_zbb::RORW;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbb_rtypew(rs2, rs1, rd, op, ctx)
}
fn parse_andn(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = brop_zbb::ANDN;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbb_rtype(rs2, rs1, rd, op, ctx)
}
fn parse_orn(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = brop_zbb::ORN;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbb_rtype(rs2, rs1, rd, op, ctx)
}
fn parse_xnor(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = brop_zbb::XNOR;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbb_rtype(rs2, rs1, rd, op, ctx)
}
fn parse_max(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = brop_zbb::MAX;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbb_rtype(rs2, rs1, rd, op, ctx)
}
fn parse_maxu(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = brop_zbb::MAXU;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbb_rtype(rs2, rs1, rd, op, ctx)
}
fn parse_min(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = brop_zbb::MIN;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbb_rtype(rs2, rs1, rd, op, ctx)
}
fn parse_minu(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = brop_zbb::MINU;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbb_rtype(rs2, rs1, rd, op, ctx)
}
fn parse_rol(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = brop_zbb::ROL;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbb_rtype(rs2, rs1, rd, op, ctx)
}
fn parse_ror(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = brop_zbb::ROR;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbb_rtype(rs2, rs1, rd, op, ctx)
}
fn parse_sext_b(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = extop_zbb::SEXTB;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zbb_extop(rs1, rd, op, ctx)
}
fn parse_sext_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = extop_zbb::SEXTH;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zbb_extop(rs1, rd, op, ctx)
}
fn parse_zext_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = extop_zbb::ZEXTH;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zbb_extop(rs1, rd, op, ctx)
}
fn parse_rev8(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_rev8(rs1, rd, ctx)
}
fn parse_orc_b(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_orcb(rs1, rd)
}
fn parse_cpop(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_cpop(rs1, rd)
}
fn parse_cpopw(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_cpopw(rs1, rd)
}
fn parse_clz(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_clz(rs1, rd)
}
fn parse_clzw(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_clzw(rs1, rd)
}
fn parse_ctz(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_ctz(rs1, rd)
}
fn parse_ctzw(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_ctzw(rs1, rd)
}
fn parse_clmul(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_clmul(rs2, rs1, rd)
}
fn parse_clmulh(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_clmulh(rs2, rs1, rd)
}
fn parse_clmulr(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_clmulr(rs2, rs1, rd)
}
fn parse_bclri(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = biop_zbs::BCLRI;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<6>()?;
    encode_zbs_iop(shamt, rs1, rd, op, ctx)
}
fn parse_bexti(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = biop_zbs::BEXTI;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<6>()?;
    encode_zbs_iop(shamt, rs1, rd, op, ctx)
}
fn parse_binvi(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = biop_zbs::BINVI;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<6>()?;
    encode_zbs_iop(shamt, rs1, rd, op, ctx)
}
fn parse_bseti(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = biop_zbs::BSETI;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<6>()?;
    encode_zbs_iop(shamt, rs1, rd, op, ctx)
}
fn parse_bclr(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = brop_zbs::BCLR;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbs_rtype(rs2, rs1, rd, op, ctx)
}
fn parse_bext(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = brop_zbs::BEXT;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbs_rtype(rs2, rs1, rd, op, ctx)
}
fn parse_binv(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = brop_zbs::BINV;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbs_rtype(rs2, rs1, rd, op, ctx)
}
fn parse_bset(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = brop_zbs::BSET;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbs_rtype(rs2, rs1, rd, op, ctx)
}
fn parse_c_addi4spn(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rdc = creg_name(parser)?;
    parser.expect_comma()?;
    sp_reg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<10>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<2>::from_subword(tmp.val >> 0);
    let expected = BitVector::<2>::new(0b00);
    if actual != expected {
        return err!("bits [1:0] have to be 0b00, got 0b{:02b}", actual.val);
    }
    let nzimm = BitVector::<8>::from_subword(tmp.val >> 2);
    if !(nzimm != BitVector::<8> { val: 0b00000000 }) {
        return err!("argument `nzimm` must not be equal 0b00000000");
    }

    encode_c_addi4spn(rdc, nzimm)
}
fn parse_c_lw(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rdc = creg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<7>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<2>::from_subword(tmp.val >> 0);
    let expected = BitVector::<2>::new(0b00);
    if actual != expected {
        return err!("bits [1:0] have to be 0b00, got 0b{:02b}", actual.val);
    }
    let uimm = BitVector::<5>::from_subword(tmp.val >> 2);
    parser.expect("(")?;
    let rsc = creg_name(parser)?;
    parser.expect(")")?;
    encode_c_lw(uimm, rsc, rdc)
}
fn parse_c_ld(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // parse arguments
    let rdc = creg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<8>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<3>::from_subword(tmp.val >> 0);
    let expected = BitVector::<3>::new(0b000);
    if actual != expected {
        return err!("bits [2:0] have to be 0b000, got 0b{:03b}", actual.val);
    }
    let uimm = BitVector::<5>::from_subword(tmp.val >> 3);
    parser.expect("(")?;
    let rsc = creg_name(parser)?;
    parser.expect(")")?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_c_ld(uimm, rsc, rdc)
}
fn parse_c_sw(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsc2 = creg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<7>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<2>::from_subword(tmp.val >> 0);
    let expected = BitVector::<2>::new(0b00);
    if actual != expected {
        return err!("bits [1:0] have to be 0b00, got 0b{:02b}", actual.val);
    }
    let uimm = BitVector::<5>::from_subword(tmp.val >> 2);
    parser.expect("(")?;
    let rsc1 = creg_name(parser)?;
    parser.expect(")")?;
    encode_c_sw(uimm, rsc1, rsc2)
}
fn parse_c_sd(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // parse arguments
    let rsc2 = creg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<8>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<3>::from_subword(tmp.val >> 0);
    let expected = BitVector::<3>::new(0b000);
    if actual != expected {
        return err!("bits [2:0] have to be 0b000, got 0b{:03b}", actual.val);
    }
    let uimm = BitVector::<5>::from_subword(tmp.val >> 3);
    parser.expect("(")?;
    let rsc1 = creg_name(parser)?;
    parser.expect(")")?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_c_sd(uimm, rsc1, rsc2)
}
fn parse_c_addi(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsd = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<6>()?;
    if !(rsd != zreg) {
        return err!("argument `rsd` must not be equal x0");
    }

    encode_c_addi(imm, rsd)
}
fn parse_c_jal(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // parse arguments
    let tmp = parser.expect_signed_immediate::<12>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<1>::from_subword(tmp.val >> 0);
    let expected = BitVector::<1>::new(0b0);
    if actual != expected {
        return err!("bits [0:0] have to be 0b0, got 0b{:01b}", actual.val);
    }
    let imm = BitVector::<11>::from_subword(tmp.val >> 1);
    if !(ctx.is_32bit()) {
        return err!("available only in 32-bit mode");
    }

    encode_c_jal(imm)
}
fn parse_c_addiw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // parse arguments
    let rsd = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<6>()?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_c_addiw(imm, rsd)
}
fn parse_c_li(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<6>()?;
    encode_c_li(imm, rd)
}
fn parse_c_addi16sp(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    sp_reg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_signed_immediate::<10>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<4>::from_subword(tmp.val >> 0);
    let expected = BitVector::<4>::new(0b0000);
    if actual != expected {
        return err!("bits [3:0] have to be 0b0000, got 0b{:04b}", actual.val);
    }
    let imm = BitVector::<6>::from_subword(tmp.val >> 4);
    if !(imm != BitVector::<6> { val: 0b000000 }) {
        return err!("argument `imm` must not be equal 0b000000");
    }

    encode_c_addi16sp(imm)
}
fn parse_c_lui(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<6>()?;
    if !(rd != sp) {
        return err!("argument `rd` must not be equal sp");
    }
    if !(imm != BitVector::<6> { val: 0b000000 }) {
        return err!("argument `imm` must not be equal 0b000000");
    }

    encode_c_lui(imm, rd)
}
fn parse_c_srli(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsd = creg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<6>()?;
    encode_c_srli(shamt, rsd)
}
fn parse_c_srai(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsd = creg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<6>()?;
    encode_c_srai(shamt, rsd)
}
fn parse_c_andi(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsd = creg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<6>()?;
    encode_c_andi(imm, rsd)
}
fn parse_c_sub(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsd = creg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = creg_name(parser)?;
    encode_c_sub(rsd, rs2)
}
fn parse_c_xor(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsd = creg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = creg_name(parser)?;
    encode_c_xor(rsd, rs2)
}
fn parse_c_or(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsd = creg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = creg_name(parser)?;
    encode_c_or(rsd, rs2)
}
fn parse_c_and(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsd = creg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = creg_name(parser)?;
    encode_c_and(rsd, rs2)
}
fn parse_c_subw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // parse arguments
    let rsd = creg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = creg_name(parser)?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_c_subw(rsd, rs2)
}
fn parse_c_addw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // parse arguments
    let rsd = creg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = creg_name(parser)?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_c_addw(rsd, rs2)
}
fn parse_c_j(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let tmp = parser.expect_signed_immediate::<12>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<1>::from_subword(tmp.val >> 0);
    let expected = BitVector::<1>::new(0b0);
    if actual != expected {
        return err!("bits [0:0] have to be 0b0, got 0b{:01b}", actual.val);
    }
    let imm = BitVector::<11>::from_subword(tmp.val >> 1);
    encode_c_j(imm)
}
fn parse_c_beqz(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rs = creg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_signed_immediate::<9>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<1>::from_subword(tmp.val >> 0);
    let expected = BitVector::<1>::new(0b0);
    if actual != expected {
        return err!("bits [0:0] have to be 0b0, got 0b{:01b}", actual.val);
    }
    let imm = BitVector::<8>::from_subword(tmp.val >> 1);
    encode_c_beqz(imm, rs)
}
fn parse_c_bnez(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rs = creg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_signed_immediate::<9>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<1>::from_subword(tmp.val >> 0);
    let expected = BitVector::<1>::new(0b0);
    if actual != expected {
        return err!("bits [0:0] have to be 0b0, got 0b{:01b}", actual.val);
    }
    let imm = BitVector::<8>::from_subword(tmp.val >> 1);
    encode_c_bnez(imm, rs)
}
fn parse_c_slli(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsd = reg_name(parser)?;
    parser.expect_comma()?;
    let shamt = parser.expect_unsigned_immediate::<6>()?;
    encode_c_slli(shamt, rsd)
}
fn parse_c_lwsp(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<8>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<2>::from_subword(tmp.val >> 0);
    let expected = BitVector::<2>::new(0b00);
    if actual != expected {
        return err!("bits [1:0] have to be 0b00, got 0b{:02b}", actual.val);
    }
    let uimm = BitVector::<6>::from_subword(tmp.val >> 2);
    parser.expect("(")?;
    sp_reg_name(parser)?;
    parser.expect(")")?;
    if !(rd != zreg) {
        return err!("argument `rd` must not be equal x0");
    }

    encode_c_lwsp(uimm, rd)
}
fn parse_c_ldsp(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<9>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<3>::from_subword(tmp.val >> 0);
    let expected = BitVector::<3>::new(0b000);
    if actual != expected {
        return err!("bits [2:0] have to be 0b000, got 0b{:03b}", actual.val);
    }
    let uimm = BitVector::<6>::from_subword(tmp.val >> 3);
    parser.expect("(")?;
    sp_reg_name(parser)?;
    parser.expect(")")?;
    if !(rd != zreg) {
        return err!("argument `rd` must not be equal x0");
    }
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_c_ldsp(uimm, rd)
}
fn parse_c_swsp(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<8>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<2>::from_subword(tmp.val >> 0);
    let expected = BitVector::<2>::new(0b00);
    if actual != expected {
        return err!("bits [1:0] have to be 0b00, got 0b{:02b}", actual.val);
    }
    let uimm = BitVector::<6>::from_subword(tmp.val >> 2);
    parser.expect("(")?;
    sp_reg_name(parser)?;
    parser.expect(")")?;
    encode_c_swsp(uimm, rs2)
}
fn parse_c_sdsp(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // parse arguments
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<9>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<3>::from_subword(tmp.val >> 0);
    let expected = BitVector::<3>::new(0b000);
    if actual != expected {
        return err!("bits [2:0] have to be 0b000, got 0b{:03b}", actual.val);
    }
    let uimm = BitVector::<6>::from_subword(tmp.val >> 3);
    parser.expect("(")?;
    sp_reg_name(parser)?;
    parser.expect(")")?;
    if !(ctx.is_64bit()) {
        return err!("available only in 64-bit mode");
    }

    encode_c_sdsp(uimm, rs2)
}
fn parse_c_jr(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rs1 = reg_name(parser)?;
    if !(rs1 != zreg) {
        return err!("argument `rs1` must not be equal x0");
    }

    encode_c_jr(rs1)
}
fn parse_c_jalr(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rs1 = reg_name(parser)?;
    if !(rs1 != zreg) {
        return err!("argument `rs1` must not be equal x0");
    }

    encode_c_jalr(rs1)
}
fn parse_c_mv(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    if !(rs2 != zreg) {
        return err!("argument `rs2` must not be equal x0");
    }

    encode_c_mv(rd, rs2)
}
fn parse_c_ebreak() -> crate::Result<u32> {
    encode_c_ebreak()
}
fn parse_c_add(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    if !(rs2 != zreg) {
        return err!("argument `rs2` must not be equal x0");
    }

    encode_c_add(rsd, rs2)
}
fn parse_c_lbu(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rdc = creg_name(parser)?;
    parser.expect_comma()?;
    let uimm = parser.expect_unsigned_immediate::<2>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rsc1 = creg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_c_lbu(uimm, rdc, rsc1)
}
fn parse_c_lhu(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rdc = creg_name(parser)?;
    parser.expect_comma()?;
    let uimm = parser.expect_unsigned_immediate::<2>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rsc1 = creg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_c_lhu(uimm, rdc, rsc1)
}
fn parse_c_lh(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rdc = creg_name(parser)?;
    parser.expect_comma()?;
    let uimm = parser.expect_unsigned_immediate::<2>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rsc1 = creg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_c_lh(uimm, rdc, rsc1)
}
fn parse_c_sb(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsc2 = creg_name(parser)?;
    parser.expect_comma()?;
    let uimm = parser.expect_unsigned_immediate::<2>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rsc1 = creg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_c_sb(uimm, rsc1, rsc2)
}
fn parse_c_sh(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsc2 = creg_name(parser)?;
    parser.expect_comma()?;
    let uimm = parser.expect_unsigned_immediate::<2>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rsc1 = creg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_c_sh(uimm, rsc1, rsc2)
}
fn parse_c_zext_b(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsdc = creg_name(parser)?;
    encode_c_zext_b(rsdc)
}
fn parse_c_sext_b(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsdc = creg_name(parser)?;
    encode_c_sext_b(rsdc)
}
fn parse_c_zext_h(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsdc = creg_name(parser)?;
    encode_c_zext_h(rsdc)
}
fn parse_c_sext_h(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsdc = creg_name(parser)?;
    encode_c_sext_h(rsdc)
}
fn parse_c_zext_w(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsdc = creg_name(parser)?;
    encode_c_zext_w(rsdc)
}
fn parse_c_not(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsdc = creg_name(parser)?;
    encode_c_not(rsdc)
}
fn parse_c_mul(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsdc = creg_name(parser)?;
    parser.expect_comma()?;
    let rsc2 = creg_name(parser)?;
    encode_c_mul(rsdc, rsc2)
}
fn parse_flb(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let width = 1;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_load_fp(imm, rs1, rd, width)
}
fn parse_flh(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let width = 2;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_load_fp(imm, rs1, rd, width)
}
fn parse_flw(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let width = 4;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_load_fp(imm, rs1, rd, width)
}
fn parse_fld(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let width = 8;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_load_fp(imm, rs1, rd, width)
}
fn parse_fsb(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let width = 1;
    // parse arguments
    let rs2 = freg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_store_fp(imm, rs2, rs1, width)
}
fn parse_fsh(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let width = 2;
    // parse arguments
    let rs2 = freg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_store_fp(imm, rs2, rs1, width)
}
fn parse_fsw(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let width = 4;
    // parse arguments
    let rs2 = freg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_store_fp(imm, rs2, rs1, width)
}
fn parse_fsd(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let width = 8;
    // parse arguments
    let rs2 = freg_name(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_signed_immediate::<12>()?;
    parser.skip_ws();
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_store_fp(imm, rs2, rs1, width)
}
fn parse_fmadd_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_madd_op_S::FMADD_S;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs3 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_madd_type_s(rs3, rs2, rs1, rm, rd, op, ctx)
}
fn parse_fmsub_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_madd_op_S::FMSUB_S;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs3 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_madd_type_s(rs3, rs2, rs1, rm, rd, op, ctx)
}
fn parse_fnmsub_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_madd_op_S::FNMSUB_S;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs3 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_madd_type_s(rs3, rs2, rs1, rm, rd, op, ctx)
}
fn parse_fnmadd_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_madd_op_S::FNMADD_S;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs3 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_madd_type_s(rs3, rs2, rs1, rm, rd, op, ctx)
}
fn parse_fadd_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_rm_op_S::FADD_S;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_bin_rm_type_s(rs2, rs1, rm, rd, op, ctx)
}
fn parse_fsub_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_rm_op_S::FSUB_S;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_bin_rm_type_s(rs2, rs1, rm, rd, op, ctx)
}
fn parse_fmul_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_rm_op_S::FMUL_S;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_bin_rm_type_s(rs2, rs1, rm, rd, op, ctx)
}
fn parse_fdiv_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_rm_op_S::FDIV_S;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_bin_rm_type_s(rs2, rs1, rm, rd, op, ctx)
}
fn parse_fsqrt_s(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_ff_type_s(rs1, rm, rd, f_un_rm_ff_op_S::FSQRT_S)
}
fn parse_fcvt_w_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_fx_op_S::FCVT_W_S;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_fx_type_s(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_wu_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_fx_op_S::FCVT_WU_S;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_fx_type_s(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_l_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_fx_op_S::FCVT_L_S;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_fx_type_s(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_lu_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_fx_op_S::FCVT_LU_S;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_fx_type_s(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_s_w(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_xf_op_S::FCVT_S_W;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_xf_type_s(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_s_wu(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_xf_op_S::FCVT_S_WU;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_xf_type_s(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_s_l(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_xf_op_S::FCVT_S_L;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_xf_type_s(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_s_lu(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_xf_op_S::FCVT_S_LU;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_xf_type_s(rs1, rm, rd, op, ctx)
}
fn parse_fsgnj_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_op_f_S::FSGNJ_S;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_type_f_s(rs2, rs1, rd, op, ctx)
}
fn parse_fsgnjn_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_op_f_S::FSGNJN_S;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_type_f_s(rs2, rs1, rd, op, ctx)
}
fn parse_fsgnjx_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_op_f_S::FSGNJX_S;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_type_f_s(rs2, rs1, rd, op, ctx)
}
fn parse_fmin_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_op_f_S::FMIN_S;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_type_f_s(rs2, rs1, rd, op, ctx)
}
fn parse_fmax_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_op_f_S::FMAX_S;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_type_f_s(rs2, rs1, rd, op, ctx)
}
fn parse_feq_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_op_x_S::FEQ_S;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_type_x_s(rs2, rs1, rd, op, ctx)
}
fn parse_flt_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_op_x_S::FLT_S;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_type_x_s(rs2, rs1, rd, op, ctx)
}
fn parse_fle_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_op_x_S::FLE_S;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_type_x_s(rs2, rs1, rd, op, ctx)
}
fn parse_fclass_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_op_x_S::FCLASS_S;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    encode_f_un_type_x_s(rs1, rd, op, ctx)
}
fn parse_fmv_x_w(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_op_x_S::FMV_X_W;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    encode_f_un_type_x_s(rs1, rd, op, ctx)
}
fn parse_fmv_w_x(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_op_f_S::FMV_W_X;
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_f_un_type_f_s(rs1, rd, op)
}
fn parse_c_flwsp(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<8>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<2>::from_subword(tmp.val >> 0);
    let expected = BitVector::<2>::new(0b00);
    if actual != expected {
        return err!("bits [1:0] have to be 0b00, got 0b{:02b}", actual.val);
    }
    let imm = BitVector::<6>::from_subword(tmp.val >> 2);
    parser.expect("(")?;
    sp_reg_name(parser)?;
    parser.expect(")")?;
    if !(ctx.is_32bit()) {
        return err!("available only in 32-bit mode");
    }

    encode_c_flwsp(imm, rd)
}
fn parse_c_fswsp(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // parse arguments
    let rs2 = freg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<8>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<2>::from_subword(tmp.val >> 0);
    let expected = BitVector::<2>::new(0b00);
    if actual != expected {
        return err!("bits [1:0] have to be 0b00, got 0b{:02b}", actual.val);
    }
    let uimm = BitVector::<6>::from_subword(tmp.val >> 2);
    parser.expect("(")?;
    sp_reg_name(parser)?;
    parser.expect(")")?;
    if !(ctx.is_32bit()) {
        return err!("available only in 32-bit mode");
    }

    encode_c_fswsp(uimm, rs2)
}
fn parse_c_flw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // parse arguments
    let rdc = cfreg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<7>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<2>::from_subword(tmp.val >> 0);
    let expected = BitVector::<2>::new(0b00);
    if actual != expected {
        return err!("bits [1:0] have to be 0b00, got 0b{:02b}", actual.val);
    }
    let uimm = BitVector::<5>::from_subword(tmp.val >> 2);
    parser.expect("(")?;
    let rsc = creg_name(parser)?;
    parser.expect(")")?;
    if !(ctx.is_32bit()) {
        return err!("available only in 32-bit mode");
    }

    encode_c_flw(uimm, rsc, rdc)
}
fn parse_c_fsw(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // parse arguments
    let rsc2 = cfreg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<7>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<2>::from_subword(tmp.val >> 0);
    let expected = BitVector::<2>::new(0b00);
    if actual != expected {
        return err!("bits [1:0] have to be 0b00, got 0b{:02b}", actual.val);
    }
    let uimm = BitVector::<5>::from_subword(tmp.val >> 2);
    parser.expect("(")?;
    let rsc1 = creg_name(parser)?;
    parser.expect(")")?;
    if !(ctx.is_32bit()) {
        return err!("available only in 32-bit mode");
    }

    encode_c_fsw(uimm, rsc1, rsc2)
}
fn parse_fmadd_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_madd_op_D::FMADD_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs3 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_madd_type_d(rs3, rs2, rs1, rm, rd, op, ctx)
}
fn parse_fmsub_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_madd_op_D::FMSUB_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs3 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_madd_type_d(rs3, rs2, rs1, rm, rd, op, ctx)
}
fn parse_fnmsub_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_madd_op_D::FNMSUB_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs3 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_madd_type_d(rs3, rs2, rs1, rm, rd, op, ctx)
}
fn parse_fnmadd_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_madd_op_D::FNMADD_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs3 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_madd_type_d(rs3, rs2, rs1, rm, rd, op, ctx)
}
fn parse_fadd_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_rm_op_D::FADD_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_bin_rm_type_d(rs2, rs1, rm, rd, op, ctx)
}
fn parse_fsub_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_rm_op_D::FSUB_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_bin_rm_type_d(rs2, rs1, rm, rd, op, ctx)
}
fn parse_fmul_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_rm_op_D::FMUL_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_bin_rm_type_d(rs2, rs1, rm, rd, op, ctx)
}
fn parse_fdiv_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_rm_op_D::FDIV_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_bin_rm_type_d(rs2, rs1, rm, rd, op, ctx)
}
fn parse_fsqrt_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_ff_op_D::FSQRT_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_ff_type_d(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_s_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_ff_op_D::FCVT_S_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_ff_type_d(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_d_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_ff_op_D::FCVT_D_S;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_ff_type_d(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_w_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_fx_op_D::FCVT_W_D;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_fx_type_d(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_wu_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_fx_op_D::FCVT_WU_D;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_fx_type_d(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_l_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_fx_op_D::FCVT_L_D;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_fx_type_d(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_lu_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_fx_op_D::FCVT_LU_D;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_fx_type_d(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_d_w(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_xf_op_D::FCVT_D_W;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_xf_type_d(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_d_wu(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_xf_op_D::FCVT_D_WU;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_xf_type_d(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_d_l(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_xf_op_D::FCVT_D_L;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_xf_type_d(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_d_lu(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_xf_op_D::FCVT_D_LU;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_xf_type_d(rs1, rm, rd, op, ctx)
}
fn parse_fsgnj_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_f_op_D::FSGNJ_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_f_type_d(rs2, rs1, rd, op, ctx)
}
fn parse_fsgnjn_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_f_op_D::FSGNJN_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_f_type_d(rs2, rs1, rd, op, ctx)
}
fn parse_fsgnjx_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_f_op_D::FSGNJX_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_f_type_d(rs2, rs1, rd, op, ctx)
}
fn parse_fmin_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_f_op_D::FMIN_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_f_type_d(rs2, rs1, rd, op, ctx)
}
fn parse_fmax_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_f_op_D::FMAX_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_f_type_d(rs2, rs1, rd, op, ctx)
}
fn parse_feq_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_x_op_D::FEQ_D;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_x_type_d(rs2, rs1, rd, op, ctx)
}
fn parse_flt_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_x_op_D::FLT_D;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_x_type_d(rs2, rs1, rd, op, ctx)
}
fn parse_fle_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_x_op_D::FLE_D;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_x_type_d(rs2, rs1, rd, op, ctx)
}
fn parse_fmv_x_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_x_op_D::FMV_X_D;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    encode_f_un_x_type_d(rs1, rd, op, ctx)
}
fn parse_fclass_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_x_op_D::FCLASS_D;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    encode_f_un_x_type_d(rs1, rd, op, ctx)
}
fn parse_fmv_d_x(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_f_op_D::FMV_D_X;
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_f_un_f_type_d(rs1, rd, op)
}
fn parse_c_fldsp(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<9>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<3>::from_subword(tmp.val >> 0);
    let expected = BitVector::<3>::new(0b000);
    if actual != expected {
        return err!("bits [2:0] have to be 0b000, got 0b{:03b}", actual.val);
    }
    let uimm = BitVector::<6>::from_subword(tmp.val >> 3);
    parser.expect("(")?;
    sp_reg_name(parser)?;
    parser.expect(")")?;
    encode_c_fldsp(uimm, rd)
}
fn parse_c_fsdsp(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rs2 = freg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<9>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<3>::from_subword(tmp.val >> 0);
    let expected = BitVector::<3>::new(0b000);
    if actual != expected {
        return err!("bits [2:0] have to be 0b000, got 0b{:03b}", actual.val);
    }
    let uimm = BitVector::<6>::from_subword(tmp.val >> 3);
    parser.expect("(")?;
    sp_reg_name(parser)?;
    parser.expect(")")?;
    encode_c_fsdsp(uimm, rs2)
}
fn parse_c_fld(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rdc = cfreg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<8>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<3>::from_subword(tmp.val >> 0);
    let expected = BitVector::<3>::new(0b000);
    if actual != expected {
        return err!("bits [2:0] have to be 0b000, got 0b{:03b}", actual.val);
    }
    let uimm = BitVector::<5>::from_subword(tmp.val >> 3);
    parser.expect("(")?;
    let rsc = creg_name(parser)?;
    parser.expect(")")?;
    encode_c_fld(uimm, rsc, rdc)
}
fn parse_c_fsd(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rsc2 = cfreg_name(parser)?;
    parser.expect_comma()?;
    let tmp = parser.expect_unsigned_immediate::<8>()?;
    // deconstruction of `tmp`
    let actual = BitVector::<3>::from_subword(tmp.val >> 0);
    let expected = BitVector::<3>::new(0b000);
    if actual != expected {
        return err!("bits [2:0] have to be 0b000, got 0b{:03b}", actual.val);
    }
    let uimm = BitVector::<5>::from_subword(tmp.val >> 3);
    parser.expect("(")?;
    let rsc1 = creg_name(parser)?;
    parser.expect(")")?;
    encode_c_fsd(uimm, rsc1, rsc2)
}
fn parse_fadd_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_rm_op_H::FADD_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_bin_rm_type_h(rs2, rs1, rm, rd, op, ctx)
}
fn parse_fsub_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_rm_op_H::FSUB_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_bin_rm_type_h(rs2, rs1, rm, rd, op, ctx)
}
fn parse_fmul_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_rm_op_H::FMUL_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_bin_rm_type_h(rs2, rs1, rm, rd, op, ctx)
}
fn parse_fdiv_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_rm_op_H::FDIV_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_bin_rm_type_h(rs2, rs1, rm, rd, op, ctx)
}
fn parse_fmadd_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_madd_op_H::FMADD_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs3 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_madd_type_h(rs3, rs2, rs1, rm, rd, op, ctx)
}
fn parse_fmsub_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_madd_op_H::FMSUB_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs3 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_madd_type_h(rs3, rs2, rs1, rm, rd, op, ctx)
}
fn parse_fnmsub_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_madd_op_H::FNMSUB_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs3 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_madd_type_h(rs3, rs2, rs1, rm, rd, op, ctx)
}
fn parse_fnmadd_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_madd_op_H::FNMADD_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs3 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_madd_type_h(rs3, rs2, rs1, rm, rd, op, ctx)
}
fn parse_fsgnj_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_f_op_H::FSGNJ_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_f_type_h(rs2, rs1, rd, op, ctx)
}
fn parse_fsgnjn_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_f_op_H::FSGNJN_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_f_type_h(rs2, rs1, rd, op, ctx)
}
fn parse_fsgnjx_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_f_op_H::FSGNJX_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_f_type_h(rs2, rs1, rd, op, ctx)
}
fn parse_fmin_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_f_op_H::FMIN_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_f_type_h(rs2, rs1, rd, op, ctx)
}
fn parse_fmax_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_f_op_H::FMAX_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_f_type_h(rs2, rs1, rd, op, ctx)
}
fn parse_feq_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_x_op_H::FEQ_H;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_x_type_h(rs2, rs1, rd, op, ctx)
}
fn parse_flt_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_x_op_H::FLT_H;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_x_type_h(rs2, rs1, rd, op, ctx)
}
fn parse_fle_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_bin_x_op_H::FLE_H;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_or_reg_name(parser)?;
    encode_f_bin_x_type_h(rs2, rs1, rd, op, ctx)
}
fn parse_fsqrt_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_ff_op_H::FSQRT_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_ff_type_h(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_h_s(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_ff_op_H::FCVT_H_S;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_ff_type_h(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_h_d(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_ff_op_H::FCVT_H_D;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_ff_type_h(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_s_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_ff_op_H::FCVT_S_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_ff_type_h(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_d_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_ff_op_H::FCVT_D_H;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_ff_type_h(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_w_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_fx_op_H::FCVT_W_H;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_fx_type_h(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_wu_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_fx_op_H::FCVT_WU_H;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_fx_type_h(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_l_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_fx_op_H::FCVT_L_H;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_fx_type_h(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_lu_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_fx_op_H::FCVT_LU_H;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_fx_type_h(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_h_w(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_xf_op_H::FCVT_H_W;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_xf_type_h(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_h_wu(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_xf_op_H::FCVT_H_WU;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_xf_type_h(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_h_l(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_xf_op_H::FCVT_H_L;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_xf_type_h(rs1, rm, rd, op, ctx)
}
fn parse_fcvt_h_lu(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_rm_xf_op_H::FCVT_H_LU;
    // parse arguments
    let rd = freg_or_reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_f_un_rm_xf_type_h(rs1, rm, rd, op, ctx)
}
fn parse_fmv_h_x(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_f_op_H::FMV_H_X;
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_f_un_f_type_h(rs1, rd, op)
}
fn parse_fmv_x_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_x_op_H::FMV_X_H;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    encode_f_un_x_type_h(rs1, rd, op, ctx)
}
fn parse_fclass_h(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = f_un_x_op_H::FCLASS_H;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    encode_f_un_x_type_h(rs1, rd, op, ctx)
}
fn parse_fli_h(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let constantidx = parser.expect_unsigned_immediate::<5>()?;
    encode_fli_h(constantidx, rd)
}
fn parse_fli_s(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let constantidx = parser.expect_unsigned_immediate::<5>()?;
    encode_fli_s(constantidx, rd)
}
fn parse_fli_d(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let constantidx = parser.expect_unsigned_immediate::<5>()?;
    encode_fli_d(constantidx, rd)
}
fn parse_fminm_h(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_name(parser)?;
    encode_fminm_h(rs2, rs1, rd)
}
fn parse_fmaxm_h(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_name(parser)?;
    encode_fmaxm_h(rs2, rs1, rd)
}
fn parse_fminm_s(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_name(parser)?;
    encode_fminm_s(rs2, rs1, rd)
}
fn parse_fmaxm_s(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_name(parser)?;
    encode_fmaxm_s(rs2, rs1, rd)
}
fn parse_fminm_d(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_name(parser)?;
    encode_fminm_d(rs2, rs1, rd)
}
fn parse_fmaxm_d(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_name(parser)?;
    encode_fmaxm_d(rs2, rs1, rd)
}
fn parse_fround_h(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_fround_h(rs1, rm, rd)
}
fn parse_froundnx_h(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_froundnx_h(rs1, rm, rd)
}
fn parse_fround_s(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_fround_s(rs1, rm, rd)
}
fn parse_froundnx_s(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_froundnx_s(rs1, rm, rd)
}
fn parse_fround_d(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_fround_d(rs1, rm, rd)
}
fn parse_froundnx_d(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_froundnx_d(rs1, rm, rd)
}
fn parse_fmvh_x_d(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    encode_fmvh_x_d(rs1, rd)
}
fn parse_fmvp_d_x(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_fmvp_d_x(rs2, rs1, rd)
}
fn parse_fleq_h(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_name(parser)?;
    encode_fleq_h(rs2, rs1, rd)
}
fn parse_fltq_h(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_name(parser)?;
    encode_fltq_h(rs2, rs1, rd)
}
fn parse_fleq_s(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_name(parser)?;
    encode_fleq_s(rs2, rs1, rd)
}
fn parse_fltq_s(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_name(parser)?;
    encode_fltq_s(rs2, rs1, rd)
}
fn parse_fleq_d(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_name(parser)?;
    encode_fleq_d(rs2, rs1, rd)
}
fn parse_fltq_d(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = freg_name(parser)?;
    encode_fltq_d(rs2, rs1, rd)
}
fn parse_fcvtmod_w_d(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("rtz")?;
    encode_fcvtmod_w_d(rs1, rd)
}
fn parse_vsetvli(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let tmp = vtype_assembly(parser)?;
    let val = (tmp >> 0) & ((1 << 3) - 1);
    let lmul = BitVector::<3>::new(val);
    let val = (tmp >> 3) & ((1 << 3) - 1);
    let sew = BitVector::<3>::new(val);
    let val = (tmp >> 6) & ((1 << 1) - 1);
    let ta = BitVector::<1>::new(val);
    let val = (tmp >> 7) & ((1 << 1) - 1);
    let ma = BitVector::<1>::new(val);
    encode_vsetvli(ma, ta, sew, lmul, rs1, rd)
}
fn parse_vsetvl(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_vsetvl(rs2, rs1, rd)
}
fn parse_vsetivli(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let uimm = parser.expect_unsigned_immediate::<5>()?;
    parser.expect_comma()?;
    let tmp = vtype_assembly(parser)?;
    let val = (tmp >> 0) & ((1 << 3) - 1);
    let lmul = BitVector::<3>::new(val);
    let val = (tmp >> 3) & ((1 << 3) - 1);
    let sew = BitVector::<3>::new(val);
    let val = (tmp >> 6) & ((1 << 1) - 1);
    let ta = BitVector::<1>::new(val);
    let val = (tmp >> 7) & ((1 << 1) - 1);
    let ma = BitVector::<1>::new(val);
    encode_vsetivli(ma, ta, sew, lmul, uimm, rd)
}
fn parse_vadd_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vsub_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vand_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VAND;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vor_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VOR;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vxor_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VXOR;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vrgather_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VRGATHER;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vrgatherei16_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VRGATHEREI16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vsaddu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VSADDU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vsadd_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VSADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vssubu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VSSUBU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vssub_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VSSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vsll_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VSLL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vsmul_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VSMUL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vsrl_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VSRL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vsra_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VSRA;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vssrl_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VSSRL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vssra_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VSSRA;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vminu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VMINU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmin_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VMIN;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmaxu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VMAXU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmax_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvfunct6::VV_VMAX;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vnsrl_wv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = nvsfunct6::NVS_VNSRL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_nvstype(funct6, vm, vs2, vs1, vd)
}
fn parse_vnsra_wv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = nvsfunct6::NVS_VNSRA;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_nvstype(funct6, vm, vs2, vs1, vd)
}
fn parse_vnclipu_wv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = nvfunct6::NV_VNCLIPU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_nvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vnclip_wv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = nvfunct6::NV_VNCLIP;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_nvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmerge_vvm(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("v0")?;
    encode_masktypev(vs2, vs1, vd)
}
fn parse_vmv_v_v(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_movetypev(vs1, vd)
}
fn parse_vadd_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vsub_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vrsub_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VRSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vand_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VAND;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vor_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VOR;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vxor_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VXOR;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vsaddu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VSADDU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vsadd_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VSADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vssubu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VSSUBU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vssub_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VSSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vsll_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VSLL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vsmul_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VSMUL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vsrl_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VSRL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vsra_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VSRA;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vssrl_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VSSRL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vssra_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VSSRA;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vminu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VMINU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmin_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VMIN;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmaxu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VMAXU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmax_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxfunct6::VX_VMAX;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vnsrl_wx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = nxsfunct6::NXS_VNSRL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_nxstype(funct6, vm, vs2, rs1, vd)
}
fn parse_vnsra_wx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = nxsfunct6::NXS_VNSRA;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_nxstype(funct6, vm, vs2, rs1, vd)
}
fn parse_vnclipu_wx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = nxfunct6::NX_VNCLIPU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_nxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vnclip_wx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = nxfunct6::NX_VNCLIP;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_nxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vslideup_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxsgfunct6::VX_VSLIDEUP;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxsg(funct6, vm, vs2, rs1, vd)
}
fn parse_vslidedown_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxsgfunct6::VX_VSLIDEDOWN;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxsg(funct6, vm, vs2, rs1, vd)
}
fn parse_vrgather_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxsgfunct6::VX_VRGATHER;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxsg(funct6, vm, vs2, rs1, vd)
}
fn parse_vmerge_vxm(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("v0")?;
    encode_masktypex(vs2, rs1, vd)
}
fn parse_vmv_v_x(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_movetypex(rs1, vd)
}
fn parse_vadd_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vifunct6::VI_VADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vitype(funct6, vm, vs2, simm, vd)
}
fn parse_vrsub_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vifunct6::VI_VRSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vitype(funct6, vm, vs2, simm, vd)
}
fn parse_vand_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vifunct6::VI_VAND;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vitype(funct6, vm, vs2, simm, vd)
}
fn parse_vor_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vifunct6::VI_VOR;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vitype(funct6, vm, vs2, simm, vd)
}
fn parse_vxor_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vifunct6::VI_VXOR;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vitype(funct6, vm, vs2, simm, vd)
}
fn parse_vsaddu_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vifunct6::VI_VSADDU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vitype(funct6, vm, vs2, simm, vd)
}
fn parse_vsadd_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vifunct6::VI_VSADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vitype(funct6, vm, vs2, simm, vd)
}
fn parse_vsll_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vifunct6::VI_VSLL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vitype(funct6, vm, vs2, simm, vd)
}
fn parse_vsrl_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vifunct6::VI_VSRL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vitype(funct6, vm, vs2, simm, vd)
}
fn parse_vsra_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vifunct6::VI_VSRA;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vitype(funct6, vm, vs2, simm, vd)
}
fn parse_vssrl_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vifunct6::VI_VSSRL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vitype(funct6, vm, vs2, simm, vd)
}
fn parse_vssra_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vifunct6::VI_VSSRA;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vitype(funct6, vm, vs2, simm, vd)
}
fn parse_vnsrl_wi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = nisfunct6::NIS_VNSRL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let uimm = parser.expect_unsigned_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_nistype(funct6, vm, vs2, uimm, vd)
}
fn parse_vnsra_wi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = nisfunct6::NIS_VNSRA;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let uimm = parser.expect_unsigned_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_nistype(funct6, vm, vs2, uimm, vd)
}
fn parse_vnclipu_wi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = nifunct6::NI_VNCLIPU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let uimm = parser.expect_unsigned_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_nitype(funct6, vm, vs2, uimm, vd)
}
fn parse_vnclip_wi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = nifunct6::NI_VNCLIP;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let uimm = parser.expect_unsigned_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_nitype(funct6, vm, vs2, uimm, vd)
}
fn parse_vslideup_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = visgfunct6::VI_VSLIDEUP;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_visg(funct6, vm, vs2, simm, vd)
}
fn parse_vslidedown_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = visgfunct6::VI_VSLIDEDOWN;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_visg(funct6, vm, vs2, simm, vd)
}
fn parse_vrgather_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = visgfunct6::VI_VRGATHER;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_visg(funct6, vm, vs2, simm, vd)
}
fn parse_vmerge_vim(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    parser.expect_comma()?;
    parser.expect("v0")?;
    encode_masktypei(vs2, simm, vd)
}
fn parse_vmv_v_i(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    encode_movetypei(vd, simm)
}
fn parse_vmv1r_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nreg = 1;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vmvrtype(vs2, nreg, vd)
}
fn parse_vmv2r_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nreg = 2;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vmvrtype(vs2, nreg, vd)
}
fn parse_vmv4r_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nreg = 4;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vmvrtype(vs2, nreg, vd)
}
fn parse_vmv8r_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nreg = 8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vmvrtype(vs2, nreg, vd)
}
fn parse_vaaddu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvfunct6::MVV_VAADDU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vaadd_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvfunct6::MVV_VAADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vasubu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvfunct6::MVV_VASUBU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vasub_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvfunct6::MVV_VASUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmul_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvfunct6::MVV_VMUL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmulh_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvfunct6::MVV_VMULH;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmulhu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvfunct6::MVV_VMULHU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmulhsu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvfunct6::MVV_VMULHSU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vdivu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvfunct6::MVV_VDIVU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vdiv_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvfunct6::MVV_VDIV;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vremu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvfunct6::MVV_VREMU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vrem_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvfunct6::MVV_VREM;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmacc_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvmafunct6::MVV_VMACC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvmatype(funct6, vm, vs2, vs1, vd)
}
fn parse_vnmsac_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvmafunct6::MVV_VNMSAC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvmatype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmadd_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvmafunct6::MVV_VMADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvmatype(funct6, vm, vs2, vs1, vd)
}
fn parse_vnmsub_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvvmafunct6::MVV_VNMSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvvmatype(funct6, vm, vs2, vs1, vd)
}
fn parse_vwadd_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvvfunct6::WVV_VADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vwsub_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvvfunct6::WVV_VSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vwaddu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvvfunct6::WVV_VADDU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vwsubu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvvfunct6::WVV_VSUBU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vwmul_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvvfunct6::WVV_VWMUL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vwmulu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvvfunct6::WVV_VWMULU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vwmulsu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvvfunct6::WVV_VWMULSU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vwadd_wv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvfunct6::WV_VADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vwsub_wv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvfunct6::WV_VSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vwaddu_wv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvfunct6::WV_VADDU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vwsubu_wv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvfunct6::WV_VSUBU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vwmaccu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wmvvfunct6::WMVV_VWMACCU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wmvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vwmacc_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wmvvfunct6::WMVV_VWMACC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wmvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vwmaccsu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wmvvfunct6::WMVV_VWMACCSU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wmvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vzext_vf2(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vextfunct6::VEXT2_ZVF2;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vexttype(funct6, vm, vs2, vd)
}
fn parse_vsext_vf2(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vextfunct6::VEXT2_SVF2;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vexttype(funct6, vm, vs2, vd)
}
fn parse_vzext_vf4(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vextfunct6::VEXT4_ZVF4;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vexttype(funct6, vm, vs2, vd)
}
fn parse_vsext_vf4(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vextfunct6::VEXT4_SVF4;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vexttype(funct6, vm, vs2, vd)
}
fn parse_vzext_vf8(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vextfunct6::VEXT8_ZVF8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vexttype(funct6, vm, vs2, vd)
}
fn parse_vsext_vf8(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vextfunct6::VEXT8_SVF8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vexttype(funct6, vm, vs2, vd)
}
fn parse_vmv_x_s(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vmvxs(vs2, rd)
}
fn parse_vcompress_vm(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_mvvcompress(vs2, vs1, vd)
}
fn parse_vaaddu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxfunct6::MVX_VAADDU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vaadd_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxfunct6::MVX_VAADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vasubu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxfunct6::MVX_VASUBU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vasub_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxfunct6::MVX_VASUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vslide1up_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxfunct6::MVX_VSLIDE1UP;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vslide1down_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxfunct6::MVX_VSLIDE1DOWN;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmul_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxfunct6::MVX_VMUL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmulh_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxfunct6::MVX_VMULH;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmulhu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxfunct6::MVX_VMULHU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmulhsu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxfunct6::MVX_VMULHSU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vdivu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxfunct6::MVX_VDIVU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vdiv_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxfunct6::MVX_VDIV;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vremu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxfunct6::MVX_VREMU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vrem_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxfunct6::MVX_VREM;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmacc_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxmafunct6::MVX_VMACC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxmatype(funct6, vm, vs2, rs1, vd)
}
fn parse_vnmsac_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxmafunct6::MVX_VNMSAC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxmatype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmadd_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxmafunct6::MVX_VMADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxmatype(funct6, vm, vs2, rs1, vd)
}
fn parse_vnmsub_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mvxmafunct6::MVX_VNMSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_mvxmatype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwadd_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvxfunct6::WVX_VADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwsub_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvxfunct6::WVX_VSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwaddu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvxfunct6::WVX_VADDU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwsubu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvxfunct6::WVX_VSUBU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwmul_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvxfunct6::WVX_VWMUL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwmulu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvxfunct6::WVX_VWMULU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwmulsu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wvxfunct6::WVX_VWMULSU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwadd_wx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wxfunct6::WX_VADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwsub_wx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wxfunct6::WX_VSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwaddu_wx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wxfunct6::WX_VADDU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwsubu_wx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wxfunct6::WX_VSUBU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwmaccu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wmvxfunct6::WMVX_VWMACCU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wmvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwmacc_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wmvxfunct6::WMVX_VWMACC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wmvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwmaccus_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wmvxfunct6::WMVX_VWMACCUS;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wmvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwmaccsu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = wmvxfunct6::WMVX_VWMACCSU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_wmvxtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmv_s_x(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_vmvsx(rs1, vd)
}
fn parse_vfadd_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvfunct6::FVV_VADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfsub_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvfunct6::FVV_VSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfmin_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvfunct6::FVV_VMIN;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfmax_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvfunct6::FVV_VMAX;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfsgnj_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvfunct6::FVV_VSGNJ;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfsgnjn_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvfunct6::FVV_VSGNJN;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfsgnjx_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvfunct6::FVV_VSGNJX;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfdiv_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvfunct6::FVV_VDIV;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfmul_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvfunct6::FVV_VMUL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfmadd_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvmafunct6::FVV_VMADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvmatype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfnmadd_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvmafunct6::FVV_VNMADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvmatype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfmsub_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvmafunct6::FVV_VMSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvmatype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfnmsub_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvmafunct6::FVV_VNMSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvmatype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfmacc_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvmafunct6::FVV_VMACC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvmatype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfnmacc_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvmafunct6::FVV_VNMACC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvmatype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfmsac_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvmafunct6::FVV_VMSAC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvmatype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfnmsac_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvmafunct6::FVV_VNMSAC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvmatype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfwadd_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvvfunct6::FWVV_VADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfwsub_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvvfunct6::FWVV_VSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfwmul_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvvfunct6::FWVV_VMUL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfwmacc_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvvmafunct6::FWVV_VMACC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvvmatype(funct6, vm, vs1, vs2, vd)
}
fn parse_vfwnmacc_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvvmafunct6::FWVV_VNMACC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvvmatype(funct6, vm, vs1, vs2, vd)
}
fn parse_vfwmsac_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvvmafunct6::FWVV_VMSAC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvvmatype(funct6, vm, vs1, vs2, vd)
}
fn parse_vfwnmsac_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvvmafunct6::FWVV_VNMSAC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvvmatype(funct6, vm, vs1, vs2, vd)
}
fn parse_vfwadd_wv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvfunct6::FWV_VADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfwsub_wv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvfunct6::FWV_VSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfcvt_xu_f_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfunary0 = vfunary0::FV_CVT_XU_F;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfunary0(vm, vs2, vfunary0, vd)
}
fn parse_vfcvt_x_f_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfunary0 = vfunary0::FV_CVT_X_F;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfunary0(vm, vs2, vfunary0, vd)
}
fn parse_vfcvt_f_xu_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfunary0 = vfunary0::FV_CVT_F_XU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfunary0(vm, vs2, vfunary0, vd)
}
fn parse_vfcvt_f_x_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfunary0 = vfunary0::FV_CVT_F_X;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfunary0(vm, vs2, vfunary0, vd)
}
fn parse_vfcvt_rtz_xu_f_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfunary0 = vfunary0::FV_CVT_RTZ_XU_F;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfunary0(vm, vs2, vfunary0, vd)
}
fn parse_vfcvt_rtz_x_f_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfunary0 = vfunary0::FV_CVT_RTZ_X_F;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfunary0(vm, vs2, vfunary0, vd)
}
fn parse_vfwcvt_xu_f_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfwunary0 = vfwunary0::FWV_CVT_XU_F;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfwunary0(vm, vs2, vfwunary0, vd)
}
fn parse_vfwcvt_x_f_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfwunary0 = vfwunary0::FWV_CVT_X_F;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfwunary0(vm, vs2, vfwunary0, vd)
}
fn parse_vfwcvt_f_xu_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfwunary0 = vfwunary0::FWV_CVT_F_XU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfwunary0(vm, vs2, vfwunary0, vd)
}
fn parse_vfwcvt_f_x_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfwunary0 = vfwunary0::FWV_CVT_F_X;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfwunary0(vm, vs2, vfwunary0, vd)
}
fn parse_vfwcvt_f_f_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfwunary0 = vfwunary0::FWV_CVT_F_F;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfwunary0(vm, vs2, vfwunary0, vd)
}
fn parse_vfwcvt_rtz_xu_f_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfwunary0 = vfwunary0::FWV_CVT_RTZ_XU_F;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfwunary0(vm, vs2, vfwunary0, vd)
}
fn parse_vfwcvt_rtz_x_f_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfwunary0 = vfwunary0::FWV_CVT_RTZ_X_F;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfwunary0(vm, vs2, vfwunary0, vd)
}
fn parse_vfncvt_xu_f_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfnunary0 = vfnunary0::FNV_CVT_XU_F;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfnunary0(vm, vs2, vfnunary0, vd)
}
fn parse_vfncvt_x_f_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfnunary0 = vfnunary0::FNV_CVT_X_F;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfnunary0(vm, vs2, vfnunary0, vd)
}
fn parse_vfncvt_f_xu_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfnunary0 = vfnunary0::FNV_CVT_F_XU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfnunary0(vm, vs2, vfnunary0, vd)
}
fn parse_vfncvt_f_x_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfnunary0 = vfnunary0::FNV_CVT_F_X;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfnunary0(vm, vs2, vfnunary0, vd)
}
fn parse_vfncvt_f_f_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfnunary0 = vfnunary0::FNV_CVT_F_F;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfnunary0(vm, vs2, vfnunary0, vd)
}
fn parse_vfncvt_rod_f_f_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfnunary0 = vfnunary0::FNV_CVT_ROD_F_F;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfnunary0(vm, vs2, vfnunary0, vd)
}
fn parse_vfncvt_rtz_xu_f_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfnunary0 = vfnunary0::FNV_CVT_RTZ_XU_F;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfnunary0(vm, vs2, vfnunary0, vd)
}
fn parse_vfncvt_rtz_x_f_w(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfnunary0 = vfnunary0::FNV_CVT_RTZ_X_F;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfnunary0(vm, vs2, vfnunary0, vd)
}
fn parse_vfsqrt_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfunary1 = vfunary1::FVV_VSQRT;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfunary1(vm, vs2, vfunary1, vd)
}
fn parse_vfrsqrt7_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfunary1 = vfunary1::FVV_VRSQRT7;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfunary1(vm, vs2, vfunary1, vd)
}
fn parse_vfrec7_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfunary1 = vfunary1::FVV_VREC7;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfunary1(vm, vs2, vfunary1, vd)
}
fn parse_vfclass_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let vfunary1 = vfunary1::FVV_VCLASS;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfunary1(vm, vs2, vfunary1, vd)
}
fn parse_vfmv_f_s(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vfmvfs(vs2, rd)
}
fn parse_vfadd_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvffunct6::VF_VADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfsub_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvffunct6::VF_VSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfmin_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvffunct6::VF_VMIN;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfmax_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvffunct6::VF_VMAX;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfsgnj_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvffunct6::VF_VSGNJ;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfsgnjn_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvffunct6::VF_VSGNJN;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfsgnjx_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvffunct6::VF_VSGNJX;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfslide1up_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvffunct6::VF_VSLIDE1UP;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfslide1down_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvffunct6::VF_VSLIDE1DOWN;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfdiv_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvffunct6::VF_VDIV;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfrdiv_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvffunct6::VF_VRDIV;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfmul_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvffunct6::VF_VMUL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfrsub_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvffunct6::VF_VRSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfmadd_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvfmafunct6::VF_VMADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvfmatype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfnmadd_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvfmafunct6::VF_VNMADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvfmatype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfmsub_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvfmafunct6::VF_VMSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvfmatype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfnmsub_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvfmafunct6::VF_VNMSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvfmatype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfmacc_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvfmafunct6::VF_VMACC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvfmatype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfnmacc_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvfmafunct6::VF_VNMACC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvfmatype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfmsac_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvfmafunct6::VF_VMSAC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvfmatype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfnmsac_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvfmafunct6::VF_VNMSAC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvfmatype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfwadd_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvffunct6::FWVF_VADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfwsub_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvffunct6::FWVF_VSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfwmul_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvffunct6::FWVF_VMUL;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfwmacc_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvfmafunct6::FWVF_VMACC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvfmatype(funct6, vm, rs1, vs2, vd)
}
fn parse_vfwnmacc_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvfmafunct6::FWVF_VNMACC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvfmatype(funct6, vm, rs1, vs2, vd)
}
fn parse_vfwmsac_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvfmafunct6::FWVF_VMSAC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvfmatype(funct6, vm, rs1, vs2, vd)
}
fn parse_vfwnmsac_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwvfmafunct6::FWVF_VNMSAC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwvfmatype(funct6, vm, rs1, vs2, vd)
}
fn parse_vfwadd_wf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwffunct6::FWF_VADD;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfwsub_wf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fwffunct6::FWF_VSUB;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fwftype(funct6, vm, vs2, rs1, vd)
}
fn parse_vfmerge_vfm(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("v0")?;
    encode_vfmerge(vs2, rs1, vd)
}
fn parse_vfmv_v_f(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    encode_vfmv(rs1, vd)
}
fn parse_vfmv_s_f(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    encode_vfmvsf(rs1, vd)
}
fn parse_vle8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg2e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg3e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg4e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg5e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg6e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg7e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg8e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vle16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg2e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg3e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg4e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg5e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg6e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg7e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg8e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vle32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg2e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg3e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg4e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg5e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg6e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg7e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg8e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vle64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg2e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg3e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg4e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg5e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg6e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg7e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vlseg8e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegtype(nf, vm, rs1, width, vd)
}
fn parse_vle8ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg2e8ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg3e8ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg4e8ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg5e8ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg6e8ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg7e8ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg8e8ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vle16ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg2e16ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg3e16ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg4e16ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg5e16ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg6e16ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg7e16ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg8e16ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vle32ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg2e32ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg3e32ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg4e32ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg5e32ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg6e32ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg7e32ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg8e32ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vle64ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg2e64ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg3e64ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg4e64ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg5e64ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg6e64ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg7e64ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vlseg8e64ff_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vlsegfftype(nf, vm, rs1, width, vd)
}
fn parse_vse8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg2e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg3e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg4e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg5e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg6e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg7e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg8e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vse16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg2e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg3e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg4e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg5e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg6e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg7e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg8e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vse32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg2e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg3e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg4e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg5e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg6e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg7e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg8e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vse64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg2e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg3e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg4e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg5e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg6e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg7e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vsseg8e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    let vm = maybe_vmask(parser)?;
    encode_vssegtype(nf, vm, rs1, width, vs3)
}
fn parse_vlse8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg2e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg3e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg4e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg5e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg6e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg7e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg8e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlse16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg2e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg3e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg4e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg5e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg6e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg7e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg8e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlse32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg2e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg3e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg4e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg5e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg6e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg7e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg8e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlse64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg2e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg3e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg4e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg5e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg6e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg7e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vlsseg8e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlssegtype(nf, vm, rs2, rs1, width, vd)
}
fn parse_vsse8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg2e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg3e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg4e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg5e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg6e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg7e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg8e8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vsse16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg2e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg3e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg4e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg5e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg6e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg7e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg8e16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vsse32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg2e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg3e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg4e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg5e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg6e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg7e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg8e32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vsse64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg2e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg3e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 3;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg4e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg5e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 5;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg6e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 6;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg7e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 7;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vssseg8e64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsssegtype(nf, vm, rs2, rs1, width, vs3)
}
fn parse_vluxei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 1;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 1;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg2ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 2;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg2ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 2;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg3ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 3;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg3ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 3;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg4ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 4;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg4ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 4;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg5ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 5;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg5ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 5;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg6ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 6;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg6ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 6;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg7ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 7;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg7ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 7;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg8ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 8;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg8ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 8;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 1;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 1;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg2ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 2;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg2ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 2;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg3ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 3;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg3ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 3;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg4ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 4;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg4ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 4;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg5ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 5;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg5ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 5;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg6ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 6;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg6ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 6;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg7ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 7;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg7ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 7;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg8ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 8;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg8ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 8;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 1;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 1;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg2ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 2;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg2ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 2;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg3ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 3;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg3ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 3;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg4ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 4;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg4ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 4;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg5ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 5;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg5ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 5;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg6ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 6;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg6ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 6;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg7ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 7;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg7ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 7;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg8ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 8;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg8ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 8;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 1;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 1;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg2ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 2;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg2ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 2;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg3ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 3;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg3ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 3;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg4ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 4;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg4ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 4;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg5ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 5;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg5ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 5;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg6ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 6;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg6ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 6;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg7ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 7;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg7ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 7;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vluxseg8ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 8;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vloxseg8ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 8;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vlxsegtype(nf, vm, vs2, rs1, width, vd, mop)
}
fn parse_vsuxei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 1;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 1;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg2ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 2;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg2ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 2;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg3ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 3;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg3ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 3;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg4ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 4;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg4ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 4;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg5ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 5;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg5ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 5;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg6ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 6;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg6ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 6;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg7ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 7;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg7ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 7;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg8ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 8;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg8ei8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 8;
    let width = vlewidth::VLE8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 1;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 1;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg2ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 2;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg2ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 2;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg3ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 3;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg3ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 3;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg4ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 4;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg4ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 4;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg5ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 5;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg5ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 5;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg6ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 6;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg6ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 6;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg7ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 7;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg7ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 7;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg8ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 8;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg8ei16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 8;
    let width = vlewidth::VLE16;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 1;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 1;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg2ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 2;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg2ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 2;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg3ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 3;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg3ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 3;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg4ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 4;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg4ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 4;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg5ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 5;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg5ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 5;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg6ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 6;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg6ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 6;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg7ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 7;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg7ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 7;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg8ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 8;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg8ei32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 8;
    let width = vlewidth::VLE32;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 1;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 1;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg2ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 2;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg2ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 2;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg3ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 3;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg3ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 3;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg4ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 4;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg4ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 4;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg5ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 5;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg5ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 5;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg6ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 6;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg6ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 6;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg7ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 7;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg7ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 7;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsuxseg8ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_UNORDERED;
    let nf = 8;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vsoxseg8ei64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = indexed_mop::INDEXED_ORDERED;
    let nf = 8;
    let width = vlewidth::VLE64;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vsxsegtype(nf, vm, vs2, rs1, width, vs3, mop)
}
fn parse_vl1re8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vl2re8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vl4re8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vl8re8_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE8;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vl1re16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vl2re16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vl4re16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vl8re16_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE16;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vl1re32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vl2re32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vl4re32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vl8re32_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE32;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vl1re64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vl2re64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vl4re64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vl8re64_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    let width = vlewidth::VLE64;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vlretype(nf, rs1, width, vd)
}
fn parse_vs1r_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 1;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vsretype(nf, rs1, vs3)
}
fn parse_vs2r_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 2;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vsretype(nf, rs1, vs3)
}
fn parse_vs4r_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 4;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vsretype(nf, rs1, vs3)
}
fn parse_vs8r_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let nf = 8;
    // parse arguments
    let vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vsretype(nf, rs1, vs3)
}
fn parse_vlm_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = vmlsop::VLM;
    // parse arguments
    let vd_or_vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vmtype(rs1, vd_or_vs3, op)
}
fn parse_vsm_v(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = vmlsop::VSM;
    // parse arguments
    let vd_or_vs3 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("(")?;
    let rs1 = reg_name(parser)?;
    parser.expect(")")?;
    encode_vmtype(rs1, vd_or_vs3, op)
}
fn parse_vmand_mm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mmfunct6::MM_VMAND;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_mmtype(funct6, vs2, vs1, vd)
}
fn parse_vmnand_mm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mmfunct6::MM_VMNAND;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_mmtype(funct6, vs2, vs1, vd)
}
fn parse_vmandn_mm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mmfunct6::MM_VMANDN;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_mmtype(funct6, vs2, vs1, vd)
}
fn parse_vmxor_mm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mmfunct6::MM_VMXOR;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_mmtype(funct6, vs2, vs1, vd)
}
fn parse_vmor_mm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mmfunct6::MM_VMOR;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_mmtype(funct6, vs2, vs1, vd)
}
fn parse_vmnor_mm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mmfunct6::MM_VMNOR;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_mmtype(funct6, vs2, vs1, vd)
}
fn parse_vmorn_mm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mmfunct6::MM_VMORN;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_mmtype(funct6, vs2, vs1, vd)
}
fn parse_vmxnor_mm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = mmfunct6::MM_VMXNOR;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_mmtype(funct6, vs2, vs1, vd)
}
fn parse_vcpop_m(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vcpop_m(vm, vs2, rd)
}
fn parse_vfirst_m(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfirst_m(vm, vs2, rd)
}
fn parse_vmsbf_m(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vmsbf_m(vm, vs2, vd)
}
fn parse_vmsif_m(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vmsif_m(vm, vs2, vd)
}
fn parse_vmsof_m(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vmsof_m(vm, vs2, vd)
}
fn parse_viota_m(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_viota_m(vm, vs2, vd)
}
fn parse_vid_v(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vid_v(vm, vd)
}
fn parse_vmadc_vvm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvmfunct6::VVM_VMADC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("v0")?;
    encode_vvmtype(funct6, vs2, vs1, vd)
}
fn parse_vmsbc_vvm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvmfunct6::VVM_VMSBC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("v0")?;
    encode_vvmtype(funct6, vs2, vs1, vd)
}
fn parse_vmadc_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvmcfunct6::VVMC_VMADC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_vvmctype(funct6, vs2, vs1, vd)
}
fn parse_vmsbc_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvmcfunct6::VVMC_VMSBC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_vvmctype(funct6, vs2, vs1, vd)
}
fn parse_vadc_vvm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvmsfunct6::VVMS_VADC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("v0")?;
    encode_vvmstype(funct6, vs2, vs1, vd)
}
fn parse_vsbc_vvm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvmsfunct6::VVMS_VSBC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("v0")?;
    encode_vvmstype(funct6, vs2, vs1, vd)
}
fn parse_vmseq_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvcmpfunct6::VVCMP_VMSEQ;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvcmptype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmsne_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvcmpfunct6::VVCMP_VMSNE;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvcmptype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmsltu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvcmpfunct6::VVCMP_VMSLTU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvcmptype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmslt_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvcmpfunct6::VVCMP_VMSLT;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvcmptype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmsleu_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvcmpfunct6::VVCMP_VMSLEU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvcmptype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmsle_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vvcmpfunct6::VVCMP_VMSLE;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vvcmptype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmadc_vxm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxmfunct6::VXM_VMADC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("v0")?;
    encode_vxmtype(funct6, vs2, rs1, vd)
}
fn parse_vmsbc_vxm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxmfunct6::VXM_VMSBC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("v0")?;
    encode_vxmtype(funct6, vs2, rs1, vd)
}
fn parse_vmadc_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxmcfunct6::VXMC_VMADC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_vxmctype(funct6, vs2, rs1, vd)
}
fn parse_vmsbc_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxmcfunct6::VXMC_VMSBC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_vxmctype(funct6, vs2, rs1, vd)
}
fn parse_vadc_vxm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxmsfunct6::VXMS_VADC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("v0")?;
    encode_vxmstype(funct6, vs2, rs1, vd)
}
fn parse_vsbc_vxm(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxmsfunct6::VXMS_VSBC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    parser.expect("v0")?;
    encode_vxmstype(funct6, vs2, rs1, vd)
}
fn parse_vmseq_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxcmpfunct6::VXCMP_VMSEQ;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxcmptype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmsne_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxcmpfunct6::VXCMP_VMSNE;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxcmptype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmsltu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxcmpfunct6::VXCMP_VMSLTU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxcmptype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmslt_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxcmpfunct6::VXCMP_VMSLT;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxcmptype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmsleu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxcmpfunct6::VXCMP_VMSLEU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxcmptype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmsle_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxcmpfunct6::VXCMP_VMSLE;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxcmptype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmsgtu_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxcmpfunct6::VXCMP_VMSGTU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxcmptype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmsgt_vx(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vxcmpfunct6::VXCMP_VMSGT;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vxcmptype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmadc_vim(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vimfunct6::VIM_VMADC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    parser.expect_comma()?;
    parser.expect("v0")?;
    encode_vimtype(funct6, vs2, simm, vd)
}
fn parse_vmadc_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vimcfunct6::VIMC_VMADC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    encode_vimctype(funct6, vs2, simm, vd)
}
fn parse_vadc_vim(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vimsfunct6::VIMS_VADC;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    parser.expect_comma()?;
    parser.expect("v0")?;
    encode_vimstype(funct6, vs2, simm, vd)
}
fn parse_vmseq_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vicmpfunct6::VICMP_VMSEQ;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vicmptype(funct6, vm, vs2, simm, vd)
}
fn parse_vmsne_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vicmpfunct6::VICMP_VMSNE;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vicmptype(funct6, vm, vs2, simm, vd)
}
fn parse_vmsleu_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vicmpfunct6::VICMP_VMSLEU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vicmptype(funct6, vm, vs2, simm, vd)
}
fn parse_vmsle_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vicmpfunct6::VICMP_VMSLE;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vicmptype(funct6, vm, vs2, simm, vd)
}
fn parse_vmsgtu_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vicmpfunct6::VICMP_VMSGTU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vicmptype(funct6, vm, vs2, simm, vd)
}
fn parse_vmsgt_vi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = vicmpfunct6::VICMP_VMSGT;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let simm = parser.expect_signed_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vicmptype(funct6, vm, vs2, simm, vd)
}
fn parse_vmfeq_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvmfunct6::FVVM_VMFEQ;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvmtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmfle_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvmfunct6::FVVM_VMFLE;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvmtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmflt_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvmfunct6::FVVM_VMFLT;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvmtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmfne_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvvmfunct6::FVVM_VMFNE;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvvmtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vmfeq_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvfmfunct6::VFM_VMFEQ;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvfmtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmfle_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvfmfunct6::VFM_VMFLE;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvfmtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmflt_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvfmfunct6::VFM_VMFLT;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvfmtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmfne_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvfmfunct6::VFM_VMFNE;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvfmtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmfgt_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvfmfunct6::VFM_VMFGT;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvfmtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vmfge_vf(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = fvfmfunct6::VFM_VMFGE;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_fvfmtype(funct6, vm, vs2, rs1, vd)
}
fn parse_vwredsumu_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rivvfunct6::IVV_VWREDSUMU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rivvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vwredsum_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rivvfunct6::IVV_VWREDSUM;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rivvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vredsum_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rmvvfunct6::MVV_VREDSUM;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rmvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vredand_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rmvvfunct6::MVV_VREDAND;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rmvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vredor_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rmvvfunct6::MVV_VREDOR;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rmvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vredxor_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rmvvfunct6::MVV_VREDXOR;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rmvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vredminu_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rmvvfunct6::MVV_VREDMINU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rmvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vredmin_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rmvvfunct6::MVV_VREDMIN;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rmvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vredmaxu_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rmvvfunct6::MVV_VREDMAXU;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rmvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vredmax_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rmvvfunct6::MVV_VREDMAX;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rmvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfredosum_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rfvvfunct6::FVV_VFREDOSUM;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rfvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfredusum_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rfvvfunct6::FVV_VFREDUSUM;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rfvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfredmax_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rfvvfunct6::FVV_VFREDMAX;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rfvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfredmin_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rfvvfunct6::FVV_VFREDMIN;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rfvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfwredosum_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rfwvvfunct6::FVV_VFWREDOSUM;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rfwvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_vfwredusum_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = rfwvvfunct6::FVV_VFWREDUSUM;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_rfwvvtype(funct6, vm, vs2, vs1, vd)
}
fn parse_sha256sig0(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_sha256sig0(rs1, rd)
}
fn parse_sha256sig1(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_sha256sig1(rs1, rd)
}
fn parse_sha256sum0(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_sha256sum0(rs1, rd)
}
fn parse_sha256sum1(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_sha256sum1(rs1, rd)
}
fn parse_aes32esmi(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    let bs = parser.expect_unsigned_immediate::<2>()?;
    encode_aes32esmi(bs, rs2, rs1, rd)
}
fn parse_aes32esi(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    let bs = parser.expect_unsigned_immediate::<2>()?;
    encode_aes32esi(bs, rs2, rs1, rd)
}
fn parse_aes32dsmi(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    let bs = parser.expect_unsigned_immediate::<2>()?;
    encode_aes32dsmi(bs, rs2, rs1, rd)
}
fn parse_aes32dsi(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    let bs = parser.expect_unsigned_immediate::<2>()?;
    encode_aes32dsi(bs, rs2, rs1, rd)
}
fn parse_sha512sig0l(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_sha512sig0l(rs2, rs1, rd)
}
fn parse_sha512sig0h(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_sha512sig0h(rs2, rs1, rd)
}
fn parse_sha512sig1l(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_sha512sig1l(rs2, rs1, rd)
}
fn parse_sha512sig1h(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_sha512sig1h(rs2, rs1, rd)
}
fn parse_sha512sum0r(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_sha512sum0r(rs2, rs1, rd)
}
fn parse_sha512sum1r(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_sha512sum1r(rs2, rs1, rd)
}
fn parse_aes64ks1i(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rnum = parser.expect_unsigned_immediate::<4>()?;
    encode_aes64ks1i(rnum, rs1, rd)
}
fn parse_aes64ks2(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_aes64ks2(rs2, rs1, rd)
}
fn parse_aes64im(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_aes64im(rs1, rd)
}
fn parse_aes64esm(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_aes64esm(rs2, rs1, rd)
}
fn parse_aes64es(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_aes64es(rs2, rs1, rd)
}
fn parse_aes64dsm(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_aes64dsm(rs2, rs1, rd)
}
fn parse_aes64ds(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_aes64ds(rs2, rs1, rd)
}
fn parse_sha512sig0(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_sha512sig0(rs1, rd)
}
fn parse_sha512sig1(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_sha512sig1(rs1, rd)
}
fn parse_sha512sum0(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_sha512sum0(rs1, rd)
}
fn parse_sha512sum1(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_sha512sum1(rs1, rd)
}
fn parse_sm3p0(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_sm3p0(rs1, rd)
}
fn parse_sm3p1(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_sm3p1(rs1, rd)
}
fn parse_sm4ed(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    let bs = parser.expect_unsigned_immediate::<2>()?;
    encode_sm4ed(bs, rs2, rs1, rd)
}
fn parse_sm4ks(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    parser.expect_comma()?;
    let bs = parser.expect_unsigned_immediate::<2>()?;
    encode_sm4ks(bs, rs2, rs1, rd)
}
fn parse_pack(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = brop_zbkb::PACK;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbkb_rtype(rs2, rs1, rd, op, ctx)
}
fn parse_packh(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let op = brop_zbkb::PACKH;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbkb_rtype(rs2, rs1, rd, op, ctx)
}
fn parse_packw(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zbkb_packw(rs2, rs1, rd)
}
fn parse_zip(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zip(rs1, rd)
}
fn parse_unzip(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_unzip(rs1, rd)
}
fn parse_brev8(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_brev8(rs1, rd)
}
fn parse_xperm8(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_xperm8(rs2, rs1, rd)
}
fn parse_xperm4(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_xperm4(rs2, rs1, rd)
}
fn parse_vandn_vv(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vandn_vv(vm, vs1, vs2, vd)
}
fn parse_vandn_vx(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vandn_vx(vm, vs2, rs1, vd)
}
fn parse_vbrev_v(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vbrev_v(vm, vs2, vd)
}
fn parse_vbrev8_v(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vbrev8_v(vm, vs2, vd)
}
fn parse_vrev8_v(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vrev8_v(vm, vs2, vd)
}
fn parse_vclz_v(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vclz_v(vm, vs2, vd)
}
fn parse_vctz_v(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vctz_v(vm, vs2, vd)
}
fn parse_vcpop_v(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vcpop_v(vm, vs2, vd)
}
fn parse_vrol_vv(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vrol_vv(vm, vs1, vs2, vd)
}
fn parse_vrol_vx(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vrol_vx(vm, vs2, rs1, vd)
}
fn parse_vror_vv(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vror_vv(vm, vs1, vs2, vd)
}
fn parse_vror_vx(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vror_vx(vm, vs2, rs1, vd)
}
fn parse_vror_vi(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let uimm = parser.expect_unsigned_immediate::<6>()?;
    let vm = maybe_vmask(parser)?;
    encode_vror_vi(vm, vs2, uimm, vd)
}
fn parse_vwsll_vv(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vwsll_vv(vm, vs2, vs1, vd)
}
fn parse_vwsll_vx(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vwsll_vx(vm, vs2, rs1, vd)
}
fn parse_vwsll_vi(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let uimm = parser.expect_unsigned_immediate::<5>()?;
    let vm = maybe_vmask(parser)?;
    encode_vwsll_vi(vm, vs2, uimm, vd)
}
fn parse_vclmul_vv(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vclmul_vv(vm, vs2, vs1, vd)
}
fn parse_vclmul_vx(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vclmul_vx(vm, vs2, rs1, vd)
}
fn parse_vclmulh_vv(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vclmulh_vv(vm, vs2, vs1, vd)
}
fn parse_vclmulh_vx(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vclmulh_vx(vm, vs2, rs1, vd)
}
fn parse_vghsh_vv(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_vghsh_vv(vs2, vs1, vd)
}
fn parse_vgmul_vv(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vgmul_vv(vs2, vd)
}
fn parse_vaesdf_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = zvk_vaesdf_funct6::ZVK_VAESDF_VV;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vaesdf(funct6, vs2, vd)
}
fn parse_vaesdf_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = zvk_vaesdf_funct6::ZVK_VAESDF_VS;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vaesdf(funct6, vs2, vd)
}
fn parse_vaesdm_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = zvk_vaesdm_funct6::ZVK_VAESDM_VV;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vaesdm(funct6, vs2, vd)
}
fn parse_vaesdm_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = zvk_vaesdm_funct6::ZVK_VAESDM_VS;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vaesdm(funct6, vs2, vd)
}
fn parse_vaesef_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = zvk_vaesef_funct6::ZVK_VAESEF_VV;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vaesef(funct6, vs2, vd)
}
fn parse_vaesef_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = zvk_vaesef_funct6::ZVK_VAESEF_VS;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vaesef(funct6, vs2, vd)
}
fn parse_vaesem_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = zvk_vaesem_funct6::ZVK_VAESEM_VV;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vaesem(funct6, vs2, vd)
}
fn parse_vaesem_vs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = zvk_vaesem_funct6::ZVK_VAESEM_VS;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vaesem(funct6, vs2, vd)
}
fn parse_vaeskf1_vi(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rnd = parser.expect_unsigned_immediate::<5>()?;
    encode_vaeskf1_vi(vs2, rnd, vd)
}
fn parse_vaeskf2_vi(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let rnd = parser.expect_unsigned_immediate::<5>()?;
    encode_vaeskf2_vi(vs2, rnd, vd)
}
fn parse_vaesz_vs(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_vaesz_vs(vs2, vd)
}
fn parse_vsm4k_vi(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let uimm = parser.expect_unsigned_immediate::<5>()?;
    encode_vsm4k_vi(vs2, uimm, vd)
}
fn parse_vsm4r_vv(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let funct6 = zvk_vsm4r_funct6::ZVK_VSM4R_VV;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_zvksm4rtype(funct6, vs2, vd, ctx)
}
fn parse_vsm4r_vs(parser: &mut Parser, ctx: &Context) -> crate::Result<u32> {
    // predefined constants
    let funct6 = zvk_vsm4r_funct6::ZVK_VSM4R_VS;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    encode_zvksm4rtype(funct6, vs2, vd, ctx)
}
fn parse_vsha2ms_vv(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_vsha2ms_vv(vs2, vs1, vd)
}
fn parse_vsha2ch_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = zvk_vsha2_funct6::ZVK_VSHA2CH_VV;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_zvksha2type(funct6, vs2, vs1, vd)
}
fn parse_vsha2cl_vv(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let funct6 = zvk_vsha2_funct6::ZVK_VSHA2CL_VV;
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_zvksha2type(funct6, vs2, vs1, vd)
}
fn parse_vsm3me_vv(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    encode_vsm3me_vv(vs2, vs1, vd)
}
fn parse_vsm3c_vi(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let uimm = parser.expect_unsigned_immediate::<5>()?;
    encode_vsm3c_vi(vs2, uimm, vd)
}
fn parse_csrrwi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = csrop::CSRRW;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let csr = csr_name_map(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_unsigned_immediate::<5>()?;
    encode_csrimm(csr, imm, rd, op)
}
fn parse_csrrsi(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = csrop::CSRRS;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let csr = csr_name_map(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_unsigned_immediate::<5>()?;
    encode_csrimm(csr, imm, rd, op)
}
fn parse_csrrci(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = csrop::CSRRC;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let csr = csr_name_map(parser)?;
    parser.expect_comma()?;
    let imm = parser.expect_unsigned_immediate::<5>()?;
    encode_csrimm(csr, imm, rd, op)
}
fn parse_csrrw(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = csrop::CSRRW;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let csr = csr_name_map(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_csrreg(csr, rs1, rd, op)
}
fn parse_csrrs(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = csrop::CSRRS;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let csr = csr_name_map(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_csrreg(csr, rs1, rd, op)
}
fn parse_csrrc(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = csrop::CSRRC;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let csr = csr_name_map(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_csrreg(csr, rs1, rd, op)
}
fn parse_sinval_vma(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_sinval_vma(rs1, rs2)
}
fn parse_sfence_w_inval() -> crate::Result<u32> {
    encode_sfence_w_inval()
}
fn parse_sfence_inval_ir() -> crate::Result<u32> {
    encode_sfence_inval_ir()
}
fn parse_wrs_sto() -> crate::Result<u32> {
    encode_wrs(wrsop::WRS_STO)
}
fn parse_wrs_nto() -> crate::Result<u32> {
    encode_wrs(wrsop::WRS_NTO)
}
fn parse_czero_eqz(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = zicondop::CZERO_EQZ;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zicond_rtype(rs2, rs1, rd, op)
}
fn parse_czero_nez(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let op = zicondop::CZERO_NEZ;
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zicond_rtype(rs2, rs1, rd, op)
}
fn parse_cbo_clean(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let cbop = cbop_zicbom::CBO_CLEAN;
    // parse arguments
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_zicbom(cbop, rs1)
}
fn parse_cbo_flush(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let cbop = cbop_zicbom::CBO_FLUSH;
    // parse arguments
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_zicbom(cbop, rs1)
}
fn parse_cbo_inval(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let cbop = cbop_zicbom::CBO_INVAL;
    // parse arguments
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_zicbom(cbop, rs1)
}
fn parse_cbo_zero(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    parser.expect("(")?;
    parser.skip_ws();
    let rs1 = reg_name(parser)?;
    parser.skip_ws();
    parser.expect(")")?;
    encode_zicboz(rs1)
}
fn parse_fcvt_bf16_s(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_fcvt_bf16_s(rs1, rm, rd)
}
fn parse_fcvt_s_bf16(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = freg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = freg_name(parser)?;
    parser.expect_comma()?;
    let rm = frm_mnemonic(parser)?;
    encode_fcvt_s_bf16(rs1, rm, rd)
}
fn parse_vfncvtbf16_f_f_w(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vm = maybe_vmask(parser)?;
    encode_vfncvtbf16_f_f_w(vm, vs2, vd)
}
fn parse_vfwcvtbf16_f_f_v(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs2 = vreg_name(parser)?;
    parser.expect_comma()?;
    let vm = maybe_vmask(parser)?;
    encode_vfwcvtbf16_f_f_v(vm, vs2, vd)
}
fn parse_vfwmaccbf16_vv(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = vreg_name(parser)?;
    parser.skip_ws();
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfwmaccbf16_vv(vm, vs2, vs1, vd)
}
fn parse_vfwmaccbf16_vf(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let vd = vreg_name(parser)?;
    parser.expect_comma()?;
    let vs1 = freg_name(parser)?;
    parser.skip_ws();
    let vs2 = vreg_name(parser)?;
    let vm = maybe_vmask(parser)?;
    encode_vfwmaccbf16_vf(vm, vs2, vs1, vd)
}
fn parse_mop_r_0(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b00000);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_1(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b00001);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_2(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b00010);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_3(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b00011);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_4(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b00100);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_5(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b00101);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_6(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b00110);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_7(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b00111);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_8(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b01000);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_9(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b01001);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_10(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b01010);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_11(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b01011);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_12(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b01100);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_13(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b01101);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_14(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b01110);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_15(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b01111);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_16(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b10000);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_17(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b10001);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_18(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b10010);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_19(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b10011);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_20(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b10100);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_21(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b10101);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_22(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b10110);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_23(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b10111);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_24(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b11000);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_25(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b11001);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_26(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b11010);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_27(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b11011);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_28(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b11100);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_29(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b11101);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_30(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b11110);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_r_31(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<5>::new(0b11111);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_zimop_mop_r(mop, rs1, rd)
}
fn parse_mop_rr_0(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b000);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zimop_mop_rr(mop, rs2, rs1, rd)
}
fn parse_mop_rr_1(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b001);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zimop_mop_rr(mop, rs2, rs1, rd)
}
fn parse_mop_rr_2(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b010);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zimop_mop_rr(mop, rs2, rs1, rd)
}
fn parse_mop_rr_3(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b011);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zimop_mop_rr(mop, rs2, rs1, rd)
}
fn parse_mop_rr_4(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b100);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zimop_mop_rr(mop, rs2, rs1, rd)
}
fn parse_mop_rr_5(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b101);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zimop_mop_rr(mop, rs2, rs1, rd)
}
fn parse_mop_rr_6(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b110);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zimop_mop_rr(mop, rs2, rs1, rd)
}
fn parse_mop_rr_7(parser: &mut Parser) -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b111);
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    parser.expect_comma()?;
    let rs2 = reg_name(parser)?;
    encode_zimop_mop_rr(mop, rs2, rs1, rd)
}
fn parse_c_mop_1() -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b000);
    encode_zcmop(mop)
}
fn parse_c_mop_3() -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b001);
    encode_zcmop(mop)
}
fn parse_c_mop_5() -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b010);
    encode_zcmop(mop)
}
fn parse_c_mop_7() -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b011);
    encode_zcmop(mop)
}
fn parse_c_mop_9() -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b100);
    encode_zcmop(mop)
}
fn parse_c_mop_11() -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b101);
    encode_zcmop(mop)
}
fn parse_c_mop_13() -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b110);
    encode_zcmop(mop)
}
fn parse_c_mop_15() -> crate::Result<u32> {
    // predefined constants
    let mop = BitVector::<3>::new(0b111);
    encode_zcmop(mop)
}
fn parse_illegal(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let s = parser.expect_unsigned_immediate::<32>()?;
    encode_illegal(s)
}
fn parse_c_illegal(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let s = parser.expect_unsigned_immediate::<16>()?;
    encode_c_illegal(s)
}
fn parse_c_nop(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let imm = maybe_nonzero_imm_6(parser)?;
    encode_c_nop(imm)
}
fn parse_mv(parser: &mut Parser) -> crate::Result<u32> {
    // parse arguments
    let rd = reg_name(parser)?;
    parser.expect_comma()?;
    let rs1 = reg_name(parser)?;
    encode_itype(
        BitVector::<12> {
            val: 0b000000000000,
        },
        rs1,
        rd,
        iop::ADDI,
    )
}
fn encdec_amoop(v: amoop) -> BitVector<5> {
    match v {
        amoop::AMOSWAP => BitVector::<5>::new(0b00001),
        amoop::AMOADD => BitVector::<5>::new(0b00000),
        amoop::AMOXOR => BitVector::<5>::new(0b00100),
        amoop::AMOAND => BitVector::<5>::new(0b01100),
        amoop::AMOOR => BitVector::<5>::new(0b01000),
        amoop::AMOMIN => BitVector::<5>::new(0b10000),
        amoop::AMOMAX => BitVector::<5>::new(0b10100),
        amoop::AMOMINU => BitVector::<5>::new(0b11000),
        amoop::AMOMAXU => BitVector::<5>::new(0b11100),
        amoop::AMOCAS => BitVector::<5>::new(0b00101),
    }
}
fn encdec_cbop(v: cbop_zicbom) -> BitVector<12> {
    match v {
        cbop_zicbom::CBO_CLEAN => BitVector::<12>::new(0b000000000001),
        cbop_zicbom::CBO_FLUSH => BitVector::<12>::new(0b000000000010),
        cbop_zicbom::CBO_INVAL => BitVector::<12>::new(0b000000000000),
    }
}
fn encdec_cbop_zicbop(v: cbop_zicbop) -> BitVector<5> {
    match v {
        cbop_zicbop::PREFETCH_I => BitVector::<5>::new(0b00000),
        cbop_zicbop::PREFETCH_R => BitVector::<5>::new(0b00001),
        cbop_zicbop::PREFETCH_W => BitVector::<5>::new(0b00011),
    }
}
fn encdec_csrop(v: csrop) -> BitVector<2> {
    match v {
        csrop::CSRRW => BitVector::<2>::new(0b01),
        csrop::CSRRS => BitVector::<2>::new(0b10),
        csrop::CSRRC => BitVector::<2>::new(0b11),
    }
}
fn encdec_fvffunct6(v: fvffunct6) -> BitVector<6> {
    match v {
        fvffunct6::VF_VADD => BitVector::<6>::new(0b000000),
        fvffunct6::VF_VSUB => BitVector::<6>::new(0b000010),
        fvffunct6::VF_VMIN => BitVector::<6>::new(0b000100),
        fvffunct6::VF_VMAX => BitVector::<6>::new(0b000110),
        fvffunct6::VF_VSGNJ => BitVector::<6>::new(0b001000),
        fvffunct6::VF_VSGNJN => BitVector::<6>::new(0b001001),
        fvffunct6::VF_VSGNJX => BitVector::<6>::new(0b001010),
        fvffunct6::VF_VDIV => BitVector::<6>::new(0b100000),
        fvffunct6::VF_VRDIV => BitVector::<6>::new(0b100001),
        fvffunct6::VF_VMUL => BitVector::<6>::new(0b100100),
        fvffunct6::VF_VRSUB => BitVector::<6>::new(0b100111),
        fvffunct6::VF_VSLIDE1UP => BitVector::<6>::new(0b001110),
        fvffunct6::VF_VSLIDE1DOWN => BitVector::<6>::new(0b001111),
    }
}
fn encdec_fvfmafunct6(v: fvfmafunct6) -> BitVector<6> {
    match v {
        fvfmafunct6::VF_VMADD => BitVector::<6>::new(0b101000),
        fvfmafunct6::VF_VNMADD => BitVector::<6>::new(0b101001),
        fvfmafunct6::VF_VMSUB => BitVector::<6>::new(0b101010),
        fvfmafunct6::VF_VNMSUB => BitVector::<6>::new(0b101011),
        fvfmafunct6::VF_VMACC => BitVector::<6>::new(0b101100),
        fvfmafunct6::VF_VNMACC => BitVector::<6>::new(0b101101),
        fvfmafunct6::VF_VMSAC => BitVector::<6>::new(0b101110),
        fvfmafunct6::VF_VNMSAC => BitVector::<6>::new(0b101111),
    }
}
fn encdec_fvfmfunct6(v: fvfmfunct6) -> BitVector<6> {
    match v {
        fvfmfunct6::VFM_VMFEQ => BitVector::<6>::new(0b011000),
        fvfmfunct6::VFM_VMFLE => BitVector::<6>::new(0b011001),
        fvfmfunct6::VFM_VMFLT => BitVector::<6>::new(0b011011),
        fvfmfunct6::VFM_VMFNE => BitVector::<6>::new(0b011100),
        fvfmfunct6::VFM_VMFGT => BitVector::<6>::new(0b011101),
        fvfmfunct6::VFM_VMFGE => BitVector::<6>::new(0b011111),
    }
}
fn encdec_fvvfunct6(v: fvvfunct6) -> BitVector<6> {
    match v {
        fvvfunct6::FVV_VADD => BitVector::<6>::new(0b000000),
        fvvfunct6::FVV_VSUB => BitVector::<6>::new(0b000010),
        fvvfunct6::FVV_VMIN => BitVector::<6>::new(0b000100),
        fvvfunct6::FVV_VMAX => BitVector::<6>::new(0b000110),
        fvvfunct6::FVV_VSGNJ => BitVector::<6>::new(0b001000),
        fvvfunct6::FVV_VSGNJN => BitVector::<6>::new(0b001001),
        fvvfunct6::FVV_VSGNJX => BitVector::<6>::new(0b001010),
        fvvfunct6::FVV_VDIV => BitVector::<6>::new(0b100000),
        fvvfunct6::FVV_VMUL => BitVector::<6>::new(0b100100),
    }
}
fn encdec_fvvmafunct6(v: fvvmafunct6) -> BitVector<6> {
    match v {
        fvvmafunct6::FVV_VMADD => BitVector::<6>::new(0b101000),
        fvvmafunct6::FVV_VNMADD => BitVector::<6>::new(0b101001),
        fvvmafunct6::FVV_VMSUB => BitVector::<6>::new(0b101010),
        fvvmafunct6::FVV_VNMSUB => BitVector::<6>::new(0b101011),
        fvvmafunct6::FVV_VMACC => BitVector::<6>::new(0b101100),
        fvvmafunct6::FVV_VNMACC => BitVector::<6>::new(0b101101),
        fvvmafunct6::FVV_VMSAC => BitVector::<6>::new(0b101110),
        fvvmafunct6::FVV_VNMSAC => BitVector::<6>::new(0b101111),
    }
}
fn encdec_fvvmfunct6(v: fvvmfunct6) -> BitVector<6> {
    match v {
        fvvmfunct6::FVVM_VMFEQ => BitVector::<6>::new(0b011000),
        fvvmfunct6::FVVM_VMFLE => BitVector::<6>::new(0b011001),
        fvvmfunct6::FVVM_VMFLT => BitVector::<6>::new(0b011011),
        fvvmfunct6::FVVM_VMFNE => BitVector::<6>::new(0b011100),
    }
}
fn encdec_fwffunct6(v: fwffunct6) -> BitVector<6> {
    match v {
        fwffunct6::FWF_VADD => BitVector::<6>::new(0b110100),
        fwffunct6::FWF_VSUB => BitVector::<6>::new(0b110110),
    }
}
fn encdec_fwvffunct6(v: fwvffunct6) -> BitVector<6> {
    match v {
        fwvffunct6::FWVF_VADD => BitVector::<6>::new(0b110000),
        fwvffunct6::FWVF_VSUB => BitVector::<6>::new(0b110010),
        fwvffunct6::FWVF_VMUL => BitVector::<6>::new(0b111000),
    }
}
fn encdec_fwvfmafunct6(v: fwvfmafunct6) -> BitVector<6> {
    match v {
        fwvfmafunct6::FWVF_VMACC => BitVector::<6>::new(0b111100),
        fwvfmafunct6::FWVF_VNMACC => BitVector::<6>::new(0b111101),
        fwvfmafunct6::FWVF_VMSAC => BitVector::<6>::new(0b111110),
        fwvfmafunct6::FWVF_VNMSAC => BitVector::<6>::new(0b111111),
    }
}
fn encdec_fwvfunct6(v: fwvfunct6) -> BitVector<6> {
    match v {
        fwvfunct6::FWV_VADD => BitVector::<6>::new(0b110100),
        fwvfunct6::FWV_VSUB => BitVector::<6>::new(0b110110),
    }
}
fn encdec_fwvvfunct6(v: fwvvfunct6) -> BitVector<6> {
    match v {
        fwvvfunct6::FWVV_VADD => BitVector::<6>::new(0b110000),
        fwvvfunct6::FWVV_VSUB => BitVector::<6>::new(0b110010),
        fwvvfunct6::FWVV_VMUL => BitVector::<6>::new(0b111000),
    }
}
fn encdec_fwvvmafunct6(v: fwvvmafunct6) -> BitVector<6> {
    match v {
        fwvvmafunct6::FWVV_VMACC => BitVector::<6>::new(0b111100),
        fwvvmafunct6::FWVV_VNMACC => BitVector::<6>::new(0b111101),
        fwvvmafunct6::FWVV_VMSAC => BitVector::<6>::new(0b111110),
        fwvvmafunct6::FWVV_VNMSAC => BitVector::<6>::new(0b111111),
    }
}
fn encdec_indexed_mop(v: indexed_mop) -> BitVector<2> {
    match v {
        indexed_mop::INDEXED_UNORDERED => BitVector::<2>::new(0b01),
        indexed_mop::INDEXED_ORDERED => BitVector::<2>::new(0b11),
    }
}
fn encdec_iop(v: iop) -> BitVector<3> {
    match v {
        iop::ADDI => BitVector::<3>::new(0b000),
        iop::SLTI => BitVector::<3>::new(0b010),
        iop::SLTIU => BitVector::<3>::new(0b011),
        iop::XORI => BitVector::<3>::new(0b100),
        iop::ORI => BitVector::<3>::new(0b110),
        iop::ANDI => BitVector::<3>::new(0b111),
    }
}
fn encdec_mmfunct6(v: mmfunct6) -> BitVector<6> {
    match v {
        mmfunct6::MM_VMAND => BitVector::<6>::new(0b011001),
        mmfunct6::MM_VMNAND => BitVector::<6>::new(0b011101),
        mmfunct6::MM_VMANDN => BitVector::<6>::new(0b011000),
        mmfunct6::MM_VMXOR => BitVector::<6>::new(0b011011),
        mmfunct6::MM_VMOR => BitVector::<6>::new(0b011010),
        mmfunct6::MM_VMNOR => BitVector::<6>::new(0b011110),
        mmfunct6::MM_VMORN => BitVector::<6>::new(0b011100),
        mmfunct6::MM_VMXNOR => BitVector::<6>::new(0b011111),
    }
}
fn encdec_mvvfunct6(v: mvvfunct6) -> BitVector<6> {
    match v {
        mvvfunct6::MVV_VAADDU => BitVector::<6>::new(0b001000),
        mvvfunct6::MVV_VAADD => BitVector::<6>::new(0b001001),
        mvvfunct6::MVV_VASUBU => BitVector::<6>::new(0b001010),
        mvvfunct6::MVV_VASUB => BitVector::<6>::new(0b001011),
        mvvfunct6::MVV_VMUL => BitVector::<6>::new(0b100101),
        mvvfunct6::MVV_VMULH => BitVector::<6>::new(0b100111),
        mvvfunct6::MVV_VMULHU => BitVector::<6>::new(0b100100),
        mvvfunct6::MVV_VMULHSU => BitVector::<6>::new(0b100110),
        mvvfunct6::MVV_VDIVU => BitVector::<6>::new(0b100000),
        mvvfunct6::MVV_VDIV => BitVector::<6>::new(0b100001),
        mvvfunct6::MVV_VREMU => BitVector::<6>::new(0b100010),
        mvvfunct6::MVV_VREM => BitVector::<6>::new(0b100011),
    }
}
fn encdec_mvvmafunct6(v: mvvmafunct6) -> BitVector<6> {
    match v {
        mvvmafunct6::MVV_VMACC => BitVector::<6>::new(0b101101),
        mvvmafunct6::MVV_VNMSAC => BitVector::<6>::new(0b101111),
        mvvmafunct6::MVV_VMADD => BitVector::<6>::new(0b101001),
        mvvmafunct6::MVV_VNMSUB => BitVector::<6>::new(0b101011),
    }
}
fn encdec_mvxfunct6(v: mvxfunct6) -> BitVector<6> {
    match v {
        mvxfunct6::MVX_VAADDU => BitVector::<6>::new(0b001000),
        mvxfunct6::MVX_VAADD => BitVector::<6>::new(0b001001),
        mvxfunct6::MVX_VASUBU => BitVector::<6>::new(0b001010),
        mvxfunct6::MVX_VASUB => BitVector::<6>::new(0b001011),
        mvxfunct6::MVX_VSLIDE1UP => BitVector::<6>::new(0b001110),
        mvxfunct6::MVX_VSLIDE1DOWN => BitVector::<6>::new(0b001111),
        mvxfunct6::MVX_VMUL => BitVector::<6>::new(0b100101),
        mvxfunct6::MVX_VMULH => BitVector::<6>::new(0b100111),
        mvxfunct6::MVX_VMULHU => BitVector::<6>::new(0b100100),
        mvxfunct6::MVX_VMULHSU => BitVector::<6>::new(0b100110),
        mvxfunct6::MVX_VDIVU => BitVector::<6>::new(0b100000),
        mvxfunct6::MVX_VDIV => BitVector::<6>::new(0b100001),
        mvxfunct6::MVX_VREMU => BitVector::<6>::new(0b100010),
        mvxfunct6::MVX_VREM => BitVector::<6>::new(0b100011),
    }
}
fn encdec_mvxmafunct6(v: mvxmafunct6) -> BitVector<6> {
    match v {
        mvxmafunct6::MVX_VMACC => BitVector::<6>::new(0b101101),
        mvxmafunct6::MVX_VNMSAC => BitVector::<6>::new(0b101111),
        mvxmafunct6::MVX_VMADD => BitVector::<6>::new(0b101001),
        mvxmafunct6::MVX_VNMSUB => BitVector::<6>::new(0b101011),
    }
}
fn encdec_nifunct6(v: nifunct6) -> BitVector<6> {
    match v {
        nifunct6::NI_VNCLIPU => BitVector::<6>::new(0b101110),
        nifunct6::NI_VNCLIP => BitVector::<6>::new(0b101111),
    }
}
fn encdec_nisfunct6(v: nisfunct6) -> BitVector<6> {
    match v {
        nisfunct6::NIS_VNSRL => BitVector::<6>::new(0b101100),
        nisfunct6::NIS_VNSRA => BitVector::<6>::new(0b101101),
    }
}
fn encdec_ntl(v: ntl_type) -> BitVector<5> {
    match v {
        ntl_type::NTL_P1 => BitVector::<5>::new(0b00010),
        ntl_type::NTL_PALL => BitVector::<5>::new(0b00011),
        ntl_type::NTL_S1 => BitVector::<5>::new(0b00100),
        ntl_type::NTL_ALL => BitVector::<5>::new(0b00101),
    }
}
fn encdec_nvfunct6(v: nvfunct6) -> BitVector<6> {
    match v {
        nvfunct6::NV_VNCLIPU => BitVector::<6>::new(0b101110),
        nvfunct6::NV_VNCLIP => BitVector::<6>::new(0b101111),
    }
}
fn encdec_nvsfunct6(v: nvsfunct6) -> BitVector<6> {
    match v {
        nvsfunct6::NVS_VNSRL => BitVector::<6>::new(0b101100),
        nvsfunct6::NVS_VNSRA => BitVector::<6>::new(0b101101),
    }
}
fn encdec_nxfunct6(v: nxfunct6) -> BitVector<6> {
    match v {
        nxfunct6::NX_VNCLIPU => BitVector::<6>::new(0b101110),
        nxfunct6::NX_VNCLIP => BitVector::<6>::new(0b101111),
    }
}
fn encdec_nxsfunct6(v: nxsfunct6) -> BitVector<6> {
    match v {
        nxsfunct6::NXS_VNSRL => BitVector::<6>::new(0b101100),
        nxsfunct6::NXS_VNSRA => BitVector::<6>::new(0b101101),
    }
}
fn encdec_rfvvfunct6(v: rfvvfunct6) -> BitVector<6> {
    match v {
        rfvvfunct6::FVV_VFREDOSUM => BitVector::<6>::new(0b000011),
        rfvvfunct6::FVV_VFREDUSUM => BitVector::<6>::new(0b000001),
        rfvvfunct6::FVV_VFREDMAX => BitVector::<6>::new(0b000111),
        rfvvfunct6::FVV_VFREDMIN => BitVector::<6>::new(0b000101),
    }
}
fn encdec_rfwvvfunct6(v: rfwvvfunct6) -> BitVector<6> {
    match v {
        rfwvvfunct6::FVV_VFWREDOSUM => BitVector::<6>::new(0b110011),
        rfwvvfunct6::FVV_VFWREDUSUM => BitVector::<6>::new(0b110001),
    }
}
fn encdec_rivvfunct6(v: rivvfunct6) -> BitVector<6> {
    match v {
        rivvfunct6::IVV_VWREDSUMU => BitVector::<6>::new(0b110000),
        rivvfunct6::IVV_VWREDSUM => BitVector::<6>::new(0b110001),
    }
}
fn encdec_rmvvfunct6(v: rmvvfunct6) -> BitVector<6> {
    match v {
        rmvvfunct6::MVV_VREDSUM => BitVector::<6>::new(0b000000),
        rmvvfunct6::MVV_VREDAND => BitVector::<6>::new(0b000001),
        rmvvfunct6::MVV_VREDOR => BitVector::<6>::new(0b000010),
        rmvvfunct6::MVV_VREDXOR => BitVector::<6>::new(0b000011),
        rmvvfunct6::MVV_VREDMINU => BitVector::<6>::new(0b000100),
        rmvvfunct6::MVV_VREDMIN => BitVector::<6>::new(0b000101),
        rmvvfunct6::MVV_VREDMAXU => BitVector::<6>::new(0b000110),
        rmvvfunct6::MVV_VREDMAX => BitVector::<6>::new(0b000111),
    }
}
fn encdec_rounding_mode(v: rounding_mode) -> BitVector<3> {
    match v {
        rounding_mode::RM_RNE => BitVector::<3>::new(0b000),
        rounding_mode::RM_RTZ => BitVector::<3>::new(0b001),
        rounding_mode::RM_RDN => BitVector::<3>::new(0b010),
        rounding_mode::RM_RUP => BitVector::<3>::new(0b011),
        rounding_mode::RM_RMM => BitVector::<3>::new(0b100),
        rounding_mode::RM_DYN => BitVector::<3>::new(0b111),
    }
}
fn encdec_uop(v: uop) -> BitVector<7> {
    match v {
        uop::LUI => BitVector::<7>::new(0b0110111),
        uop::AUIPC => BitVector::<7>::new(0b0010111),
    }
}
fn vext_vs1(v: vextfunct6) -> BitVector<5> {
    match v {
        vextfunct6::VEXT2_ZVF2 => BitVector::<5>::new(0b00110),
        vextfunct6::VEXT2_SVF2 => BitVector::<5>::new(0b00111),
        vextfunct6::VEXT4_ZVF4 => BitVector::<5>::new(0b00100),
        vextfunct6::VEXT4_SVF4 => BitVector::<5>::new(0b00101),
        vextfunct6::VEXT8_ZVF8 => BitVector::<5>::new(0b00010),
        vextfunct6::VEXT8_SVF8 => BitVector::<5>::new(0b00011),
    }
}
fn encdec_vfnunary0_vs1(v: vfnunary0) -> BitVector<5> {
    match v {
        vfnunary0::FNV_CVT_XU_F => BitVector::<5>::new(0b10000),
        vfnunary0::FNV_CVT_X_F => BitVector::<5>::new(0b10001),
        vfnunary0::FNV_CVT_F_XU => BitVector::<5>::new(0b10010),
        vfnunary0::FNV_CVT_F_X => BitVector::<5>::new(0b10011),
        vfnunary0::FNV_CVT_F_F => BitVector::<5>::new(0b10100),
        vfnunary0::FNV_CVT_ROD_F_F => BitVector::<5>::new(0b10101),
        vfnunary0::FNV_CVT_RTZ_XU_F => BitVector::<5>::new(0b10110),
        vfnunary0::FNV_CVT_RTZ_X_F => BitVector::<5>::new(0b10111),
    }
}
fn encdec_vfunary0_vs1(v: vfunary0) -> BitVector<5> {
    match v {
        vfunary0::FV_CVT_XU_F => BitVector::<5>::new(0b00000),
        vfunary0::FV_CVT_X_F => BitVector::<5>::new(0b00001),
        vfunary0::FV_CVT_F_XU => BitVector::<5>::new(0b00010),
        vfunary0::FV_CVT_F_X => BitVector::<5>::new(0b00011),
        vfunary0::FV_CVT_RTZ_XU_F => BitVector::<5>::new(0b00110),
        vfunary0::FV_CVT_RTZ_X_F => BitVector::<5>::new(0b00111),
    }
}
fn encdec_vfunary1_vs1(v: vfunary1) -> BitVector<5> {
    match v {
        vfunary1::FVV_VSQRT => BitVector::<5>::new(0b00000),
        vfunary1::FVV_VRSQRT7 => BitVector::<5>::new(0b00100),
        vfunary1::FVV_VREC7 => BitVector::<5>::new(0b00101),
        vfunary1::FVV_VCLASS => BitVector::<5>::new(0b10000),
    }
}
fn encdec_vfwunary0_vs1(v: vfwunary0) -> BitVector<5> {
    match v {
        vfwunary0::FWV_CVT_XU_F => BitVector::<5>::new(0b01000),
        vfwunary0::FWV_CVT_X_F => BitVector::<5>::new(0b01001),
        vfwunary0::FWV_CVT_F_XU => BitVector::<5>::new(0b01010),
        vfwunary0::FWV_CVT_F_X => BitVector::<5>::new(0b01011),
        vfwunary0::FWV_CVT_F_F => BitVector::<5>::new(0b01100),
        vfwunary0::FWV_CVT_RTZ_XU_F => BitVector::<5>::new(0b01110),
        vfwunary0::FWV_CVT_RTZ_X_F => BitVector::<5>::new(0b01111),
    }
}
fn encdec_vicmpfunct6(v: vicmpfunct6) -> BitVector<6> {
    match v {
        vicmpfunct6::VICMP_VMSEQ => BitVector::<6>::new(0b011000),
        vicmpfunct6::VICMP_VMSNE => BitVector::<6>::new(0b011001),
        vicmpfunct6::VICMP_VMSLEU => BitVector::<6>::new(0b011100),
        vicmpfunct6::VICMP_VMSLE => BitVector::<6>::new(0b011101),
        vicmpfunct6::VICMP_VMSGTU => BitVector::<6>::new(0b011110),
        vicmpfunct6::VICMP_VMSGT => BitVector::<6>::new(0b011111),
    }
}
fn encdec_vifunct6(v: vifunct6) -> BitVector<6> {
    match v {
        vifunct6::VI_VADD => BitVector::<6>::new(0b000000),
        vifunct6::VI_VRSUB => BitVector::<6>::new(0b000011),
        vifunct6::VI_VAND => BitVector::<6>::new(0b001001),
        vifunct6::VI_VOR => BitVector::<6>::new(0b001010),
        vifunct6::VI_VXOR => BitVector::<6>::new(0b001011),
        vifunct6::VI_VSADDU => BitVector::<6>::new(0b100000),
        vifunct6::VI_VSADD => BitVector::<6>::new(0b100001),
        vifunct6::VI_VSLL => BitVector::<6>::new(0b100101),
        vifunct6::VI_VSRL => BitVector::<6>::new(0b101000),
        vifunct6::VI_VSRA => BitVector::<6>::new(0b101001),
        vifunct6::VI_VSSRL => BitVector::<6>::new(0b101010),
        vifunct6::VI_VSSRA => BitVector::<6>::new(0b101011),
    }
}
fn encdec_vimcfunct6(v: vimcfunct6) -> BitVector<6> {
    match v {
        vimcfunct6::VIMC_VMADC => BitVector::<6>::new(0b010001),
    }
}
fn encdec_vimfunct6(v: vimfunct6) -> BitVector<6> {
    match v {
        vimfunct6::VIM_VMADC => BitVector::<6>::new(0b010001),
    }
}
fn encdec_vimsfunct6(v: vimsfunct6) -> BitVector<6> {
    match v {
        vimsfunct6::VIMS_VADC => BitVector::<6>::new(0b010000),
    }
}
fn encdec_visgfunct6(v: visgfunct6) -> BitVector<6> {
    match v {
        visgfunct6::VI_VSLIDEUP => BitVector::<6>::new(0b001110),
        visgfunct6::VI_VSLIDEDOWN => BitVector::<6>::new(0b001111),
        visgfunct6::VI_VRGATHER => BitVector::<6>::new(0b001100),
    }
}
fn encdec_vlewidth(v: vlewidth) -> BitVector<3> {
    match v {
        vlewidth::VLE8 => BitVector::<3>::new(0b000),
        vlewidth::VLE16 => BitVector::<3>::new(0b101),
        vlewidth::VLE32 => BitVector::<3>::new(0b110),
        vlewidth::VLE64 => BitVector::<3>::new(0b111),
    }
}
fn encdec_lsop(v: vmlsop) -> BitVector<7> {
    match v {
        vmlsop::VLM => BitVector::<7>::new(0b0000111),
        vmlsop::VSM => BitVector::<7>::new(0b0100111),
    }
}
fn encdec_vvcmpfunct6(v: vvcmpfunct6) -> BitVector<6> {
    match v {
        vvcmpfunct6::VVCMP_VMSEQ => BitVector::<6>::new(0b011000),
        vvcmpfunct6::VVCMP_VMSNE => BitVector::<6>::new(0b011001),
        vvcmpfunct6::VVCMP_VMSLTU => BitVector::<6>::new(0b011010),
        vvcmpfunct6::VVCMP_VMSLT => BitVector::<6>::new(0b011011),
        vvcmpfunct6::VVCMP_VMSLEU => BitVector::<6>::new(0b011100),
        vvcmpfunct6::VVCMP_VMSLE => BitVector::<6>::new(0b011101),
    }
}
fn encdec_vvfunct6(v: vvfunct6) -> BitVector<6> {
    match v {
        vvfunct6::VV_VADD => BitVector::<6>::new(0b000000),
        vvfunct6::VV_VSUB => BitVector::<6>::new(0b000010),
        vvfunct6::VV_VMINU => BitVector::<6>::new(0b000100),
        vvfunct6::VV_VMIN => BitVector::<6>::new(0b000101),
        vvfunct6::VV_VMAXU => BitVector::<6>::new(0b000110),
        vvfunct6::VV_VMAX => BitVector::<6>::new(0b000111),
        vvfunct6::VV_VAND => BitVector::<6>::new(0b001001),
        vvfunct6::VV_VOR => BitVector::<6>::new(0b001010),
        vvfunct6::VV_VXOR => BitVector::<6>::new(0b001011),
        vvfunct6::VV_VRGATHER => BitVector::<6>::new(0b001100),
        vvfunct6::VV_VRGATHEREI16 => BitVector::<6>::new(0b001110),
        vvfunct6::VV_VSADDU => BitVector::<6>::new(0b100000),
        vvfunct6::VV_VSADD => BitVector::<6>::new(0b100001),
        vvfunct6::VV_VSSUBU => BitVector::<6>::new(0b100010),
        vvfunct6::VV_VSSUB => BitVector::<6>::new(0b100011),
        vvfunct6::VV_VSLL => BitVector::<6>::new(0b100101),
        vvfunct6::VV_VSMUL => BitVector::<6>::new(0b100111),
        vvfunct6::VV_VSRL => BitVector::<6>::new(0b101000),
        vvfunct6::VV_VSRA => BitVector::<6>::new(0b101001),
        vvfunct6::VV_VSSRL => BitVector::<6>::new(0b101010),
        vvfunct6::VV_VSSRA => BitVector::<6>::new(0b101011),
    }
}
fn encdec_vvmcfunct6(v: vvmcfunct6) -> BitVector<6> {
    match v {
        vvmcfunct6::VVMC_VMADC => BitVector::<6>::new(0b010001),
        vvmcfunct6::VVMC_VMSBC => BitVector::<6>::new(0b010011),
    }
}
fn encdec_vvmfunct6(v: vvmfunct6) -> BitVector<6> {
    match v {
        vvmfunct6::VVM_VMADC => BitVector::<6>::new(0b010001),
        vvmfunct6::VVM_VMSBC => BitVector::<6>::new(0b010011),
    }
}
fn encdec_vvmsfunct6(v: vvmsfunct6) -> BitVector<6> {
    match v {
        vvmsfunct6::VVMS_VADC => BitVector::<6>::new(0b010000),
        vvmsfunct6::VVMS_VSBC => BitVector::<6>::new(0b010010),
    }
}
fn encdec_vxcmpfunct6(v: vxcmpfunct6) -> BitVector<6> {
    match v {
        vxcmpfunct6::VXCMP_VMSEQ => BitVector::<6>::new(0b011000),
        vxcmpfunct6::VXCMP_VMSNE => BitVector::<6>::new(0b011001),
        vxcmpfunct6::VXCMP_VMSLTU => BitVector::<6>::new(0b011010),
        vxcmpfunct6::VXCMP_VMSLT => BitVector::<6>::new(0b011011),
        vxcmpfunct6::VXCMP_VMSLEU => BitVector::<6>::new(0b011100),
        vxcmpfunct6::VXCMP_VMSLE => BitVector::<6>::new(0b011101),
        vxcmpfunct6::VXCMP_VMSGTU => BitVector::<6>::new(0b011110),
        vxcmpfunct6::VXCMP_VMSGT => BitVector::<6>::new(0b011111),
    }
}
fn encdec_vxfunct6(v: vxfunct6) -> BitVector<6> {
    match v {
        vxfunct6::VX_VADD => BitVector::<6>::new(0b000000),
        vxfunct6::VX_VSUB => BitVector::<6>::new(0b000010),
        vxfunct6::VX_VRSUB => BitVector::<6>::new(0b000011),
        vxfunct6::VX_VMINU => BitVector::<6>::new(0b000100),
        vxfunct6::VX_VMIN => BitVector::<6>::new(0b000101),
        vxfunct6::VX_VMAXU => BitVector::<6>::new(0b000110),
        vxfunct6::VX_VMAX => BitVector::<6>::new(0b000111),
        vxfunct6::VX_VAND => BitVector::<6>::new(0b001001),
        vxfunct6::VX_VOR => BitVector::<6>::new(0b001010),
        vxfunct6::VX_VXOR => BitVector::<6>::new(0b001011),
        vxfunct6::VX_VSADDU => BitVector::<6>::new(0b100000),
        vxfunct6::VX_VSADD => BitVector::<6>::new(0b100001),
        vxfunct6::VX_VSSUBU => BitVector::<6>::new(0b100010),
        vxfunct6::VX_VSSUB => BitVector::<6>::new(0b100011),
        vxfunct6::VX_VSLL => BitVector::<6>::new(0b100101),
        vxfunct6::VX_VSMUL => BitVector::<6>::new(0b100111),
        vxfunct6::VX_VSRL => BitVector::<6>::new(0b101000),
        vxfunct6::VX_VSRA => BitVector::<6>::new(0b101001),
        vxfunct6::VX_VSSRL => BitVector::<6>::new(0b101010),
        vxfunct6::VX_VSSRA => BitVector::<6>::new(0b101011),
    }
}
fn encdec_vxmcfunct6(v: vxmcfunct6) -> BitVector<6> {
    match v {
        vxmcfunct6::VXMC_VMADC => BitVector::<6>::new(0b010001),
        vxmcfunct6::VXMC_VMSBC => BitVector::<6>::new(0b010011),
    }
}
fn encdec_vxmfunct6(v: vxmfunct6) -> BitVector<6> {
    match v {
        vxmfunct6::VXM_VMADC => BitVector::<6>::new(0b010001),
        vxmfunct6::VXM_VMSBC => BitVector::<6>::new(0b010011),
    }
}
fn encdec_vxmsfunct6(v: vxmsfunct6) -> BitVector<6> {
    match v {
        vxmsfunct6::VXMS_VADC => BitVector::<6>::new(0b010000),
        vxmsfunct6::VXMS_VSBC => BitVector::<6>::new(0b010010),
    }
}
fn encdec_vxsgfunct6(v: vxsgfunct6) -> BitVector<6> {
    match v {
        vxsgfunct6::VX_VSLIDEUP => BitVector::<6>::new(0b001110),
        vxsgfunct6::VX_VSLIDEDOWN => BitVector::<6>::new(0b001111),
        vxsgfunct6::VX_VRGATHER => BitVector::<6>::new(0b001100),
    }
}
fn encdec_wmvvfunct6(v: wmvvfunct6) -> BitVector<6> {
    match v {
        wmvvfunct6::WMVV_VWMACCU => BitVector::<6>::new(0b111100),
        wmvvfunct6::WMVV_VWMACC => BitVector::<6>::new(0b111101),
        wmvvfunct6::WMVV_VWMACCSU => BitVector::<6>::new(0b111111),
    }
}
fn encdec_wmvxfunct6(v: wmvxfunct6) -> BitVector<6> {
    match v {
        wmvxfunct6::WMVX_VWMACCU => BitVector::<6>::new(0b111100),
        wmvxfunct6::WMVX_VWMACC => BitVector::<6>::new(0b111101),
        wmvxfunct6::WMVX_VWMACCUS => BitVector::<6>::new(0b111110),
        wmvxfunct6::WMVX_VWMACCSU => BitVector::<6>::new(0b111111),
    }
}
fn encdec_wrsop(v: wrsop) -> BitVector<12> {
    match v {
        wrsop::WRS_STO => BitVector::<12>::new(0b000000011101),
        wrsop::WRS_NTO => BitVector::<12>::new(0b000000001101),
    }
}
fn encdec_wvfunct6(v: wvfunct6) -> BitVector<6> {
    match v {
        wvfunct6::WV_VADD => BitVector::<6>::new(0b110101),
        wvfunct6::WV_VSUB => BitVector::<6>::new(0b110111),
        wvfunct6::WV_VADDU => BitVector::<6>::new(0b110100),
        wvfunct6::WV_VSUBU => BitVector::<6>::new(0b110110),
    }
}
fn encdec_wvvfunct6(v: wvvfunct6) -> BitVector<6> {
    match v {
        wvvfunct6::WVV_VADD => BitVector::<6>::new(0b110001),
        wvvfunct6::WVV_VSUB => BitVector::<6>::new(0b110011),
        wvvfunct6::WVV_VADDU => BitVector::<6>::new(0b110000),
        wvvfunct6::WVV_VSUBU => BitVector::<6>::new(0b110010),
        wvvfunct6::WVV_VWMUL => BitVector::<6>::new(0b111011),
        wvvfunct6::WVV_VWMULU => BitVector::<6>::new(0b111000),
        wvvfunct6::WVV_VWMULSU => BitVector::<6>::new(0b111010),
    }
}
fn encdec_wvxfunct6(v: wvxfunct6) -> BitVector<6> {
    match v {
        wvxfunct6::WVX_VADD => BitVector::<6>::new(0b110001),
        wvxfunct6::WVX_VSUB => BitVector::<6>::new(0b110011),
        wvxfunct6::WVX_VADDU => BitVector::<6>::new(0b110000),
        wvxfunct6::WVX_VSUBU => BitVector::<6>::new(0b110010),
        wvxfunct6::WVX_VWMUL => BitVector::<6>::new(0b111011),
        wvxfunct6::WVX_VWMULU => BitVector::<6>::new(0b111000),
        wvxfunct6::WVX_VWMULSU => BitVector::<6>::new(0b111010),
    }
}
fn encdec_wxfunct6(v: wxfunct6) -> BitVector<6> {
    match v {
        wxfunct6::WX_VADD => BitVector::<6>::new(0b110101),
        wxfunct6::WX_VSUB => BitVector::<6>::new(0b110111),
        wxfunct6::WX_VADDU => BitVector::<6>::new(0b110100),
        wxfunct6::WX_VSUBU => BitVector::<6>::new(0b110110),
    }
}
fn encdec_zicondop(v: zicondop) -> BitVector<3> {
    match v {
        zicondop::CZERO_EQZ => BitVector::<3>::new(0b101),
        zicondop::CZERO_NEZ => BitVector::<3>::new(0b111),
    }
}
fn encdec_vaesdf(v: zvk_vaesdf_funct6) -> BitVector<6> {
    match v {
        zvk_vaesdf_funct6::ZVK_VAESDF_VV => BitVector::<6>::new(0b101000),
        zvk_vaesdf_funct6::ZVK_VAESDF_VS => BitVector::<6>::new(0b101001),
    }
}
fn encdec_vaesdm(v: zvk_vaesdm_funct6) -> BitVector<6> {
    match v {
        zvk_vaesdm_funct6::ZVK_VAESDM_VV => BitVector::<6>::new(0b101000),
        zvk_vaesdm_funct6::ZVK_VAESDM_VS => BitVector::<6>::new(0b101001),
    }
}
fn encdec_vaesef(v: zvk_vaesef_funct6) -> BitVector<6> {
    match v {
        zvk_vaesef_funct6::ZVK_VAESEF_VV => BitVector::<6>::new(0b101000),
        zvk_vaesef_funct6::ZVK_VAESEF_VS => BitVector::<6>::new(0b101001),
    }
}
fn encdec_vaesem(v: zvk_vaesem_funct6) -> BitVector<6> {
    match v {
        zvk_vaesem_funct6::ZVK_VAESEM_VV => BitVector::<6>::new(0b101000),
        zvk_vaesem_funct6::ZVK_VAESEM_VS => BitVector::<6>::new(0b101001),
    }
}
fn encdec_vsha2(v: zvk_vsha2_funct6) -> BitVector<6> {
    match v {
        zvk_vsha2_funct6::ZVK_VSHA2CH_VV => BitVector::<6>::new(0b101110),
        zvk_vsha2_funct6::ZVK_VSHA2CL_VV => BitVector::<6>::new(0b101111),
    }
}
fn encode_zicbop(cbop: cbop_zicbop, rs1: regidx, arg2: BitVector<12>) -> crate::Result<u32> {
    // deconstruction of `arg2`
    let actual = BitVector::<5>::from_subword(arg2.val >> 0);
    let expected = BitVector::<5>::new(0b00000);
    if actual != expected {
        return err!("bits [4:0] have to be 0b00000, got 0b{:05b}", actual.val);
    }
    let offset11_5 = BitVector::<7>::from_subword(arg2.val >> 5);
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b110_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_cbop_zicbop(cbop).val << 20)
        | (offset11_5.val << 25))
}
fn encode_ntl(op: ntl_type) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b000_u32 << 12)
        | (0b00000_u32 << 15)
        | (encdec_ntl(op).val << 20)
        | (0b0000000_u32 << 25))
}
fn encode_c_ntl(op: ntl_type) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b10_u32 << 0)
        | (encdec_ntl(op).val << 2)
        | (0b00000_u32 << 7)
        | (0b1_u32 << 12)
        | (0b100_u32 << 13))
}
fn encode_pause() -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0001111_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b000_u32 << 12)
        | (0b00000_u32 << 15)
        | (0b0000_u32 << 20)
        | (0b0001_u32 << 24)
        | (0b0000_u32 << 28))
}
fn encode_lpad(lpl: BitVector<20>) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010111_u32 << 0) | (0b00000_u32 << 7) | (lpl.val << 12))
}
fn encode_utype(imm: BitVector<20>, rd: regidx, op: uop) -> crate::Result<u32> {
    // instruction assembling
    Ok((encdec_uop(op).val << 0) | (encdec_reg(rd).val << 7) | (imm.val << 12))
}
fn encode_jalr(imm: BitVector<12>, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1100111_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (imm.val << 20))
}
fn encode_itype(imm: BitVector<12>, rs1: regidx, rd: regidx, op: iop) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (encdec_iop(op).val << 12)
        | (encdec_reg(rs1).val << 15)
        | (imm.val << 20))
}
fn encode_shiftiop(shamt: BitVector<6>, rs1: regidx, rd: regidx, arg3: sop) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        sop::SLLI => Ok((0b0010011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b001_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (shamt.val << 20)
            | (0b000000_u32 << 26)),
        sop::SRLI => Ok((0b0010011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b101_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (shamt.val << 20)
            | (0b000000_u32 << 26)),
        sop::SRAI => Ok((0b0010011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b101_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (shamt.val << 20)
            | (0b010000_u32 << 26)),
    }
}
fn encode_rtype(rs2: regidx, rs1: regidx, rd: regidx, arg3: rop) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        rop::ADD => Ok((0b0110011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b000_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (encdec_reg(rs2).val << 20)
            | (0b0000000_u32 << 25)),
        rop::SLT => Ok((0b0110011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b010_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (encdec_reg(rs2).val << 20)
            | (0b0000000_u32 << 25)),
        rop::SLTU => Ok((0b0110011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b011_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (encdec_reg(rs2).val << 20)
            | (0b0000000_u32 << 25)),
        rop::AND => Ok((0b0110011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b111_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (encdec_reg(rs2).val << 20)
            | (0b0000000_u32 << 25)),
        rop::OR => Ok((0b0110011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b110_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (encdec_reg(rs2).val << 20)
            | (0b0000000_u32 << 25)),
        rop::XOR => Ok((0b0110011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b100_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (encdec_reg(rs2).val << 20)
            | (0b0000000_u32 << 25)),
        rop::SLL => Ok((0b0110011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b001_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (encdec_reg(rs2).val << 20)
            | (0b0000000_u32 << 25)),
        rop::SRL => Ok((0b0110011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b101_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (encdec_reg(rs2).val << 20)
            | (0b0000000_u32 << 25)),
        rop::SUB => Ok((0b0110011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b000_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (encdec_reg(rs2).val << 20)
            | (0b0100000_u32 << 25)),
        rop::SRA => Ok((0b0110011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b101_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (encdec_reg(rs2).val << 20)
            | (0b0100000_u32 << 25)),
    }
}
fn encode_load(
    imm: BitVector<12>,
    rs1: regidx,
    rd: regidx,
    is_unsigned: bool,
    width: word_width,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0000011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (width_enc(width).val << 12)
        | (bool_bit(is_unsigned).val << 14)
        | (encdec_reg(rs1).val << 15)
        | (imm.val << 20))
}
fn encode_store(
    imm: BitVector<12>,
    rs2: regidx,
    rs1: regidx,
    width: word_width,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0100011_u32 << 0)
        | ((imm.val >> 4) & 0b1)
        | (width_enc(width).val << 8)
        | (0b0_u32 << 10)
        | (encdec_reg(rs1).val << 11)
        | (encdec_reg(rs2).val << 16)
        | ((imm.val >> 11) & 0b1))
}
fn encode_addiw(imm: BitVector<12>, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0011011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (imm.val << 20))
}
fn encode_rtypew(
    rs2: regidx,
    rs1: regidx,
    rd: regidx,
    arg3: ropw,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        ropw::ADDW => {
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b0111011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b000_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0000000_u32 << 25))
        }
        ropw::SUBW => {
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b0111011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b000_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0100000_u32 << 25))
        }
        ropw::SLLW => {
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b0111011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0000000_u32 << 25))
        }
        ropw::SRLW => {
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b0111011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b101_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0000000_u32 << 25))
        }
        ropw::SRAW => {
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b0111011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b101_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0100000_u32 << 25))
        }
    }
}
fn encode_shiftiwop(
    shamt: BitVector<5>,
    rs1: regidx,
    rd: regidx,
    arg3: sopw,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        sopw::SLLIW => {
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b0011011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (shamt.val << 20)
                | (0b0000000_u32 << 25))
        }
        sopw::SRLIW => {
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b0011011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b101_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (shamt.val << 20)
                | (0b0000000_u32 << 25))
        }
        sopw::SRAIW => {
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b0011011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b101_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (shamt.val << 20)
                | (0b0100000_u32 << 25))
        }
    }
}
fn encode_fence_tso() -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0001111_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b000_u32 << 12)
        | (0b00000_u32 << 15)
        | (0b0011_u32 << 20)
        | (0b0011_u32 << 24)
        | (0b1000_u32 << 28))
}
fn encode_ecall() -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110011_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b000_u32 << 12)
        | (0b00000_u32 << 15)
        | (0b000000000000_u32 << 20))
}
fn encode_mret() -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110011_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b000_u32 << 12)
        | (0b00000_u32 << 15)
        | (0b00010_u32 << 20)
        | (0b0011000_u32 << 25))
}
fn encode_sret() -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110011_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b000_u32 << 12)
        | (0b00000_u32 << 15)
        | (0b00010_u32 << 20)
        | (0b0001000_u32 << 25))
}
fn encode_ebreak() -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110011_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b000_u32 << 12)
        | (0b00000_u32 << 15)
        | (0b000000000001_u32 << 20))
}
fn encode_wfi() -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110011_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b000_u32 << 12)
        | (0b00000_u32 << 15)
        | (0b000100000101_u32 << 20))
}
fn encode_sfence_vma(rs1: regidx, rs2: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110011_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0001001_u32 << 25))
}
fn encode_amo(
    op: amoop,
    aq: bool,
    rl: bool,
    rs2: regidx,
    rs1: regidx,
    size: word_width_wide,
    rd: regidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0101111_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (width_enc_wide(size).val << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (bool_bit(rl).val << 25)
        | (bool_bit(aq).val << 26)
        | (encdec_amoop(op).val << 27))
}
fn encode_loadres(
    aq: bool,
    rl: bool,
    rs1: regidx,
    width: word_width,
    rd: regidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0101111_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (width_enc(width).val << 12)
        | (0b0_u32 << 14)
        | (encdec_reg(rs1).val << 15)
        | (0b00000_u32 << 20)
        | (bool_bit(rl).val << 25)
        | (bool_bit(aq).val << 26)
        | (0b00010_u32 << 27))
}
fn encode_storecon(
    aq: bool,
    rl: bool,
    rs2: regidx,
    rs1: regidx,
    width: word_width,
    rd: regidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0101111_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (width_enc(width).val << 12)
        | (0b0_u32 << 14)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (bool_bit(rl).val << 25)
        | (bool_bit(aq).val << 26)
        | (0b00011_u32 << 27))
}
fn encode_mul(rs2: regidx, rs1: regidx, rd: regidx, mul_op: mul_op) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (encdec_mul_op(mul_op).val << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0000001_u32 << 25))
}
fn encode_div(rs2: regidx, rs1: regidx, rd: regidx, is_unsigned: bool) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (bool_bit(is_unsigned).val << 12)
        | (0b10_u32 << 13)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0000001_u32 << 25))
}
fn encode_rem(rs2: regidx, rs1: regidx, rd: regidx, is_unsigned: bool) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (bool_bit(is_unsigned).val << 12)
        | (0b11_u32 << 13)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0000001_u32 << 25))
}
fn encode_mulw(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0111011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0000001_u32 << 25))
}
fn encode_divw(rs2: regidx, rs1: regidx, rd: regidx, is_unsigned: bool) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0111011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (bool_bit(is_unsigned).val << 12)
        | (0b10_u32 << 13)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0000001_u32 << 25))
}
fn encode_remw(rs2: regidx, rs1: regidx, rd: regidx, is_unsigned: bool) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0111011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (bool_bit(is_unsigned).val << 12)
        | (0b11_u32 << 13)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0000001_u32 << 25))
}
fn encode_slliuw(shamt: BitVector<6>, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0011011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (shamt.val << 20)
        | (0b000010_u32 << 26))
}
fn encode_zba_rtypeuw(
    rs2: regidx,
    rs1: regidx,
    rd: regidx,
    shamt: shamt_zba,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match shamt {
        BitVector::<2> { val: 0b00 } => {
            if !(ctx.extensions.Ext_Zba) {
                return err!("requried extension Zba");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b0111011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b0_u32 << 12)
                | (0b00_u32 << 13)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0000100_u32 << 25))
        }
        _ => {
            if !(ctx.extensions.Ext_Zba) {
                return err!("requried extension Zba");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b0111011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b0_u32 << 12)
                | (shamt.val << 13)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0010000_u32 << 25))
        }
    }
}
fn encode_zba_rtype(rs2: regidx, rs1: regidx, rd: regidx, shamt: shamt_zba) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b0_u32 << 12)
        | (shamt.val << 13)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0010000_u32 << 25))
}
fn encode_roriw(shamt: BitVector<5>, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0011011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (shamt.val << 20)
        | (0b0110000_u32 << 25))
}
fn encode_rori(shamt: BitVector<6>, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (shamt.val << 20)
        | (0b011000_u32 << 26))
}
fn encode_zbb_rtypew(
    rs2: regidx,
    rs1: regidx,
    rd: regidx,
    arg3: bropw_zbb,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        bropw_zbb::ROLW => {
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b0111011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0110000_u32 << 25))
        }
        bropw_zbb::RORW => {
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b0111011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b101_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0110000_u32 << 25))
        }
    }
}
fn encode_zbb_rtype(
    rs2: regidx,
    rs1: regidx,
    rd: regidx,
    arg3: brop_zbb,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        brop_zbb::ANDN => Ok((0b0110011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b111_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (encdec_reg(rs2).val << 20)
            | (0b0100000_u32 << 25)),
        brop_zbb::ORN => Ok((0b0110011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b110_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (encdec_reg(rs2).val << 20)
            | (0b0100000_u32 << 25)),
        brop_zbb::XNOR => Ok((0b0110011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b100_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (encdec_reg(rs2).val << 20)
            | (0b0100000_u32 << 25)),
        brop_zbb::MAX => {
            if !(ctx.extensions.Ext_Zbb) {
                return err!("requried extension Zbb");
            }

            Ok((0b0110011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b110_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0000101_u32 << 25))
        }
        brop_zbb::MAXU => {
            if !(ctx.extensions.Ext_Zbb) {
                return err!("requried extension Zbb");
            }

            Ok((0b0110011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b111_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0000101_u32 << 25))
        }
        brop_zbb::MIN => {
            if !(ctx.extensions.Ext_Zbb) {
                return err!("requried extension Zbb");
            }

            Ok((0b0110011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b100_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0000101_u32 << 25))
        }
        brop_zbb::MINU => {
            if !(ctx.extensions.Ext_Zbb) {
                return err!("requried extension Zbb");
            }

            Ok((0b0110011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b101_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0000101_u32 << 25))
        }
        brop_zbb::ROL => Ok((0b0110011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b001_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (encdec_reg(rs2).val << 20)
            | (0b0110000_u32 << 25)),
        brop_zbb::ROR => Ok((0b0110011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b101_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (encdec_reg(rs2).val << 20)
            | (0b0110000_u32 << 25)),
    }
}
fn encode_zbb_extop(rs1: regidx, rd: regidx, arg2: extop_zbb, ctx: &Context) -> crate::Result<u32> {
    // instruction assembling
    match arg2 {
        extop_zbb::SEXTB => {
            if !(ctx.extensions.Ext_Zbb) {
                return err!("requried extension Zbb");
            }

            Ok((0b0010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00100_u32 << 20)
                | (0b0110000_u32 << 25))
        }
        extop_zbb::SEXTH => {
            if !(ctx.extensions.Ext_Zbb) {
                return err!("requried extension Zbb");
            }

            Ok((0b0010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00101_u32 << 20)
                | (0b0110000_u32 << 25))
        }
        extop_zbb::ZEXTH => {
            if !(ctx.extensions.Ext_Zbb) {
                return err!("requried extension Zbb");
            }
            if !(ctx.is_32bit()) {
                return err!("available only in 32-bit mode");
            }

            Ok((0b0110011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b100_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b0000100_u32 << 25))
        }
        extop_zbb::ZEXTH => {
            if !(ctx.extensions.Ext_Zbb) {
                return err!("requried extension Zbb");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b0111011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b100_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b0000100_u32 << 25))
        }
    }
}
fn encode_rev8(rs1: regidx, rd: regidx, ctx: &Context) -> crate::Result<u32> {
    // instruction assembling
    if ctx.is_32bit() {
        return Ok((0b0010011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b101_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (0b011010011000_u32 << 20));
    }
    if ctx.is_64bit() {
        return Ok((0b0010011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b101_u32 << 12)
            | (encdec_reg(rs1).val << 15)
            | (0b011010111000_u32 << 20));
    }
    err!("instruction is available conditionally, none of conditions met")
}
fn encode_orcb(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b001010000111_u32 << 20))
}
fn encode_cpop(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00010_u32 << 20)
        | (0b0110000_u32 << 25))
}
fn encode_cpopw(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0011011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00010_u32 << 20)
        | (0b0110000_u32 << 25))
}
fn encode_clz(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00000_u32 << 20)
        | (0b0110000_u32 << 25))
}
fn encode_clzw(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0011011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00000_u32 << 20)
        | (0b0110000_u32 << 25))
}
fn encode_ctz(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00001_u32 << 20)
        | (0b0110000_u32 << 25))
}
fn encode_ctzw(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0011011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00001_u32 << 20)
        | (0b0110000_u32 << 25))
}
fn encode_clmul(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0000101_u32 << 25))
}
fn encode_clmulh(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b011_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0000101_u32 << 25))
}
fn encode_clmulr(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0000101_u32 << 25))
}
fn encode_zbs_iop(
    shamt: BitVector<6>,
    rs1: regidx,
    rd: regidx,
    arg3: biop_zbs,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        biop_zbs::BCLRI => {
            if !(ctx.extensions.Ext_Zbs) {
                return err!("requried extension Zbs");
            }

            Ok((0b0010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (shamt.val << 20)
                | (0b010010_u32 << 26))
        }
        biop_zbs::BEXTI => {
            if !(ctx.extensions.Ext_Zbs) {
                return err!("requried extension Zbs");
            }

            Ok((0b0010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b101_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (shamt.val << 20)
                | (0b010010_u32 << 26))
        }
        biop_zbs::BINVI => {
            if !(ctx.extensions.Ext_Zbs) {
                return err!("requried extension Zbs");
            }

            Ok((0b0010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (shamt.val << 20)
                | (0b011010_u32 << 26))
        }
        biop_zbs::BSETI => {
            if !(ctx.extensions.Ext_Zbs) {
                return err!("requried extension Zbs");
            }

            Ok((0b0010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (shamt.val << 20)
                | (0b001010_u32 << 26))
        }
    }
}
fn encode_zbs_rtype(
    rs2: regidx,
    rs1: regidx,
    rd: regidx,
    arg3: brop_zbs,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        brop_zbs::BCLR => {
            if !(ctx.extensions.Ext_Zbs) {
                return err!("requried extension Zbs");
            }

            Ok((0b0110011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0100100_u32 << 25))
        }
        brop_zbs::BEXT => {
            if !(ctx.extensions.Ext_Zbs) {
                return err!("requried extension Zbs");
            }

            Ok((0b0110011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b101_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0100100_u32 << 25))
        }
        brop_zbs::BINV => {
            if !(ctx.extensions.Ext_Zbs) {
                return err!("requried extension Zbs");
            }

            Ok((0b0110011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0110100_u32 << 25))
        }
        brop_zbs::BSET => {
            if !(ctx.extensions.Ext_Zbs) {
                return err!("requried extension Zbs");
            }

            Ok((0b0110011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0010100_u32 << 25))
        }
    }
}
fn encode_c_addi4spn(rd: cregidx, nzimm: BitVector<8>) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b00_u32 << 0)
        | (encdec_creg(rd).val << 2)
        | ((nzimm.val >> 1) & 0b1)
        | ((nzimm.val >> 0) & 0b1)
        | ((nzimm.val >> 7) & 0b1)
        | ((nzimm.val >> 3) & 0b1)
        | (0b000_u32 << 9))
}
fn encode_c_lw(uimm: BitVector<5>, rs1: cregidx, rd: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b00_u32 << 0)
        | (encdec_creg(rd).val << 2)
        | ((uimm.val >> 4) & 0b1)
        | ((uimm.val >> 0) & 0b1)
        | (encdec_creg(rs1).val << 7)
        | ((uimm.val >> 3) & 0b1)
        | (0b010_u32 << 11))
}
fn encode_c_ld(uimm: BitVector<5>, rs1: cregidx, rd: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b00_u32 << 0)
        | (encdec_creg(rd).val << 2)
        | ((uimm.val >> 4) & 0b1)
        | (encdec_creg(rs1).val << 6)
        | ((uimm.val >> 2) & 0b1)
        | (0b011_u32 << 10))
}
fn encode_c_sw(uimm: BitVector<5>, rs1: cregidx, rs2: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b00_u32 << 0)
        | (encdec_creg(rs2).val << 2)
        | ((uimm.val >> 4) & 0b1)
        | ((uimm.val >> 0) & 0b1)
        | (encdec_creg(rs1).val << 7)
        | ((uimm.val >> 3) & 0b1)
        | (0b110_u32 << 11))
}
fn encode_c_sd(uimm: BitVector<5>, rs1: cregidx, rs2: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b00_u32 << 0)
        | (encdec_creg(rs2).val << 2)
        | ((uimm.val >> 4) & 0b1)
        | (encdec_creg(rs1).val << 6)
        | ((uimm.val >> 2) & 0b1)
        | (0b111_u32 << 10))
}
fn encode_c_addi(imm: BitVector<6>, rsd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | ((imm.val >> 4) & 0b1)
        | (encdec_reg(rsd).val << 3)
        | ((imm.val >> 5) & 0b1)
        | (0b000_u32 << 9))
}
fn encode_c_jal(imm: BitVector<11>) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | ((imm.val >> 4) & 0b1)
        | ((imm.val >> 2) & 0b1)
        | ((imm.val >> 6) & 0b1)
        | ((imm.val >> 5) & 0b1)
        | ((imm.val >> 9) & 0b1)
        | ((imm.val >> 8) & 0b1)
        | ((imm.val >> 3) & 0b1)
        | ((imm.val >> 10) & 0b1)
        | (0b001_u32 << 10))
}
fn encode_c_addiw(imm: BitVector<6>, rsd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | ((imm.val >> 4) & 0b1)
        | (encdec_reg(rsd).val << 3)
        | ((imm.val >> 5) & 0b1)
        | (0b001_u32 << 9))
}
fn encode_c_li(imm: BitVector<6>, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | ((imm.val >> 4) & 0b1)
        | (encdec_reg(rd).val << 3)
        | ((imm.val >> 5) & 0b1)
        | (0b010_u32 << 9))
}
fn encode_c_addi16sp(nzimm: BitVector<6>) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | ((nzimm.val >> 1) & 0b1)
        | ((nzimm.val >> 4) & 0b1)
        | ((nzimm.val >> 2) & 0b1)
        | ((nzimm.val >> 0) & 0b1)
        | (0b00010_u32 << 6)
        | ((nzimm.val >> 5) & 0b1)
        | (0b011_u32 << 12))
}
fn encode_c_lui(imm: BitVector<6>, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | ((imm.val >> 4) & 0b1)
        | (encdec_reg(rd).val << 3)
        | ((imm.val >> 5) & 0b1)
        | (0b011_u32 << 9))
}
fn encode_c_srli(shamt: BitVector<6>, rsd: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | ((shamt.val >> 4) & 0b1)
        | (encdec_creg(rsd).val << 3)
        | (0b00_u32 << 6)
        | ((shamt.val >> 5) & 0b1)
        | (0b100_u32 << 9))
}
fn encode_c_srai(shamt: BitVector<6>, rsd: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | ((shamt.val >> 4) & 0b1)
        | (encdec_creg(rsd).val << 3)
        | (0b01_u32 << 6)
        | ((shamt.val >> 5) & 0b1)
        | (0b100_u32 << 9))
}
fn encode_c_andi(imm: BitVector<6>, rsd: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | ((imm.val >> 4) & 0b1)
        | (encdec_creg(rsd).val << 3)
        | (0b10_u32 << 6)
        | ((imm.val >> 5) & 0b1)
        | (0b100_u32 << 9))
}
fn encode_c_sub(rsd: cregidx, rs2: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | (encdec_creg(rs2).val << 2)
        | (0b00_u32 << 5)
        | (encdec_creg(rsd).val << 7)
        | (0b11_u32 << 10)
        | (0b0_u32 << 12)
        | (0b100_u32 << 13))
}
fn encode_c_xor(rsd: cregidx, rs2: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | (encdec_creg(rs2).val << 2)
        | (0b01_u32 << 5)
        | (encdec_creg(rsd).val << 7)
        | (0b11_u32 << 10)
        | (0b0_u32 << 12)
        | (0b100_u32 << 13))
}
fn encode_c_or(rsd: cregidx, rs2: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | (encdec_creg(rs2).val << 2)
        | (0b10_u32 << 5)
        | (encdec_creg(rsd).val << 7)
        | (0b11_u32 << 10)
        | (0b0_u32 << 12)
        | (0b100_u32 << 13))
}
fn encode_c_and(rsd: cregidx, rs2: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | (encdec_creg(rs2).val << 2)
        | (0b11_u32 << 5)
        | (encdec_creg(rsd).val << 7)
        | (0b11_u32 << 10)
        | (0b0_u32 << 12)
        | (0b100_u32 << 13))
}
fn encode_c_subw(rsd: cregidx, rs2: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | (encdec_creg(rs2).val << 2)
        | (0b00_u32 << 5)
        | (encdec_creg(rsd).val << 7)
        | (0b11_u32 << 10)
        | (0b1_u32 << 12)
        | (0b100_u32 << 13))
}
fn encode_c_addw(rsd: cregidx, rs2: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | (encdec_creg(rs2).val << 2)
        | (0b01_u32 << 5)
        | (encdec_creg(rsd).val << 7)
        | (0b11_u32 << 10)
        | (0b1_u32 << 12)
        | (0b100_u32 << 13))
}
fn encode_c_j(imm: BitVector<11>) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | ((imm.val >> 4) & 0b1)
        | ((imm.val >> 2) & 0b1)
        | ((imm.val >> 6) & 0b1)
        | ((imm.val >> 5) & 0b1)
        | ((imm.val >> 9) & 0b1)
        | ((imm.val >> 8) & 0b1)
        | ((imm.val >> 3) & 0b1)
        | ((imm.val >> 10) & 0b1)
        | (0b101_u32 << 10))
}
fn encode_c_beqz(imm: BitVector<8>, rs: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | ((imm.val >> 4) & 0b1)
        | ((imm.val >> 1) & 0b1)
        | ((imm.val >> 6) & 0b1)
        | (encdec_creg(rs).val << 5)
        | ((imm.val >> 3) & 0b1)
        | ((imm.val >> 7) & 0b1)
        | (0b110_u32 << 10))
}
fn encode_c_bnez(imm: BitVector<8>, rs: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | ((imm.val >> 4) & 0b1)
        | ((imm.val >> 1) & 0b1)
        | ((imm.val >> 6) & 0b1)
        | (encdec_creg(rs).val << 5)
        | ((imm.val >> 3) & 0b1)
        | ((imm.val >> 7) & 0b1)
        | (0b111_u32 << 10))
}
fn encode_c_slli(shamt: BitVector<6>, rsd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b10_u32 << 0)
        | ((shamt.val >> 4) & 0b1)
        | (encdec_reg(rsd).val << 3)
        | ((shamt.val >> 5) & 0b1)
        | (0b000_u32 << 9))
}
fn encode_c_lwsp(uimm: BitVector<6>, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b10_u32 << 0)
        | ((uimm.val >> 5) & 0b1)
        | ((uimm.val >> 2) & 0b1)
        | (encdec_reg(rd).val << 4)
        | ((uimm.val >> 3) & 0b1)
        | (0b010_u32 << 10))
}
fn encode_c_ldsp(uimm: BitVector<6>, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b10_u32 << 0)
        | ((uimm.val >> 5) & 0b1)
        | ((uimm.val >> 1) & 0b1)
        | (encdec_reg(rd).val << 4)
        | ((uimm.val >> 2) & 0b1)
        | (0b011_u32 << 10))
}
fn encode_c_swsp(uimm: BitVector<6>, rs2: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b10_u32 << 0)
        | (encdec_reg(rs2).val << 2)
        | ((uimm.val >> 5) & 0b1)
        | ((uimm.val >> 3) & 0b1)
        | (0b110_u32 << 9))
}
fn encode_c_sdsp(uimm: BitVector<6>, rs2: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b10_u32 << 0)
        | (encdec_reg(rs2).val << 2)
        | ((uimm.val >> 5) & 0b1)
        | ((uimm.val >> 2) & 0b1)
        | (0b111_u32 << 9))
}
fn encode_c_jr(rs1: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b10_u32 << 0)
        | (0b00000_u32 << 2)
        | (encdec_reg(rs1).val << 7)
        | (0b0_u32 << 12)
        | (0b100_u32 << 13))
}
fn encode_c_jalr(rs1: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b10_u32 << 0)
        | (0b00000_u32 << 2)
        | (encdec_reg(rs1).val << 7)
        | (0b1_u32 << 12)
        | (0b100_u32 << 13))
}
fn encode_c_mv(rd: regidx, rs2: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b10_u32 << 0)
        | (encdec_reg(rs2).val << 2)
        | (encdec_reg(rd).val << 7)
        | (0b0_u32 << 12)
        | (0b100_u32 << 13))
}
fn encode_c_ebreak() -> crate::Result<u32> {
    // instruction assembling
    Ok((0b10_u32 << 0) | (0b0000000000_u32 << 2) | (0b1001_u32 << 12))
}
fn encode_c_add(rsd: regidx, rs2: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b10_u32 << 0)
        | (encdec_reg(rs2).val << 2)
        | (encdec_reg(rsd).val << 7)
        | (0b1_u32 << 12)
        | (0b100_u32 << 13))
}
fn encode_c_lbu(uimm: BitVector<2>, rdc: cregidx, rsc1: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b00_u32 << 0)
        | (encdec_creg(rdc).val << 2)
        | ((uimm.val >> 1) & 0b1)
        | ((uimm.val >> 0) & 0b1)
        | (encdec_creg(rsc1).val << 7)
        | (0b000_u32 << 10)
        | (0b100_u32 << 13))
}
fn encode_c_lhu(arg0: BitVector<2>, rdc: cregidx, rsc1: cregidx) -> crate::Result<u32> {
    // deconstruction of `arg0`
    let actual = BitVector::<1>::from_subword(arg0.val >> 0);
    let expected = BitVector::<1>::new(0b0);
    if actual != expected {
        return err!("bits [0:0] have to be 0b0, got 0b{:01b}", actual.val);
    }
    let uimm1 = BitVector::<1>::from_subword(arg0.val >> 1);
    // instruction assembling
    Ok((0b00_u32 << 0)
        | (encdec_creg(rdc).val << 2)
        | (uimm1.val << 5)
        | (0b0_u32 << 6)
        | (encdec_creg(rsc1).val << 7)
        | (0b001_u32 << 10)
        | (0b100_u32 << 13))
}
fn encode_c_lh(arg0: BitVector<2>, rdc: cregidx, rsc1: cregidx) -> crate::Result<u32> {
    // deconstruction of `arg0`
    let actual = BitVector::<1>::from_subword(arg0.val >> 0);
    let expected = BitVector::<1>::new(0b0);
    if actual != expected {
        return err!("bits [0:0] have to be 0b0, got 0b{:01b}", actual.val);
    }
    let uimm1 = BitVector::<1>::from_subword(arg0.val >> 1);
    // instruction assembling
    Ok((0b00_u32 << 0)
        | (encdec_creg(rdc).val << 2)
        | (uimm1.val << 5)
        | (0b1_u32 << 6)
        | (encdec_creg(rsc1).val << 7)
        | (0b001_u32 << 10)
        | (0b100_u32 << 13))
}
fn encode_c_sb(uimm: BitVector<2>, rsc1: cregidx, rsc2: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b00_u32 << 0)
        | (encdec_creg(rsc2).val << 2)
        | ((uimm.val >> 1) & 0b1)
        | ((uimm.val >> 0) & 0b1)
        | (encdec_creg(rsc1).val << 7)
        | (0b010_u32 << 10)
        | (0b100_u32 << 13))
}
fn encode_c_sh(arg0: BitVector<2>, rsc1: cregidx, rsc2: cregidx) -> crate::Result<u32> {
    // deconstruction of `arg0`
    let actual = BitVector::<1>::from_subword(arg0.val >> 0);
    let expected = BitVector::<1>::new(0b0);
    if actual != expected {
        return err!("bits [0:0] have to be 0b0, got 0b{:01b}", actual.val);
    }
    let uimm1 = BitVector::<1>::from_subword(arg0.val >> 1);
    // instruction assembling
    Ok((0b00_u32 << 0)
        | (encdec_creg(rsc2).val << 2)
        | (uimm1.val << 5)
        | (0b0_u32 << 6)
        | (encdec_creg(rsc1).val << 7)
        | (0b011_u32 << 10)
        | (0b100_u32 << 13))
}
fn encode_c_zext_b(rsdc: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | (0b000_u32 << 2)
        | (0b11_u32 << 5)
        | (encdec_creg(rsdc).val << 7)
        | (0b111_u32 << 10)
        | (0b100_u32 << 13))
}
fn encode_c_sext_b(rsdc: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | (0b001_u32 << 2)
        | (0b11_u32 << 5)
        | (encdec_creg(rsdc).val << 7)
        | (0b111_u32 << 10)
        | (0b100_u32 << 13))
}
fn encode_c_zext_h(rsdc: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | (0b010_u32 << 2)
        | (0b11_u32 << 5)
        | (encdec_creg(rsdc).val << 7)
        | (0b111_u32 << 10)
        | (0b100_u32 << 13))
}
fn encode_c_sext_h(rsdc: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | (0b011_u32 << 2)
        | (0b11_u32 << 5)
        | (encdec_creg(rsdc).val << 7)
        | (0b111_u32 << 10)
        | (0b100_u32 << 13))
}
fn encode_c_zext_w(rsdc: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | (0b100_u32 << 2)
        | (0b11_u32 << 5)
        | (encdec_creg(rsdc).val << 7)
        | (0b111_u32 << 10)
        | (0b100_u32 << 13))
}
fn encode_c_not(rsdc: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | (0b101_u32 << 2)
        | (0b11_u32 << 5)
        | (encdec_creg(rsdc).val << 7)
        | (0b111_u32 << 10)
        | (0b100_u32 << 13))
}
fn encode_c_mul(rsdc: cregidx, rsc2: cregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | (encdec_creg(rsc2).val << 2)
        | (0b10_u32 << 5)
        | (encdec_creg(rsdc).val << 7)
        | (0b111_u32 << 10)
        | (0b100_u32 << 13))
}
fn encode_load_fp(
    imm: BitVector<12>,
    rs1: regidx,
    rd: fregidx,
    width: word_width,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0000111_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (width_enc(width).val << 12)
        | (0b0_u32 << 14)
        | (encdec_reg(rs1).val << 15)
        | (imm.val << 20))
}
fn encode_store_fp(
    imm: BitVector<12>,
    rs2: fregidx,
    rs1: regidx,
    width: word_width,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0100111_u32 << 0)
        | ((imm.val >> 4) & 0b1)
        | (width_enc(width).val << 8)
        | (0b0_u32 << 10)
        | (encdec_reg(rs1).val << 11)
        | (encdec_freg(rs2).val << 16)
        | ((imm.val >> 11) & 0b1))
}
fn encode_f_madd_type_s(
    rs3: fregidx,
    rs2: fregidx,
    rs1: fregidx,
    rm: rounding_mode,
    rd: fregidx,
    arg5: f_madd_op_S,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg5 {
        f_madd_op_S::FMADD_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1000011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b00_u32 << 25)
                | (encdec_freg(rs3).val << 27))
        }
        f_madd_op_S::FMSUB_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1000111_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b00_u32 << 25)
                | (encdec_freg(rs3).val << 27))
        }
        f_madd_op_S::FNMSUB_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1001011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b00_u32 << 25)
                | (encdec_freg(rs3).val << 27))
        }
        f_madd_op_S::FNMADD_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1001111_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b00_u32 << 25)
                | (encdec_freg(rs3).val << 27))
        }
    }
}
fn encode_f_bin_rm_type_s(
    rs2: fregidx,
    rs1: fregidx,
    rm: rounding_mode,
    rd: fregidx,
    arg4: f_bin_rm_op_S,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg4 {
        f_bin_rm_op_S::FADD_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0000000_u32 << 25))
        }
        f_bin_rm_op_S::FSUB_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0000100_u32 << 25))
        }
        f_bin_rm_op_S::FMUL_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0001000_u32 << 25))
        }
        f_bin_rm_op_S::FDIV_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0001100_u32 << 25))
        }
    }
}
fn encode_f_un_rm_ff_type_s(
    rs1: fregidx,
    rm: rounding_mode,
    rd: fregidx,
    arg3: f_un_rm_ff_op_S,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (encdec_rounding_mode(rm).val << 12)
        | (encdec_freg(rs1).val << 15)
        | (0b00000_u32 << 20)
        | (0b0101100_u32 << 25))
}
fn encode_f_un_rm_fx_type_s(
    rs1: fregidx,
    rm: rounding_mode,
    rd: regidx,
    arg3: f_un_rm_fx_op_S,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        f_un_rm_fx_op_S::FCVT_W_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b1100000_u32 << 25))
        }
        f_un_rm_fx_op_S::FCVT_WU_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00001_u32 << 20)
                | (0b1100000_u32 << 25))
        }
        f_un_rm_fx_op_S::FCVT_L_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00010_u32 << 20)
                | (0b1100000_u32 << 25))
        }
        f_un_rm_fx_op_S::FCVT_LU_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00011_u32 << 20)
                | (0b1100000_u32 << 25))
        }
    }
}
fn encode_f_un_rm_xf_type_s(
    rs1: regidx,
    rm: rounding_mode,
    rd: fregidx,
    arg3: f_un_rm_xf_op_S,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        f_un_rm_xf_op_S::FCVT_S_W => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b1101000_u32 << 25))
        }
        f_un_rm_xf_op_S::FCVT_S_WU => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00001_u32 << 20)
                | (0b1101000_u32 << 25))
        }
        f_un_rm_xf_op_S::FCVT_S_L => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00010_u32 << 20)
                | (0b1101000_u32 << 25))
        }
        f_un_rm_xf_op_S::FCVT_S_LU => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00011_u32 << 20)
                | (0b1101000_u32 << 25))
        }
    }
}
fn encode_f_bin_type_f_s(
    rs2: fregidx,
    rs1: fregidx,
    rd: fregidx,
    arg3: f_bin_op_f_S,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        f_bin_op_f_S::FSGNJ_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (0b000_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0010000_u32 << 25))
        }
        f_bin_op_f_S::FSGNJN_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0010000_u32 << 25))
        }
        f_bin_op_f_S::FSGNJX_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (0b010_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0010000_u32 << 25))
        }
        f_bin_op_f_S::FMIN_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (0b000_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0010100_u32 << 25))
        }
        f_bin_op_f_S::FMAX_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0010100_u32 << 25))
        }
    }
}
fn encode_f_bin_type_x_s(
    rs2: fregidx,
    rs1: fregidx,
    rd: regidx,
    arg3: f_bin_op_x_S,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        f_bin_op_x_S::FEQ_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b010_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b1010000_u32 << 25))
        }
        f_bin_op_x_S::FLT_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b1010000_u32 << 25))
        }
        f_bin_op_x_S::FLE_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b000_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b1010000_u32 << 25))
        }
    }
}
fn encode_f_un_type_x_s(
    rs1: fregidx,
    rd: regidx,
    arg2: f_un_op_x_S,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg2 {
        f_un_op_x_S::FCLASS_S => {
            if !(ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx) {
                return err!("required extension F or Zfinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b1110000_u32 << 25))
        }
        f_un_op_x_S::FMV_X_W => {
            if !(ctx.extensions.Ext_F) {
                return err!("requried extension F");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b000_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b1110000_u32 << 25))
        }
    }
}
fn encode_f_un_type_f_s(rs1: regidx, rd: fregidx, arg2: f_un_op_f_S) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00000_u32 << 20)
        | (0b1111000_u32 << 25))
}
fn encode_c_flwsp(uimm: BitVector<6>, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b10_u32 << 0)
        | ((uimm.val >> 5) & 0b1)
        | ((uimm.val >> 2) & 0b1)
        | (encdec_freg(rd).val << 4)
        | ((uimm.val >> 3) & 0b1)
        | (0b011_u32 << 10))
}
fn encode_c_fswsp(uimm: BitVector<6>, rs2: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b10_u32 << 0)
        | (encdec_freg(rs2).val << 2)
        | ((uimm.val >> 5) & 0b1)
        | ((uimm.val >> 3) & 0b1)
        | (0b111_u32 << 9))
}
fn encode_c_flw(uimm: BitVector<5>, rs1: cregidx, rd: cfregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b00_u32 << 0)
        | (encdec_cfreg(rd).val << 2)
        | ((uimm.val >> 4) & 0b1)
        | ((uimm.val >> 0) & 0b1)
        | (encdec_creg(rs1).val << 7)
        | ((uimm.val >> 3) & 0b1)
        | (0b011_u32 << 11))
}
fn encode_c_fsw(uimm: BitVector<5>, rs1: cregidx, rs2: cfregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b00_u32 << 0)
        | (encdec_cfreg(rs2).val << 2)
        | ((uimm.val >> 4) & 0b1)
        | ((uimm.val >> 0) & 0b1)
        | (encdec_creg(rs1).val << 7)
        | ((uimm.val >> 3) & 0b1)
        | (0b111_u32 << 11))
}
fn encode_f_madd_type_d(
    rs3: fregidx,
    rs2: fregidx,
    rs1: fregidx,
    rm: rounding_mode,
    rd: fregidx,
    arg5: f_madd_op_D,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg5 {
        f_madd_op_D::FMADD_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1000011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b01_u32 << 25)
                | (encdec_freg(rs3).val << 27))
        }
        f_madd_op_D::FMSUB_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1000111_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b01_u32 << 25)
                | (encdec_freg(rs3).val << 27))
        }
        f_madd_op_D::FNMSUB_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1001011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b01_u32 << 25)
                | (encdec_freg(rs3).val << 27))
        }
        f_madd_op_D::FNMADD_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1001111_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b01_u32 << 25)
                | (encdec_freg(rs3).val << 27))
        }
    }
}
fn encode_f_bin_rm_type_d(
    rs2: fregidx,
    rs1: fregidx,
    rm: rounding_mode,
    rd: fregidx,
    arg4: f_bin_rm_op_D,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg4 {
        f_bin_rm_op_D::FADD_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0000001_u32 << 25))
        }
        f_bin_rm_op_D::FSUB_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0000101_u32 << 25))
        }
        f_bin_rm_op_D::FMUL_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0001001_u32 << 25))
        }
        f_bin_rm_op_D::FDIV_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0001101_u32 << 25))
        }
    }
}
fn encode_f_un_rm_ff_type_d(
    rs1: fregidx,
    rm: rounding_mode,
    rd: fregidx,
    arg3: f_un_rm_ff_op_D,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        f_un_rm_ff_op_D::FSQRT_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b0101101_u32 << 25))
        }
        f_un_rm_ff_op_D::FCVT_S_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00001_u32 << 20)
                | (0b0100000_u32 << 25))
        }
        f_un_rm_ff_op_D::FCVT_D_S => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b0100001_u32 << 25))
        }
    }
}
fn encode_f_un_rm_fx_type_d(
    rs1: fregidx,
    rm: rounding_mode,
    rd: regidx,
    arg3: f_un_rm_fx_op_D,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        f_un_rm_fx_op_D::FCVT_W_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b1100001_u32 << 25))
        }
        f_un_rm_fx_op_D::FCVT_WU_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00001_u32 << 20)
                | (0b1100001_u32 << 25))
        }
        f_un_rm_fx_op_D::FCVT_L_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00010_u32 << 20)
                | (0b1100001_u32 << 25))
        }
        f_un_rm_fx_op_D::FCVT_LU_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00011_u32 << 20)
                | (0b1100001_u32 << 25))
        }
    }
}
fn encode_f_un_rm_xf_type_d(
    rs1: regidx,
    rm: rounding_mode,
    rd: fregidx,
    arg3: f_un_rm_xf_op_D,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        f_un_rm_xf_op_D::FCVT_D_W => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b1101001_u32 << 25))
        }
        f_un_rm_xf_op_D::FCVT_D_WU => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00001_u32 << 20)
                | (0b1101001_u32 << 25))
        }
        f_un_rm_xf_op_D::FCVT_D_L => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00010_u32 << 20)
                | (0b1101001_u32 << 25))
        }
        f_un_rm_xf_op_D::FCVT_D_LU => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00011_u32 << 20)
                | (0b1101001_u32 << 25))
        }
    }
}
fn encode_f_bin_f_type_d(
    rs2: fregidx,
    rs1: fregidx,
    rd: fregidx,
    arg3: f_bin_f_op_D,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        f_bin_f_op_D::FSGNJ_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (0b000_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0010001_u32 << 25))
        }
        f_bin_f_op_D::FSGNJN_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0010001_u32 << 25))
        }
        f_bin_f_op_D::FSGNJX_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (0b010_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0010001_u32 << 25))
        }
        f_bin_f_op_D::FMIN_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (0b000_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0010101_u32 << 25))
        }
        f_bin_f_op_D::FMAX_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0010101_u32 << 25))
        }
    }
}
fn encode_f_bin_x_type_d(
    rs2: fregidx,
    rs1: fregidx,
    rd: regidx,
    arg3: f_bin_x_op_D,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        f_bin_x_op_D::FEQ_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b010_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b1010001_u32 << 25))
        }
        f_bin_x_op_D::FLT_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b1010001_u32 << 25))
        }
        f_bin_x_op_D::FLE_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b000_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b1010001_u32 << 25))
        }
    }
}
fn encode_f_un_x_type_d(
    rs1: fregidx,
    rd: regidx,
    arg2: f_un_x_op_D,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg2 {
        f_un_x_op_D::FCLASS_D => {
            if !(ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx) {
                return err!("required extension D or Zdinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b1110001_u32 << 25))
        }
        f_un_x_op_D::FMV_X_D => {
            if !(ctx.extensions.Ext_D) {
                return err!("requried extension D");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b000_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b1110001_u32 << 25))
        }
    }
}
fn encode_f_un_f_type_d(rs1: regidx, rd: fregidx, arg2: f_un_f_op_D) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00000_u32 << 20)
        | (0b1111001_u32 << 25))
}
fn encode_c_fldsp(uimm: BitVector<6>, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b10_u32 << 0)
        | ((uimm.val >> 5) & 0b1)
        | ((uimm.val >> 1) & 0b1)
        | (encdec_freg(rd).val << 4)
        | ((uimm.val >> 2) & 0b1)
        | (0b001_u32 << 10))
}
fn encode_c_fsdsp(uimm: BitVector<6>, rs2: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b10_u32 << 0)
        | (encdec_freg(rs2).val << 2)
        | ((uimm.val >> 5) & 0b1)
        | ((uimm.val >> 2) & 0b1)
        | (0b101_u32 << 9))
}
fn encode_c_fld(uimm: BitVector<5>, rs1: cregidx, rd: cfregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b00_u32 << 0)
        | (encdec_cfreg(rd).val << 2)
        | ((uimm.val >> 4) & 0b1)
        | (encdec_creg(rs1).val << 6)
        | ((uimm.val >> 2) & 0b1)
        | (0b001_u32 << 10))
}
fn encode_c_fsd(uimm: BitVector<5>, rs1: cregidx, rs2: cfregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b00_u32 << 0)
        | (encdec_cfreg(rs2).val << 2)
        | ((uimm.val >> 4) & 0b1)
        | (encdec_creg(rs1).val << 6)
        | ((uimm.val >> 2) & 0b1)
        | (0b101_u32 << 10))
}
fn encode_f_bin_rm_type_h(
    rs2: fregidx,
    rs1: fregidx,
    rm: rounding_mode,
    rd: fregidx,
    arg4: f_bin_rm_op_H,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg4 {
        f_bin_rm_op_H::FADD_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0000010_u32 << 25))
        }
        f_bin_rm_op_H::FSUB_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0000110_u32 << 25))
        }
        f_bin_rm_op_H::FMUL_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0001010_u32 << 25))
        }
        f_bin_rm_op_H::FDIV_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0001110_u32 << 25))
        }
    }
}
fn encode_f_madd_type_h(
    rs3: fregidx,
    rs2: fregidx,
    rs1: fregidx,
    rm: rounding_mode,
    rd: fregidx,
    arg5: f_madd_op_H,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg5 {
        f_madd_op_H::FMADD_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1000011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b10_u32 << 25)
                | (encdec_freg(rs3).val << 27))
        }
        f_madd_op_H::FMSUB_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1000111_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b10_u32 << 25)
                | (encdec_freg(rs3).val << 27))
        }
        f_madd_op_H::FNMSUB_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1001011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b10_u32 << 25)
                | (encdec_freg(rs3).val << 27))
        }
        f_madd_op_H::FNMADD_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1001111_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b10_u32 << 25)
                | (encdec_freg(rs3).val << 27))
        }
    }
}
fn encode_f_bin_f_type_h(
    rs2: fregidx,
    rs1: fregidx,
    rd: fregidx,
    arg3: f_bin_f_op_H,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        f_bin_f_op_H::FSGNJ_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (0b000_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0010010_u32 << 25))
        }
        f_bin_f_op_H::FSGNJN_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0010010_u32 << 25))
        }
        f_bin_f_op_H::FSGNJX_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (0b010_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0010010_u32 << 25))
        }
        f_bin_f_op_H::FMIN_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (0b000_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0010110_u32 << 25))
        }
        f_bin_f_op_H::FMAX_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b0010110_u32 << 25))
        }
    }
}
fn encode_f_bin_x_type_h(
    rs2: fregidx,
    rs1: fregidx,
    rd: regidx,
    arg3: f_bin_x_op_H,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        f_bin_x_op_H::FEQ_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b010_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b1010010_u32 << 25))
        }
        f_bin_x_op_H::FLT_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b1010010_u32 << 25))
        }
        f_bin_x_op_H::FLE_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b000_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (encdec_freg(rs2).val << 20)
                | (0b1010010_u32 << 25))
        }
    }
}
fn encode_f_un_rm_ff_type_h(
    rs1: fregidx,
    rm: rounding_mode,
    rd: fregidx,
    arg3: f_un_rm_ff_op_H,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        f_un_rm_ff_op_H::FSQRT_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b0101110_u32 << 25))
        }
        f_un_rm_ff_op_H::FCVT_H_S => {
            if !(ctx.extensions.Ext_Zfh
                || ctx.extensions.Ext_Zhinx
                || ctx.extensions.Ext_Zfhmin
                || ctx.extensions.Ext_Zhinxmin)
            {
                return err!("required extension Zfh or Zhinx or Zfhmin or Zhinxmin");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b0100010_u32 << 25))
        }
        f_un_rm_ff_op_H::FCVT_H_D => {
            if !(ctx.extensions.Ext_Zfh
                || ctx.extensions.Ext_Zhinx
                || ctx.extensions.Ext_Zfhmin
                || ctx.extensions.Ext_Zhinxmin)
            {
                return err!("required extension Zfh or Zhinx or Zfhmin or Zhinxmin");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00001_u32 << 20)
                | (0b0100010_u32 << 25))
        }
        f_un_rm_ff_op_H::FCVT_S_H => {
            if !(ctx.extensions.Ext_Zfh
                || ctx.extensions.Ext_Zhinx
                || ctx.extensions.Ext_Zfhmin
                || ctx.extensions.Ext_Zhinxmin)
            {
                return err!("required extension Zfh or Zhinx or Zfhmin or Zhinxmin");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00010_u32 << 20)
                | (0b0100000_u32 << 25))
        }
        f_un_rm_ff_op_H::FCVT_D_H => {
            if !(ctx.extensions.Ext_Zfh
                || ctx.extensions.Ext_Zhinx
                || ctx.extensions.Ext_Zfhmin
                || ctx.extensions.Ext_Zhinxmin)
            {
                return err!("required extension Zfh or Zhinx or Zfhmin or Zhinxmin");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00010_u32 << 20)
                | (0b0100001_u32 << 25))
        }
    }
}
fn encode_f_un_rm_fx_type_h(
    rs1: fregidx,
    rm: rounding_mode,
    rd: regidx,
    arg3: f_un_rm_fx_op_H,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        f_un_rm_fx_op_H::FCVT_W_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b1100010_u32 << 25))
        }
        f_un_rm_fx_op_H::FCVT_WU_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00001_u32 << 20)
                | (0b1100010_u32 << 25))
        }
        f_un_rm_fx_op_H::FCVT_L_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00010_u32 << 20)
                | (0b1100010_u32 << 25))
        }
        f_un_rm_fx_op_H::FCVT_LU_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00011_u32 << 20)
                | (0b1100010_u32 << 25))
        }
    }
}
fn encode_f_un_rm_xf_type_h(
    rs1: regidx,
    rm: rounding_mode,
    rd: fregidx,
    arg3: f_un_rm_xf_op_H,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        f_un_rm_xf_op_H::FCVT_H_W => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b1101010_u32 << 25))
        }
        f_un_rm_xf_op_H::FCVT_H_WU => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00001_u32 << 20)
                | (0b1101010_u32 << 25))
        }
        f_un_rm_xf_op_H::FCVT_H_L => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00010_u32 << 20)
                | (0b1101010_u32 << 25))
        }
        f_un_rm_xf_op_H::FCVT_H_LU => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }
            if !(ctx.is_64bit()) {
                return err!("available only in 64-bit mode");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_freg(rd).val << 7)
                | (encdec_rounding_mode(rm).val << 12)
                | (encdec_reg(rs1).val << 15)
                | (0b00011_u32 << 20)
                | (0b1101010_u32 << 25))
        }
    }
}
fn encode_f_un_f_type_h(rs1: regidx, rd: fregidx, arg2: f_un_f_op_H) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00000_u32 << 20)
        | (0b1111010_u32 << 25))
}
fn encode_f_un_x_type_h(
    rs1: fregidx,
    rd: regidx,
    arg2: f_un_x_op_H,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg2 {
        f_un_x_op_H::FCLASS_H => {
            if !(ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx) {
                return err!("required extension Zfh or Zhinx");
            }

            Ok((0b1010011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b001_u32 << 12)
                | (encdec_freg(rs1).val << 15)
                | (0b00000_u32 << 20)
                | (0b1110010_u32 << 25))
        }
        f_un_x_op_H::FMV_X_H => Ok((0b1010011_u32 << 0)
            | (encdec_reg(rd).val << 7)
            | (0b000_u32 << 12)
            | (encdec_freg(rs1).val << 15)
            | (0b00000_u32 << 20)
            | (0b1110010_u32 << 25)),
    }
}
fn encode_fli_h(constantidx: BitVector<5>, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (0b000_u32 << 12)
        | (constantidx.val << 15)
        | (0b00001_u32 << 20)
        | (0b1111010_u32 << 25))
}
fn encode_fli_s(constantidx: BitVector<5>, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (0b000_u32 << 12)
        | (constantidx.val << 15)
        | (0b00001_u32 << 20)
        | (0b1111000_u32 << 25))
}
fn encode_fli_d(constantidx: BitVector<5>, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (0b000_u32 << 12)
        | (constantidx.val << 15)
        | (0b00001_u32 << 20)
        | (0b1111001_u32 << 25))
}
fn encode_fminm_h(rs2: fregidx, rs1: fregidx, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_freg(rs2).val << 20)
        | (0b0010110_u32 << 25))
}
fn encode_fmaxm_h(rs2: fregidx, rs1: fregidx, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (0b011_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_freg(rs2).val << 20)
        | (0b0010110_u32 << 25))
}
fn encode_fminm_s(rs2: fregidx, rs1: fregidx, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_freg(rs2).val << 20)
        | (0b0010100_u32 << 25))
}
fn encode_fmaxm_s(rs2: fregidx, rs1: fregidx, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (0b011_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_freg(rs2).val << 20)
        | (0b0010100_u32 << 25))
}
fn encode_fminm_d(rs2: fregidx, rs1: fregidx, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_freg(rs2).val << 20)
        | (0b0010101_u32 << 25))
}
fn encode_fmaxm_d(rs2: fregidx, rs1: fregidx, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (0b011_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_freg(rs2).val << 20)
        | (0b0010101_u32 << 25))
}
fn encode_fround_h(rs1: fregidx, rm: rounding_mode, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (encdec_rounding_mode(rm).val << 12)
        | (encdec_freg(rs1).val << 15)
        | (0b00100_u32 << 20)
        | (0b0100010_u32 << 25))
}
fn encode_froundnx_h(rs1: fregidx, rm: rounding_mode, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (encdec_rounding_mode(rm).val << 12)
        | (encdec_freg(rs1).val << 15)
        | (0b00101_u32 << 20)
        | (0b0100010_u32 << 25))
}
fn encode_fround_s(rs1: fregidx, rm: rounding_mode, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (encdec_rounding_mode(rm).val << 12)
        | (encdec_freg(rs1).val << 15)
        | (0b00100_u32 << 20)
        | (0b0100000_u32 << 25))
}
fn encode_froundnx_s(rs1: fregidx, rm: rounding_mode, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (encdec_rounding_mode(rm).val << 12)
        | (encdec_freg(rs1).val << 15)
        | (0b00101_u32 << 20)
        | (0b0100000_u32 << 25))
}
fn encode_fround_d(rs1: fregidx, rm: rounding_mode, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (encdec_rounding_mode(rm).val << 12)
        | (encdec_freg(rs1).val << 15)
        | (0b00100_u32 << 20)
        | (0b0100001_u32 << 25))
}
fn encode_froundnx_d(rs1: fregidx, rm: rounding_mode, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (encdec_rounding_mode(rm).val << 12)
        | (encdec_freg(rs1).val << 15)
        | (0b00101_u32 << 20)
        | (0b0100001_u32 << 25))
}
fn encode_fmvh_x_d(rs1: fregidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (0b00001_u32 << 20)
        | (0b1110001_u32 << 25))
}
fn encode_fmvp_d_x(rs2: regidx, rs1: regidx, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b1011001_u32 << 25))
}
fn encode_fleq_h(rs2: fregidx, rs1: fregidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_freg(rs2).val << 20)
        | (0b1010010_u32 << 25))
}
fn encode_fltq_h(rs2: fregidx, rs1: fregidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_freg(rs2).val << 20)
        | (0b1010010_u32 << 25))
}
fn encode_fleq_s(rs2: fregidx, rs1: fregidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_freg(rs2).val << 20)
        | (0b1010000_u32 << 25))
}
fn encode_fltq_s(rs2: fregidx, rs1: fregidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_freg(rs2).val << 20)
        | (0b1010000_u32 << 25))
}
fn encode_fleq_d(rs2: fregidx, rs1: fregidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_freg(rs2).val << 20)
        | (0b1010001_u32 << 25))
}
fn encode_fltq_d(rs2: fregidx, rs1: fregidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_freg(rs2).val << 20)
        | (0b1010001_u32 << 25))
}
fn encode_fcvtmod_w_d(rs1: fregidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (0b01000_u32 << 20)
        | (0b1100001_u32 << 25))
}
fn encode_vsetvli(
    ma: BitVector<1>,
    ta: BitVector<1>,
    sew: BitVector<3>,
    lmul: BitVector<3>,
    rs1: regidx,
    rd: regidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b111_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (lmul.val << 20)
        | (sew.val << 23)
        | (ta.val << 26)
        | (ma.val << 27)
        | (0b0000_u32 << 28))
}
fn encode_vsetvl(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b111_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b1000000_u32 << 25))
}
fn encode_vsetivli(
    ma: BitVector<1>,
    ta: BitVector<1>,
    sew: BitVector<3>,
    lmul: BitVector<3>,
    uimm: BitVector<5>,
    rd: regidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b111_u32 << 12)
        | (uimm.val << 15)
        | (lmul.val << 20)
        | (sew.val << 23)
        | (ta.val << 26)
        | (ma.val << 27)
        | (0b1100_u32 << 28))
}
fn encode_vvtype(
    funct6: vvfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_vvfunct6(funct6).val << 26))
}
fn encode_nvstype(
    funct6: nvsfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_nvsfunct6(funct6).val << 26))
}
fn encode_nvtype(
    funct6: nvfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_nvfunct6(funct6).val << 26))
}
fn encode_masktypev(vs2: vregidx, vs1: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b0_u32 << 25)
        | (0b010111_u32 << 26))
}
fn encode_movetypev(vs1: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (0b00000_u32 << 20)
        | (0b1_u32 << 25)
        | (0b010111_u32 << 26))
}
fn encode_vxtype(
    funct6: vxfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: regidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_vxfunct6(funct6).val << 26))
}
fn encode_nxstype(
    funct6: nxsfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: regidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_nxsfunct6(funct6).val << 26))
}
fn encode_nxtype(
    funct6: nxfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: regidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_nxfunct6(funct6).val << 26))
}
fn encode_vxsg(
    funct6: vxsgfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: regidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_vxsgfunct6(funct6).val << 26))
}
fn encode_masktypex(vs2: vregidx, rs1: regidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b0_u32 << 25)
        | (0b010111_u32 << 26))
}
fn encode_movetypex(rs1: regidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00000_u32 << 20)
        | (0b1_u32 << 25)
        | (0b010111_u32 << 26))
}
fn encode_vitype(
    funct6: vifunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    simm: BitVector<5>,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b011_u32 << 12)
        | (simm.val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_vifunct6(funct6).val << 26))
}
fn encode_nistype(
    funct6: nisfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    uimm: BitVector<5>,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b011_u32 << 12)
        | (uimm.val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_nisfunct6(funct6).val << 26))
}
fn encode_nitype(
    funct6: nifunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    uimm: BitVector<5>,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b011_u32 << 12)
        | (uimm.val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_nifunct6(funct6).val << 26))
}
fn encode_visg(
    funct6: visgfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    simm: BitVector<5>,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b011_u32 << 12)
        | (simm.val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_visgfunct6(funct6).val << 26))
}
fn encode_masktypei(vs2: vregidx, simm: BitVector<5>, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b011_u32 << 12)
        | (simm.val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b0_u32 << 25)
        | (0b010111_u32 << 26))
}
fn encode_movetypei(vd: vregidx, simm: BitVector<5>) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b011_u32 << 12)
        | (simm.val << 15)
        | (0b00000_u32 << 20)
        | (0b1_u32 << 25)
        | (0b010111_u32 << 26))
}
fn encode_vmvrtype(vs2: vregidx, nreg: usize, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b011_u32 << 12)
        | (encdec_nreg(nreg).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (0b100111_u32 << 26))
}
fn encode_mvvtype(
    funct6: mvvfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_mvvfunct6(funct6).val << 26))
}
fn encode_mvvmatype(
    funct6: mvvmafunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_mvvmafunct6(funct6).val << 26))
}
fn encode_wvvtype(
    funct6: wvvfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_wvvfunct6(funct6).val << 26))
}
fn encode_wvtype(
    funct6: wvfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_wvfunct6(funct6).val << 26))
}
fn encode_wmvvtype(
    funct6: wmvvfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_wmvvfunct6(funct6).val << 26))
}
fn encode_vexttype(
    funct6: vextfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (vext_vs1(funct6).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010010_u32 << 26))
}
fn encode_vmvxs(vs2: vregidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b010_u32 << 12)
        | (0b00000_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (0b010000_u32 << 26))
}
fn encode_mvvcompress(vs2: vregidx, vs1: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (0b010111_u32 << 26))
}
fn encode_mvxtype(
    funct6: mvxfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: regidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b110_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_mvxfunct6(funct6).val << 26))
}
fn encode_mvxmatype(
    funct6: mvxmafunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: regidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b110_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_mvxmafunct6(funct6).val << 26))
}
fn encode_wvxtype(
    funct6: wvxfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: regidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b110_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_wvxfunct6(funct6).val << 26))
}
fn encode_wxtype(
    funct6: wxfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: regidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b110_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_wxfunct6(funct6).val << 26))
}
fn encode_wmvxtype(
    funct6: wmvxfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: regidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b110_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_wmvxfunct6(funct6).val << 26))
}
fn encode_vmvsx(rs1: regidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b110_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00000_u32 << 20)
        | (0b1_u32 << 25)
        | (0b010000_u32 << 26))
}
fn encode_fvvtype(
    funct6: fvvfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_fvvfunct6(funct6).val << 26))
}
fn encode_fvvmatype(
    funct6: fvvmafunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_fvvmafunct6(funct6).val << 26))
}
fn encode_fwvvtype(
    funct6: fwvvfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_fwvvfunct6(funct6).val << 26))
}
fn encode_fwvvmatype(
    funct6: fwvvmafunct6,
    vm: BitVector<1>,
    vs1: vregidx,
    vs2: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_vreg(vs2).val << 15)
        | (encdec_vreg(vs1).val << 20)
        | (vm.val << 25)
        | (encdec_fwvvmafunct6(funct6).val << 26))
}
fn encode_fwvtype(
    funct6: fwvfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_fwvfunct6(funct6).val << 26))
}
fn encode_vfunary0(
    vm: BitVector<1>,
    vs2: vregidx,
    vfunary0: vfunary0,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_vfunary0_vs1(vfunary0).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010010_u32 << 26))
}
fn encode_vfwunary0(
    vm: BitVector<1>,
    vs2: vregidx,
    vfwunary0: vfwunary0,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_vfwunary0_vs1(vfwunary0).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010010_u32 << 26))
}
fn encode_vfnunary0(
    vm: BitVector<1>,
    vs2: vregidx,
    vfnunary0: vfnunary0,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_vfnunary0_vs1(vfnunary0).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010010_u32 << 26))
}
fn encode_vfunary1(
    vm: BitVector<1>,
    vs2: vregidx,
    vfunary1: vfunary1,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_vfunary1_vs1(vfunary1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010011_u32 << 26))
}
fn encode_vfmvfs(vs2: vregidx, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (0b001_u32 << 12)
        | (0b00000_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (0b010000_u32 << 26))
}
fn encode_fvftype(
    funct6: fvffunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: fregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_fvffunct6(funct6).val << 26))
}
fn encode_fvfmatype(
    funct6: fvfmafunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: fregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_fvfmafunct6(funct6).val << 26))
}
fn encode_fwvftype(
    funct6: fwvffunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: fregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_fwvffunct6(funct6).val << 26))
}
fn encode_fwvfmatype(
    funct6: fwvfmafunct6,
    vm: BitVector<1>,
    rs1: fregidx,
    vs2: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_fwvfmafunct6(funct6).val << 26))
}
fn encode_fwftype(
    funct6: fwffunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: fregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_fwffunct6(funct6).val << 26))
}
fn encode_vfmerge(vs2: vregidx, rs1: fregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b0_u32 << 25)
        | (0b010111_u32 << 26))
}
fn encode_vfmv(rs1: fregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (0b00000_u32 << 20)
        | (0b1_u32 << 25)
        | (0b010111_u32 << 26))
}
fn encode_vfmvsf(rs1: fregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (0b00000_u32 << 20)
        | (0b1_u32 << 25)
        | (0b010000_u32 << 26))
}
fn encode_vlsegtype(
    nf: nfields,
    vm: BitVector<1>,
    rs1: regidx,
    width: vlewidth,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0000111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (encdec_vlewidth(width).val << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00000_u32 << 20)
        | (vm.val << 25)
        | (0b00_u32 << 26)
        | (0b0_u32 << 28)
        | (encdec_nfields(nf).val << 29))
}
fn encode_vlsegfftype(
    nf: nfields,
    vm: BitVector<1>,
    rs1: regidx,
    width: vlewidth,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0000111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (encdec_vlewidth(width).val << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b10000_u32 << 20)
        | (vm.val << 25)
        | (0b00_u32 << 26)
        | (0b0_u32 << 28)
        | (encdec_nfields(nf).val << 29))
}
fn encode_vssegtype(
    nf: nfields,
    vm: BitVector<1>,
    rs1: regidx,
    width: vlewidth,
    vs3: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0100111_u32 << 0)
        | (encdec_vreg(vs3).val << 7)
        | (encdec_vlewidth(width).val << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00000_u32 << 20)
        | (vm.val << 25)
        | (0b00_u32 << 26)
        | (0b0_u32 << 28)
        | (encdec_nfields(nf).val << 29))
}
fn encode_vlssegtype(
    nf: nfields,
    vm: BitVector<1>,
    rs2: regidx,
    rs1: regidx,
    width: vlewidth,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0000111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (encdec_vlewidth(width).val << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (vm.val << 25)
        | (0b10_u32 << 26)
        | (0b0_u32 << 28)
        | (encdec_nfields(nf).val << 29))
}
fn encode_vsssegtype(
    nf: nfields,
    vm: BitVector<1>,
    rs2: regidx,
    rs1: regidx,
    width: vlewidth,
    vs3: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0100111_u32 << 0)
        | (encdec_vreg(vs3).val << 7)
        | (encdec_vlewidth(width).val << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (vm.val << 25)
        | (0b10_u32 << 26)
        | (0b0_u32 << 28)
        | (encdec_nfields(nf).val << 29))
}
fn encode_vlxsegtype(
    nf: nfields,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: regidx,
    width: vlewidth,
    vd: vregidx,
    mop: indexed_mop,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0000111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (encdec_vlewidth(width).val << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_indexed_mop(mop).val << 26)
        | (0b0_u32 << 28)
        | (encdec_nfields(nf).val << 29))
}
fn encode_vsxsegtype(
    nf: nfields,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: regidx,
    width: vlewidth,
    vs3: vregidx,
    mop: indexed_mop,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0100111_u32 << 0)
        | (encdec_vreg(vs3).val << 7)
        | (encdec_vlewidth(width).val << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_indexed_mop(mop).val << 26)
        | (0b0_u32 << 28)
        | (encdec_nfields(nf).val << 29))
}
fn encode_vlretype(
    nf: nfields_pow2,
    rs1: regidx,
    width: vlewidth,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0000111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (encdec_vlewidth(width).val << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b01000_u32 << 20)
        | (0b1_u32 << 25)
        | (0b00_u32 << 26)
        | (0b0_u32 << 28)
        | (encdec_nfields_pow2(nf).val << 29))
}
fn encode_vsretype(nf: nfields_pow2, rs1: regidx, vs3: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0100111_u32 << 0)
        | (encdec_vreg(vs3).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b01000_u32 << 20)
        | (0b1_u32 << 25)
        | (0b00_u32 << 26)
        | (0b0_u32 << 28)
        | (encdec_nfields_pow2(nf).val << 29))
}
fn encode_vmtype(rs1: regidx, vd_or_vs3: vregidx, op: vmlsop) -> crate::Result<u32> {
    // instruction assembling
    Ok((encdec_lsop(op).val << 0)
        | (encdec_vreg(vd_or_vs3).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b01011_u32 << 20)
        | (0b1_u32 << 25)
        | (0b00_u32 << 26)
        | (0b0_u32 << 28)
        | (0b000_u32 << 29))
}
fn encode_mmtype(funct6: mmfunct6, vs2: vregidx, vs1: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (encdec_mmfunct6(funct6).val << 26))
}
fn encode_vcpop_m(vm: BitVector<1>, vs2: vregidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b010_u32 << 12)
        | (0b10000_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010000_u32 << 26))
}
fn encode_vfirst_m(vm: BitVector<1>, vs2: vregidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b010_u32 << 12)
        | (0b10001_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010000_u32 << 26))
}
fn encode_vmsbf_m(vm: BitVector<1>, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b00001_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010100_u32 << 26))
}
fn encode_vmsif_m(vm: BitVector<1>, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b00011_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010100_u32 << 26))
}
fn encode_vmsof_m(vm: BitVector<1>, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b00010_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010100_u32 << 26))
}
fn encode_viota_m(vm: BitVector<1>, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b10000_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010100_u32 << 26))
}
fn encode_vid_v(vm: BitVector<1>, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b10001_u32 << 15)
        | (0b00000_u32 << 20)
        | (vm.val << 25)
        | (0b010100_u32 << 26))
}
fn encode_vvmtype(
    funct6: vvmfunct6,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b0_u32 << 25)
        | (encdec_vvmfunct6(funct6).val << 26))
}
fn encode_vvmctype(
    funct6: vvmcfunct6,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (encdec_vvmcfunct6(funct6).val << 26))
}
fn encode_vvmstype(
    funct6: vvmsfunct6,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b0_u32 << 25)
        | (encdec_vvmsfunct6(funct6).val << 26))
}
fn encode_vvcmptype(
    funct6: vvcmpfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_vvcmpfunct6(funct6).val << 26))
}
fn encode_vxmtype(funct6: vxmfunct6, vs2: vregidx, rs1: regidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b0_u32 << 25)
        | (encdec_vxmfunct6(funct6).val << 26))
}
fn encode_vxmctype(
    funct6: vxmcfunct6,
    vs2: vregidx,
    rs1: regidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (encdec_vxmcfunct6(funct6).val << 26))
}
fn encode_vxmstype(
    funct6: vxmsfunct6,
    vs2: vregidx,
    rs1: regidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b0_u32 << 25)
        | (encdec_vxmsfunct6(funct6).val << 26))
}
fn encode_vxcmptype(
    funct6: vxcmpfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: regidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_vxcmpfunct6(funct6).val << 26))
}
fn encode_vimtype(
    funct6: vimfunct6,
    vs2: vregidx,
    simm: BitVector<5>,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b011_u32 << 12)
        | (simm.val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b0_u32 << 25)
        | (encdec_vimfunct6(funct6).val << 26))
}
fn encode_vimctype(
    funct6: vimcfunct6,
    vs2: vregidx,
    simm: BitVector<5>,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b011_u32 << 12)
        | (simm.val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (encdec_vimcfunct6(funct6).val << 26))
}
fn encode_vimstype(
    funct6: vimsfunct6,
    vs2: vregidx,
    simm: BitVector<5>,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b011_u32 << 12)
        | (simm.val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b0_u32 << 25)
        | (encdec_vimsfunct6(funct6).val << 26))
}
fn encode_vicmptype(
    funct6: vicmpfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    simm: BitVector<5>,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b011_u32 << 12)
        | (simm.val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_vicmpfunct6(funct6).val << 26))
}
fn encode_fvvmtype(
    funct6: fvvmfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_fvvmfunct6(funct6).val << 26))
}
fn encode_fvfmtype(
    funct6: fvfmfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: fregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_fvfmfunct6(funct6).val << 26))
}
fn encode_rivvtype(
    funct6: rivvfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_rivvfunct6(funct6).val << 26))
}
fn encode_rmvvtype(
    funct6: rmvvfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_rmvvfunct6(funct6).val << 26))
}
fn encode_rfvvtype(
    funct6: rfvvfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_rfvvfunct6(funct6).val << 26))
}
fn encode_rfwvvtype(
    funct6: rfwvvfunct6,
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (encdec_rfwvvfunct6(funct6).val << 26))
}
fn encode_sha256sig0(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00010_u32 << 20)
        | (0b01000_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_sha256sig1(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00011_u32 << 20)
        | (0b01000_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_sha256sum0(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00000_u32 << 20)
        | (0b01000_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_sha256sum1(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00001_u32 << 20)
        | (0b01000_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_aes32esmi(bs: BitVector<2>, rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b10011_u32 << 25)
        | (bs.val << 30))
}
fn encode_aes32esi(bs: BitVector<2>, rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b10001_u32 << 25)
        | (bs.val << 30))
}
fn encode_aes32dsmi(bs: BitVector<2>, rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b10111_u32 << 25)
        | (bs.val << 30))
}
fn encode_aes32dsi(bs: BitVector<2>, rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b10101_u32 << 25)
        | (bs.val << 30))
}
fn encode_sha512sig0l(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b01010_u32 << 25)
        | (0b01_u32 << 30))
}
fn encode_sha512sig0h(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b01110_u32 << 25)
        | (0b01_u32 << 30))
}
fn encode_sha512sig1l(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b01011_u32 << 25)
        | (0b01_u32 << 30))
}
fn encode_sha512sig1h(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b01111_u32 << 25)
        | (0b01_u32 << 30))
}
fn encode_sha512sum0r(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b01000_u32 << 25)
        | (0b01_u32 << 30))
}
fn encode_sha512sum1r(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b01001_u32 << 25)
        | (0b01_u32 << 30))
}
fn encode_aes64ks1i(rnum: BitVector<4>, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (rnum.val << 20)
        | (0b1_u32 << 24)
        | (0b11000_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_aes64ks2(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b11111_u32 << 25)
        | (0b01_u32 << 30))
}
fn encode_aes64im(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00000_u32 << 20)
        | (0b11000_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_aes64esm(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b11011_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_aes64es(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b11001_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_aes64dsm(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b11111_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_aes64ds(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b11101_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_sha512sig0(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00110_u32 << 20)
        | (0b01000_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_sha512sig1(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00111_u32 << 20)
        | (0b01000_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_sha512sum0(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00100_u32 << 20)
        | (0b01000_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_sha512sum1(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b00101_u32 << 20)
        | (0b01000_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_sm3p0(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b01000_u32 << 20)
        | (0b01000_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_sm3p1(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b01001_u32 << 20)
        | (0b01000_u32 << 25)
        | (0b00_u32 << 30))
}
fn encode_sm4ed(bs: BitVector<2>, rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b11000_u32 << 25)
        | (bs.val << 30))
}
fn encode_sm4ks(bs: BitVector<2>, rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b11010_u32 << 25)
        | (bs.val << 30))
}
fn encode_zbkb_rtype(
    rs2: regidx,
    rs1: regidx,
    rd: regidx,
    arg3: brop_zbkb,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg3 {
        brop_zbkb::PACK => {
            if !(ctx.extensions.Ext_Zbkb) {
                return err!("requried extension Zbkb");
            }

            Ok((0b0110011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b100_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0000100_u32 << 25))
        }
        brop_zbkb::PACKH => {
            if !(ctx.extensions.Ext_Zbkb) {
                return err!("requried extension Zbkb");
            }

            Ok((0b0110011_u32 << 0)
                | (encdec_reg(rd).val << 7)
                | (0b111_u32 << 12)
                | (encdec_reg(rs1).val << 15)
                | (encdec_reg(rs2).val << 20)
                | (0b0000100_u32 << 25))
        }
    }
}
fn encode_zbkb_packw(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0111011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0000100_u32 << 25))
}
fn encode_zip(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b000010001111_u32 << 20))
}
fn encode_unzip(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b000010001111_u32 << 20))
}
fn encode_brev8(rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0010011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b011010000111_u32 << 20))
}
fn encode_xperm8(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0010100_u32 << 25))
}
fn encode_xperm4(rs2: regidx, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0010100_u32 << 25))
}
fn encode_vandn_vv(
    vm: BitVector<1>,
    vs1: vregidx,
    vs2: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b000001_u32 << 26))
}
fn encode_vandn_vx(vm: BitVector<1>, vs2: vregidx, rs1: regidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b000001_u32 << 26))
}
fn encode_vbrev_v(vm: BitVector<1>, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b01010_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010010_u32 << 26))
}
fn encode_vbrev8_v(vm: BitVector<1>, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b01000_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010010_u32 << 26))
}
fn encode_vrev8_v(vm: BitVector<1>, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b01001_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010010_u32 << 26))
}
fn encode_vclz_v(vm: BitVector<1>, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b01100_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010010_u32 << 26))
}
fn encode_vctz_v(vm: BitVector<1>, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b01101_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010010_u32 << 26))
}
fn encode_vcpop_v(vm: BitVector<1>, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b01110_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010010_u32 << 26))
}
fn encode_vrol_vv(vm: BitVector<1>, vs1: vregidx, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010101_u32 << 26))
}
fn encode_vrol_vx(vm: BitVector<1>, vs2: vregidx, rs1: regidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010101_u32 << 26))
}
fn encode_vror_vv(vm: BitVector<1>, vs1: vregidx, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010100_u32 << 26))
}
fn encode_vror_vx(vm: BitVector<1>, vs2: vregidx, rs1: regidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010100_u32 << 26))
}
fn encode_vror_vi(
    vm: BitVector<1>,
    vs2: vregidx,
    uimm: BitVector<6>,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b011_u32 << 12)
        | ((uimm.val >> 4) & 0b1)
        | (encdec_vreg(vs2).val << 16)
        | (vm.val << 21)
        | ((uimm.val >> 5) & 0b1)
        | (0b01010_u32 << 23))
}
fn encode_vwsll_vv(
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b000_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b110101_u32 << 26))
}
fn encode_vwsll_vx(vm: BitVector<1>, vs2: vregidx, rs1: regidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b110101_u32 << 26))
}
fn encode_vwsll_vi(
    vm: BitVector<1>,
    vs2: vregidx,
    uimm: BitVector<5>,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b011_u32 << 12)
        | (uimm.val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b110101_u32 << 26))
}
fn encode_vclmul_vv(
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b001100_u32 << 26))
}
fn encode_vclmul_vx(
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: regidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b110_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b001100_u32 << 26))
}
fn encode_vclmulh_vv(
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b001101_u32 << 26))
}
fn encode_vclmulh_vx(
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: regidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b110_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b001101_u32 << 26))
}
fn encode_vghsh_vv(vs2: vregidx, vs1: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1011001_u32 << 25))
}
fn encode_vgmul_vv(vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b10001_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1010001_u32 << 25))
}
fn encode_vaesdf(funct6: zvk_vaesdf_funct6, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b00001_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (encdec_vaesdf(funct6).val << 26))
}
fn encode_vaesdm(funct6: zvk_vaesdm_funct6, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b00000_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (encdec_vaesdm(funct6).val << 26))
}
fn encode_vaesef(funct6: zvk_vaesef_funct6, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b00011_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (encdec_vaesef(funct6).val << 26))
}
fn encode_vaesem(funct6: zvk_vaesem_funct6, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b00010_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (encdec_vaesem(funct6).val << 26))
}
fn encode_vaeskf1_vi(vs2: vregidx, rnd: BitVector<5>, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (rnd.val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1000101_u32 << 25))
}
fn encode_vaeskf2_vi(vs2: vregidx, rnd: BitVector<5>, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (rnd.val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (0b101010_u32 << 26))
}
fn encode_vaesz_vs(vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (0b00111_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (0b101001_u32 << 26))
}
fn encode_vsm4k_vi(vs2: vregidx, uimm: BitVector<5>, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (uimm.val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (0b100001_u32 << 26))
}
fn encode_zvksm4rtype(
    arg0: zvk_vsm4r_funct6,
    vs2: vregidx,
    vd: vregidx,
    ctx: &Context,
) -> crate::Result<u32> {
    // instruction assembling
    match arg0 {
        zvk_vsm4r_funct6::ZVK_VSM4R_VV => {
            if !(ctx.extensions.Ext_Zvksed) {
                return err!("requried extension Zvksed");
            }

            Ok((0b1110111_u32 << 0)
                | (encdec_vreg(vd).val << 7)
                | (0b010_u32 << 12)
                | (0b10000_u32 << 15)
                | (encdec_vreg(vs2).val << 20)
                | (0b1_u32 << 25)
                | (0b101000_u32 << 26))
        }
        zvk_vsm4r_funct6::ZVK_VSM4R_VS => {
            if !(ctx.extensions.Ext_Zvksed) {
                return err!("requried extension Zvksed");
            }

            Ok((0b1110111_u32 << 0)
                | (encdec_vreg(vd).val << 7)
                | (0b010_u32 << 12)
                | (0b10000_u32 << 15)
                | (encdec_vreg(vs2).val << 20)
                | (0b1_u32 << 25)
                | (0b101001_u32 << 26))
        }
    }
}
fn encode_vsha2ms_vv(vs2: vregidx, vs1: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (0b101101_u32 << 26))
}
fn encode_zvksha2type(
    funct6: zvk_vsha2_funct6,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (encdec_vsha2(funct6).val << 26))
}
fn encode_vsm3me_vv(vs2: vregidx, vs1: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1_u32 << 25)
        | (0b100000_u32 << 26))
}
fn encode_vsm3c_vi(vs2: vregidx, uimm: BitVector<5>, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b010_u32 << 12)
        | (uimm.val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (0b1010111_u32 << 25))
}
fn encode_csrimm(csr: csreg, imm: BitVector<5>, rd: regidx, op: csrop) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (encdec_csrop(op).val << 12)
        | (0b1_u32 << 14)
        | (imm.val << 15)
        | (csr.val << 20))
}
fn encode_csrreg(csr: csreg, rs1: regidx, rd: regidx, op: csrop) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (encdec_csrop(op).val << 12)
        | (0b0_u32 << 14)
        | (encdec_reg(rs1).val << 15)
        | (csr.val << 20))
}
fn encode_sinval_vma(rs1: regidx, rs2: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110011_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b000_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0001011_u32 << 25))
}
fn encode_sfence_w_inval() -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110011_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b000_u32 << 12)
        | (0b00000_u32 << 15)
        | (0b00000_u32 << 20)
        | (0b0001100_u32 << 25))
}
fn encode_sfence_inval_ir() -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110011_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b000_u32 << 12)
        | (0b00000_u32 << 15)
        | (0b00001_u32 << 20)
        | (0b0001100_u32 << 25))
}
fn encode_wrs(op: wrsop) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1110011_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b000_u32 << 12)
        | (0b00000_u32 << 15)
        | (encdec_wrsop(op).val << 20))
}
fn encode_zicond_rtype(rs2: regidx, rs1: regidx, rd: regidx, op: zicondop) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (encdec_zicondop(op).val << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b0000111_u32 << 25))
}
fn encode_zicbom(cbop: cbop_zicbom, rs1: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0001111_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b010_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_cbop(cbop).val << 20))
}
fn encode_zicboz(rs1: regidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b0001111_u32 << 0)
        | (0b00000_u32 << 7)
        | (0b010_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (0b000000000100_u32 << 20))
}
fn encode_fcvt_bf16_s(rs1: fregidx, rm: rounding_mode, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (encdec_rounding_mode(rm).val << 12)
        | (encdec_freg(rs1).val << 15)
        | (0b01000_u32 << 20)
        | (0b10_u32 << 25)
        | (0b01000_u32 << 27))
}
fn encode_fcvt_s_bf16(rs1: fregidx, rm: rounding_mode, rd: fregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010011_u32 << 0)
        | (encdec_freg(rd).val << 7)
        | (encdec_rounding_mode(rm).val << 12)
        | (encdec_freg(rs1).val << 15)
        | (0b00110_u32 << 20)
        | (0b00_u32 << 25)
        | (0b01000_u32 << 27))
}
fn encode_vfncvtbf16_f_f_w(vm: BitVector<1>, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b001_u32 << 12)
        | (0b11101_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010010_u32 << 26))
}
fn encode_vfwcvtbf16_f_f_v(vm: BitVector<1>, vs2: vregidx, vd: vregidx) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b001_u32 << 12)
        | (0b01101_u32 << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b010010_u32 << 26))
}
fn encode_vfwmaccbf16_vv(
    vm: BitVector<1>,
    vs2: vregidx,
    vs1: vregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b001_u32 << 12)
        | (encdec_vreg(vs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b111011_u32 << 26))
}
fn encode_vfwmaccbf16_vf(
    vm: BitVector<1>,
    vs2: vregidx,
    rs1: fregidx,
    vd: vregidx,
) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b1010111_u32 << 0)
        | (encdec_vreg(vd).val << 7)
        | (0b101_u32 << 12)
        | (encdec_freg(rs1).val << 15)
        | (encdec_vreg(vs2).val << 20)
        | (vm.val << 25)
        | (0b111011_u32 << 26))
}
fn encode_zimop_mop_r(arg0: BitVector<5>, rs1: regidx, rd: regidx) -> crate::Result<u32> {
    // deconstruction of `arg0`
    let mop_21_20 = BitVector::<2>::from_subword(arg0.val >> 0);
    let mop_27_26 = BitVector::<2>::from_subword(arg0.val >> 2);
    let mop_30 = BitVector::<1>::from_subword(arg0.val >> 4);
    // instruction assembling
    Ok((0b1110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (mop_21_20.val << 20)
        | (0b0111_u32 << 22)
        | (mop_27_26.val << 26)
        | (0b00_u32 << 28)
        | (mop_30.val << 30)
        | (0b1_u32 << 31))
}
fn encode_zimop_mop_rr(
    arg0: BitVector<3>,
    rs2: regidx,
    rs1: regidx,
    rd: regidx,
) -> crate::Result<u32> {
    // deconstruction of `arg0`
    let mop_27_26 = BitVector::<2>::from_subword(arg0.val >> 0);
    let mop_30 = BitVector::<1>::from_subword(arg0.val >> 2);
    // instruction assembling
    Ok((0b1110011_u32 << 0)
        | (encdec_reg(rd).val << 7)
        | (0b100_u32 << 12)
        | (encdec_reg(rs1).val << 15)
        | (encdec_reg(rs2).val << 20)
        | (0b1_u32 << 25)
        | (mop_27_26.val << 26)
        | (0b00_u32 << 28)
        | (mop_30.val << 30)
        | (0b1_u32 << 31))
}
fn encode_zcmop(mop: BitVector<3>) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0) | (0b100000_u32 << 2) | (mop.val << 8) | (0b01100_u32 << 11))
}
fn encode_illegal(s: BitVector<32>) -> crate::Result<u32> {
    // instruction assembling
    Ok(s.val)
}
fn encode_c_illegal(s: BitVector<16>) -> crate::Result<u32> {
    // instruction assembling
    Ok(s.val as u32)
}
fn encode_c_nop(imm: BitVector<6>) -> crate::Result<u32> {
    // instruction assembling
    Ok((0b01_u32 << 0)
        | ((imm.val >> 4) & 0b1)
        | (0b00000_u32 << 3)
        | ((imm.val >> 5) & 0b1)
        | (0b000_u32 << 9))
}
