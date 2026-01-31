use super::Type;
use crate::sail::Parser;
use crate::sail::parse::parse_union;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct Union(pub BTreeMap<String, Type>);

impl Union {
    pub fn parse(p: &mut Parser) -> crate::Result<Self> {
        parse_union(p)
    }
}
