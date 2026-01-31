use crate::err;
use crate::model::BitVector;
use crate::model::Value;

pub fn join_bitvectors(values: &[Value]) -> crate::Result<BitVector> {
    let mut result = BitVector {
        val: 0,
        bit_width: 0,
    };
    for (k, val) in values.iter().enumerate() {
        if let Value::BitVector(bv) = val {
            result = result.join(bv);
        } else {
            return err!("item #{k}: expected bitvector value, got {val:?}");
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn bitvector(val: u64, bit_width: usize) -> Value {
        let bv = BitVector { val, bit_width };

        Value::BitVector(bv)
    }

    fn test_join_bitvectors() {
        let concat = vec![
            bitvector(0b101, 3),
            bitvector(0b1110, 4),
            bitvector(0b01, 2),
        ];

        let got = join_bitvectors(&concat).unwrap();

        assert_eq!(got.val, 0b101_1110_01);
        assert_eq!(got.bit_width, 3 + 4 + 2);
    }
}
