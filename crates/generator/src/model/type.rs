use crate::err;
use crate::sail::Parser;
use crate::sail::parse::parse_type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unit,
    Ident(String),
    BitVector(usize),
    Boolean,
    Set(Vec<u64>),
    Tuple(Vec<Type>),
    Enum(String),
    Mapping(String),
    Struct(String),
    String,
}

impl Type {
    pub fn parse(p: &mut Parser) -> crate::Result<Self> {
        parse_type(p)
    }

    pub fn is_bitvector(&self) -> bool {
        matches!(self, Self::BitVector(..))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String)
    }

    pub fn ident(&self) -> Option<&String> {
        match self {
            Type::Ident(name) => Some(name),
            _ => None,
        }
    }

    pub fn as_enum(&self) -> Option<&String> {
        match self {
            Type::Enum(name) => Some(name),
            _ => None,
        }
    }

    pub fn as_struct(&self) -> Option<&String> {
        match self {
            Type::Struct(name) => Some(name),
            _ => None,
        }
    }

    pub fn as_bitvector(&self) -> crate::Result<usize> {
        match self {
            Type::BitVector(bit_width) => Ok(*bit_width),
            _ => err!("expected bitvector type, got `{:?}`", self),
        }
    }

    pub fn assume_string(&self) -> crate::Result<()> {
        match self {
            Type::String => Ok(()),
            _ => err!("expected string type, got `{:?}`", self),
        }
    }

    pub fn as_tuple(&self) -> crate::Result<&Vec<Type>> {
        match self {
            Type::Tuple(types) => Ok(types),
            _ => err!("expected tuple type, got `{:?}`", self),
        }
    }
}
