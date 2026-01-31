use super::Value;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct FunctionInvocation {
    pub name: String,
    pub args: Vec<Value>,
}
