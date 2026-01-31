val pseudoinstructions : instruction <-> string

mapping pseudoinstructions = {
    /*!
        ADDI rd, rs1, 0 is used to implement the MV rd, rs1 assembler pseudoinstruction
    */
    ITYPE((0 : bitvector(12)), rs1, rd, ADDI) <-> "mv" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs1),

    /*!
         Note, SLTIU rd, rs1, 1 sets rd to 1 if rs1 equals zero, otherwise
         sets rd to 0 (assembler pseudoinstruction SEQZ rd, rs)
    */
    ITYPE((1 : bitvector(12)), rs1, rd, SLTIU) <-> "seqz" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs1),

    /*!
        Note, XORI rd, rs1, -1 performs a bitwise logical inversion of register
        rs1 (assembler pseudoinstruction NOT rd, rs).
    */
    ITYPE((-1 : bitvector(12)), rs1, rd, XORI) <-> "not" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs1),

    /*!
         Note, SLTU rd, x0, rs2 sets rd to 1 if rs2 is not equal to zero, otherwise
         sets rd to zero (assembler pseudoinstruction SNEZ rd, rs)
    */
    RTYPE(zreg, rs1, rd, SLTU) <-> "snez" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs1),

    /*!
        Plain unconditional jumps (assembler pseudoinstruction J) are encoded as a JAL with rd=x0.
    */
    JAL(imm, zreg) <-> "j" ^ spc(()) ^ hex_bits_signed_21(imm),

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
        Note, ADDIW rd, rs1, 0 writes the sign extension of the lower 32 bits of register
        rs1 into register rd (assembler pseudoinstruction SEXT.W).
    */
    ADDIW((0 : bitvector(12)), rs1, rd) <-> "sext.w" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ reg_name(rs1),

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


    BTYPE(imm, zreg, rs1, BEQ) <-> "beqz" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ resolve_label(imm, 13),
    BTYPE(imm, zreg, rs1, BNE) <-> "bnez" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ resolve_label(imm, 13),
    BTYPE(imm, zreg, rs1, BGE) <-> "bgez" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ resolve_label(imm, 13),
    BTYPE(imm, zreg, rs1, BLT) <-> "bltz" ^ spc(()) ^ reg_name(rs1) ^ sep(()) ^ resolve_label(imm, 13)
}
