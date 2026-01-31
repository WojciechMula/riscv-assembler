use super::parse_invocation;
use crate::err;
use crate::model::BinaryConcatenation;
use crate::model::FunctionInvocation;
use crate::model::Type;
use crate::model::Value;
use crate::model::convert_to_binvector;
use crate::sail::Parser;
use crate::sail::Token;

pub fn parse_binary_expr(p: &mut Parser) -> crate::Result<Value> {
    let mut args = Vec::<Value>::new();
    loop {
        let token = p.peek();
        match token {
            Token::Binary(_) => {
                let val = convert_to_binvector(token)?;
                args.push(val);
                p.consume();
            }
            Token::Number(v) => {
                let val = Value::Integer(*v as i64);
                args.push(val);
                p.consume();
            }
            Token::Identifier(ident) => {
                if matches!(p.lookahead(1), Token::OpenParen) {
                    let call = parse_invocation(p)?;
                    args.push(call);
                } else if matches!(p.lookahead(1), Token::OpenSquareParen) {
                    // ident[hi .. lo]
                    let ident = ident.to_owned();
                    p.consume();
                    p.expect(Token::OpenSquareParen)?;
                    let hi = p.number()?;
                    p.expect(Token::RangeSeparator)?;
                    let lo = p.number()?;
                    p.expect(Token::CloseSquareParen)?;

                    let fun = FunctionInvocation {
                        name: "asm::bitvector_subvector".to_string(),
                        args: vec![
                            Value::Symbol(ident),
                            Value::Integer(hi.try_into()?),
                            Value::Integer(lo.try_into()?),
                        ],
                    };

                    args.push(Value::FunctionInvocation(fun));
                } else {
                    let ident = ident.to_owned();
                    p.consume();
                    args.push(Value::Symbol(ident));
                }
            }
            Token::OpenParen => {
                // (ident: bitvector(n))
                p.consume();
                let ident = p.identifier()?;
                p.expect(Token::Colon)?;
                let typ = Type::parse(p)?;
                let _ = typ.as_bitvector()?;

                p.expect(Token::CloseParen)?;

                args.push(Value::SymbolCast(ident, typ));
            }
            _ => {
                if args.is_empty() {
                    break;
                } else {
                    return err!("expected binary expression, got `{token:?}`");
                }
            }
        }

        if !p.try_consume(Token::BinConcat) {
            break;
        }
    }

    if args.len() == 1 {
        Ok(args.pop().unwrap())
    } else {
        Ok(Value::BinaryConcatenation(BinaryConcatenation(args)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::BitVector;
    use crate::sail::Sail;

    fn bin(val: u64, bit_width: usize) -> Value {
        Value::BitVector(BitVector { val, bit_width })
    }

    fn sym(v: &str, width: usize) -> Value {
        let typ = Type::BitVector(width);

        Value::SymbolCast(v.to_string(), typ)
    }

    fn concat(v: Vec<Value>) -> Value {
        Value::BinaryConcatenation(BinaryConcatenation(v))
    }

    #[test]
    fn test_parse_binary_expr() {
        let cases: &[(&str, Result<Value, String>)] = &[
            ("0b10101", Ok(bin(0b10101, 5))),
            ("0b1 @ 0b01", Ok(concat(vec![bin(0b1, 1), bin(0b01, 2)]))),
            (
                "0b1 @ 0b01 @ 0b1111",
                Ok(concat(vec![bin(0b1, 1), bin(0b01, 2), bin(0b1111, 4)])),
            ),
            (
                "0b1 @ 0b01 @ 0b1111 <->",
                Ok(concat(vec![bin(0b1, 1), bin(0b01, 2), bin(0b1111, 4)])),
            ),
            (
                "(a: bitvector(3)) @ (bb: bitvector(2)) @ 0b010",
                Ok(concat(vec![sym("a", 3), sym("bb", 2), bin(0b010, 3)])),
            ),
            (
                "0b11 @ true",
                Err("expected binary expression, got `True`".to_string()),
            ),
        ];

        for (input, expected) in cases {
            let sail = Sail::new(input.to_string());
            let mut parser = sail.parser(0);

            let val = parse_binary_expr(&mut parser).map_err(|err| err.to_string());
            assert_eq!(*expected, val);
        }
    }
}
