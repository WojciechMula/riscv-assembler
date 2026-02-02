use super::parse_array;
use super::parse_invocation;
use super::parse_struct;
use crate::err;
use crate::model::BinaryConcatenation;
use crate::model::Builtin;
use crate::model::FunctionInvocation;
use crate::model::Type;
use crate::model::Value;
use crate::model::convert_to_binvector;
use crate::sail::Parser;
use crate::sail::Token;

pub fn parse_expression(p: &mut Parser) -> crate::Result<Value> {
    let mut expr = parse_expression_aux(p)?;

    if matches!(p.peek(), Token::BinXor) {
        let mut exprs = vec![expr];
        while p.try_consume(Token::BinXor) {
            let expr = parse_expression_aux(p)?;
            exprs.push(expr);
        }

        expr = Value::StringConcatenation(exprs);
    }

    if matches!(p.peek(), Token::BinConcat) {
        let mut exprs = vec![expr];
        while p.try_consume(Token::BinConcat) {
            let expr = parse_expression_aux(p)?;
            exprs.push(expr);
        }

        let bc = BinaryConcatenation(exprs);
        expr = Value::BinaryConcatenation(bc);
    }

    Ok(expr)
}

fn parse_expression_aux(p: &mut Parser) -> crate::Result<Value> {
    let expr = parse_single_expression(p)?;
    if p.try_consume(Token::Colon) {
        let typ = Type::parse(p)?;

        Ok(Value::Cast(Box::new(expr), typ))
    } else {
        Ok(expr)
    }
}

fn parse_single_expression(p: &mut Parser) -> crate::Result<Value> {
    let token = p.peek();
    match token {
        Token::None => err!("unexpected end of input"),
        Token::Unit => {
            p.consume();
            Ok(Value::Unit)
        }
        Token::Number(val) => {
            let val = Value::Integer(*val as i64);
            p.consume();
            Ok(val)
        }
        Token::String(s) => {
            let val = Value::String(s.clone());
            p.consume();

            Ok(val)
        }
        Token::True => {
            p.consume();
            Ok(Value::Boolean(true))
        }
        Token::False => {
            p.consume();
            Ok(Value::Boolean(true))
        }
        Token::Binary(_) => {
            let val = convert_to_binvector(token)?;
            p.consume();
            Ok(val)
        }
        Token::Struct => {
            p.consume();
            let val = parse_struct(p)?;
            Ok(Value::Struct(val))
        }
        Token::Identifier(ident) => {
            let ident = ident.to_owned();
            if matches!(p.lookahead(1), Token::OpenParen) {
                let call = parse_invocation(p)?;
                Ok(call)
            } else if matches!(p.lookahead(1), Token::OpenSquareParen) {
                // ident[hi .. lo]
                let ident = ident.to_owned();
                p.consume();
                p.expect(Token::OpenSquareParen)?;
                let hi = p.number()?;
                p.expect(Token::RangeSeparator)?;
                let lo = p.number()?;
                p.expect(Token::CloseSquareParen)?;

                let fun = Builtin::SubVector {
                    binding: ident,
                    lo: lo.try_into()?,
                    hi: hi.try_into()?,
                };

                Ok(fun.into())
            } else if matches!(p.lookahead(1), Token::Unit) {
                // matches "function()" -> [ident, unit]
                p.consume();
                p.expect(Token::Unit)?;
                Ok(Value::FunctionInvocation(FunctionInvocation {
                    name: ident,
                    args: Vec::new(),
                }))
            } else {
                p.consume();
                Ok(Value::Symbol(ident))
            }
        }
        Token::OpenParen => {
            p.consume();

            let mut exprs = Vec::<Value>::new();
            loop {
                let expr = parse_expression(p)?;
                exprs.push(expr);

                if !p.try_consume(Token::Comma) {
                    break;
                }
            }
            p.expect(Token::CloseParen)?;

            if exprs.len() != 1 {
                Ok(Value::Tuple(exprs))
            } else {
                Ok(exprs.pop().unwrap())
            }
        }
        Token::OpenSquareParen => {
            p.consume();
            let array = parse_array(p)?;
            Ok(Value::Tuple(array))
        }
        _ => {
            err!("while parsing expression: unexpected token {token:?}")
        }
    }
}
