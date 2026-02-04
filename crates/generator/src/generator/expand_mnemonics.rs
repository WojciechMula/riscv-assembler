use crate::TypesRepository;
use crate::err;
use crate::errfmt;
use crate::model::BinaryConcatenation;
use crate::model::BitVector;
use crate::model::Builtin;
use crate::model::EnumLabel;
use crate::model::Instruction;
use crate::model::Mapping;
use crate::model::Pair;
use crate::model::StringConstructor;
use crate::model::Type;
use crate::model::Value;
use crate::model::rewrite;
use crate::sail::Sail;
use log::debug;
use std::collections::BTreeMap;
use std::iter::zip;

/*
    Sometimes mapping of instruction into a string does not encode directly
    the mnemonic of instruction. For example:

        mapping assembly = {
            ...
            CSRReg(csr, rs1, rd, op) <-> csr_mnemonic(op) ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ [...]
            ...
        }

    In this case the it's another mapping, called `csr_mnemonic`, that defines the
    exact list of mnemonics:

        val csr_mnemonic : csrop <-> string

        mapping csr_mnemonic = {
          CSRRW <-> "csrrw",
          CSRRS <-> "csrrs",
          CSRRC <-> "csrrc"
        }

    From the above we learn there are three different mnemonics for constructor `CSRReg`.
    The outcome of the procedure is the following expansion from our example:

            ...
            CSRReg(csr, rs1, rd, CSRRW) <-> "csrrw" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ [...]
            CSRReg(csr, rs1, rd, CSRRS) <-> "csrrs" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ [...]
            CSRReg(csr, rs1, rd, CSRRC) <-> "csrrc" ^ spc(()) ^ reg_name(rd) ^ sep(()) ^ [...]
            ...
*/
pub fn expand_mnemonics(
    types: &mut TypesRepository,
    assembly: &Mapping,
    sail: &Sail,
) -> crate::Result<Vec<Instruction>> {
    let mut result = Vec::<Instruction>::with_capacity(assembly.pairs.len());

    for Pair { lhs, rhs, cond } in &assembly.pairs {
        let constructor = lhs.as_fn_call()?;
        debug!("expanding {}", constructor.name);

        if let Value::StringConcatenation(args) = rhs {
            let (mnemonic, arguments) = split_at_space(args);
            let expanded = expand_functions(types, &mnemonic, sail)?;
            for item in expanded {
                let lhs = rewrite(lhs, &|val: &Value| -> Option<Value> {
                    if let Value::Symbol(symbol) = &val {
                        item.substituted.get(symbol).cloned()
                    } else {
                        None
                    }
                });

                let instruction = Instruction {
                    constructor: constructor.clone(),
                    signature: Type::Unit,
                    mnemonic: item.mnemonic(),
                    constants: item.substituted.clone(),
                    string: StringConstructor {
                        constr: lhs.try_into()?,
                        args: arguments.clone(),
                        cond: cond.clone(),
                    },
                    binary: Vec::new(),
                };

                result.push(instruction)
            }
        } else if let Value::String(mnemonic) = &rhs {
            let instruction = Instruction {
                constructor: constructor.clone(),
                signature: Type::Unit,
                mnemonic: mnemonic.clone(),
                constants: BTreeMap::new(),
                string: StringConstructor::default(),
                binary: Vec::new(),
            };

            result.push(instruction)
        } else {
            return err!("unsupported `{rhs:?}`");
        }
    }

    Ok(result)
}

fn split_at_space(args: &[Value]) -> (Vec<Value>, Vec<Value>) {
    let index = args
        .iter()
        .position(|val| matches!(val, Value::Builtin(Builtin::Space)));

    if let Some(index) = index {
        let before = args[..index].to_vec();
        let after = args[index + 1..].to_vec();

        (before, after)
    } else {
        let before: Vec<Value> = args.to_vec();
        let after = Vec::<Value>::new();

        (before, after)
    }
}

fn expand_functions(
    types: &mut TypesRepository,
    mnemonic: &[Value],
    sail: &Sail,
) -> crate::Result<Vec<Expanded>> {
    let mut result = Vec::<Expanded>::new();

    let mut mappings = Vec::<Option<Mapping>>::new();
    let mut n = 1;
    for value in mnemonic {
        if let Ok(fun) = value.as_fn_call() {
            let mapping = types.mapping(&fun.name, sail)?;

            if fun.args.len() != 1 {
                return err!(
                    "function `{}` is expected to take exactly one argument",
                    fun.name
                );
            }

            n *= mapping.pairs.len();
            mappings.push(Some(mapping.clone()));
        } else {
            mappings.push(None);
        }
    }

    for index in 0..n {
        let mut idx = index;

        let mut expanded = Expanded::default();
        let mut update = true;
        for (i, value) in mnemonic.iter().enumerate() {
            if let Ok(fun) = value.as_fn_call() {
                let mapping = mappings[i].as_ref().unwrap();

                let k = mapping.pairs.len();
                let pair = &mapping.pairs[idx % k];
                expanded.arguments.push(pair.rhs.clone());

                match &fun.args[0] {
                    Value::Symbol(name) => {
                        if let Some(typename) = mapping.signature.lhs.as_enum() {
                            let label = pair.lhs.as_symbol()?;
                            expanded.substituted.insert(
                                name.clone(),
                                Value::EnumLabel(EnumLabel {
                                    typename: typename.clone(),
                                    label: label.clone(),
                                }),
                            );
                        } else if let Some(typename) = mapping.signature.lhs.as_struct() {
                            let struct_type = types.get_struct(typename, sail)?;
                            let mut struct_val = pair.lhs.as_struct()?.clone();

                            for (label, val) in struct_val.values.iter_mut() {
                                if let Value::Symbol(ident) = val {
                                    let typ = match struct_type.fields.get(label) {
                                        Some(typ) => typ,
                                        None => {
                                            return err!(
                                                "no field named {label} in struct {typename}"
                                            );
                                        }
                                    };
                                    if let Some(enum_typename) = typ.as_enum() {
                                        *val = Value::EnumLabel(EnumLabel {
                                            typename: enum_typename.clone(),
                                            label: ident.clone(),
                                        });
                                    }
                                }
                            }

                            expanded
                                .substituted
                                .insert(name.clone(), Value::Struct(struct_val));
                        } else {
                            expanded.substituted.insert(name.clone(), pair.lhs.clone());
                        }
                    }
                    Value::Tuple(variables) => {
                        let values = pair.lhs.as_tuple()?;
                        if values.len() != values.len() {
                            return err!("mismatched lengths");
                        }

                        for (var, val) in zip(variables, values) {
                            let symbol = var.as_symbol()?;
                            expanded.substituted.insert(symbol.clone(), val.clone());
                        }
                    }
                    Value::BinaryConcatenation(bc) => {
                        let bv = pair.lhs.as_bitvector()?;
                        match match_bitconcatenation(bv, bc) {
                            MatchResult::Err(err) => return Err(err),
                            MatchResult::NoMatch => {
                                update = false;
                            }
                            MatchResult::Match(vars) => {
                                for (var, val) in vars {
                                    expanded.substituted.insert(var, Value::BitVector(val));
                                }
                            }
                        }
                    }
                    _ => {
                        return err!("unsupported `{:?}`", fun.args[0]);
                    }
                }

                idx /= k;
            } else {
                expanded.arguments.push(value.clone());
            }
        }

        if update {
            result.push(expanded);
        }
    }

    Ok(result)
}

#[derive(Debug, Clone, Default)]
struct Expanded {
    substituted: BTreeMap<String, Value>,
    arguments: Vec<Value>,
}

impl Expanded {
    pub fn mnemonic(&self) -> String {
        let mut result = String::new();
        for arg in self.arguments.iter() {
            match arg {
                Value::String(s) => {
                    result += s;
                }
                _ => panic!("unexpected `{arg:?}`"),
            }
        }

        result
    }
}

#[derive(Debug)]
enum MatchResult {
    Err(crate::Error),
    NoMatch,
    Match(BTreeMap<String, BitVector>),
}

impl MatchResult {
    pub fn err(s: String) -> Self {
        Self::Err(s.into())
    }
}

fn match_bitconcatenation(v: &BitVector, c: &BinaryConcatenation) -> MatchResult {
    let Some(total_bit_width) = c.total_width() else {
        debug!("{c:?}");
        return MatchResult::err(errfmt!("binary concatenation has undefined width"));
    };

    if total_bit_width != v.bit_width {
        return MatchResult::err(errfmt!(
            "bit width does not match {} != {total_bit_width}",
            v.bit_width
        ));
    }

    let mut variables = BTreeMap::<String, BitVector>::new();

    let mut input = v.clone();

    for val in c.0.iter().rev() {
        match val {
            Value::BitVector(bv) => {
                if (input.val & bv.mask()) != bv.val {
                    return MatchResult::NoMatch;
                }

                input = input.shr(bv.bit_width);
            }
            Value::Cast(item, Type::BitVector(bit_width)) => {
                let bit_width = *bit_width;
                let mask = (1_u64 << bit_width) - 1;
                let v = BitVector::try_new(input.val & mask, bit_width).unwrap();

                let Value::Symbol(name) = item.as_ref() else {
                    return MatchResult::Err(errfmt!("unsupported cast from {val:?}").into());
                };

                variables.insert(name.clone(), v);

                input = input.shr(bit_width);
            }
            _ => return MatchResult::Err(errfmt!("unsupported {val:?}").into()),
        }
    }

    MatchResult::Match(variables)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expand_mnemonics() {
        let code = r#"
        val assembly : instruction <-> string

        mapping assembly = {
            CSRReg(csr, rs1, rd, op) <-> csr_mnemonic(op) ^ spc(()) ^ "test"
        }

        enum csrop = {CSRRW, CSRRS, CSRRC}

        val csr_mnemonic : csrop <-> string

        mapping csr_mnemonic = {
          CSRRW <-> "csrrw",
          CSRRS <-> "csrrs",
          CSRRC <-> "csrrc"
        }
        "#;

        let sail = Sail::new(code.to_string());
        let mapping = sail.mapping("assembly").unwrap();
        let mut types = TypesRepository::default();

        let expanded = expand_mnemonics(&mut types, &mapping, &sail).unwrap();

        assert_eq!(expanded.len(), 3);
        assert_eq!(expanded[2].mnemonic, "csrrc");
        assert_eq!(
            *expanded[2].constants.get("op").unwrap(),
            enum_label("csrop", "CSRRC")
        );

        assert_eq!(expanded[1].mnemonic, "csrrs");
        assert_eq!(
            *expanded[1].constants.get("op").unwrap(),
            enum_label("csrop", "CSRRS")
        );

        assert_eq!(expanded[0].mnemonic, "csrrw");
        assert_eq!(
            *expanded[0].constants.get("op").unwrap(),
            enum_label("csrop", "CSRRW")
        );
    }

    fn enum_label(typename: &str, label: &str) -> Value {
        let el = EnumLabel {
            typename: typename.to_string(),
            label: label.to_string(),
        };

        Value::EnumLabel(el)
    }

    #[test]
    fn test_match_binary_case1() {
        let bv = BitVector::try_new(0b11_1101_010, 9).unwrap();
        let bc = BinaryConcatenation(vec![
            Value::BitVector(BitVector::try_new(0b11, 2).unwrap()),
            Value::BitVector(BitVector::try_new(0b1101, 4).unwrap()),
            Value::BitVector(BitVector::try_new(0b010, 3).unwrap()),
        ]);

        match match_bitconcatenation(&bv, &bc) {
            MatchResult::Match(set) => assert!(set.is_empty()),
            _ => assert!(false),
        }
    }

    fn sym(s: &str) -> Box<Value> {
        Box::new(Value::Symbol(s.into()))
    }

    #[test]
    fn test_match_binary_case2() {
        let bv = BitVector::try_new(0b11_110_1_010, 9).unwrap();
        let bc = BinaryConcatenation(vec![
            Value::BitVector(BitVector::try_new(0b11, 2).unwrap()),
            Value::Cast(sym("foo"), Type::BitVector(3)),
            Value::Cast(sym("bar"), Type::BitVector(1)),
            Value::BitVector(BitVector::try_new(0b010, 3).unwrap()),
        ]);

        let got = match_bitconcatenation(&bv, &bc);
        match got {
            MatchResult::Match(set) => {
                assert_eq!(set.len(), 2);
                assert_eq!(
                    *set.get("foo").unwrap(),
                    BitVector::try_new(0b110, 3).unwrap()
                );
                assert_eq!(
                    *set.get("bar").unwrap(),
                    BitVector::try_new(0b1, 1).unwrap()
                );
            }
            _ => assert!(false, "{got:?}"),
        }
    }
}
