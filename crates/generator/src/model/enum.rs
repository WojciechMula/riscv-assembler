use crate::sail::Parser;
use crate::sail::parse;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Enum {
    pub name: String,
    pub labels: Vec<String>,
}

impl Enum {
    pub fn parse(p: &mut Parser) -> crate::Result<Self> {
        parse::parse_enum(p)
    }

    pub fn has_label(&self, name: &String) -> bool {
        self.labels.contains(name)
    }
}
