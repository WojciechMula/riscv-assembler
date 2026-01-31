use crate::err;
use crate::model::Type;
use crate::model::Union;
use crate::sail::Parser;
use crate::sail::Token;

pub fn parse_union(p: &mut Parser) -> crate::Result<Union> {
    p.identifier()?;
    p.expect(Token::Equals)?;
    p.expect(Token::OpenCurlyParen)?;

    let mut result = Union::default();
    loop {
        let ident = p.identifier()?;
        p.expect(Token::Colon)?;
        let typ = Type::parse(p)?;

        result.0.insert(ident, typ);

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
                return err!("parsing union: unexpected token `{token:?}`");
            }
        }
    }

    Ok(result)
}
