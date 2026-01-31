use super::parse_expression;
use crate::err;
use crate::model::Value;
use crate::sail::Parser;
use crate::sail::Token;

pub fn parse_list(p: &mut Parser) -> crate::Result<Vec<Value>> {
    let mut list = Vec::<Value>::new();

    loop {
        match p.peek() {
            Token::None => {
                return err!("parsing list: unexpected end of input");
            }
            Token::Comma => {
                p.consume();
            }
            Token::CloseParen => {
                p.consume();
                break;
            }
            _ => {
                let expr = parse_expression(p)?;
                list.push(expr);
            }
        }
    }

    Ok(list)
}
