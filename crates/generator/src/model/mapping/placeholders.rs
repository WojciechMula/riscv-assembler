use crate::model::BitVector;
use crate::model::Pair;
use crate::model::Type;
use crate::model::Value;
use crate::model::join_bitvectors;
use crate::model::positioned_product;
use crate::model::rewrite;
use std::collections::HashSet;

pub fn expand_bitvector_placeholders(p: &Pair, already_defined: &HashSet<u64>) -> Vec<Pair> {
    let Value::BinaryConcatenation(args) = &p.lhs else {
        return Vec::new();
    };

    let mut variables = Vec::<String>::new();
    let mut possible_values = Vec::<Vec<Value>>::new();

    for arg in &args.0 {
        match arg {
            Value::BitVector(_) => {
                variables.push(String::new());
                possible_values.push(vec![arg.clone()]);
            }
            Value::SymbolCast(name, typ) => {
                let Type::BitVector(bit_width) = typ else {
                    panic!("expected bit vector type, got {typ:?}");
                };
                let bit_width = *bit_width;

                let max_value = 1_u64 << bit_width;
                let values = (0..max_value)
                    .map(|val| Value::BitVector(BitVector { val, bit_width }))
                    .collect();

                variables.push(name.clone());
                possible_values.push(values);
            }
            _ => panic!("{arg:?}"),
        }
    }

    let mut result = Vec::<Pair>::new();

    positioned_product(&possible_values, |bitvectors| {
        let bv = join_bitvectors(bitvectors).unwrap();
        if !already_defined.contains(&bv.val) {
            let substitute = |value: &Value| -> Option<Value> {
                let Value::Symbol(name) = value else {
                    return None;
                };

                let position = variables.iter().position(|var| var == name)?;

                Some(bitvectors[position].clone())
            };

            let lhs = Value::BitVector(bv);
            let rhs = rewrite(&p.rhs, &substitute);

            result.push(Pair {
                lhs,
                rhs,
                cond: None,
            });
        }
    });

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::FunctionInvocation;
    use crate::sail::Sail;

    fn bin(val: u64, bit_width: usize) -> Value {
        Value::BitVector(BitVector { val, bit_width })
    }

    fn string(s: &str) -> Value {
        Value::String(s.to_string())
    }

    fn call(name: &str, arg: Value) -> Value {
        let inv = FunctionInvocation {
            name: name.to_string(),
            args: vec![arg],
        };

        Value::FunctionInvocation(inv)
    }

    fn concat(a: Value, b: Value) -> Value {
        Value::StringConcatenation(vec![a, b])
    }

    #[test]
    fn test_mapping_binary_string() {
        let s = r#"
            val test : bitvector(5) <-> string

            mapping test = {
                0b00000 <-> "all zeros",
                0b11100 <-> "all ones",
                (foo: bitvector(1)) @ (bar: bitvector(2)) @ 0b00 <-> fn_foo(foo) ^ fn_bar(bar)
            }
        "#;

        let mut sail = Sail::new(s.to_string());

        let mapping = sail.mapping("test").unwrap();

        assert_eq!(
            mapping.pairs[0],
            Pair {
                lhs: bin(0b00000, 5),
                rhs: string("all zeros"),
                cond: None,
            }
        );

        assert_eq!(
            mapping.pairs[1],
            Pair {
                lhs: bin(0b11100, 5),
                rhs: string("all ones"),
                cond: None,
            }
        );

        assert_eq!(
            mapping.pairs[2],
            Pair {
                lhs: bin(0b1_00_00, 5),
                rhs: concat(call("fn_foo", bin(0b1, 1)), call("fn_bar", bin(0b00, 2))),
                cond: None,
            }
        );

        assert_eq!(
            mapping.pairs[3],
            Pair {
                lhs: bin(0b0_01_00, 5),
                rhs: concat(call("fn_foo", bin(0b0, 1)), call("fn_bar", bin(0b01, 2))),
                cond: None,
            }
        );

        assert_eq!(
            mapping.pairs[4],
            Pair {
                lhs: bin(0b1_01_00, 5),
                rhs: concat(call("fn_foo", bin(0b1, 1)), call("fn_bar", bin(0b01, 2))),
                cond: None,
            }
        );

        assert_eq!(
            mapping.pairs[5],
            Pair {
                lhs: bin(0b0_10_00, 5),
                rhs: concat(call("fn_foo", bin(0b0, 1)), call("fn_bar", bin(0b10, 2))),
                cond: None,
            }
        );

        assert_eq!(
            mapping.pairs[6],
            Pair {
                lhs: bin(0b1_10_00, 5),
                rhs: concat(call("fn_foo", bin(0b1, 1)), call("fn_bar", bin(0b10, 2))),
                cond: None,
            }
        );

        assert_eq!(
            mapping.pairs[7],
            Pair {
                lhs: bin(0b0_11_00, 5),
                rhs: concat(call("fn_foo", bin(0b0, 1)), call("fn_bar", bin(0b11, 2))),
                cond: None,
            }
        );

        assert_eq!(mapping.pairs.len(), 8);
    }
}
