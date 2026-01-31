use crate::err;
use crate::model::Value;
use log::debug;

type RustSymbol = String;

#[derive(Default)]
pub struct RustSignatureBuilder {
    arg_names: Vec<Option<RustSymbol>>,
}

impl RustSignatureBuilder {
    pub fn update(&mut self, args: &[Value]) -> crate::Result<()> {
        if self.arg_names.is_empty() {
            self.arg_names.resize(args.len(), None);
        }

        if args.len() != self.arg_names.len() {
            return err!("incompatible number of arguments");
        }

        for (i, arg) in args.iter().enumerate() {
            let name = match arg {
                Value::Symbol(sym) => sym,
                Value::EnumLabel(_) | Value::BitVector(_) | Value::BinaryConcatenation(_) => {
                    continue;
                }
                _ => {
                    debug!("item #{i}: unhandled value {args:?}");
                    return err!("unhandled value type {}", arg.typ());
                }
            };

            if let Some(existing_name) = &self.arg_names[i] {
                if name != existing_name {
                    return err!(
                        "item #{i}: not matching argument names: '{existing_name}' vs '{name}'"
                    );
                }
            } else {
                self.arg_names[i] = Some(name.clone());
            }
        }

        Ok(())
    }

    pub fn capture(&self) -> Vec<RustSymbol> {
        let mut result = Vec::<RustSymbol>::with_capacity(self.arg_names.len());

        for (i, arg) in self.arg_names.iter().enumerate() {
            if let Some(name) = arg {
                result.push(name.clone());
            } else {
                result.push(format!("arg{i}"));
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::BitVector;

    #[test]
    fn test_signature_builder_case1() {
        let mut builder = RustSignatureBuilder::default();

        builder
            .update(&[sym("a"), bitvector(), bitvector(), sym("d")])
            .unwrap();
        builder
            .update(&[sym("a"), sym("b"), bitvector(), sym("d")])
            .unwrap();
        builder
            .update(&[sym("a"), sym("b"), sym("c"), sym("d")])
            .unwrap();

        let got = builder.capture();
        let want = [
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];

        assert_eq!(got, want);
    }

    #[test]
    fn test_signature_builder_case2() {
        let mut builder = RustSignatureBuilder::default();

        builder
            .update(&[sym("a"), bitvector(), bitvector(), sym("d")])
            .unwrap();

        let got = builder.capture();
        let want = [
            "a".to_string(),
            "arg1".to_string(),
            "arg2".to_string(),
            "d".to_string(),
        ];

        assert_eq!(got, want);
    }

    fn sym(name: &str) -> Value {
        Value::Symbol(name.to_string())
    }

    fn bitvector() -> Value {
        Value::BitVector(BitVector::try_new(0, 1).unwrap())
    }
}
