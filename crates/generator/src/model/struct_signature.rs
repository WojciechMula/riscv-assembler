use super::Type;
use crate::err;
use crate::sail::Parser;
use crate::sail::Token;
use std::collections::BTreeMap;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct StructSignature {
    pub name: String,
    pub fields: BTreeMap<String, Type>,
}

impl StructSignature {
    pub fn parse(p: &mut Parser) -> crate::Result<Self> {
        let mut result = Self {
            name: p.identifier()?,
            fields: BTreeMap::new(),
        };

        p.expect(Token::Equals)?;
        p.expect(Token::OpenCurlyParen)?;
        loop {
            let name = p.identifier()?;
            p.expect(Token::Colon)?;
            let typ = Type::parse(p)?;

            result.fields.insert(name, typ);

            let token = p.peek();
            match token {
                Token::Comma => {
                    p.consume();
                }
                Token::CloseCurlyParen => {
                    p.consume();
                    break;
                }
                _ => return err!("parsing struct: unexpected token `{token:?}`"),
            }
        }

        Ok(result)
    }
}
