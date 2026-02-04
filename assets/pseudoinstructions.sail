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

    /*!
        Plain unconditional jumps (assembler pseudoinstruction J) are encoded as a JAL with rd=x0.
    */
    JAL(imm, zreg) <-> "j" ^ spc(()) ^ label(imm, 21),

    /*!
        Plain unconditional indirect jumps (assembler pseudoinstruction JR)
        are encoded as a JALR with rd=x0.
    */
    JALR(imm, rs1, zreg) <-> "jr" ^ spc(()) ^ hex_bits_signed_12(imm) ^ "(" ^ reg_name(rs1) ^ ")",

    /*!
        Procedure returns in the standard calling convention (assembler pseudoinstruction RET)
        are encoded as a JALR with rd=x0, rs1=x1, and imm=0.
    */
    JALR((0 : bitvector(12)), x1, zreg) <-> "ret",

    /*!
        The assembler pseudoinstruction to read a CSR, CSRR rd, csr, is encoded as CSRRS rd, csr, x0.
    */
    CSRReg(csr, zreg, rd, CSRRS) <-> "csrr" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ csr_name_map(csr),

    /*!
        The assembler pseudoinstruction to write a CSR, CSRW csr, rs1, is encoded as CSRRW x0, csr, rs1
    */
    CSRReg(csr, rs1, zreg, CSRRW) <-> "csrw" ^ spc(()) ^ csr_name_map(csr) ^ sep(()) ^ reg_name(rs1),

    /*!
        [...] while CSRWI csr, uimm, is encoded as CSRRWI x0, csr, uimm.

        Further assembler pseudoinstructions are defined to set and clear bits in the CSR when the old value
is not required: CSRS/CSRC csr, rs1; CSRSI/CSRCI csr, uimm.
    */

    BTYPE(imm, zreg, rs1, BEQ) <-> "beqz" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13),
    BTYPE(imm, zreg, rs1, BNE) <-> "bnez" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13),
    BTYPE(imm, zreg, rs1, BGE) <-> "bgez" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13),
    BTYPE(imm, zreg, rs1, BLT) <-> "bltz" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ label(imm, 13)
}
