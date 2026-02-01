use crate::err;
use crate::model::BinaryConcatenation;
use crate::model::BitVector;
use crate::model::EnumLabel;
use crate::model::FunctionInvocation;
use crate::model::Struct;
use crate::model::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Unit,
    String(String),
    BitVector(BitVector),
    Integer(i64),
    Boolean(bool),
    EnumLabel(EnumLabel),
    Cast(Box<Value>, Type),
    Symbol(String),
    StringConcatenation(Vec<Value>),
    BinaryConcatenation(BinaryConcatenation),
    FunctionInvocation(FunctionInvocation),
    Tuple(Vec<Value>),
    Struct(Struct),
}

impl Value {
    pub fn as_bitvector(&self) -> crate::Result<&BitVector> {
        match self {
            Value::BitVector(v) => Ok(v),
            _ => err!("expected bitvector, got {self:?}"),
        }
    }

    pub fn as_binary_concatenation(&self) -> crate::Result<&BinaryConcatenation> {
        match self {
            Value::BinaryConcatenation(v) => Ok(v),
            _ => err!("expected binary concatenation, got {self:?}"),
        }
    }

    pub fn as_fn_call(&self) -> crate::Result<&FunctionInvocation> {
        match self {
            Value::FunctionInvocation(v) => Ok(v),
            _ => err!("expected function invocation, got {self:?}"),
        }
    }

    pub fn as_symbol(&self) -> crate::Result<&String> {
        match self {
            Value::Symbol(v) => Ok(v),
            _ => err!("expected symbol, got {self:?}"),
        }
    }

    pub fn as_string(&self) -> crate::Result<&String> {
        match self {
            Value::String(v) => Ok(v),
            _ => err!("expected string, got {self:?}"),
        }
    }

    pub fn as_tuple(&self) -> crate::Result<&Vec<Value>> {
        match self {
            Value::Tuple(v) => Ok(v),
            _ => err!("expected tuple, got {self:?}"),
        }
    }

    pub fn as_integer(&self) -> crate::Result<i64> {
        match self {
            Value::Integer(v) => Ok(*v),
            _ => err!("expected integer, got {self:?}"),
        }
    }

    pub fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }

    pub fn is_bitvector(&self) -> bool {
        matches!(self, Self::BitVector(..))
    }

    pub fn is_binary_concatenation(&self) -> bool {
        matches!(self, Self::BinaryConcatenation(..))
    }

    pub fn typ(&self) -> &'static str {
        match self {
            Self::Unit => "unit",
            Self::String(..) => "string",
            Self::BitVector(..) => "bitvector",
            Self::Integer(..) => "integer",
            Self::Boolean(..) => "bool",
            Self::EnumLabel(..) => "enum label",
            Self::Cast(..) => "cast",
            Self::Symbol(..) => "symbol",
            Self::StringConcatenation(..) => "string concatenation",
            Self::BinaryConcatenation(..) => "binary concatenation",
            Self::FunctionInvocation(..) => "function invocation",
            Self::Tuple(..) => "tuple",
            Self::Struct(..) => "struct",
        }
    }
}

impl TryFrom<Value> for FunctionInvocation {
    type Error = crate::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::FunctionInvocation(v) = value {
            Ok(v)
        } else {
            err!("expected function invocation, got {:?}", value)
        }
    }
}
