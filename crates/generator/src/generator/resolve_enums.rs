use crate::TypesRepository;
use crate::err;
use crate::model::BitVector;
use crate::model::FunctionInvocation;
use crate::model::Instruction;
use crate::model::Mapping;
use crate::model::Pair;
use crate::model::Type;
use crate::model::Union;
use crate::model::Value;
use crate::sail::IdentifierKind;
use crate::sail::Sail;
use crate::types_repository::ResolvedEnum;
use crate::types_repository::ResolvedEnumItem;
use log::debug;
use std::collections::BTreeSet;

pub fn resolve_enums(
    types: &mut TypesRepository,
    instructions: &Union,
    sail: &Sail,
) -> crate::Result<()> {
    let mut names = BTreeSet::<String>::new();
    for (name, signature) in &instructions.0 {
        debug!("resolving constructor {name}");
        match signature {
            Type::Tuple(types) => {
                for typ in types {
                    if let Some(name) = type_name(typ) {
                        names.insert(name.clone());
                    };
                }
            }
            _ => {
                if let Some(name) = type_name(signature) {
                    names.insert(name.clone());
                }
            }
        }
    }

    for name in names {
        debug!("enum `{name}`: resolve labels");
        match sail.what_is(&name) {
            IdentifierKind::Enum => {
                let val = sail.get_enum(&name)?;

                let mut resolved = ResolvedEnum::new(&name);
                for label in val.labels {
                    resolved.items.push(ResolvedEnumItem::new(label));
                }
                types.enums.insert(name.clone(), resolved);
            }
            IdentifierKind::Struct | IdentifierKind::Alias => {}
            _ => panic!("type `{name}` cannot be identified"),
        }
    }

    Ok(())
}

pub fn type_name(typ: &Type) -> Option<String> {
    match typ {
        Type::BitVector(_) | Type::Boolean | Type::Unit | Type::Set(_) => None,
        Type::Ident(name) | Type::Enum(name) | Type::Struct(name) => {
            if is_type_overriden(name) {
                None
            } else {
                Some(name.clone())
            }
        }
        _ => panic!("unsupported type {typ:?}"),
    }
}

fn is_type_overriden(name: &str) -> bool {
    matches!(
        name,
        "regidx"
            | "vregidx"
            | "cregidx"
            | "fregidx"
            | "csreg"
            | "cfregidx"
            | "half"
            | "nfields"
            | "nfields_pow2"
            | "shamt_zba"
            | "word"
            | "word_width"
            | "word_width_wide"
    )
}

pub fn resolve_string_enums(
    types: &mut TypesRepository,
    instructions: &[Instruction],
    sail: &Sail,
) -> crate::Result<()> {
    let mut seen = BTreeSet::<String>::new();

    for instr in instructions {
        debug!("checking {}", instr.mnemonic);
        for item in &instr.string.args {
            let Ok(FunctionInvocation {
                name: mapping_name, ..
            }) = item.as_fn_call()
            else {
                continue;
            };

            if !seen.insert(mapping_name.clone()) {
                continue;
            }

            let mut tmp = Vec::<(String, String)>::new();
            let mut enum_typename = String::new();
            {
                let Ok(mapping) = types.mapping(mapping_name, sail) else {
                    continue;
                };
                if !mapping.signature.rhs.is_string() {
                    continue;
                };

                let Some(name) = mapping.signature.lhs.as_enum() else {
                    continue;
                };
                enum_typename = name.clone();

                for Pair { lhs, rhs, .. } in &mapping.pairs {
                    let label = lhs.as_symbol()?;
                    let string = rhs.as_string()?;

                    tmp.push((label.to_string(), string.clone()));
                }
            }

            let Some(resolved) = types.enums.get_mut(&enum_typename) else {
                return err!("unhandled `{enum_typename}`");
            };

            debug!("resolving {enum_typename}");
            resolved.string_to_enum = mapping_name.clone();
            for (label, string) in tmp.drain(..) {
                let item = resolved.get_mut_by_label(&label)?;
                item.string = string;
            }
        }
    }

    Ok(())
}

pub fn resolve_bitvector_enums(
    types: &mut TypesRepository,
    mapping: &Mapping,
    sail: &Sail,
) -> crate::Result<()> {
    let mut seen = BTreeSet::<String>::new();

    for Pair { lhs, rhs, .. } in &mapping.pairs {
        let constr = lhs.as_fn_call()?;
        debug!("checking {}", constr.name);
        match rhs {
            Value::BinaryConcatenation(bc) => {
                for item in &bc.0 {
                    let Ok(FunctionInvocation {
                        name: mapping_name, ..
                    }) = item.as_fn_call()
                    else {
                        continue;
                    };

                    if !seen.insert(mapping_name.clone()) {
                        continue;
                    }

                    let mut tmp = Vec::<(String, BitVector)>::new();
                    let mut enum_typename = String::new();
                    {
                        let Ok(mapping) = types.mapping(mapping_name, sail) else {
                            continue;
                        };

                        if !mapping.signature.rhs.is_bitvector() {
                            continue;
                        };

                        let Some(name) = mapping.signature.lhs.as_enum() else {
                            continue;
                        };
                        enum_typename = name.clone();

                        for Pair { lhs, rhs, .. } in &mapping.pairs {
                            let label = lhs.as_symbol()?;
                            let bv = rhs.as_bitvector()?;

                            tmp.push((label.to_string(), bv.clone()));
                        }
                    }

                    let Some(resolved) = types.enums.get_mut(&enum_typename) else {
                        return err!("unhandled `{enum_typename}`");
                    };

                    debug!("resolving {enum_typename}");
                    resolved.enum_to_bitvector = mapping_name.clone();
                    for (label, bv) in tmp.drain(..) {
                        let item = resolved.get_mut_by_label(&label)?;
                        item.value = bv;
                    }
                }
            }
            Value::Symbol(..) => (),
            _ => panic!("unsupported {rhs:?}"),
        }
    }

    Ok(())
}
