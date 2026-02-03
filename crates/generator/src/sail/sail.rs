use super::Parser;
use super::Token;
use super::utils::find_line;
use crate::err;
use crate::model::Enum;
use crate::model::ExpandMapping;
use crate::model::Mapping;
use crate::model::MappingSignature;
use crate::model::StructSignature;
use crate::model::Type;
use crate::model::Union;
use log::error;
use logos::Logos;
use logos::Span;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Sail {
    pub input: String,
    pub tokens: Vec<(Token, Span)>,
    pub vals: BTreeMap<String, usize>,
    pub mappings: BTreeMap<String, usize>,
    pub enums: BTreeMap<String, usize>,
    pub unions: BTreeMap<String, usize>,
    pub structs: BTreeMap<String, usize>,
    pub aliases: BTreeMap<String, usize>,
}

impl Sail {
    pub fn new(s: String) -> Self {
        let mut res = Self {
            input: s,
            ..Self::default()
        };

        let lex = Token::lexer(&res.input).spanned();
        let mut annotation = FilterOutAnnotations::default();
        let mut comment = FilterOutComments::default();
        let mut prev = Token::None;
        for (token, span) in lex {
            let Ok(token) = token else {
                continue;
            };

            if !comment.check(&token) {
                continue;
            }

            if !annotation.check(&token) {
                continue;
            }

            let token = match combine_operator(&prev, &token) {
                CombineOperatorResult::Skip => {
                    prev = token.clone();
                    continue;
                }
                CombineOperatorResult::Passthrough => token,
                CombineOperatorResult::Replace(token) => token,
            };

            if let Token::Identifier(ident) = &token {
                match prev {
                    Token::Val => {
                        res.vals.insert(ident.clone(), res.tokens.len());
                    }
                    Token::Mapping => {
                        res.mappings.insert(ident.clone(), res.tokens.len());
                    }
                    Token::Enum => {
                        res.enums.insert(ident.clone(), res.tokens.len());
                    }
                    Token::Union => {
                        res.unions.insert(ident.clone(), res.tokens.len());
                    }
                    Token::Struct => {
                        if !res.structs.contains_key(ident) {
                            res.structs.insert(ident.clone(), res.tokens.len());
                        }
                    }
                    Token::Type => {
                        res.aliases.insert(ident.clone(), res.tokens.len());
                    }
                    _ => (),
                }
            }

            prev = token.clone();
            res.tokens.push((token, span));
        }

        res
    }

    pub fn parser<'a>(&'a self, offset: usize) -> Parser<'a> {
        Parser::new(&self.tokens, offset)
    }

    pub fn get_enum(&self, name: &str) -> crate::Result<Enum> {
        let Some(offset) = self.enums.get(name) else {
            return err!("enum `{name}` not found");
        };

        let mut p = self.parser(*offset);

        match Enum::parse(&mut p) {
            Ok(v) => Ok(v),
            Err(err) => Err(self.error(&p, err.to_string())),
        }
    }

    pub fn get_struct(&self, name: &str) -> crate::Result<StructSignature> {
        let Some(offset) = self.structs.get(name) else {
            return err!("structure `{name}` not found");
        };

        let mut p = self.parser(*offset);

        match StructSignature::parse(&mut p) {
            Ok(v) => Ok(v),
            Err(err) => Err(self.error(&p, err.to_string())),
        }
    }

    pub fn get_union(&self, name: &str) -> crate::Result<Union> {
        let Some(offset) = self.unions.get(name) else {
            return err!("union `{name}` not found");
        };

        let mut p = self.parser(*offset);

        match Union::parse(&mut p) {
            Ok(v) => Ok(v),
            Err(err) => Err(self.error(&p, err.to_string())),
        }
    }

    pub fn get_alias(&self, name: &str) -> crate::Result<Type> {
        let Some(offset) = self.aliases.get(name) else {
            return err!("alias `{name}` not found");
        };

        let mut p = self.parser(*offset);

        match Type::parse(&mut p) {
            Ok(v) => Ok(v),
            Err(err) => Err(self.error(&p, err.to_string())),
        }
    }

    pub fn mapping(&self, name: &str) -> crate::Result<Mapping> {
        self.mapping_aux(name, ExpandMapping::BitVector)
    }

    pub fn mapping_raw(&self, name: &str) -> crate::Result<Mapping> {
        self.mapping_aux(name, ExpandMapping::None)
    }

    fn mapping_aux(&self, name: &str, exp: ExpandMapping) -> crate::Result<Mapping> {
        let Some(offset) = self.mappings.get(name) else {
            return err!("mapping `{name}` not found");
        };

        let sig = self.mapping_signature(name)?;

        let mut p = self.parser(*offset);

        match Mapping::parse(&mut p, sig, exp) {
            Ok(v) => Ok(v),
            Err(msg) => Err(self.error(&p, msg.to_string())),
        }
    }

    fn error(&self, parser: &Parser, msg: String) -> crate::Error {
        let Some(range) = parser.current_pos() else {
            return msg.into();
        };

        let Some(res) = find_line(&self.input, range.start) else {
            return msg.into();
        };

        error!("line {}: {}", res.lineno, res.line);

        let space = " ".repeat(range.start - res.offset);
        let underline = "^".repeat(range.end - range.start);
        error!("line {}: {}{}", res.lineno, space, underline);
        error!("line {}: {}", res.lineno, msg);

        msg.into()
    }

    pub fn mapping_signature(&self, name: &str) -> crate::Result<MappingSignature> {
        let Some(offset) = self.vals.get(name) else {
            return err!("signature for `{name}` not found");
        };

        let mut p = self.parser(*offset);

        match MappingSignature::parse(&mut p) {
            Ok(sig) => Ok(sig),
            Err(err) => Err(self.error(&p, err.to_string())),
        }
    }

    pub fn what_is(&self, name: &str) -> IdentifierKind {
        if self.mappings.contains_key(name) {
            return IdentifierKind::Mapping;
        }

        if self.enums.contains_key(name) {
            return IdentifierKind::Enum;
        }

        if self.structs.contains_key(name) {
            return IdentifierKind::Struct;
        }

        if self.aliases.contains_key(name) {
            return IdentifierKind::Alias;
        }

        IdentifierKind::Other
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IdentifierKind {
    Mapping,
    Enum,
    Struct,
    Alias,
    Other,
}

#[derive(Default)]
struct FilterOutAnnotations {
    expected: Vec<Token>,
}

impl FilterOutAnnotations {
    pub fn check(&mut self, token: &Token) -> bool {
        if self.expected.is_empty() {
            if matches!(token, Token::StartAnnotation) {
                self.expected.push(Token::CloseSquareParen);

                false
            } else {
                true
            }
        } else {
            match token {
                Token::OpenParen => {
                    self.expected.push(Token::CloseParen);
                }
                Token::OpenCurlyParen => {
                    self.expected.push(Token::CloseCurlyParen);
                }
                Token::OpenSquareParen => {
                    self.expected.push(Token::CloseSquareParen);
                }
                Token::CloseParen | Token::CloseCurlyParen | Token::CloseSquareParen => {
                    let close = self.expected.pop().unwrap();
                    assert_eq!(close, *token);
                }
                _ => (),
            }

            false
        }
    }
}

#[derive(Default)]
struct FilterOutComments {
    level: usize,
}

impl FilterOutComments {
    pub fn check(&mut self, token: &Token) -> bool {
        if self.level == 0 {
            if matches!(token, Token::StartComment) {
                self.level += 1;

                false
            } else {
                true
            }
        } else {
            match token {
                Token::StartComment => {
                    self.level += 1;
                }
                Token::EndComment => {
                    self.level -= 1;
                }
                _ => (),
            }

            false
        }
    }
}

fn combine_operator(prev: &Token, token: &Token) -> CombineOperatorResult {
    if matches!(prev, Token::Operator) {
        if let Token::Identifier(ident) = token {
            let op = match ident.as_str() {
                "<_s" => "__lt_signed",
                ">_s" => "__gt_signed",
                "<=_s" => "__le_signed",
                ">=_s" => "__ge_signed",
                "<_u" => "__lt_unsigned",
                ">_u" => "__gt_unsigned",
                "<=_u" => "__le_unsigned",
                ">=_u" => "__ge_unsigned",
                _ => return CombineOperatorResult::Passthrough,
            };

            return CombineOperatorResult::Replace(Token::Identifier(op.to_string()));
        }
    }

    if matches!(token, Token::Operator) {
        CombineOperatorResult::Skip
    } else {
        CombineOperatorResult::Passthrough
    }
}

enum CombineOperatorResult {
    Passthrough,
    Replace(Token),
    Skip,
}
