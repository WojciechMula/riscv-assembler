use crate::model::Type;
use crate::model::Struct;
use crate::err;
use crate::sail::Parser;
use crate::sail::Token;
use std::collections::BTreeMap;

    pub fn parse_struct(p: &mut Parser) -> crate::Result<Struct> {
        let mut result = Struct {
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
