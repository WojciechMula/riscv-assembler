use crate::model::BTreeMap;
use crate::model::Value;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Struct {
    pub typename: String,
    pub values: BTreeMap<String, Value>,
}
