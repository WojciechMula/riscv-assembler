================================================================================
                RISC-V assembler based on Sail spec
================================================================================

This is a programming library providing a procedure for converting **a single
RISC-V instruction** into its binary representation::

    fn assemble(asm: &str) -> Result<u32, ErrorType>

For example::

    let instr = assemble("addi x1, x2, 42").expect("this should be a valid instr")

The goal of this project is to use `The Formal Specification of RISC-V 
<https://github.com/riscv/sail-riscv/>`_ to obtain the expected assembler
syntax and appropriate binary representation. The spec uses `Sail language
<https://github.com/rems-project/sail>`_.

The major goal when creating this library was to minimize the amount of
manual work. Ideally, refreshing the parser with the latest spec would not
require any manual work.
