use crate::err;
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
    pub mappings: BTreeMap<String, Mapping>,
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
    pub fn resolve_identifier(&mut self, name: &str, sail: &Sail) -> crate::Result<Option<Type>> {
        if is_type_overriden(name) {
            return Ok(None);
        }

        if self.enums.contains_key(name) {
            return Ok(Some(Type::Enum(name.to_string())));
        }
        if self.structs.contains_key(name) {
            return Ok(Some(Type::Struct(name.to_string())));
        }
        if self.mappings.contains_key(name) {
            return Ok(Some(Type::Mapping(name.to_string())));
        }

        match sail.what_is(name) {
            IdentifierKind::Mapping => {
                let _ = self.mapping(name, sail)?;

                Ok(Some(Type::Mapping(name.to_string())))
            }
            IdentifierKind::Struct => {
                let mut struct_type = sail.get_struct(name)?;
                for typ in struct_type.fields.values_mut() {
                    match typ {
                        Type::Ident(ident) => {
                            if let Some(new_type) = self.resolve_identifier(ident, sail)? {
                                *typ = new_type;
                            }
                        }
                        _ => todo!("{typ:?}"),
                    }
                }

                self.structs.insert(name.to_string(), struct_type);

                Ok(Some(Type::Struct(name.to_string())))
            }
            IdentifierKind::Enum => {
                let val = sail.get_enum(name)?;

                let mut resolved = ResolvedEnum::new(name);
                for label in val.labels {
                    resolved.items.push(ResolvedEnumItem::new(label));
                }
                self.enums.insert(name.to_string(), resolved);

                Ok(Some(Type::Enum(name.to_string())))
            }
            IdentifierKind::Other => {
                err!("type {name} cannot be identified")
            }
        }
    }

    pub fn resolve_types(&mut self, typ: &Type, sail: &Sail) -> crate::Result<Type> {
        match typ {
            Type::Ident(ident) => match self.resolve_identifier(ident, sail)? {
                Some(new_typ) => Ok(new_typ),
                None => Ok(typ.clone()),
            },
            Type::Tuple(types) => {
                let mut new_types = Vec::<Type>::with_capacity(types.len());
                for typ in types {
                    new_types.push(self.resolve_types(typ, sail)?);
                }

                Ok(Type::Tuple(new_types))
            }
            Type::String | Type::Boolean | Type::BitVector(_) | Type::Set(_) => Ok(typ.clone()),
            _ => todo!("{typ:?}"),
        }
    }

    pub fn get_struct(&mut self, name: &str, sail: &Sail) -> crate::Result<&StructSignature> {
        if !self.structs.contains_key(name) {
            let mut struct_signature = sail.get_struct(name)?;
            for typ in struct_signature.fields.values_mut() {
                *typ = self.resolve_types(&typ, sail)?;
            }

            self.structs.insert(name.to_string(), struct_signature);
        }

        Ok(self.structs.get(name).as_ref().unwrap())
    }

    pub fn mapping(&mut self, name: &str, sail: &Sail) -> crate::Result<&Mapping> {
        if !self.mappings.contains_key(name) {
            let mut mapping = sail.mapping(name)?;
            mapping.signature.lhs = self.resolve_types(&mapping.signature.lhs, sail)?;
            mapping.signature.rhs = self.resolve_types(&mapping.signature.rhs, sail)?;
            self.mappings.insert(name.to_string(), mapping);
        }

        Ok(self.mappings.get(name).as_ref().unwrap())
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
            IdentifierKind::Struct => {}
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
