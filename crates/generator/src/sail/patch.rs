use crate::model::FunctionInvocation;
use crate::model::Mapping;
use crate::model::Pair;
use crate::model::Value;

pub fn patch_assembly_definitions(mapping: &mut Mapping) {
    patch_c_nop(mapping);
    patch_load(mapping);
}

fn patch_c_nop(mapping: &mut Mapping) {
    /*
        before:

        C_NOP(0b000000) <-> "c.nop",
        C_NOP(imm) if neq_bits(imm, zeros(6)) <-> "c.nop" ^ spc(()) ^ hex_bits_signed_6(imm)

        after:

        C_NOP(imm) <-> "c.nop" ^ spc(()) ^ maybe_nonzero_imm_6(imm)
    */

    mapping.pairs.retain(|pair| {
        if let Value::FunctionInvocation(f) = &pair.lhs {
            f.name != "C_NOP"
        } else {
            true
        }
    });

    let var = Value::Symbol("imm".to_string());

    fn mk_fun(name: &str, args: Vec<Value>) -> Value {
        let fun = FunctionInvocation {
            name: name.to_string(),
            args,
        };

        Value::FunctionInvocation(fun)
    }

    let lhs = mk_fun("C_NOP", vec![var.clone()]);

    let rhs = Value::StringConcatenation(vec![
        Value::String("c.nop".to_string()),
        mk_fun("spc", vec![]),
        mk_fun("maybe_nonzero_imm_6", vec![var.clone()]),
    ]);

    mapping.pairs.push(Pair {
        lhs,
        rhs,
        cond: None,
    })
}

fn patch_load(mapping: &mut Mapping) {
    /*
        before:

        LOAD(imm, rs1, rd, is_unsigned, width) <-> "l" ^ width_mnemonic(width) ^ maybe_u(is_unsigned) ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ hex_bits_signed_12(imm) ^ "(" ^ reg_name(rs1) ^ ")",

        after -- in practice offset is optional

        LOAD(imm, rs1, rd, is_unsigned, width) <-> "l" ^ width_mnemonic(width) ^ maybe_u(is_unsigned) ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ optional_signed_12(imm) ^ "(" ^ reg_name(rs1) ^ ")",
    */
    let pair = mapping
        .pairs
        .iter_mut()
        .find(|pair| is_function(&pair.lhs, "LOAD"))
        .expect("LOAD not found");

    let Value::StringConcatenation(args) = &mut pair.rhs else {
        panic!("expected string concatenation");
    };

    let pos = args
        .iter()
        .position(|v| is_function(v, "hex_bits_signed_12"))
        .expect("'hex_bits_signed_12' not found");

    let Value::FunctionInvocation(fun) = &mut args[pos] else {
        unreachable!();
    };

    fun.name = "optional_signed_12".to_string();
}

fn is_function(v: &Value, name: &str) -> bool {
    let Value::FunctionInvocation(fun) = v else {
        return false;
    };

    fun.name == name
}
