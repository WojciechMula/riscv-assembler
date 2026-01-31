use super::parse_array;
use super::parse_binary_expr;
use super::parse_invocation;
use super::parse_list;
use crate::err;
use crate::model::FunctionInvocation;
use crate::model::Type;
use crate::model::Value;
use crate::model::convert_to_binvector;
use crate::sail::Parser;
use crate::sail::Token;

pub fn parse_expression(p: &mut Parser) -> crate::Result<Value> {
    let expr = parse_expression_aux(p)?;
    if p.try_consume(Token::Colon) {
        let typ = Type::parse(p)?;

        Ok(Value::Cast(Box::new(expr), typ))
    } else {
        Ok(expr)
    }
}

pub fn parse_expression_aux(p: &mut Parser) -> crate::Result<Value> {
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
        Token::Identifier(ident) => {
            let ident = ident.to_owned();
            if matches!(p.lookahead(1), Token::OpenParen) {
                let call = parse_invocation(p)?;
                Ok(call)
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
            if matches!(p.lookahead(1), Token::Identifier(_))
                && matches!(p.lookahead(2), Token::Colon)
            {
                let concat = parse_binary_expr(p)?;
                Ok(concat)
            } else {
                p.consume();
                let mut exprs = parse_list(p)?;
                if exprs.len() != 1 {
                    Ok(Value::Tuple(exprs))
                } else {
                    Ok(exprs.pop().unwrap())
                }
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
