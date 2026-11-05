val pseudoinstructions : instruction <-> string

/*!

    For the whole list of pseudoinstructions see Table 5:

        https://github.com/riscv-non-isa/riscv-asm-manual/blob/main/src/asm-manual.adoc

    In this file we define only pseudoinstructions implemented by a single instruction.
*/

mapping pseudoinstructions = {
    /*! nop  =>  addi x0, x0, 0 */
    ITYPE((0 : bitvector(12)), x0, x0, ADDI) <-> "nop",

    /*! mv rd, rs  => addi rd, rd, 0 */
    ITYPE((0 : bitvector(12)), rs, rd, ADDI) <-> "mv" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! not rd, rs  =>  xori rd, rd, -1 */
    ITYPE((-1 : bitvector(12)), rs, rd, XORI) <-> "not" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! neg rd, rs  =>  sub rd, x0, rd */
    RTYPE(rs, x0, rd, SUB) <-> "neg" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! negw rd, rs  =>  subw rd, x0, rd */
    RTYPEW(x0, rs, rd, SUBW) if eq_int(xlen, 64) <-> "negw" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs) if eq_int(xlen, 64),

    /*! sext.w rd, rs  =>  addiw rd, rs, 0 */
    ADDIW((0 : bitvector(12)), rs1, rd) <-> "sext.w" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs1),

    /*! zext.b rd, rs  =>  andi rd, rs, 255 */
    ITYPE((255 : bitvector(12)), rs, rd, ANDI) <-> "zext.b" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! seqz rd, rs  =>  sltiu rd, rs, 1 */
    ITYPE((1 : bitvector(12)), rs, rd, SLTIU) <-> "seqz" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! snez rd, rs  =>  sltu rd, x0, rs */
    RTYPE(x0, rs, rd, SLTU) <-> "snez" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! sltz rd, rsa  =>  slt rd, rs, x0 */
    RTYPE(x0, rs, rd, SLT) <-> "sltz" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! sgtz rd, rs  =>  slt rd, x0, rs */
    RTYPE(rs, x0, rd, SLT) <-> "sgtz" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

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
    BTYPE(imm, x0, rs1, BEQ) <-> "beqz" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13),

    /*! bnez rs, offset  =>  bne rs, x0, offset */
    BTYPE(imm, x0, rs1, BNE) <-> "bnez" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13),

    /*! blez rs, offset  =>  bge x0, rs, offset */
    BTYPE(imm, rs1, x0, BGE) <-> "blez" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13),

    /*! bgez rs, offset  =>  bge x0, rs, offset */
    BTYPE(imm, x0, rs1, BGE) <-> "bgez" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13),

    /*! bltz rs, offset  =>  blt rs, x0, offset */
    BTYPE(imm, x0, rs1, BLT) <-> "bltz" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13),

    /*! bgtz rs, offset  =>  blt x0, rs, offset */
    BTYPE(imm, rs1, x0, BLT) <-> "bgtz" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13),

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
    JAL(imm, x0) <-> "j" ^ spc(()) ^ label(imm, 21),

    /*! jr rs  =>  jalr x0, rs, 0 */
    JALR(imm, rs1, x0) <-> "jr" ^ spc(()) ^ hex_bits_signed_12(imm) ^ "(" ^ reg_name(rs1) ^ ")",

    /*!
        jalr rs  =>  jalr x1, rs, 0

        Procedure returns in the standard calling convention (assembler pseudoinstruction RET)
        are encoded as a JALR with rd=x0, rs1=x1, and imm=0.
    */
    JALR((0 : bitvector(12)), x1, x0) <-> "ret",

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

    /*! vmmv.m vd, vs  =>  vmand.mm vd, vs, vs */
    MMTYPE(MM_VMAND, vs, vs, vd) <-> "vmmv.m" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs),

    /*! vmnot.m vd, vs  =>  vmnand.mm vd, vs, vs */
    MMTYPE(MM_VMNAND, vs2, vs1, vd) <-> "vmnot.m" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ sep(()) ^ vreg_name(vs1),

    /*! vmset.m vd  =>  vmxnor.mm vd, vd, vd */
    MMTYPE(MM_VMXNOR, vs2, vs1, vd) <-> "vmset.m" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ sep(()) ^ vreg_name(vs1),

    /*! vmsge.vi vd, va, i, vm  =>  vmsgt.vi vd, va, i-1, vm */
    VICMPTYPE(VICMP_VMSGT, vm, vs2, simm, vd) <-> "vmsge.vi" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ sep(()) ^ simm5_plus_1(simm) ^ maybe_vmask(vm),

    /*! vmsgeu.vi vd, va, i, vm  =>  vmsgtu.vi vd, va, i-1, vm */
    VICMPTYPE(VICMP_VMSGTU, vm, vs2, simm, vd) <-> "vmsge.vi" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ sep(()) ^ simm5_plus_1(simm) ^ maybe_vmask(vm),

    /*! vmsge.vv vd, va, vb, vm  =>  vmsle.vv vd, vb, va, vm */
    VVCMPTYPE(VVCMP_VMSLE, vm, vs2, vs1, vd) <-> "vmsge.vv" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ sep(()) ^ vreg_name(vs1) ^ maybe_vmask(vm),

    /*! vmsgeu.vv vd, va, vb, vm  =>  vmsleu.vv vd, vb, va, vm */
    VVCMPTYPE(VVCMP_VMSLEU, vm, vs2, vs1, vd) <-> "vmsgeu.vv" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ sep(()) ^ vreg_name(vs1) ^ maybe_vmask(vm),

    /*! vmsgt.vv vd, va, vb, vm  =>  vmslt.vv vd, vb, va, vm */
    VVCMPTYPE(VVCMP_VMSLT, vm, vs2, vs1, vd) <-> "vmsgt.vv" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ sep(()) ^ vreg_name(vs1) ^ maybe_vmask(vm),

    /*! vmsgtu.vv vd, va, vb, vm  =>  vmsltu.vv vd, vb, va, vm */
    VVCMPTYPE(VVCMP_VMSLTU, vm, vs2, vs1, vd) <-> "vmsgtu.vv" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ sep(()) ^ vreg_name(vs1) ^ maybe_vmask(vm),

    /*! vmslt.vi vd, va, i, vm  =>  vmsle.vi vd, va, i-1, vm */
    VICMPTYPE(VICMP_VMSLE, vm, vs2, simm, vd) <-> "vmslt.vi" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ sep(()) ^ simm5_plus_1(simm) ^ maybe_vmask(vm),

    /*! vmsltu.vi vd, va, i, vm  =>  vmsleu.vi vd, va, i-1, vm */
    VICMPTYPE(VICMP_VMSLEU, vm, vs2, simm, vd) <-> "vmsltu.vi" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ sep(()) ^ simm5_plus_1(simm) ^ maybe_vmask(vm),

    /*! vneg.v vd,vs  =>  vrsub.vx vd,vs,x0 */
    VXTYPE(VX_VRSUB, vm, vs2, x0, vd) <-> "vneg.v" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ maybe_vmask(vm),

    /*! vnot.v vd,vs,vm  =>  vxor.vi vd, vs, -1, vm */
    VITYPE(VI_VXOR, vm, vs2, (-1 : bitvector(5)), vd) <-> "vnot.v" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ maybe_vmask(vm),

    /*! vncvt.x.x.w vd,vs,vm  =>  vnsrl.wx vd,vs,x0,vm */
    NXSTYPE(NXS_VNSRL, vm, vs2, x0, vd) <-> "vncvt.x.x.w" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ maybe_vmask(vm),

    /*! vwcvt.x.x.v vd,vs,vm  =>  vwadd.vx vd,vs,x0,vm */
    WVXTYPE(WVX_VADD, vm, vs2, x0, vd) <-> "vwcvt.x.x.v" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ maybe_vmask(vm),

    /*! vwcvtu.x.x.v vd,vs,vm  =>  vwaddu.vx vd,vs,x0,vm */
    WVXTYPE(WVX_VADDU, vm, vs2, x0, vd) <-> "vwcvtu.x.x.v" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ vreg_name(vs2) ^ maybe_vmask(vm),

    /*!
        vl1r.v vd, rs  =>  vl1re8.v vd, rs
        vl2r.v vd, rs  =>  vl1re8.v vd, rs
        vl4r.v vd, rs  =>  vl1re8.v vd, rs
        vl8r.v vd, rs  =>  vl1re8.v vd, rs
    */
    VLRETYPE(nf, rs1, VLE8, vd) <-> "vl" ^ nfields_pow2_string(nf) ^ "r.v" ^ spc(()) ^ vreg_name(vd) ^ sep(()) ^ "(" ^ reg_name(rs1) ^ ")",

    /*! fence  =>  fence iorw, iorw */
    FENCE((0b0000 : bitvector(4)), (0b1111 : bitvector(4)), (0b1111 : bitvector(4)), x0, x0) <-> "fence",

    /*! pause  =>  fence w, 0

        According to the spec, that instruction is provided for extension Zihintpause
    */

    /*!
        The assembler pseudoinstruction to read a CSR, CSRR rd, csr, is encoded as CSRRS rd, csr, x0.
    */
    CSRReg(csr, x0, rd, CSRRS) <-> "csrr" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ csr_name_map(csr),

    /*!
        The assembler pseudoinstruction to write a CSR, CSRW csr, rs1, is encoded as CSRRW x0, csr, rs1
    */
    CSRReg(csr, rs, x0, CSRRW) <-> "csrw" ^ spc(()) ^ csr_name_map(csr) ^ sep(()) ^ reg_name(rs),

    /*! csrs csr, rs  =>  csrrs x0, csr, rs */
    CSRReg(csr, x0, rs, CSRRS) <-> "csrs" ^ spc(()) ^ csr_name_map(csr) ^ sep(()) ^ reg_name(rs),

    /*! csrc csr, rs  =>  csrrc x0, csr, rs */
    CSRReg(csr, x0, rs, CSRRC) <-> "csrc" ^ spc(()) ^ csr_name_map(csr) ^ sep(()) ^ reg_name(rs),

    /*! csrwi csr, imm  =>  csrrwi x0, csr, imm */
    CSRImm(csr, imm, x0, CSRRW) <-> "csrwi" ^ spc(()) ^ csr_name_map(csr) ^ sep(()) ^ hex_bits_5(imm),

    /*! csrsi csr, imm  =>  csrrsi x0, csr, imm */
    CSRImm(csr, imm, x0, CSRRS) <-> "csrsi" ^ spc(()) ^ csr_name_map(csr) ^ sep(()) ^ hex_bits_5(imm),

    /*! csrci csr, imm  =>  csrrci x0, csr, imm */
    CSRImm(csr, imm, x0, CSRRC) <-> "csrci" ^ spc(()) ^ csr_name_map(csr) ^ sep(()) ^ hex_bits_5(imm),

    /*! frcsr rd  =>  csrrs rd, fcsr, x0 */
    CSRReg(csr_name_map("fcsr"), x0, rd, CSRRS) <-> "frcsr" ^ spc(()) ^ reg_name(rd),

    /*! fscsr rd, rs  =>  csrrw rd, fcsr, rs */
    CSRReg(csr_name_map("fcsr"), rs, rd, CSRRW) <-> "fscsr" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! fscsr rs  =>  csrrw x0, fcsr, rs */
    CSRReg(csr_name_map("fcsr"), rs, x0, CSRRW) <-> "fscsr" ^ spc(()) ^ reg_name(rs),

    /*! frrm rd  =>  csrrs rd, frm, x0 */
    CSRReg(csr_name_map("frm"), x0, rd, CSRRS) <-> "frrm" ^ spc(()) ^ reg_name(rd),

    /*! fsrm rd, rs  =>  csrrw rd, frm, rs */
    CSRReg(csr_name_map("frm"), rs, rd, CSRRW) <-> "fsrm" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! fsrm rs  =>  csrrw x0, frm, rs */
    CSRReg(csr_name_map("frm"), rs, x0, CSRRW) <-> "fsrm" ^ spc(()) ^ reg_name(rs),

    /*! fsrmi rd, imm  =>  csrrwi rd, frm, imm */
    CSRImm(csr_name_map("frm"), imm, rd, CSRRW) <-> "fsrmi" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ hex_bits_5(imm),

    /*! fsrmi imm  =>  csrrwi x0, frm, imm */
    CSRImm(csr_name_map("frm"), imm, x0, CSRRW) <-> "fsrmi" ^ spc(()) ^ hex_bits_5(imm),

    /*! frflags rd  =>  csrrs rd, fflags, x0 */
    CSRReg(csr_name_map("fflags"), x0, rd, CSRRS) <-> "frflags" ^ spc(()) ^ reg_name(rd),

    /*! fsflags rd, rs  =>  csrrw rd, fflags, rs */
    CSRReg(csr_name_map("fflags"), rs, rd, CSRRW) <-> "fsflags" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs),

    /*! fsflags rs  =>  csrrw x0, fflags, rs */
    CSRReg(csr_name_map("fflags"), rs, x0, CSRRW) <-> "fsflags" ^ spc(()) ^ reg_name(rs),

    /*! fsflagsi rd, imm  =>  csrrwi rd, fflags, imm */
    CSRImm(csr_name_map("fflags"), imm, rd, CSRRW) <-> "fsflagsi" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ hex_bits_5(imm),

    /*! fsflagsi imm  =>  csrrwi x0, fflags, imm */
    CSRImm(csr_name_map("fflags"), imm, x0, CSRRW) <-> "fsflagsi" ^ spc(()) ^ hex_bits_5(imm)
}
