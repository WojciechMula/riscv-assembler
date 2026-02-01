use crate::err;
use crate::errfmt;
use crate::generator::Extensions;
use crate::generator::Instruction;
use crate::generator::ResolvedTypes;
use crate::generator::csr::CsrList;
use crate::generator::resolve_conditions::resolve_arch_condition;
use crate::generator::resolve_conditions::resolve_runtime_cond;
use crate::generator::resolve_conditions::simplify_condition;
use crate::generator::resolve_enums::ResolvedEnum;
use crate::generator::rust::bitvector_constructor;
use crate::generator::rust::evaluate_bitvector_cast;
use crate::generator::rust::format_bitvector;
use crate::generator::rust::rust_constructor;
use crate::generator::rust::rust_expression;
use crate::generator::rust_signature::RustSignatureBuilder;
use crate::model::BinaryConcatenation;
use crate::model::BinaryConstructor;
use crate::model::BitVector;
use crate::model::FunctionInvocation;
use crate::model::MappingSignature;
use crate::model::StructSignature;
use crate::model::Type;
use crate::model::Union;
use crate::model::Value;
use crate::sail::Sail;
use log::debug;
use log::error;
use log::warn;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;

macro_rules! w {
    ($f:expr, $($args:expr),*) => {
        $f.push_str(format!($($args),*).as_str());
        $f.push('\n')
    };
    ($target:expr) => {
        $f.push('\n')
    };
}

type RustSymbol = String;
type RustTypename = String;
type RustName = String;
type Mnemonic = String;
type RustCode = String;

#[derive(Default)]
pub struct CodeGenerator {
    emit_comments: bool,
    signatures: Union,
    types: ResolvedTypes,
    csr: CsrList,
    extensions: Extensions,

    structs: Vec<RustCode>,
    enums: Vec<RustCode>,
    parsers: Vec<RustCode>,
    encoder_names: HashSet<RustName>,
    encoders: Vec<RustCode>,
    mnemonic2parser: BTreeMap<Mnemonic, RustName>,
    needs_ctx: HashSet<RustName>,
    needs_parser: HashSet<RustName>,
    needs_label: HashSet<RustName>,
}

impl CodeGenerator {
    pub fn new(
        signatures: Union,
        types: ResolvedTypes,
        csr: CsrList,
        extensions: Extensions,
    ) -> Self {
        let mut result = Self {
            signatures,
            types,
            csr,
            extensions,
            emit_comments: true,
            ..Self::default()
        };

        for resolved_enum in result.types.enums.values() {
            result.enums.push(Self::emit_enum(resolved_enum));
            if !resolved_enum.enum_to_bitvector.is_empty() {
                result
                    .encoders
                    .push(Self::emit_binary_encoder(resolved_enum));
            }
        }

        for resolved_struct in result.types.structs.values() {
            result.structs.push(Self::emit_struct(resolved_struct));
        }

        result
    }

    fn emit_enum(resolved: &ResolvedEnum) -> String {
        let mut f = String::new();

        w!(f, "#[allow(non_camel_case_types)]");
        w!(f, "pub(crate) enum {} {{", resolved.typename);
        for item in &resolved.items {
            w!(f, "  {},", item.label);
        }
        w!(f, "}}");

        f
    }

    fn emit_struct(s: &StructSignature) -> String {
        let mut f = String::new();

        w!(f, "#[allow(non_camel_case_types)]");
        w!(f, "pub(crate) struct {} {{", s.name);
        for (name, typ) in &s.fields {
            w!(f, "  pub(crate) {}: {},", name, rust_typename(typ));
        }
        w!(f, "}}");

        f
    }

    fn emit_binary_encoder(v: &ResolvedEnum) -> String {
        let mut f = String::new();

        let bit_width = v.items[0].value.bit_width;

        w!(
            f,
            "fn {}(v: {}) -> {} {{",
            v.enum_to_bitvector,
            v.typename,
            rust_bitvector_type(bit_width)
        );
        w!(f, "match v {{");

        for item in &v.items {
            w!(
                f,
                "{}::{} => {},",
                v.typename,
                item.label,
                rust_bitvector_safe_new(&item.value)
            );
        }

        w!(f, "}}");
        w!(f, "}}");

        f
    }

    pub fn add_instruction(&mut self, instruction: &Instruction, sail: &Sail) -> crate::Result<()> {
        let encoder_name = encoder_fn_name(instruction);
        if self.encoder_names.insert(encoder_name.clone()) {
            let (encoder, needs_ctx) =
                Self::emit_instruction_encoder(&encoder_name, instruction, &self.signatures, sail)?;
            self.encoders.push(encoder);
            if needs_ctx {
                self.needs_ctx.insert(encoder_name);
            }
        }

        let parser_name = parser_fn_name(instruction);
        let (parser, needs_ctx, needs_parser, needs_label) =
            self.emit_instruction_parser(&parser_name, instruction, sail)?;
        self.parsers.push(parser);
        if needs_ctx {
            self.needs_ctx.insert(parser_name.clone());
        }
        if needs_parser {
            self.needs_parser.insert(parser_name.clone());
        }
        if needs_label {
            self.needs_label.insert(parser_name.clone());
        }

        self.mnemonic2parser
            .insert(instruction.mnemonic.clone(), parser_name);

        Ok(())
    }

    pub fn write_file(&self, output: &Path) -> crate::Result<()> {
        let mut f = File::create(output)?;

        writeln!(f, "// Code generated automatically; DO NOT EDIT")?;
        writeln!(f, "use crate::helpers::*;")?;
        writeln!(f, "use crate::err;")?;

        f.write_all(self.emit_dispatcher_function().as_bytes())?;
        f.write_all(self.csr.rust()?.as_bytes())?;
        f.write_all(self.extensions.rust().as_bytes())?;

        for code in &self.enums {
            f.write_all(code.as_bytes())?;
        }

        for code in &self.structs {
            f.write_all(code.as_bytes())?;
        }

        for code in &self.parsers {
            f.write_all(code.as_bytes())?;
        }

        for code in &self.encoders {
            f.write_all(code.as_bytes())?;
        }

        Ok(())
    }

    fn emit_instruction_parser(
        &self,
        name: &str,
        instruction: &Instruction,
        sail: &Sail,
    ) -> crate::Result<(RustCode, bool, bool, bool)> {
        let mut f = String::new();

        let encoder_name = encoder_fn_name(instruction);
        let encoder_needs_ctx = self.needs_ctx.contains(&encoder_name);
        let mut needs_ctx = encoder_needs_ctx;
        let needs_parser = !instruction.string.args.is_empty();
        let mut needs_label = false;

        if !instruction.constants.is_empty() {
            if self.emit_comments {
                w!(f, "// predefined constants");
            }
            for (name, val) in &instruction.constants {
                match val {
                    Value::Symbol(ident) => {
                        w!(f, "let {name} = {ident};");
                    }
                    Value::Boolean(flag) => {
                        w!(f, "let {name} = {flag};");
                    }
                    Value::Integer(value) => {
                        w!(f, "let {name} = {value};");
                    }
                    Value::Struct(structure) => {
                        w!(f, "let {name} = {}{{", structure.typename);
                        for (fieldname, value) in &structure.values {
                            w!(f, "{fieldname}: {},", rust_expression(&value));
                        }
                        w!(f, "}};");
                    }
                    Value::BitVector(bv) => {
                        w!(f, "let {name} = {};", rust_bitvector_safe_new(bv));
                    }
                    Value::EnumLabel(..) => {
                        w!(f, "let {name} = {};", rust_expression(val));
                    }
                    _ => return err!("unsupported value {val:?}"),
                }
            }
        }

        if self.emit_comments && !instruction.string.args.is_empty() {
            w!(f, "// parse arguments");
        }
        for val in &instruction.string.args {
            match val {
                Value::FunctionInvocation(FunctionInvocation { name, args }) => {
                    if let Some(name) = crate::custom_function(name) {
                        match name {
                            _ => todo!(),
                        }
                    } else if name == "sep" {
                        w!(f, "parser.expect_comma()?;");
                    } else if name == "opt_spc" || name == "spc" {
                        w!(f, "parser.skip_ws();");
                    } else if name == "sp_reg_name" {
                        crate::assert_equals(args.len(), 0, "".to_string())?;
                        w!(f, "{name}(parser)?;");
                    } else if name == "maybe_nonzero_imm_6" {
                        crate::assert_equals(args.len(), 1, "".to_string())?;
                        let binding = args[0].as_symbol()?;
                        w!(f, "let {binding} = {name}(parser)?;");
                    } else if name == "csr_name_map" {
                        crate::assert_equals(args.len(), 1, "".to_string())?;
                        let binding = args[0].as_symbol()?;
                        w!(f, "let {binding} = {name}(parser)?;");
                    } else if name == "optional_signed_12" {
                        crate::assert_equals(args.len(), 1, "".to_string())?;
                        let binding = args[0].as_symbol()?;
                        w!(f, "let {binding} = {name}(parser)?;");
                    } else if name == "resolve_label" {
                        crate::assert_equals(args.len(), 2, "".to_string())?;
                        let binding = args[0].as_symbol()?;
                        w!(f, "let {binding} = {name}::<13>(parser, l)?;"); // XXX
                        needs_label = true;
                    } else {
                        let sig = sail.mapping_signature(name)?;

                        crate::assert_equals(args.len(), 1, errfmt!("{name}"))?;
                        let tmp = "tmp";
                        let binding = match &args[0] {
                            Value::Symbol(binding) => binding,
                            Value::BinaryConcatenation(_) | Value::Tuple(_) => tmp,
                            _ => return err!("unsupported `{:?}`", args[0]),
                        };

                        if let Some((bit_width, signed)) = is_number_conversion(name, &sig) {
                            if signed {
                                w!(
                                    f,
                                    "let {binding} = parser.expect_signed_immediate::<{bit_width}>()?;"
                                );
                            } else {
                                w!(
                                    f,
                                    "let {binding} = parser.expect_unsigned_immediate::<{bit_width}>()?;"
                                );
                            }
                        } else {
                            w!(f, "let {binding} = {name}(parser)?;");
                        }

                        if let Value::BinaryConcatenation(bc) = &args[0] {
                            if bc.needs_normalisation() {
                                let bc = bc.normalize(&sig.lhs)?;

                                f += &make_binary_concatenation_deconstruction(binding, &bc)?;
                            } else {
                                f += &make_binary_concatenation_deconstruction(binding, bc)?;
                            }
                        }

                        if let Value::Tuple(args) = &args[0] {
                            let fn_sig = sail.mapping_signature(name)?;
                            let types = fn_sig.rhs.as_tuple()?;
                            let mut shift = 0;

                            if args.len() != types.len() {
                                return err!(
                                    "incompatible values and type: `{args:?}` and `{types:?}`"
                                );
                            }

                            for i in (0..args.len()).rev() {
                                let symbol = args[i].as_symbol()?;
                                let bit_width = types[i].as_bitvector()?;

                                w!(
                                    f,
                                    "let val = ({binding} >> {shift}) & ((1 << {bit_width}) - 1);"
                                );
                                let bit_vector = rust_bitvector_type(bit_width);
                                w!(f, "let {symbol} = {bit_vector}::new(val);");

                                shift += bit_width;
                            }
                        }
                    }
                }
                Value::String(fixed) => {
                    w!(f, "parser.expect(\"{fixed}\")?;");
                }
                _ => return err!("unsupported `{val:?}`"),
            }
        }

        if let Some(cond) = &instruction.string.cond {
            w!(f, "{}", Self::emit_condition(cond, &mut needs_ctx)?);
        }

        w!(f, "{}(", encoder_name);
        for (i, arg) in instruction.constructor.args.iter().enumerate() {
            if i > 0 {
                w!(f, ", ");
            }

            match arg {
                Value::Symbol(binding) => {
                    w!(f, "{binding}");
                }
                Value::Boolean(v) => {
                    w!(f, "{v}");
                }
                Value::Integer(v) => {
                    w!(f, "0x{v:x}");
                }
                Value::BitVector(BitVector { val, bit_width }) => {
                    w!(f, "0b{:01$b}", val, bit_width);
                }
                Value::EnumLabel(..) => {
                    w!(f, "{}", rust_expression(arg));
                }
                Value::Cast(value, typ) => {
                    let bv = evaluate_bitvector_cast(value, typ)?;
                    w!(f, "{}", bitvector_constructor(&bv));
                }
                _ => return err!("unsupported {arg:?}"),
            }
        }

        if encoder_needs_ctx {
            if !instruction.constructor.args.is_empty() {
                w!(f, ", ");
            }

            w!(f, "ctx");
        }

        w!(f, ")");

        let mut args = Vec::<String>::new();
        if needs_parser {
            args.push("parser: &mut Parser".into());
        }
        if needs_ctx {
            args.push("ctx: &Context".into());
        }
        if needs_label {
            args.push("l: &mut dyn LabelResolverTrait".into());
        }

        let args = args.join(", ");

        let mut s = String::new();
        w!(s, "fn {name}({args}) -> crate::Result<u32> {{");

        w!(s, "{f}");
        w!(s, "}}");

        Ok((s, needs_ctx, needs_parser, needs_label))
    }

    fn emit_instruction_encoder(
        name: &str,
        instruction: &Instruction,
        signatures: &Union,
        sail: &Sail,
    ) -> crate::Result<(RustCode, bool)> {
        let mut f = String::new();
        let constr_name = &instruction.binary[0].constr.name;
        let Some(sail_sig) = signatures.0.get(constr_name) else {
            error!("{instruction:?}");
            return err!("signature for `{constr_name}` not found");
        };

        let mut needs_ctx = false;

        // 1. find out function signature
        let mut rust_sig = mk_rust_signature(&instruction.binary, sail_sig)?;

        // 2. emit argument deconstruction (if any)
        for (i, item) in instruction.binary[0].constr.args.iter().enumerate() {
            match item {
                Value::Symbol(_) => (),
                Value::EnumLabel(_) => (),
                Value::BitVector(..) => (),
                Value::BinaryConcatenation(args) => {
                    let arg_name = &rust_sig[i].0;
                    f += &make_binary_concatenation_deconstruction(arg_name, args)?;
                }
                _ => {
                    return err!("argument #{i}: unexpected argument {item:?}");
                }
            }
        }

        // 3. actual word assembling
        w!(f, "// instruction assembling");
        if instruction.binary.len() > 1 {
            let fragments = make_binary_matches(&rust_sig, &instruction.binary)?;
            if !fragments.is_empty() {
                w!(f, "match {} {{", fragments[0]);
                for (j, binary) in instruction.binary.iter().enumerate() {
                    if let Some(cond) = &binary.cond {
                        w!(f, "{} => {{", fragments[j + 1]);
                        w!(f, "{}", Self::emit_condition(cond, &mut needs_ctx)?);
                        w!(f, "Ok({})", Self::emit_binary_encoding(binary, sail)?);
                        w!(f, "}},");
                    } else {
                        w!(
                            f,
                            "{} => Ok({}),",
                            fragments[j + 1],
                            Self::emit_binary_encoding(binary, sail)?
                        );
                    }
                }
                w!(f, "}}");
            } else {
                needs_ctx = true;
                // Note: For almost all cases, the invocation of binary constructor
                //       differs, so we can obtain a proper match. However, for the
                //       REV8 the constructors are exactly the same, only the conditions
                //       are used to distinguish the cases. Added minimal code to support
                //       this special case.
                for binary in &instruction.binary {
                    let Some(cond) = &binary.cond else {
                        continue;
                    };

                    let cond = simplify_condition(cond);

                    let Some(c) = resolve_arch_condition(&cond) else {
                        return err!("{:?} is not a single conditon", cond);
                    };

                    w!(f, "if {} {{", c.rust_expr);
                    w!(
                        f,
                        "  return Ok({})",
                        Self::emit_binary_encoding(binary, sail)?
                    );
                    w!(f, "}}");
                }

                w!(
                    f,
                    "err!(\"instruction is available conditionally, none of conditions met\")"
                );
            }
        } else {
            f += "Ok(";
            if Self::is_illegal(instruction) {
                f += &Self::emit_illegal_encoding(instruction)?;
            } else {
                f += &Self::emit_binary_encoding(&instruction.binary[0], sail)?;
            }
            f += ")\n";
        }

        // build function signature
        if needs_ctx {
            rust_sig.push(("ctx".into(), "&Context".into()));
        }

        let mut s = String::new();
        s += &format!("fn {name}(");
        for (i, (arg, rust_type)) in rust_sig.iter().enumerate() {
            if i > 0 {
                s += ", ";
            }

            s += &format!("{arg}: {rust_type}");
        }

        s += ") -> crate::Result<u32> {\n";

        s += &f;
        w!(s, "}}");

        Ok((s, needs_ctx))
    }

    fn is_illegal(instr: &Instruction) -> bool {
        matches!(instr.constructor.name.as_str(), "ILLEGAL" | "C_ILLEGAL")
    }

    fn emit_illegal_encoding(instr: &Instruction) -> crate::Result<String> {
        let bit_width = instr.signature.as_bitvector()?;
        let ident = instr.binary[0].args[0].as_symbol()?; // it should be BitVector<32> or BitVector<16>
        match bit_width {
            32 => Ok(format!("{ident}.val")),
            16 => Ok(format!("{ident}.val as u32")),
            _ => err!("unsupported type"),
        }
    }

    fn emit_condition(cond: &Value, needs_ctx: &mut bool) -> crate::Result<String> {
        let mut f = String::new();

        let mut expr = vec![cond.clone()];
        if let Ok(FunctionInvocation { name, args }) = &cond.as_fn_call() {
            if name == "and_bool" {
                expr = args.clone();
            }
        }

        for cond in expr {
            if let Some(c) = resolve_arch_condition(&cond) {
                w!(f, "if !({}) {{", c.rust_expr);
                w!(f, "return err!(\"{}\");", c.error_msg);
                w!(f, "}}");
                *needs_ctx = true;
            } else if let Some(c) = resolve_runtime_cond(&cond) {
                w!(f, "if !({}) {{", c.rust_expr);
                w!(f, "return err!(\"{}\");", c.error_msg);
                w!(f, "}}");
            } else {
                warn!("unsupported condition {cond:?}");
            }
        }

        Ok(f)
    }

    fn emit_binary_encoding(binary: &BinaryConstructor, sail: &Sail) -> crate::Result<String> {
        let mut f = String::new();

        let mut offset = 0;
        let n = binary.args.len();
        for (i, val) in binary.args.iter().rev().enumerate() {
            match val {
                Value::BitVector(bv) => {
                    f += &format!("({}_u32 << {})", rust_bitvector_val(bv), offset);
                    offset += bv.bit_width;
                }
                Value::FunctionInvocation(call) => {
                    if let Some(name) = crate::custom_function(&call.name) {
                        match name {
                            "bitvector_subvector" => {
                                let symbol = call.args[0].as_symbol()?;
                                let hi = call.args[1].as_integer()?;
                                let lo = call.args[1].as_integer()?;

                                let bit_width = hi - lo + 1;
                                let mask = (1 << bit_width) - 1;

                                let rust = format!("(({symbol}.val >> {lo}) & 0b{mask:b})");
                                f += &rust;

                                offset += bit_width as usize;
                            }
                            _ => {
                                todo!();
                            }
                        }
                    } else {
                        let sig = sail.mapping_signature(&call.name)?;

                        let bit_width = if let Type::BitVector(bv) = &sig.rhs {
                            *bv
                        } else if let Type::BitVector(bv) = &sig.lhs {
                            *bv
                        } else {
                            return err!(
                                "mapping {} does not map bitvector; its signature is {:?}",
                                call.name,
                                sig
                            );
                        };

                        f += &format!("({}.val << {})", rust_fn_call(call), offset);
                        offset += bit_width;
                    }
                }
                Value::Cast(ident, typ) => {
                    let bit_width = typ.as_bitvector()?;
                    let ident = ident.as_ref().as_symbol()?;

                    f += &format!("({}.val << {})", ident, offset);
                    offset += bit_width;
                }
                Value::Symbol(ident) => {
                    w!(f, "{ident}");
                }
                _ => {
                    debug!("{binary:?}");
                    panic!("unexpected {val:?}");
                }
            }

            if i + 1 < n {
                f += " |\n";
            } else {
                f += "\n";
            }
        }

        Ok(f)
    }

    fn emit_dispatcher_function(&self) -> String {
        let mut f = String::new();

        w!(
            f,
            "pub fn assemble(s: &str, ctx: &Context, l: &mut dyn LabelResolverTrait) -> crate::Result<u32> {{"
        );
        w!(f, "let mut p = Parser::new(s);");
        w!(f, "let ident = p.consume_instruction();");
        w!(f, "match ident {{");

        for (mnemonic, parser) in &self.mnemonic2parser {
            let mut args = Vec::<&str>::with_capacity(2);
            if self.needs_parser.contains(parser) {
                args.push("&mut p");
            }
            if self.needs_ctx.contains(parser) {
                args.push("ctx");
            }
            if self.needs_label.contains(parser) {
                args.push("l");
            }

            let args = args.join(", ");
            w!(f, "\"{mnemonic}\" => {parser}({args}),");
        }
        w!(
            f,
            "_ => err!(\"`{{ident}}` is not a recognized instruction\"),"
        );
        w!(f, "}}");
        w!(f, "}}");

        f
    }
}

fn mk_rust_signature(
    constructors: &[BinaryConstructor],
    sail_type: &Type,
) -> crate::Result<Vec<(RustSymbol, RustTypename)>> {
    assert!(!constructors.is_empty());

    let mut builder = RustSignatureBuilder::default();
    for binary in constructors {
        builder.update(&binary.constr.args)?;
    }

    let args = builder.capture();

    if args.is_empty() {
        Ok(vec![])
    } else if args.len() == 1 {
        let sym = args[0].clone();
        let typ = rust_typename(sail_type);

        Ok(vec![(sym, typ)])
    } else {
        let types = sail_type.as_tuple()?;

        if types.len() != args.len() {
            return err!("mismatched argument and types counts");
        }

        let mut result = Vec::<(RustSymbol, RustTypename)>::with_capacity(types.len());
        for (name, typ) in std::iter::zip(args, types) {
            let typ = rust_typename(typ);

            result.push((name.clone(), typ));
        }

        Ok(result)
    }
}

fn make_binary_matches(
    rust_sig: &[(RustSymbol, RustTypename)],
    constructors: &[BinaryConstructor],
) -> crate::Result<Vec<RustCode>> {
    // 1. find out which arguments of functions are used to distinguish construtcor cases
    let n = rust_sig.len();
    let mut match_needed: Vec<bool> = vec![false; n];
    for binary in constructors {
        crate::assert_equals(n, binary.constr.args.len(), "mismatched lengths".into())?;

        for i in 0..n {
            let arg = &binary.constr.args[i];
            let sig = &rust_sig[i].0;

            match arg {
                Value::Symbol(ident) => {
                    crate::assert_equals(ident, sig, "mismached argument names".into())?;
                }
                Value::EnumLabel(..) | Value::BitVector(..) => {
                    match_needed[i] = true;
                }
                _ => {
                    debug!("{:?}", binary.constr);
                    return err!("unexpected {arg:?}");
                }
            }
        }
    }

    if !match_needed.iter().any(|v| *v) {
        return Ok(Vec::new());
    }

    // 2. collect needed values
    let mut tmp = Vec::<Vec<String>>::new();
    let k = constructors.len() + 1;
    for _ in 0..k {
        tmp.push(Vec::<String>::new());
    }

    for (i, needed) in match_needed.iter().enumerate() {
        if !needed {
            continue;
        }

        tmp[0].push(rust_sig[i].0.clone());
        for (j, binary) in constructors.iter().enumerate() {
            let arg = &binary.constr.args[i];
            match arg {
                Value::Symbol(_) => tmp[j + 1].push("_".to_string()),
                Value::EnumLabel(..) | Value::BitVector(..) => {
                    tmp[j + 1].push(rust_constructor(arg))
                }
                _ => unreachable!(),
            }
        }
    }

    // 3. render the final result into fragments Rust expressions
    fn format_tuple(v: &Vec<String>) -> String {
        if v.len() > 1 {
            format!("({})", v.join(", "))
        } else {
            v[0].clone()
        }
    }

    let fragments = tmp.iter().map(format_tuple).collect();

    Ok(fragments)
}

/**
    Generate code for deconstruction of assignement like:

    (name : bitvector(5)) @ 0b111 = binding
*/
fn make_binary_concatenation_deconstruction(
    binding: &str,
    bc: &BinaryConcatenation,
) -> crate::Result<String> {
    let mut f = String::new();

    w!(f, "// deconstruction of `{binding}`");

    let mut offset = 0;
    for val in bc.0.iter().rev() {
        match val {
            Value::BitVector(bv) => {
                // concrete value
                let bit_width = bv.bit_width;
                let typ = rust_bitvector_type(bit_width);
                w!(
                    f,
                    "let actual = {typ}::from_subword({binding}.val >> {offset});"
                );
                w!(f, "let expected = {typ}::new({});", format_bitvector(bv));
                w!(f, "if actual != expected {{");
                let lo = offset;
                let hi = offset + bit_width - 1;
                w!(
                    f,
                    "  return err!(\"bits [{hi}:{lo}] have to be {}, got 0b{{:0{bit_width}b}}\", actual.val);",
                    format_bitvector(bv)
                );
                w!(f, "}}");

                offset += bit_width;
            }
            Value::SymbolCast(symbol, typ) => {
                let bit_width = typ.as_bitvector()?;
                let typ = rust_bitvector_type(bit_width);
                w!(
                    f,
                    "let {symbol} = {typ}::from_subword({binding}.val >> {offset});"
                );

                offset += bit_width;
            }
            Value::Cast(symbol, typ) => {
                let bit_width = typ.as_bitvector()?;
                let symbol = symbol.as_ref().as_symbol()?;
                let typ = rust_bitvector_type(bit_width);
                w!(
                    f,
                    "let {symbol} = {typ}::from_subword({binding}.val >> {offset});"
                );

                offset += bit_width;
            }
            _ => {
                debug!("{bc:?}");
                return err!("unexpected {val:?}");
            }
        }
    }

    Ok(f)
}

fn rust_typename(t: &Type) -> String {
    match t {
        Type::Ident(ident) => ident.clone(),
        Type::Enum(ident) => ident.clone(),
        Type::BitVector(bit_width) => rust_bitvector_type(*bit_width),
        Type::Boolean => "bool".to_string(),
        Type::Set(..) => "usize".to_string(),
        Type::Struct(desc) => desc.name.clone(),
        _ => panic!("unsupported `{t:?}`"),
    }
}

fn rust_bitvector_type(bit_width: usize) -> String {
    format!("BitVector::<{bit_width}>")
}

fn rust_bitvector_val(bv: &BitVector) -> String {
    format!("0b{:01$b}", bv.val, bv.bit_width)
}

fn rust_bitvector_safe_new(bv: &BitVector) -> String {
    format!(
        "{}::new({})",
        rust_bitvector_type(bv.bit_width),
        format_bitvector(bv)
    )
}

fn rust_fn_call(call: &FunctionInvocation) -> String {
    let mut f = format!("{}(", call.name);
    for (i, val) in call.args.iter().enumerate() {
        if i > 0 {
            f += ", ";
        }

        match val {
            Value::Symbol(ident) => f += ident,
            Value::SymbolCast(ident, _) => f += ident,
            _ => panic!("unsupported `{val:?}`"),
        }
    }

    f += ")";

    f
}

fn encoder_fn_name(instr: &Instruction) -> String {
    format!("encode_{}", instr.constructor.name.to_lowercase())
}

fn parser_fn_name(instr: &Instruction) -> String {
    format!("parse_{}", instr.mnemonic.replace(".", "_"))
}

/// Checks if signature is `bitvector(bit_width)` <-> `string`
fn is_number_conversion(sail_fn_name: &str, sig: &MappingSignature) -> Option<(usize, bool)> {
    if !sig.rhs.is_string() {
        return None;
    }

    let bit_width = sig.lhs.as_bitvector().ok()?;

    let is_signed = sail_fn_name.contains("signed");

    Some((bit_width, is_signed))
}
