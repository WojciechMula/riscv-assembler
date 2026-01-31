use crate::err;
use crate::model::Type;
use crate::sail::Parser;
use crate::sail::Token;

pub fn parse_type(p: &mut Parser) -> crate::Result<Type> {
    let token = p.peek();
    match token {
        Token::Unit => {
            p.consume();

            Ok(Type::Unit)
        }
        Token::Identifier(ident) => match ident.as_str() {
            "bitvector" | "bits" => {
                p.consume();
                p.expect(Token::OpenParen)?;
                let size = p.number()?;
                p.expect(Token::CloseParen)?;

                Ok(Type::BitVector(size as usize))
            }
            "word" => {
                p.consume();
                Ok(Type::BitVector(32))
            }
            "half" => {
                p.consume();
                Ok(Type::BitVector(16))
            }
            "string" => {
                p.consume();

                Ok(Type::String)
            }
            "bool" => {
                p.consume();

                Ok(Type::Boolean)
            }
            _ => {
                let ident = ident.clone();
                p.consume();

                Ok(Type::Ident(ident))
            }
        },
        Token::OpenParen => {
            p.consume();
            let mut vals = Vec::<Type>::new();
            loop {
                vals.push(parse_type(p)?);

                let token = p.peek();
                match token {
                    Token::Comma => {
                        p.consume();
                        continue;
                    }
                    Token::CloseParen => {
                        p.consume();
                        break;
                    }
                    _ => return err!("unexpected token `{token:?}`"),
                }
            }

            Ok(Type::Tuple(vals))
        }
        Token::OpenCurlyParen => {
            // parses two cases:
            // a) `{1, 2, 4, 8}`
            // b) `{(var: typ), (num1 cmp var & num2 cmp var). int(var)}`

            if matches!(p.lookahead(1), Token::OpenParen) {
                parse_gen_range(p)
            } else {
                p.consume();
                let mut vals = Vec::<u64>::new();
                loop {
                    vals.push(p.number()?);

                    let token = p.peek();
                    match token {
                        Token::Comma => {
                            p.consume();
                            continue;
                        }
                        Token::CloseCurlyParen => {
                            p.consume();
                            break;
                        }
                        _ => return err!("unexpected token `{token:?}`"),
                    }
                }

                Ok(Type::Set(vals))
            }
        }
        _ => err!("unexpected token `{token:?}`"),
    }
}

/**
    Parses a generative range into a discrete set
    `{(var: typ), (num1 cmp var & num2 cmp var). int(var)}`
*/
fn parse_gen_range(p: &mut Parser) -> crate::Result<Type> {
    p.expect(Token::OpenCurlyParen)?;
    let range = parse_gen_range_aux(p)?;
    p.expect(Token::CloseCurlyParen)?;

    Ok(range)
}

fn parse_gen_range_aux(p: &mut Parser) -> crate::Result<Type> {
    // 1. parse declation: `{(var: typ)`
    p.expect(Token::OpenParen)?;
    let _ = p.metavar()?;
    p.expect(Token::Colon)?;
    let _ = Type::parse(p)?;
    p.expect(Token::CloseParen)?;

    p.expect(Token::Comma)?;

    let expect_close = p.try_consume(Token::OpenParen);

    // 2. parse relations: `(num1 cmp var & num2 cmp var ...)`
    let mut relations = Vec::<Relation>::new();
    loop {
        relations.push(parse_single_relation(p)?);
        if !p.try_consume(Token::BinAnd) {
            break;
        }
    }

    if relations.is_empty() {
        return err!("expected at least one relation");
    }

    if expect_close {
        p.expect(Token::CloseParen)?;
    }

    let min = relations.iter().map(|rel| rel.number()).min().unwrap();
    let max = relations.iter().map(|rel| rel.number()).max().unwrap();

    let mut values = Vec::<u64>::new();
    for x in min..=max {
        if relations.iter().all(|rel| rel.eval(x)) {
            values.push(x);
        }
    }

    p.expect(Token::Dot)?;

    // 3. eat int('var)
    let ident = p.identifier()?;
    if ident != "int" {
        return err!("expected type 'int', got `{ident}`");
    }
    p.expect(Token::OpenParen)?;
    p.metavar()?;
    p.expect(Token::CloseParen)?;

    Ok(Type::Set(values))
}

#[derive(Debug)]
enum Comparison {
    Less,
    Greater,
    LessEquals,
    GreaterEquals,
}

#[derive(Debug)]
enum Relation {
    Less(u64),
    Greater(u64),
    LessEquals(u64),
    GreaterEquals(u64),
}

impl Comparison {
    pub fn swap_order(&self) -> Self {
        match self {
            Self::Less => Self::Greater,
            Self::Greater => Self::Less,
            Self::LessEquals => Self::GreaterEquals,
            Self::GreaterEquals => Self::LessEquals,
        }
    }
}

impl Relation {
    pub fn new(cmp: Comparison, num: u64) -> Self {
        match cmp {
            Comparison::Less => Self::Less(num),
            Comparison::Greater => Self::Greater(num),
            Comparison::LessEquals => Self::LessEquals(num),
            Comparison::GreaterEquals => Self::GreaterEquals(num),
        }
    }

    pub fn eval(&self, x: u64) -> bool {
        match self {
            Self::Less(num) => x < *num,
            Self::Greater(num) => x > *num,
            Self::LessEquals(num) => x <= *num,
            Self::GreaterEquals(num) => x >= *num,
        }
    }

    pub fn number(&self) -> u64 {
        match self {
            Self::Less(num) => *num,
            Self::Greater(num) => *num,
            Self::LessEquals(num) => *num,
            Self::GreaterEquals(num) => *num,
        }
    }
}

fn parse_single_relation(p: &mut Parser) -> crate::Result<Relation> {
    if p.lookahead(0).is_metavar() && is_comparison(p.lookahead(1)) && p.lookahead(2).is_number() {
        p.consume();
        let cmp = parse_comparison(p)?;
        let num = p.number()?;

        return Ok(Relation::new(cmp, num));
    }

    if p.lookahead(0).is_number() && is_comparison(p.lookahead(1)) && p.lookahead(2).is_metavar() {
        let num = p.number()?;
        let cmp = parse_comparison(p)?;
        p.consume();

        return Ok(Relation::new(cmp.swap_order(), num));
    }

    err!("expected comparison in form 'var >/>=/<=/< const")
}

fn parse_comparison(p: &mut Parser) -> crate::Result<Comparison> {
    let token = p.peek();
    match token {
        Token::Less => {
            p.consume();
            Ok(Comparison::Less)
        }
        Token::Greater => {
            p.consume();
            Ok(Comparison::Greater)
        }
        Token::LessEquals => {
            p.consume();
            Ok(Comparison::LessEquals)
        }
        Token::GreaterEquals => {
            p.consume();
            Ok(Comparison::GreaterEquals)
        }
        _ => err!("expected <, <=, >=, >, got: {token:?}"),
    }
}

fn is_comparison(token: &Token) -> bool {
    matches!(
        token,
        Token::Less | Token::Greater | Token::LessEquals | Token::GreaterEquals
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sail::Sail;

    #[test]
    fn test_parse_gen_range() {
        let cases = [
            (
                "{('q : Int), ('q > 0 & 'q < 8). int('q)}",
                Type::Set(vec![1, 2, 3, 4, 5, 6, 7]),
            ),
            (
                "{('q : Int), ('q > 0 & 'q <= 8). int('q)}",
                Type::Set(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            ),
            (
                "{('e : Int), (0 <= 'e & 'e <= 1). int('e)}",
                Type::Set(vec![0, 1]),
            ),
        ];

        for (input, want) in cases {
            let sail = Sail::new(input.to_owned());
            let mut parser = sail.parser(0);

            let got = parse_gen_range(&mut parser).unwrap();
            assert_eq!(got, want, "input = '{input}'");
        }
    }
}
