use crate::err;
use crate::is_custom_function;
use crate::model::BitVector;
use crate::model::FunctionInvocation;
use crate::model::Instruction;
use crate::model::Mapping;
use crate::model::Pair;
use crate::model::StructSignature;
use crate::model::Type;
use crate::model::Union;
use crate::model::Value;
use crate::sail::IdentifierKind;
use crate::sail::Sail;
use log::debug;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Default)]
pub struct ResolvedTypes {
    pub enums: BTreeMap<String, ResolvedEnum>,
    pub structs: BTreeMap<String, StructSignature>,
}

#[derive(Default, Debug)]
pub struct ResolvedEnum {
    pub typename: String,
    pub string_to_enum: String,
    pub enum_to_bitvector: String,
    pub items: Vec<ResolvedEnumItem>,
}

#[derive(Debug)]
pub struct ResolvedEnumItem {
    pub label: String,
    pub value: BitVector,
    pub string: String,
}

impl ResolvedTypes {
    pub fn resolve_idents(&self, typ: &Type, sail: &Sail) -> Type {
        match typ {
            Type::Enum(_) | Type::String | Type::Set(..) | Type::Boolean | Type::BitVector(..) => {
                typ.clone()
            }
            Type::Ident(ident) => {
                if self.enums.contains_key(ident) {
                    Type::Enum(ident.clone())
                } else if sail.what_is(ident) == IdentifierKind::Enum {
                    Type::Enum(ident.clone())
                } else {
                    typ.clone()
                }
            }
            Type::Tuple(types) => {
                let mut new = Vec::<Type>::with_capacity(types.len());
                for typ in types {
                    new.push(self.resolve_idents(typ, sail));
                }

                Type::Tuple(new)
            }
            Type::Struct(struct_type) => {
                let mut struct_type = struct_type.clone();
                for typ in struct_type.fields.values_mut() {
                    *typ = self.resolve_idents(typ, sail);
                }

                Type::Struct(struct_type)
            }
            _ => todo!("{typ:?}"),
        }
    }
}

pub fn resolve_enums(
    types: &mut ResolvedTypes,
    instructions: &Union,
    sail: &Sail,
) -> crate::Result<()> {
    let mut names = BTreeSet::<String>::new();
    for (name, signature) in &instructions.0 {
        debug!("resolving constructor {name}");
        match signature {
            Type::Tuple(types) => {
                for typ in types {
                    if let Some(name) = resolve_type(typ) {
                        names.insert(name.clone());
                    };
                }
            }
            _ => {
                if let Some(name) = resolve_type(signature) {
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
            IdentifierKind::Struct => {}
            _ => panic!("type `{name}` cannot be identified"),
        }
    }

    Ok(())
}

pub type EnumName = String;

pub fn resolve_type(typ: &Type) -> Option<EnumName> {
    match typ {
        Type::BitVector(_) | Type::Boolean | Type::Unit | Type::Set(_) => None,
        Type::Ident(name) | Type::Enum(name) => {
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
    types: &mut ResolvedTypes,
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

            if is_custom_function(mapping_name) {
                continue;
            }

            let Ok(sig) = sail.mapping_signature(mapping_name) else {
                continue;
            };

            if !sig.rhs.is_string() {
                continue;
            }

            let Some(enum_typename) = sig.lhs.as_enum() else {
                continue;
            };

            let Ok(mapping) = sail.mapping(mapping_name) else {
                continue;
            };

            let Some(resolved) = types.enums.get_mut(enum_typename) else {
                return err!("unhandled `{enum_typename}`");
            };

            debug!("resolving {enum_typename}");
            resolved.string_to_enum = mapping_name.clone();
            for Pair { lhs, rhs, .. } in &mapping.pairs {
                let label = lhs.as_symbol()?;
                let string = rhs.as_string()?;

                let item = resolved.get_mut_by_label(label)?;
                item.string = string.clone();
            }
        }
    }

    Ok(())
}

pub fn resolve_bitvector_enums(
    types: &mut ResolvedTypes,
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

                    if is_custom_function(mapping_name) {
                        continue;
                    }

                    let Ok(sig) = sail.mapping_signature(mapping_name) else {
                        continue;
                    };

                    if !sig.rhs.is_bitvector() {
                        continue;
                    };

                    let Some(enum_typename) = sig.lhs.as_enum() else {
                        continue;
                    };

                    let Ok(mapping) = sail.mapping(mapping_name) else {
                        continue;
                    };

                    let Some(resolved) = types.enums.get_mut(enum_typename) else {
                        return err!("unhandled `{enum_typename}`");
                    };

                    debug!("resolving {enum_typename}");
                    resolved.enum_to_bitvector = mapping_name.clone();
                    for Pair { lhs, rhs, .. } in &mapping.pairs {
                        let label = lhs.as_symbol()?;
                        let bv = rhs.as_bitvector()?;

                        let item = resolved.get_mut_by_label(label)?;
                        item.value = bv.clone();
                    }
                }
            }
            Value::Symbol(..) => (),
            _ => panic!("unsupported {rhs:?}"),
        }
    }

    Ok(())
}

impl ResolvedEnum {
    pub fn new(typename: &str) -> Self {
        Self {
            typename: typename.to_string(),
            ..Self::default()
        }
    }

    pub fn get_mut_by_label(&mut self, label: &str) -> crate::Result<&mut ResolvedEnumItem> {
        match self.items.iter_mut().find(|item| item.label == label) {
            Some(item) => Ok(item),
            None => err!("enum {} does not contain label {label}", self.typename),
        }
    }
}

impl ResolvedEnumItem {
    fn new(label: String) -> Self {
        Self {
            label,
            value: BitVector::try_new(0, 1).unwrap(),
            string: String::new(),
        }
    }
}
