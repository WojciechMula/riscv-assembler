use crate::Error;
use crate::bitvector::BitVector;
use crate::err;

pub struct Parser<'a> {
    original: &'a str,
    string: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            original: s,
            string: s,
        }
    }

    pub const fn column(&self) -> usize {
        self.original.len() - self.string.len()
    }

    pub fn skip_ws(&mut self) {
        self.string = self.string.trim_start();
    }

    pub fn expect(&mut self, literal: &str) -> crate::Result<()> {
        match self.string.strip_prefix(literal) {
            Some(suffix) => {
                self.string = suffix;
                Ok(())
            }
            None => Err(Error::Expected(self.column(), literal.to_string())),
        }
    }

    pub fn expect_comma(&mut self) -> crate::Result<()> {
        self.skip_ws();
        self.expect(",")?;
        self.skip_ws();

        Ok(())
    }

    pub fn try_consume(&mut self, prefix: &str) -> bool {
        match self.string.strip_prefix(prefix) {
            Some(suffix) => {
                self.string = suffix;
                true
            }
            None => false,
        }
    }

    pub fn expect_label(&mut self) -> Result<&str, String> {
        let s = match self
            .string
            .char_indices()
            .find(|(_, chr)| !(chr.is_ascii_alphanumeric() || *chr == '_'))
        {
            Some((index, _)) => {
                let (ident, rest) = self.string.split_at(index);
                self.string = rest;

                ident
            }
            None => {
                let res = self.string;
                self.string = "";

                res
            }
        };

        if s.is_empty() {
            err!("expected a label")
        } else {
            Ok(s)
        }
    }

    pub fn expect_alpha_num(&mut self) -> Result<&str, String> {
        let s = match self
            .string
            .char_indices()
            .find(|(_, chr)| !chr.is_ascii_alphanumeric())
        {
            Some((index, _)) => {
                let (ident, rest) = self.string.split_at(index);
                self.string = rest;

                ident
            }
            None => {
                let res = self.string;
                self.string = "";

                res
            }
        };

        if s.is_empty() {
            err!("expected an alpha-numeric string")
        } else {
            Ok(s)
        }
    }

    pub fn consume_instruction(&mut self) -> &str {
        match self
            .string
            .char_indices()
            .find(|(_, chr)| !(chr.is_ascii_alphanumeric() || *chr == '.'))
        {
            Some((index, _)) => {
                let (ident, rest) = self.string.split_at(index);
                self.string = rest.trim_start();

                ident
            }
            None => {
                let res = self.string;
                self.string = "";

                res
            }
        }
    }

    pub fn expect_unsigned_immediate<const N: usize>(&mut self) -> Result<BitVector<N>, String> {
        let s = self.expect_alpha_num()?;
        let v = parse_u64(s)?;
        let max = (1_u64 << N) - 1;
        if v <= max {
            Ok(BitVector::<N>::new(v as u32))
        } else {
            Err(format!("{} exceedes 2^{N})", v))
        }
    }

    pub fn expect_signed_immediate<const N: usize>(&mut self) -> Result<BitVector<N>, String> {
        let (s, positive) = if let Some(suffix) = self.string.strip_prefix("+") {
            (suffix, true)
        } else if let Some(suffix) = self.string.strip_prefix("-") {
            (suffix, false)
        } else {
            (self.string, true)
        };

        self.string = s;
        let s = self.expect_alpha_num()?;
        let v = parse_u64(s)?;

        if positive {
            let max = (1_u64 << (N - 1)) - 1;
            if v <= max {
                Ok(BitVector::<N>::new(v as u32))
            } else {
                Err(format!("{} exceedes 2^{}-1)", v, N))
            }
        } else {
            let min = 1_u64 << (N - 1);
            if v <= min {
                let v = -(v as i64);

                Ok(BitVector::<N>::new(v as u32))
            } else {
                Err(format!("{} exceedes 2^{})", v, N))
            }
        }
    }

    pub fn optional_signed<const N: usize>(&mut self) -> crate::Result<BitVector<N>> {
        todo!();
    }

    pub fn optional_unsigned_nonzero<const N: usize>(&mut self) -> crate::Result<BitVector<N>> {
        todo!();
    }
}

pub fn parse_u64(s: &str) -> Result<u64, String> {
    if let Some(s) = s.strip_prefix("0x") {
        if let Ok(v) = u64::from_str_radix(s, 16) {
            return Ok(v);
        }
    }

    if let Some(s) = s.strip_prefix("0b") {
        if let Ok(v) = u64::from_str_radix(s, 2) {
            return Ok(v);
        }
    }

    if let Ok(v) = u64::from_str_radix(s, 10) {
        return Ok(v);
    }

    Err("expected an unsigned number".to_string())
}

pub fn parse_i64(s: &str) -> Option<i64> {
    None
}

fn determine_number_base(s: &str) -> (&str, usize) {
    if let Some(s) = s.strip_prefix("0x") {
        return (s, 16);
    }

    (s, 10)
}
