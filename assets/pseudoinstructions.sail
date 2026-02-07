val pseudoinstructions : instruction <-> string

/*!

    For the whole list of pseudoinstructions see Table 5:

        https://github.com/riscv-non-isa/riscv-asm-manual/blob/main/src/asm-manual.adoc

    In this file we define only pseudoinstructions implemented by a single instruction.
*/

mapping pseudoinstructions = {
    /*! nop  =>  addi x0, x0, 0 */
    ITYPE((0 : bitvector(12)), zreg, zreg, ADDI) <-> "nop",

    /*! mv rd, rs  => addi rd, rd, 0 */
    ITYPE((0 : bitvector(12)), rs, rd, ADDI) <-> "mv" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! not rd, rs  =>  xori rd, rd, -1 */
    ITYPE((-1 : bitvector(12)), rs, rd, XORI) <-> "not" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! neg rd, rs  =>  sub rd, x0, rd */
    RTYPE(rs, zreg, rd, SUB) <-> "neg" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! negw rd, rs  =>  subw rd, x0, rd */
    RTYPEW(zreg, rs, rd, SUBW) if eq_int(xlen, 64) <-> "negw" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs) if eq_int(xlen, 64),

    /*! sext.w rd, rs  =>  addiw rd, rs, 0 */
    ADDIW((0 : bitvector(12)), rs1, rd) <-> "sext.w" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs1),

    /*! zext.b rd, rs  =>  andi rd, rs, 255 */
    ITYPE((255 : bitvector(12)), rs, rd, ANDI) <-> "zext.b" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! seqz rd, rs  =>  sltiu rd, rs, 1 */
    ITYPE((1 : bitvector(12)), rs, rd, SLTIU) <-> "seqz" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! snez rd, rs  =>  sltu rd, x0, rs */
    RTYPE(zreg, rs, rd, SLTU) <-> "snez" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! sltz rd, rsa  =>  slt rd, rs, x0 */
    RTYPE(zreg, rs, rd, SLT) <-> "sltz" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! sgtz rd, rs  =>  slt rd, x0, rs */
    RTYPE(rs, zreg, rd, SLT) <-> "sgtz" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! fmv.h frd, frs  =>  fsgnj.h frd, frs, frs */
    F_BIN_F_TYPE_H(rs, rs, rd, FSGNJ_H) <-> "fmv.h" ^ spc(()) ^ freg_or_reg_name(rd) ^ sep(()) ^ freg_or_reg_name(rs),

    /*! fabs.h frd, frs  =>  fsgnjx.h frd, frs, frs */
    F_BIN_F_TYPE_H(rs, rs, rd, FSGNJX_H) <-> "fabs.h" ^ spc(()) ^ freg_or_reg_name(rd) ^ sep(()) ^ freg_or_reg_name(rs),

    /*! fneg.h frd, frs  =>  fsgnjn.h frd, frs, frs */
    F_BIN_F_TYPE_H(rs, rs, rd, FSGNJN_H) <-> "fneg.h" ^ spc(()) ^ freg_or_reg_name(rd) ^ sep(()) ^ freg_or_reg_name(rs),

    /*! fgt.h rd, frs, frt  => flt.h rd, frt, frs */
    F_BIN_X_TYPE_H(rs1, rs2, rd, FLT_H) <-> "fgt.h" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ freg_or_reg_name(rs1) ^ sep(()) ^ freg_or_reg_name(rs2),

    /*! fge.h rd, frs, frt  => fle.h rd, frt, frs */
    F_BIN_X_TYPE_H(rs1, rs2, rd, FLE_H) <-> "fge.h" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ freg_or_reg_name(rs1) ^ sep(()) ^ freg_or_reg_name(rs2),

    /*! fmv.s frd, frs  =>  fsgnj.s frd, frs, frs */
    F_BIN_TYPE_F_S(rs, rs, rd, FSGNJ_S) <-> "fmv.s" ^ spc(()) ^ freg_or_reg_name(rd) ^ sep(()) ^ freg_or_reg_name(rs),

    /*! fabs.s frd, frs  =>  fsgnjx.s frd, frs, frs */
    F_BIN_TYPE_F_S(rs, rs, rd, FSGNJX_S) <-> "fabs.s" ^ spc(()) ^ freg_or_reg_name(rd) ^ sep(()) ^ freg_or_reg_name(rs),

    /*! fneg.s frd, frs  =>  fsgnjn.s frd, frs, frs */
    F_BIN_TYPE_F_S(rs, rs, rd, FSGNJN_S) <-> "fneg.s" ^ spc(()) ^ freg_or_reg_name(rd) ^ sep(()) ^ freg_or_reg_name(rs),

    /*! fgt.s rd, frs, frt  => flt.s rd, frt, frs */
    F_BIN_TYPE_X_S(rs1, rs2, rd, FLT_S) <-> "fgt.s" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ freg_or_reg_name(rs1) ^ sep(()) ^ freg_or_reg_name(rs2),

    /*! fge.s rd, frs, frt  => fle.s rd, frt, frs */
    F_BIN_TYPE_X_S(rs1, rs2, rd, FLE_S) <-> "fge.s" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ freg_or_reg_name(rs1) ^ sep(()) ^ freg_or_reg_name(rs2),

    /*! fmv.d frd, frs  =>  fsgnj.d frd, frs, frs */
    F_BIN_F_TYPE_D(rs, rs, rd, FSGNJ_D) <-> "fmv.d" ^ spc(()) ^ freg_or_reg_name(rd) ^ sep(()) ^ freg_or_reg_name(rs),

    /*! fabs.d frd, frs  =>  fsgnjx.d frd, frs, frs */
    F_BIN_F_TYPE_D(rs, rs, rd, FSGNJX_D) <-> "fabs.d" ^ spc(()) ^ freg_or_reg_name(rd) ^ sep(()) ^ freg_or_reg_name(rs),

    /*! fneg.d frd, frs  =>  fsgnjn.d frd, frs, frs */
    F_BIN_F_TYPE_D(rs, rs, rd, FSGNJN_D) <-> "fneg.d" ^ spc(()) ^ freg_or_reg_name(rd) ^ sep(()) ^ freg_or_reg_name(rs),

    /*! fgt.d rd, frs, frt  => flt.d rd, frt, frs */
    F_BIN_X_TYPE_D(rs1, rs2, rd, FLT_D) <-> "fgt.d" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ freg_or_reg_name(rs1) ^ sep(()) ^ freg_or_reg_name(rs2),

    /*! fge.d rd, frs, frt  => fle.d rd, frt, frs */
    F_BIN_X_TYPE_D(rs1, rs2, rd, FLE_D) <-> "fge.d" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ freg_or_reg_name(rs1) ^ sep(()) ^ freg_or_reg_name(rs2),

    /*! beqz rs, offset  =>  beq rs, x0, offset */
    BTYPE(imm, zreg, rs1, BEQ) <-> "beqz" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13),

    /*! bnez rs, offset  =>  bne rs, x0, offset */
    BTYPE(imm, zreg, rs1, BNE) <-> "bnez" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13),

    /*! blez rs, offset  =>  bge x0, rs, offset */
    BTYPE(imm, rs1, zreg, BGE) <-> "blez" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13),

    /*! bgez rs, offset  =>  bge x0, rs, offset */
    BTYPE(imm, zreg, rs1, BGE) <-> "bgez" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13),

    /*! bltz rs, offset  =>  blt rs, x0, offset */
    BTYPE(imm, zreg, rs1, BLT) <-> "bltz" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13),

    /*! bgtz rs, offset  =>  blt x0, rs, offset */
    BTYPE(imm, rs1, zreg, BLT) <-> "bgtz" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13),

    /*! bgt rs, rt, offset  =>  blt rt, rs, offset */
    BTYPE(imm, rs1, rs2, BLT) <-> "bgt" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ reg_name(rs2) ^ sep(()) ^ label(imm, 13),

    /*! ble rs, rt, offset  =>  bge rt, rs, offset */
    BTYPE(imm, rs1, rs2, BGE) <-> "ble" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ reg_name(rs2) ^ sep(()) ^ label(imm, 13),

    /*! bgtu rs, rt, offset  =>  bltu rt, rs, offset */
    BTYPE(imm, rs1, rs2, BLTU) <-> "bgtu" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ reg_name(rs2) ^ sep(()) ^ label(imm, 13),

    /*! bleu rs, rt, offset  =>  bgeu rt, rs, offset */
    BTYPE(imm, rs1, rs2, BGEU) <-> "bleu" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ reg_name(rs2) ^ sep(()) ^ label(imm, 13),

    /*!
        j offset => jal x0, offset

        Plain unconditional indirect jumps (assembler pseudoinstruction JR)
        are encoded as a JALR with rd=x0.
    */
    JAL(imm, zreg) <-> "j" ^ spc(()) ^ label(imm, 21),

    /*! jr rs  =>  jalr x0, rs, 0 */
    JALR(imm, rs1, zreg) <-> "jr" ^ spc(()) ^ hex_bits_signed_12(imm) ^ "(" ^ reg_name(rs1) ^ ")",

    /*!
        jalr rs  =>  jalr x1, rs, 0

        Procedure returns in the standard calling convention (assembler pseudoinstruction RET)
        are encoded as a JALR with rd=x0, rs1=x1, and imm=0.
    */
    JALR((0 : bitvector(12)), x1, zreg) <-> "ret",

    /*! vfneg.v vd, vs  =>  vfsgnjn.vv vd, vs, vs */
    FVVTYPE(FVV_VSGNJN, vm, vs, vs, vd) <-> "vfneg.v" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs) ^ maybe_vmask(vm),

    /*! vfabs.v vd, vs  =>  vfsgnjx.vv vd, vs, vs */
    FVVTYPE(FVV_VSGNJX, vm, vs, vs, vd) <-> "vfabs.v" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs) ^ maybe_vmask(vm),

    /*! vmclr.m vd  =>  vmxor.mm vd, vd, vd */
    MMTYPE(MM_VMXOR, vd, vd, vd) <-> "vmclr.m" ^ spc(()) ^ vreg_name(vd),

    /*! vmfge.vv vd, va, vb, vm  =>  vmfle.vv vd, vb, va, vm */
    FVVMTYPE(FVVM_VMFLE, vm, vs1, vs2, vd) <-> "vmfge.vv" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ sep(()) ^ vreg_name(vs1) ^ maybe_vmask(vm),

    /*! vmfgt.vv vd, va, vb, vm  =>  vmflt.vv vd, vb, va, vm */
    FVVMTYPE(FVVM_VMFLT, vm, vs1, vs2, vd) <-> "vmfgt.vv" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ sep(()) ^ vreg_name(vs1) ^ maybe_vmask(vm),

    /*!
        The assembler pseudoinstruction to read a CSR, CSRR rd, csr, is encoded as CSRRS rd, csr, x0.
    */
    CSRReg(csr, zreg, rd, CSRRS) <-> "csrr" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ csr_name_map(csr),

    /*!
        The assembler pseudoinstruction to write a CSR, CSRW csr, rs1, is encoded as CSRRW x0, csr, rs1
    */
    CSRReg(csr, rs1, zreg, CSRRW) <-> "csrw" ^ spc(()) ^ csr_name_map(csr) ^ sep(()) ^ reg_name(rs1)

    /*!
        [...] while CSRWI csr, uimm, is encoded as CSRRWI x0, csr, uimm.

        Further assembler pseudoinstructions are defined to set and clear bits in the CSR when the old value
is not required: CSRS/CSRC csr, rs1; CSRSI/CSRCI csr, uimm.
    */

}
