use super::parse_expression;
use super::parse_invocation;
use crate::err;
use crate::model::Mapping;
use crate::model::MappingSignature;
use crate::model::Pair;
use crate::model::Type;
use crate::model::Value;
use crate::sail::Parser;
use crate::sail::Token;

pub fn parse_mapping(p: &mut Parser, sig: MappingSignature) -> crate::Result<Mapping> {
    let mut result = Mapping {
        name: p.identifier()?,
        signature: sig.clone(),
        pairs: Vec::new(),
    };

    p.expect(Token::Equals)?;
    p.expect(Token::OpenCurlyParen)?;

    loop {
        let lhs = match sig.lhs {
            Type::BitVector(..)
            | Type::Enum(..)
            | Type::Set(..)
            | Type::Struct(..)
            | Type::Boolean
            | Type::String
            | Type::Tuple(..) => sig.lhs.parse_value(p)?,
            Type::Ident(ref ident) => match ident.as_str() {
                "instruction" => parse_invocation(p)?,
                "regidx" => return Ok(result),
                _ => {
                    return err!(
                        "mapping `{}`: unsupported type `{:?}`",
                        result.name,
                        sig.lhs
                    );
                }
            },
            _ => return err!("unsupported type `{:?}`", sig.lhs),
        };

        let mut lhs_cond = Value::Unit;
        if p.try_consume(Token::If) {
            lhs_cond = parse_expression(p)?;
        }

        p.expect(Token::Bidrectional)?;
        let rhs = match sig.rhs {
            Type::BitVector(..)
            | Type::Enum(..)
            | Type::Set(..)
            | Type::Struct(..)
            | Type::Boolean
            | Type::String
            | Type::Tuple(..) => sig.rhs.parse_value(p)?,
            Type::Ident(ref ident) => match ident.as_str() {
                "instruction" => parse_invocation(p)?,
                _ => return err!("unsupported type `{:?}`", sig.rhs),
            },
            _ => return err!("unsupported type `{:?}`", sig.rhs),
        };

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

        result.pairs.push(Pair { lhs, rhs, cond });

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
