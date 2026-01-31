mod binary_concatenation;
mod bitvector;
mod bitvector_utils;
mod r#enum;
mod function_invocation;
mod mapping;
mod mapping_signature;
mod pair;
mod r#struct;
mod struct_signature;
mod r#type;
mod r#union;
mod utils;
mod value;

pub use bitvector_utils::*;
pub use utils::*;

pub use binary_concatenation::BinaryConcatenation;
pub use bitvector::BitVector;
pub use r#enum::Enum;
pub use function_invocation::FunctionInvocation;
pub use mapping::ExpandMapping;
pub use mapping::Mapping;
pub use mapping_signature::MappingSignature;
pub use pair::Pair;
pub use r#struct::Struct;
pub use struct_signature::StructSignature;
pub use r#type::Type;
pub use r#union::Union;
pub use value::Value;

use std::collections::BTreeMap;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct EnumLabel {
    pub typename: String,
    pub label: String,
}

#[derive(Debug)]
pub struct Instruction {
    pub constructor: FunctionInvocation,
    pub signature: Type,
    pub mnemonic: String,
    pub constants: BTreeMap<String, Value>,
    pub string: StringConstructor,
    pub binary: Vec<BinaryConstructor>,
}

#[derive(Debug, Default)]
pub struct StringConstructor {
    pub constr: FunctionInvocation,
    pub args: Vec<Value>,
    pub cond: Option<Value>,
}

#[derive(Debug)]
pub struct BinaryConstructor {
    pub constr: FunctionInvocation,
    pub signature: Type,
    pub args: Vec<Value>,
    pub cond: Option<Value>,
}
