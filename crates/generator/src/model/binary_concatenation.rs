use crate::err;
use crate::model::BitVector;
use crate::model::Type;
use crate::model::Value;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct BinaryConcatenation(pub Vec<Value>);

impl BinaryConcatenation {
    pub fn needs_normalisation(&self) -> bool {
        self.0.iter().any(|val| match val {
            Value::BitVector(..) => false,
            Value::Cast(.., typ) => !typ.is_bitvector(),
            Value::Integer(..) => true,
            Value::BinaryConcatenation(..) => true,
            _ => todo!("{self:?}"),
        })
    }

    pub fn normalize(&self, t: &Type) -> crate::Result<Self> {
        let mut new = self.clone();
        normalize_inplace(&mut new, t)?;

        Ok(new)
    }

    pub fn total_width(&self) -> Option<usize> {
        let mut total = 0_usize;
        for val in self.0.iter() {
            let bit_width = match val {
                Value::BitVector(bv) => bv.bit_width,
                Value::Cast(_, Type::BitVector(bit_width)) => *bit_width,
                _ => return None,
            };

            total += bit_width;
        }

        Some(total)
    }
}

fn normalize_inplace(bc: &mut BinaryConcatenation, t: &Type) -> crate::Result<()> {
    let total_bit_width = t.as_bitvector()?;

    let mut k: Option<usize> = None;
    let mut value = 0_u64;

    let mut known_width = 0_usize;
    for (i, val) in bc.0.iter().enumerate() {
        match val {
            Value::BitVector(BitVector { bit_width, .. }) => {
                known_width += *bit_width;
            }
            Value::Cast(_, typ) => {
                let bit_width = typ.as_bitvector()?;
                known_width += bit_width;
            }
            Value::Integer(val) => {
                if k.is_some() {
                    return err!("item #{i}: more than one element of unknown bit width `{val:?}`");
                }

                k = Some(i);
                value = *val as u64;
            }
            _ => {
                return err!("item #{i}: unsupported value type `{val:?}`");
            }
        }
    }

    if known_width >= total_bit_width {
        return err!(
            "type denotes {total_bit_width} bits, but total known bits size is {known_width}"
        );
    }

    let bit_width = total_bit_width - known_width;
    let v = BitVector::try_new(value, bit_width)?;

    bc.0[k.unwrap()] = Value::BitVector(v);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn bitvector(val: u64, bit_width: usize) -> Value {
        let bv = BitVector { val, bit_width };

        Value::BitVector(bv)
    }

    #[test]
    fn test_normalize_bitconcat() {
        let concat = BinaryConcatenation(vec![
            bitvector(0, 3),
            bitvector(0, 4),
            Value::Integer(42),
            Value::SymbolCast("x".into(), Type::BitVector(7)),
        ]);

        let typ = Type::BitVector(20);

        let expected = vec![
            bitvector(0, 3),
            bitvector(0, 4),
            bitvector(42, 20 - (3 + 4 + 7)),
            Value::SymbolCast("x".into(), Type::BitVector(7)),
        ];

        assert!(concat.needs_normalisation());

        let got = concat.normalize(&typ).unwrap();

        assert_eq!(got.0, expected);
    }
}
