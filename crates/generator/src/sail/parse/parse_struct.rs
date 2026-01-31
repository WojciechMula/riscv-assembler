use crate::model::Struct;
use crate::sail::Parser;
use crate::sail::Token;
use crate::sail::parse::parse_expression;
use std::collections::BTreeMap;

pub fn parse_struct(p: &mut Parser) -> crate::Result<Struct> {
    let mut result = Struct {
        typename: p.identifier()?,
        values: BTreeMap::new(),
    };

    p.expect(Token::OpenCurlyParen)?;
    loop {
        let name = p.identifier()?;
        p.expect(Token::Equals)?;
        let value = parse_expression(p)?;

        result.values.insert(name, value);

        if !p.try_consume(Token::Comma) {
            break;
        }
    }
    p.expect(Token::CloseCurlyParen)?;

    Ok(result)
}
