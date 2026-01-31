use crate::err;
use crate::model::BitVector;
use crate::model::Type;
use crate::model::Value;

// XXX: rename to bitvector_value
pub fn format_bitvector(bv: &BitVector) -> String {
    format!("0b{:01$b}", bv.val, bv.bit_width)
}

pub fn bitvector_constructor(bv: &BitVector) -> String {
    format!(
        "{}{{val: {}}}",
        bitvector_typname(bv.bit_width),
        format_bitvector(bv)
    )
}

pub fn bitvector_typname(bit_width: usize) -> String {
    format!("BitVector::<{bit_width}>")
}

pub fn rust_expression(v: &Value) -> String {
    match v {
        Value::Boolean(v) => format!("{v}"),
        Value::Integer(v) => format!("{v}"),
        Value::BitVector(bv) => format_bitvector(bv),
        Value::EnumLabel(el) => {
            format!("{}::{}", el.typename, el.label)
        }
        Value::Symbol(s) => s.clone(),
        Value::Struct(name, fields) => {
            let mut f = String::new();
            for (key, val) in fields {
                f += &format!("{key}: {},", rust_expression(val));
            }

            format!("{}{{{f}}}", name)
        }
        Value::FunctionInvocation(fun) => {
            let mut f = format!("{}(", fun.name);
            for (i, val) in fun.args.iter().enumerate() {
                if i > 0 {
                    f += ", ";
                }

                f += &rust_expression(val);
            }

            f += ")";

            f
        }
        _ => panic!("unsupported `{v:?}`"),
    }
}

pub fn rust_constructor(v: &Value) -> String {
    match v {
        Value::Boolean(v) => format!("{v}"),
        Value::Integer(v) => format!("{v}"),
        Value::BitVector(bv) => bitvector_constructor(bv),
        Value::EnumLabel(el) => {
            format!("{}::{}", el.typename, el.label)
        }
        Value::Symbol(s) => s.clone(),
        Value::Struct(name, fields) => {
            let mut fragment = String::new();
            for (key, val) in fields {
                fragment += &format!("{key}: {},", rust_expression(val));
            }

            format!("{}{{{fragment}}}", name)
        }
        _ => panic!("unsupported `{v:?}`"),
    }
}

pub fn evaluate_bitvector_cast(val: &Value, typ: &Type) -> crate::Result<BitVector> {
    let bit_width = typ.as_bitvector()?;
    match val {
        Value::BitVector(bv) => {
            if bv.bit_width > bit_width {
                return err!(
                    "cast of wider bitvector ({} bits) than the targer type ({} bits)",
                    bv.bit_width,
                    bit_width
                );
            }
            BitVector::try_new(bv.val, bit_width)
        }
        Value::Integer(v) => BitVector::from_signed(*v, bit_width),
        _ => err!("unsupported cast from {val:?} to {typ:?}"),
    }
}
