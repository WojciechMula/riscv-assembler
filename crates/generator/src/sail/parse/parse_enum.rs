use crate::err;
use crate::model::Enum;
use crate::sail::Parser;
use crate::sail::Token;

pub fn parse_enum(p: &mut Parser) -> crate::Result<Enum> {
    let mut labels = Vec::<String>::new();

    let name = p.identifier()?;

    p.expect(Token::Equals)?;
    p.expect(Token::OpenCurlyParen)?;
    loop {
        let token = p.peek();
        match token {
            Token::Identifier(ident) => {
                labels.push(ident.clone());
                p.consume();
            }
            Token::Comma => {
                p.consume();
            }
            Token::CloseCurlyParen => {
                p.consume();
                break;
            }
            _ => return err!("parsing enum: unexpected token `{token:?}`"),
        }
    }

    Ok(Enum { name, labels })
}
