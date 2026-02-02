use crate::err;
use crate::model::BitVector;
use crate::model::FunctionInvocation;
use crate::model::Pair;
use crate::model::Value;

/*
    Expands mapping given in form:

        mapping foo = {dec_bits((N, s)) <-> s}

    Where N is a constant, and s is an idenifier.

    `dec_bits` is a built-in function, which converts number s (an N-bit value)
    into decimal form.
*/
pub fn expand_dec_bits_string_mapping(pair: &Pair) -> crate::Result<Vec<Pair>> {
    let bit_width = validate_signature(pair)?;

    let n = 1_u64 << bit_width;

    let mut result = Vec::<Pair>::with_capacity(n as usize);
    for v in 0..n {
        let lhs = Value::BitVector(BitVector::try_new(v, bit_width).unwrap());
        let rhs = Value::String(v.to_string());

        result.push(Pair {
            lhs,
            rhs,
            cond: None,
        });
    }

    Ok(result)
}

fn validate_signature(pair: &Pair) -> crate::Result<usize> {
    let FunctionInvocation { name, args } = pair.lhs.as_fn_call()?;

    const DEC_BITS_FN: &str = "dec_bits";
    if name != DEC_BITS_FN {
        return err!("expected invocation of functions {DEC_BITS_FN}, got {name}");
    }

    if args.len() != 1 {
        return err!("expected a single argument, got {args:?}");
    }

    let args = args[0].as_tuple()?;
    if args.len() != 2 {
        return err!("expected two element tuple, got {args:?}");
    }

    let bit_width = args[0].as_integer()?;
    if bit_width <= 0 {
        return err!("bit width should be a positive number, got {bit_width}");
    }

    let var_lhs = args[1].as_symbol()?;
    let var_rhs = pair.rhs.as_symbol()?;

    if var_lhs != var_rhs {
        return err!("variables lhs=`{var_lhs}` and rhs=`{var_rhs}` should be the same");
    }

    Ok(bit_width as usize)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expand_dec_bits_string_mapping() {
        /*
            Input:
                {dec_bits(3, x) <-> x}

            Expected:
                {0 <-> "0"}
                {1 <-> "1"}
                {2 <-> "2"}
                {3 <-> "3"}
                {4 <-> "4"}
                {5 <-> "5"}
                {6 <-> "6"}
                {7 <-> "7"}

        */
        let lhs = Value::FunctionInvocation(FunctionInvocation {
            name: "dec_bits".into(),
            args: vec![Value::Tuple(vec![
                Value::Integer(3),
                Value::Symbol("x".into()),
            ])],
        });
        let rhs = Value::Symbol("x".into());
        let pair = Pair {
            lhs,
            rhs,
            cond: None,
        };

        let expanded = expand_dec_bits_string_mapping(&pair).unwrap();

        assert_eq!(expanded.len(), 8);

        for (expected, pair) in expanded.iter().enumerate() {
            let Value::BitVector(got) = &pair.lhs else {
                panic!("wrong value: {:?}", pair.lhs);
            };
            assert_eq!(*got, BitVector::try_new(expected as u64, 3).unwrap());

            let Value::String(got) = &pair.rhs else {
                panic!("wrong value: {:?}", pair.rhs);
            };
            assert_eq!(*got, expected.to_string());
        }
    }
}
