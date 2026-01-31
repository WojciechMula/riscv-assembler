use crate::err;
use crate::generator::code_generator::CodeGenerator;
use crate::generator::csr::CsrList;
use crate::generator::extensions::Extensions;
use crate::generator::resolve_enums::ResolvedTypes;
use crate::generator::resolve_enums::resolve_bitvector_enums;
use crate::generator::resolve_enums::resolve_enums;
use crate::generator::resolve_enums::resolve_string_enums;
use crate::generator::resolve_structs::resolve_structs;
use crate::model::BinaryConstructor;
use crate::model::EnumLabel;
use crate::model::FunctionInvocation;
use crate::model::Instruction;
use crate::model::Mapping;
use crate::model::MappingSignature;
use crate::model::Pair;
use crate::model::Type;
use crate::model::Union;
use crate::model::Value;
use crate::sail::IdentifierKind;
use crate::sail::Sail;
use crate::sail::patch::patch_assembly_definitions;
use log::debug;
use log::error;
use log::info;
use std::collections::HashMap;
use std::path::Path;

mod code_generator;
mod csr;
mod expand_mnemonics;
mod extensions;
mod resolve_conditions;
mod resolve_enums;
mod resolve_structs;
mod rust;
mod rust_signature;

pub fn generate(input: &Path, pseudoinstr: &Path, output: &Path) -> crate::Result<()> {
    info!("Reading {}", input.display());
    let contents = std::fs::read_to_string(input)?;

    let mut sail = Sail::new(contents);

    let (signatures, mut map_string, map_binary) = initialize(&mut sail)?;

    info!("Patching some definitions");
    for mapping in map_string.iter_mut() {
        patch_assembly_definitions(mapping);
    }

    {
        info!("Reading pseudoinstructions from {}", pseudoinstr.display());
        let contents = std::fs::read_to_string(pseudoinstr)?;

        let sail = Sail::new(contents);
        let mapping = sail.mapping("pseudoinstructions")?;

        map_string.push(mapping);
    }

    let mut types = ResolvedTypes::default();

    info!("Resolving enums present in constructors");
    resolve_enums(&mut types, &signatures, &sail)?;

    info!("Resolving structs present in constructors");
    resolve_structs(&mut types, &signatures, &sail)?;

    info!("Collecting instruction mnemonics");
    let mut expanded = Vec::<Instruction>::new();
    for mapping in &map_string {
        let tmp = expand_mnemonics::expand_mnemonics(&types, mapping, &sail)?;
        expanded.extend(tmp);
    }

    for instr in expanded.iter_mut() {
        let Some(sig) = signatures.0.get(&instr.constructor.name) else {
            return err!("signature for {} not found", instr.constructor.name);
        };

        instr.signature = sig.clone();

        fixup_function_arguments(&mut instr.constructor, &instr.signature, &sail)?;
        fixup_function_arguments(&mut instr.string.constr, &instr.signature, &sail)?;
    }

    info!("... found {} instructions", expanded.len());

    info!("Resolving enums present in string converters");
    resolve_string_enums(&mut types, &expanded, &sail)?;

    info!("Resolving enums present in binary converters");
    for mapping in &map_binary {
        resolve_bitvector_enums(&mut types, mapping, &sail)?;
    }

    info!("Building list of instruction constructors");
    let constructors = binary_constructor_map(&map_binary, &signatures, &sail)?;
    info!("... found {} constructors", constructors.len());

    info!("Matching opcodes with their binary representations");
    match_opcodes(&mut expanded, constructors, &signatures)?;

    info!("Reading CSRs");
    let csr = CsrList::new(&sail)?;

    info!("Reading extension names");
    let extensions = Extensions::new(&sail)?;

    info!("Generating Rust code");
    let mut cg = CodeGenerator::new(signatures, types, csr, extensions);

    for instr in expanded.iter() {
        debug!("Generating assembler code for {}", instr.mnemonic);
        cg.add_instruction(instr, &sail)?;
    }

    info!("Saving to {}", output.display());
    cg.write_file(output)?;

    Ok(())
}

fn initialize(sail: &mut Sail) -> crate::Result<(Union, Vec<Mapping>, Vec<Mapping>)> {
    let instruction_type = "instruction";
    info!("Looking for `{instruction_type}` union");
    let mut instructions = sail.get_union(instruction_type)?;
    for typ in instructions.0.values_mut() {
        match typ {
            Type::Ident(ident) => match sail.what_is(ident) {
                IdentifierKind::Mapping => {
                    *typ = Type::Mapping(ident.clone());
                }
                IdentifierKind::Enum => {
                    *typ = Type::Enum(ident.clone());
                }
                _ => (),
            },
            Type::Tuple(args) => {
                for typ in args.iter_mut() {
                    if let Type::Ident(ident) = typ {
                        match sail.what_is(ident) {
                            IdentifierKind::Mapping => {
                                *typ = Type::Mapping(ident.clone());
                            }
                            IdentifierKind::Enum => {
                                *typ = Type::Enum(ident.clone());
                            }
                            _ => (),
                        }
                    }
                }
            }
            _ => (),
        }
    }

    info!("Looking for mappings from `{instruction_type}` to string");
    let map_string = collect_mappings(sail, |signature| {
        let Type::Ident(name) = &signature.lhs else {
            return false;
        };

        if name != instruction_type {
            return false;
        }

        signature.rhs.is_string()
    });

    if map_string.is_empty() {
        error!("No mappings found");
        return Err("error".into());
    } else {
        for mapping in &map_string {
            info!("Found mapping `{}`", mapping.name);
        }
    }

    info!("Looking for mappings from `{instruction_type}` to bitvectors");
    let map_binary = collect_mappings(sail, |signature| {
        let Type::Ident(name) = &signature.lhs else {
            return false;
        };

        if name != instruction_type {
            return false;
        }

        signature.rhs.is_bitvector()
    });

    if map_binary.is_empty() {
        error!("No mappings found");
        return Err("error".into());
    } else {
        for mapping in &map_binary {
            info!("Found mapping `{}`", mapping.name);
        }
    }

    Ok((instructions, map_string, map_binary))
}

fn collect_mappings(sail: &Sail, pred: impl Fn(&MappingSignature) -> bool) -> Vec<Mapping> {
    let mut res = Vec::<Mapping>::new();
    for name in sail.mappings.keys() {
        let Ok(sig) = sail.mapping_signature(name) else {
            continue;
        };

        if !pred(&sig) {
            continue;
        }

        let Ok(mapping) = sail.mapping(name) else {
            continue;
        };

        res.push(mapping);
    }

    res
}

fn binary_constructor_map(
    mappings: &[Mapping],
    signatures: &Union,
    sail: &Sail,
) -> crate::Result<HashMap<String, Vec<Pair>>> {
    let mut result = HashMap::<String, Vec<Pair>>::new();

    for mapping in mappings {
        for pair in &mapping.pairs {
            let constructor = pair.lhs.as_fn_call()?;
            let mut constructor = constructor.clone();

            let Some(sig) = signatures.0.get(&constructor.name) else {
                return err!("cannot find signature of `{}`", constructor.name);
            };

            fixup_function_arguments(&mut constructor, sig, sail)?;

            let name = constructor.name.clone();
            let pair = Pair {
                lhs: Value::FunctionInvocation(constructor),
                rhs: pair.rhs.clone(),
                cond: pair.cond.clone(),
            };

            result
                .entry(name)
                .and_modify(|vec| vec.push(pair.clone()))
                .or_insert(vec![pair.clone()]);
        }
    }

    Ok(result)
}

pub fn fixup_function_arguments(
    invocation: &mut FunctionInvocation,
    typ: &Type,
    sail: &Sail,
) -> crate::Result<()> {
    if invocation.args.len() > 1 {
        let types = typ.as_tuple()?;
        crate::assert_equals(
            invocation.args.len(),
            types.len(),
            "mismatched types and arguments".into(),
        )?;

        for (i, arg) in invocation.args.iter_mut().enumerate() {
            fixup_argument(arg, &types[i], sail);
        }
    } else if invocation.args.len() == 1 {
        fixup_argument(&mut invocation.args[0], typ, sail);
    }

    Ok(())
}

fn fixup_argument(val: &mut Value, typ: &Type, sail: &Sail) {
    let Type::Enum(typename) = typ else {
        return;
    };

    let Value::Symbol(symbol) = &val else {
        return;
    };

    let Ok(typ) = sail.get_enum(typename) else {
        return;
    };

    if !typ.has_label(symbol) {
        return;
    }

    *val = Value::EnumLabel(EnumLabel {
        typename: typename.clone(),
        label: symbol.clone(),
    });
}

fn match_opcodes(
    expanded: &mut Vec<Instruction>,
    constructors: HashMap<String, Vec<Pair>>,
    signatures: &Union,
) -> crate::Result<()> {
    for instr in expanded.iter_mut() {
        let Some(candidates) = constructors.get(&instr.constructor.name) else {
            return err!(
                "cannot find binary represenation spec for `{:?}`",
                instr.constructor
            );
        };

        let Some(sig) = signatures.0.get(&instr.constructor.name) else {
            return err!("signature for {} not found", instr.constructor.name);
        };

        fn update_instruction(
            instr: &mut Instruction,
            pair: &Pair,
            sig: Type,
        ) -> crate::Result<()> {
            let constr = pair.lhs.as_fn_call()?;

            match &pair.rhs {
                Value::BinaryConcatenation(args) => {
                    let constr = BinaryConstructor {
                        constr: constr.clone(),
                        signature: sig,
                        args: args.0.clone(),
                        cond: pair.cond.clone(),
                    };

                    instr.binary.push(constr);

                    Ok(())
                }
                Value::Symbol(_) => {
                    let constr = BinaryConstructor {
                        constr: constr.clone(),
                        signature: sig,
                        args: vec![pair.rhs.clone()],
                        cond: pair.cond.clone(),
                    };

                    instr.binary.push(constr);

                    Ok(())
                }
                _ => {
                    debug!("While processing instruction `{:?}`", instr);
                    debug!("While processing entry `{:?}`", pair);
                    err!(
                        "expected binary concatenation or a single symbol, got `{:?}`",
                        pair.rhs
                    )
                }
            }
        }

        for cand in candidates {
            update_instruction(instr, cand, sig.clone())?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fixup() {
        // given
        let code = r#"
            enum Color = {RED, GREEN, BLUE}
        "#;
        let sail = Sail::new(code.to_owned());

        let mut inv = FunctionInvocation {
            name: "test".to_owned(),
            args: vec![
                Value::Integer(42),
                Value::Symbol("RED".to_string()),
                Value::String("foo".to_string()),
            ],
        };
        let typ = Type::Tuple(vec![
            Type::Unit,
            Type::Enum("Color".to_owned()),
            Type::String,
        ]);

        // when
        fixup_function_arguments(&mut inv, &typ, &sail).unwrap();

        // then
        let want = vec![
            Value::Integer(42),
            Value::EnumLabel(EnumLabel {
                typename: "Color".to_owned(),
                label: "RED".to_string(),
            }),
            Value::String("foo".to_string()),
        ];
        assert_eq!(inv.args, want);
    }
}
