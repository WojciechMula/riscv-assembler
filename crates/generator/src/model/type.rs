use super::StructSignature;
use super::Value;
use crate::err;
use crate::sail::Parser;
use crate::sail::Token;
use crate::sail::parse::parse_binary_expr;
use crate::sail::parse::parse_string_expr;
use crate::sail::parse::parse_type;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unit,
    Ident(String),
    BitVector(usize),
    Boolean,
    Set(Vec<u64>),
    Tuple(Vec<Type>),
    Enum(String),
    Mapping(String),
    Struct(StructSignature),
    String,
}

impl Type {
    pub fn parse(p: &mut Parser) -> crate::Result<Self> {
        parse_type(p)
    }

    pub fn is_bitvector(&self) -> bool {
        matches!(self, Self::BitVector(..))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String)
    }

    pub fn parse_value(&self, p: &mut Parser) -> crate::Result<Value> {
        match self {
            Self::BitVector(_) => parse_binary_expr(p),
            Self::String => parse_string_expr(p),
            Self::Boolean => {
                let flag = p.boolean()?;

                Ok(Value::Boolean(flag))
            }
            Self::Enum(_) => {
                let label = p.identifier()?;

                Ok(Value::Symbol(label))
            }
            Self::Set(_) => {
                let label = p.number()?;

                Ok(Value::Integer(label as i64))
            }
            Self::Tuple(args) => {
                let mut values = Vec::<Value>::with_capacity(args.len());
                p.expect(Token::OpenParen)?;
                for arg in args {
                    values.push(arg.parse_value(p)?);

                    let token = p.peek();
                    match token {
                        Token::Comma => {
                            p.consume();
                        }
                        Token::CloseParen => {
                            p.consume();
                            break;
                        }
                        _ => {
                            return err!("expected ',' or '(', got `{token:?}`");
                        }
                    }
                }

                Ok(Value::Tuple(values))
            }
            Self::Struct(struct_typ) => {
                let mut values = BTreeMap::<String, Value>::new();

                p.expect(Token::Struct)?;
                p.identifier()?;
                p.expect(Token::OpenCurlyParen)?;
                for typ in struct_typ.fields.values() {
                    let name = p.identifier()?;
                    p.expect(Token::Equals)?;
                    let val = typ.parse_value(p)?;

                    values.insert(name, val);

                    if !p.try_consume(Token::Comma) {
                        break;
                    }
                }
                p.expect(Token::CloseCurlyParen)?;

                Ok(Value::Struct(struct_typ.name.clone(), values))
            }
            _ => todo!("not implemented for {self:?}"),
        }
    }

    pub fn ident(&self) -> Option<&String> {
        match self {
            Type::Ident(name) => Some(name),
            _ => None,
        }
    }

    pub fn as_enum(&self) -> Option<&String> {
        match self {
            Type::Enum(name) => Some(name),
            _ => None,
        }
    }

    pub fn as_bitvector(&self) -> crate::Result<usize> {
        match self {
            Type::BitVector(bit_width) => Ok(*bit_width),
            _ => err!("expected bitvector type, got `{:?}`", self),
        }
    }

    pub fn assume_string(&self) -> crate::Result<()> {
        match self {
            Type::String => Ok(()),
            _ => err!("expected string type, got `{:?}`", self),
        }
    }

    pub fn as_tuple(&self) -> crate::Result<&Vec<Type>> {
        match self {
            Type::Tuple(types) => Ok(types),
            _ => err!("expected tuple type, got `{:?}`", self),
        }
    }
}
