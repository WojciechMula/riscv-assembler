use super::parse_expression;
use crate::err;
use crate::model::Mapping;
use crate::model::MappingSignature;
use crate::model::Pair;
use crate::model::Value;
use crate::sail::Parser;
use crate::sail::Token;
use log::warn;

pub fn parse_mapping(p: &mut Parser, sig: MappingSignature) -> crate::Result<Mapping> {
    let mut result = Mapping {
        name: p.identifier()?,
        signature: sig.clone(),
        pairs: Vec::new(),
    };

    p.expect(Token::Equals)?;
    p.expect(Token::OpenCurlyParen)?;

    loop {
        let separator = if matches!(p.peek(), Token::Forwards | Token::Backwards) {
            p.consume();

            Token::ForwardArrow
        } else {
            Token::Bidrectional
        };

        let lhs = parse_expression(p)?;

        let mut lhs_cond = Value::Unit;
        if p.try_consume(Token::If) {
            lhs_cond = parse_expression(p)?;
        }

        p.expect(separator.clone())?;

        let rhs = parse_expression(p)?;

        let mut rhs_cond = Value::Unit;
        if p.try_consume(Token::If) {
            rhs_cond = parse_expression(p)?;
        }

        let cond = if !lhs_cond.is_unit() && !rhs_cond.is_unit() {
            if lhs_cond != rhs_cond {
                panic!("{lhs_cond:?} {rhs_cond:?}");
            }

            Some(lhs_cond)
        } else {
            None
        };

        if separator == Token::Bidrectional {
            result.pairs.push(Pair { lhs, rhs, cond });
        } else {
            warn!("ignoring forward/backward mapping: {lhs:?}");
        }

        let token = p.peek();
        match token {
            Token::Comma => {
                p.consume();
            }
            Token::CloseCurlyParen => {
                p.consume();
                break;
            }
            _ => {
                return err!("expected ',' or '}}', got `{token:?}`");
            }
        }
    }

    Ok(result)
}
