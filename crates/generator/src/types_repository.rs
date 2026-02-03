use crate::err;
use crate::model::BitVector;
use crate::model::Mapping;
use crate::model::StructSignature;
use crate::model::Type;
use crate::sail::IdentifierKind;
use crate::sail::Sail;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct TypesRepository {
    pub enums: BTreeMap<String, ResolvedEnum>,
    pub structs: BTreeMap<String, StructSignature>,
    pub mappings: BTreeMap<String, Mapping>,
    pub aliases: BTreeMap<String, Type>,
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

impl TypesRepository {
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
        if let Some(typ) = self.aliases.get(name) {
            return Ok(Some(typ.clone()));
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
            IdentifierKind::Alias => {
                let typ = sail.get_alias(name)?;

                self.aliases.insert(name.to_string(), typ.clone());

                Ok(Some(typ))
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
                *typ = self.resolve_types(typ, sail)?;
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
    pub fn new(label: String) -> Self {
        Self {
            label,
            value: BitVector::try_new(0, 1).unwrap(),
            string: String::new(),
        }
    }
}
