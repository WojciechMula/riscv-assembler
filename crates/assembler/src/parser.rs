use crate::Error;
use crate::bitvector::BitVector;
use crate::err;

pub struct Parser<'a> {
    original: &'a str,
    string: &'a str,
}

pub struct ParserState {
    consumed: usize,
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

    pub fn save(&self) -> ParserState {
        ParserState {
            consumed: self.original.len() - self.string.len(),
        }
    }

    pub fn restore(&mut self, s: ParserState) {
        self.string = &self.original[s.consumed..];
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

    pub fn expect_label(&mut self) -> crate::Result<&str> {
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

    pub fn expect_alpha_num(&mut self) -> crate::Result<&str> {
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

    pub fn expect_unsigned_immediate<const N: usize>(&mut self) -> crate::Result<BitVector<N>> {
        let s = self.expect_alpha_num()?;
        let v = parse_u64(s)?;
        let max = (1_u64 << N) - 1;
        if v <= max {
            Ok(BitVector::<N>::new(v as u32))
        } else {
            err!("{} exceedes 2^{N})", v)
        }
    }

    fn expect_i64(&mut self) -> crate::Result<(bool, u64)> {
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

        Ok((positive, v))
    }

    pub fn expect_signed_immediate<const N: usize>(&mut self) -> crate::Result<BitVector<N>> {
        let (positive, magnitude) = self.expect_i64()?;

        if positive {
            let max = (1_u64 << (N - 1)) - 1;
            if magnitude <= max {
                Ok(BitVector::<N>::new(magnitude as u32))
            } else {
                err!("{} exceedes 2^{}-1)", magnitude, N)
            }
        } else {
            let min = 1_u64 << (N - 1);
            if magnitude <= min {
                let v = -(magnitude as i64);

                Ok(BitVector::<N>::new(v as u32))
            } else {
                err!("-{} exceedes -2^{})", magnitude, N)
            }
        }
    }

    pub fn optional_signed<const N: usize>(&mut self) -> crate::Result<BitVector<N>> {
        if self.string.starts_with(['+', '-']) {
            return self.expect_signed_immediate::<N>();
        }

        let prev = self.string;
        if let Ok(value) = self.expect_signed_immediate::<N>() {
            Ok(value)
        } else {
            self.string = prev;

            Ok(BitVector::<N>::zero())
        }
    }

    pub fn optional_unsigned_nonzero<const N: usize>(&mut self) -> crate::Result<BitVector<N>> {
        if self.string.starts_with(['+', '-']) {
            return self.expect_unsigned_immediate::<N>();
        }

        let prev = self.string;
        if let Ok(value) = self.expect_unsigned_immediate::<N>() {
            Ok(value)
        } else {
            self.string = prev;

            Ok(BitVector::<N>::zero())
        }
    }

    pub fn simm5_plus_1(&mut self) -> crate::Result<BitVector<5>> {
        let (positive, magnitude) = self.expect_i64()?;

        if positive {
            if magnitude > 16 {
                return err!("{magnitude}: expected number in range -15..16");
            }

            let v = (magnitude as i64) + 16;

            Ok(BitVector::<5>::new(v as u32))
        } else {
            if magnitude > 15 {
                return err!("-{magnitude}: expected number in range -15..16");
            }

            let v = -(magnitude as i64) + 16;

            Ok(BitVector::<5>::new(v as u32))
        }
    }
}

pub fn parse_u64(s: &str) -> crate::Result<u64> {
    let initial = s;
    let (s, radix) = determine_number_base(s);
    match u64::from_str_radix(s, radix) {
        Ok(v) => Ok(v),
        Err(_) => err!("'{initial}': expected an usigned number"),
    }
}

pub fn parse_i64(s: &str) -> crate::Result<i64> {
    let initial = s;
    let (positive, s) = if let Some(s) = s.strip_prefix('+') {
        (true, s)
    } else if let Some(s) = s.strip_prefix('-') {
        (false, s)
    } else {
        (true, s)
    };

    let (s, radix) = determine_number_base(s);
    match u64::from_str_radix(s, radix) {
        Ok(v) => {
            let pos_mag_max = i64::MAX as u64;
            let neg_mag_max = pos_mag_max + 1;

            if positive {
                if v > pos_mag_max {
                    err!("{initial}: overflow i64 range")
                } else {
                    Ok(v as i64)
                }
            } else {
                if v > neg_mag_max {
                    return err!("{initial}: overflow i64 range");
                }

                if v == neg_mag_max {
                    Ok(i64::MIN)
                } else {
                    Ok(-(v as i64))
                }
            }
        }
        Err(_) => err!("'{initial}': expected a signed number"),
    }
}

fn determine_number_base(s: &str) -> (&str, u32) {
    if let Some(s) = s.strip_prefix("0x") {
        (s, 16)
    } else if let Some(s) = s.strip_prefix("0b") {
        (s, 2)
    } else {
        (s, 10)
    }
}
