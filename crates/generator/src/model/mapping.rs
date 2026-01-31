use super::Pair;
use crate::model::MappingSignature;
use crate::sail::Parser;
use crate::sail::parse::parse_mapping;

mod dec_bits;
mod expand;
mod placeholders;

#[derive(Debug, Clone)]
pub struct Mapping {
    pub name: String,
    pub signature: MappingSignature,
    pub pairs: Vec<Pair>,
}

pub enum ExpandMapping {
    None,
    BitVector,
}

impl Mapping {
    pub fn parse(
        p: &mut Parser,
        sig: MappingSignature,
        e: ExpandMapping,
    ) -> crate::Result<Mapping> {
        let mut mapping = parse_mapping(p, sig)?;

        if matches!(e, ExpandMapping::BitVector) {
            expand::expand_bitvector_mapping(&mut mapping)?;
        }

        Ok(mapping)
    }
}
