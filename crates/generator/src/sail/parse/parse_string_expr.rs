use super::parse_invocation;
use crate::err;
use crate::model::Value;
use crate::sail::Parser;
use crate::sail::Token;

pub fn parse_string_expr(p: &mut Parser) -> crate::Result<Value> {
    let mut args = Vec::<Value>::new();
    loop {
        let token = p.peek();
        match token {
            Token::String(s) => {
                args.push(Value::String(s.clone()));
                p.consume();
            }
            Token::Identifier(ident) => {
                if matches!(p.lookahead(1), Token::OpenParen) {
                    let call = parse_invocation(p)?;
                    args.push(call);
                } else {
                    let val = Value::Symbol(ident.clone());
                    p.consume();
                    args.push(val);
                }
            }
            _ => {
                if args.is_empty() {
                    break;
                } else {
                    return err!("expected string, got `{token:?}`");
                }
            }
        }

        if !p.try_consume(Token::BinXor) {
            break;
        }
    }

    if args.len() == 1 {
        Ok(args.pop().unwrap())
    } else {
        Ok(Value::StringConcatenation(args))
    }
}
