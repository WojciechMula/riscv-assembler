use super::Type;
use crate::sail::Parser;
use crate::sail::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappingSignature {
    pub lhs: Type,
    pub rhs: Type,
}

impl MappingSignature {
    pub fn parse(p: &mut Parser) -> crate::Result<Self> {
        // ident : lhs <-> rhs
        p.identifier()?;
        p.expect(Token::Colon)?;

        let lhs = Type::parse(p)?;
        p.expect(Token::Bidrectional)?;
        let rhs = Type::parse(p)?;

        Ok(Self { lhs, rhs })
    }
}
