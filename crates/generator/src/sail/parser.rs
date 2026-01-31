use super::Token;
use crate::err;
use logos::Span;
use std::ops::Range;

pub struct Parser<'a> {
    pub tokens: &'a Vec<(Token, Span)>,
    pub current: usize,
    none: Token,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<(Token, Span)>, pos: usize) -> Self {
        Self {
            tokens,
            current: pos,
            none: Token::None,
        }
    }

    pub fn expect(&mut self, token: Token) -> crate::Result<()> {
        let curr = self.peek();
        if *curr == token {
            self.consume();
            Ok(())
        } else {
            err!("expected {token:?}, got {curr:?}")
        }
    }

    pub fn boolean(&mut self) -> crate::Result<bool> {
        let curr = self.peek();
        match curr {
            Token::True => {
                self.consume();

                Ok(true)
            }
            Token::False => {
                self.consume();

                Ok(false)
            }
            _ => err!("expected boolean value, got {curr:?}"),
        }
    }

    pub fn number(&mut self) -> crate::Result<u64> {
        let curr = self.peek();
        if let Token::Number(val) = curr {
            let val = *val;
            self.consume();

            Ok(val)
        } else {
            err!("expected number, got {curr:?}")
        }
    }

    pub fn identifier(&mut self) -> crate::Result<String> {
        let curr = self.peek();
        if let Token::Identifier(ident) = curr {
            let val = ident.to_owned();
            self.consume();

            Ok(val)
        } else {
            err!("expected identifier, got {curr:?}")
        }
    }

    pub fn metavar(&mut self) -> crate::Result<String> {
        let curr = self.peek();
        if let Token::MetaVariable(ident) = curr {
            let val = ident.to_owned();
            self.consume();

            Ok(val)
        } else {
            err!("expected metavariable ('letter), got {curr:?}")
        }
    }

    fn get(&self, index: usize) -> &Token {
        self.tokens
            .get(index)
            .map(|(token, _)| token)
            .unwrap_or(&self.none)
    }

    pub fn lookahead(&self, n: usize) -> &Token {
        self.get(self.current + n)
    }

    pub fn peek(&self) -> &Token {
        self.get(self.current)
    }

    pub fn try_consume(&mut self, token: Token) -> bool {
        if *self.peek() == token {
            self.consume();

            true
        } else {
            false
        }
    }

    pub fn consume(&mut self) {
        self.current += 1;
    }

    pub fn dump(&self, n: usize) {
        let min = self.current.saturating_sub(n);
        let max = self.current + n;

        for i in min..max {
            if let Some((token, _)) = self.tokens.get(i) {
                if i == self.current {
                    println!("> {:?}", token);
                } else {
                    println!("  {:?}", token);
                }
            } else {
                break;
            }
        }
    }

    pub fn current_pos(&self) -> Option<&Range<usize>> {
        self.tokens.get(self.current).map(|(_, span)| span)
    }
}
