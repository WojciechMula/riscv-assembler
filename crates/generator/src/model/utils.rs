use super::BitVector;
use super::Value;
use crate::err;
use crate::model::BinaryConcatenation;
use crate::model::FunctionInvocation;
use crate::model::Struct;
use crate::sail::Token;
use std::collections::BTreeMap;

pub fn convert_to_binvector(token: &Token) -> crate::Result<Value> {
    let Token::Binary(bin_image) = token else {
        return err!("expected binary token, got `{token:?}`");
    };

    let Some(bits) = bin_image.strip_prefix("0b") else {
        return err!("`{token:?}` does not contain valid string");
    };

    let mut bit_width = 0;
    let mut val = 0_u64;
    for byte in bits.bytes() {
        match byte {
            b'0' => {
                val <<= 1;
                bit_width += 1;
            }
            b'1' => {
                val <<= 1;
                val |= 1;
                bit_width += 1;
            }
            b'_' => (),
            _ => {
                return err!("`{bits}` is not a valid binary number");
            }
        }
    }

    Ok(Value::BitVector(BitVector { val, bit_width }))
}

pub fn rewrite(value: &Value, callback: &dyn Fn(&Value) -> Option<Value>) -> Value {
    match value {
        Value::Unit
        | Value::String(..)
        | Value::BitVector(..)
        | Value::Integer(..)
        | Value::EnumLabel(..)
        | Value::Boolean(..)
        | Value::SymbolCast(..)
        | Value::Symbol(..) => callback(value).unwrap_or_else(|| value.clone()),
        Value::StringConcatenation(args) => {
            let newval = Value::StringConcatenation(rewrite_list(args, callback));

            callback(&newval).unwrap_or(newval)
        }
        Value::BinaryConcatenation(bc) => {
            let newval =
                Value::BinaryConcatenation(BinaryConcatenation(rewrite_list(&bc.0, callback)));

            callback(&newval).unwrap_or(newval)
        }
        Value::FunctionInvocation(FunctionInvocation { name, args }) => {
            let newval = Value::FunctionInvocation(FunctionInvocation {
                name: name.clone(),
                args: rewrite_list(args, callback),
            });

            callback(&newval).unwrap_or(newval)
        }
        Value::Tuple(values) => {
            let newval = Value::Tuple(rewrite_list(values, callback));

            callback(&newval).unwrap_or(newval)
        }
        Value::Struct(Struct { typename, values }) => {
            let mut newvalues = BTreeMap::<String, Value>::new();

            for (name, val) in values {
                let new = rewrite(val, callback);
                newvalues.insert(name.clone(), new);
            }

            let newval = Value::Struct(Struct {
                typename: typename.clone(),
                values: newvalues,
            });

            callback(&newval).unwrap_or(newval)
        }
        Value::Cast(val, _) => {
            let newval = rewrite(val, callback);

            callback(&newval).unwrap_or(newval)
        }
    }
}

pub fn rewrite_list(values: &[Value], callback: &dyn Fn(&Value) -> Option<Value>) -> Vec<Value> {
    let mut result = Vec::<Value>::with_capacity(values.len());
    for val in values {
        result.push(rewrite(val, callback))
    }

    result
}

pub fn positioned_product(components: &[Vec<Value>], mut callback: impl FnMut(&Vec<Value>)) {
    // Note: we're assuming the product is quite small
    let n = components.iter().map(|values| values.len()).product();

    let mut current = vec![Value::Unit; components.len()];

    for i in 0..n {
        let mut index = i;
        for (id, values) in components.iter().enumerate() {
            let k = values.len();

            current[id] = values[index % k].clone();
            index /= k;
        }

        callback(&current);
    }
}

pub struct ProductComponent {
    pub name: String,
    pub values: Vec<Value>,
}

#[cfg(test)]
mod test {
    use super::*;

    fn dec(x: i64) -> Value {
        Value::Integer(x)
    }

    #[test]
    fn test_positioned_product() {
        let p1 = vec![dec(1), dec(2), dec(3), dec(4)];
        let p2 = vec![dec(10)];
        let p3 = vec![dec(100), dec(101)];
        let components = [p1, p2, p3];

        let expected_list = [
            (dec(1), dec(10), dec(100)),
            (dec(2), dec(10), dec(100)),
            (dec(3), dec(10), dec(100)),
            (dec(4), dec(10), dec(100)),
            (dec(1), dec(10), dec(101)),
            (dec(2), dec(10), dec(101)),
            (dec(3), dec(10), dec(101)),
            (dec(4), dec(10), dec(101)),
        ];
        let mut expected = expected_list.iter();

        positioned_product(&components, |vec| {
            let (a, b, c) = expected.next().unwrap();

            assert_eq!(vec[0], *a);
            assert_eq!(vec[1], *b);
            assert_eq!(vec[2], *c);
        });

        assert!(expected.next().is_none());
    }
}
