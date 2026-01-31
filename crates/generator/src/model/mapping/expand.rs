use super::dec_bits::expand_dec_bits_string_mapping;
use super::placeholders::expand_bitvector_placeholders;
use crate::model::BitVector;
use crate::model::FunctionInvocation;
use crate::model::Mapping;
use crate::model::Pair;
use crate::model::Value;
use std::collections::HashSet;

pub fn expand_bitvector_mapping(mapping: &mut Mapping) -> crate::Result<()> {
    if !mapping.signature.lhs.is_bitvector() {
        return Ok(());
    };

    let mut specified_explicitely = HashSet::<u64>::new();
    for pair in &mapping.pairs {
        if let Value::BitVector(BitVector { val, .. }) = &pair.lhs {
            specified_explicitely.insert(*val);
        }
    }

    if specified_explicitely.len() == mapping.pairs.len() {
        return Ok(());
    }

    let mut newpairs = Vec::<Pair>::with_capacity(mapping.pairs.len());
    for pair in mapping.pairs.drain(..) {
        if matches!(pair.lhs, Value::BitVector(..) | Value::Integer(..)) {
            newpairs.push(pair);
        } else if matches!(pair.lhs, Value::BinaryConcatenation(..)) {
            let new = expand_bitvector_placeholders(&pair, &specified_explicitely);
            newpairs.extend(new);
        } else if let Value::FunctionInvocation(FunctionInvocation { name, .. }) = &pair.lhs {
            if name == "dec_bits" {
                let new = expand_dec_bits_string_mapping(&pair)?;
                newpairs.extend(new);
            } else if name == "hex_bits" || name == "hex_bits_signed" {
                /* nop */
            } else {
                unreachable!("{:?}", pair.lhs);
            }
        } else {
            unreachable!("{:?}", pair);
        }
    }

    mapping.pairs = newpairs;

    Ok(())
}
