use crate::model::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair {
    pub lhs: Value,
    pub rhs: Value,
    pub cond: Option<Value>,
}
