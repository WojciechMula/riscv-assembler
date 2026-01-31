use logos::Logos;

#[derive(Debug, PartialEq, Clone, Hash, Eq, Logos, Default)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[default]
    None,

    #[token("val")]
    Val,

    #[token("function")]
    Function,

    #[token("mapping")]
    Mapping,

    #[token("enum")]
    Enum,

    #[token("union")]
    Union,

    #[token("struct")]
    Struct,

    #[token("type")]
    Type,

    #[token("operator")]
    Operator,

    #[token("if", priority = 10)]
    If,

    #[token("unit")]
    #[token("()")]
    Unit,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("_")]
    Placeholder,

    #[token(".")]
    Dot,

    #[token("!")]
    Exclamation,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("/")]
    Divide,

    #[token("*")]
    Multiply,

    #[token("&")]
    BinAnd,

    #[token("^")]
    BinXor,

    #[token("|")]
    BinOr,

    #[token("@")]
    BinConcat,

    #[token("$")]
    Dollar,

    #[token("$[")]
    StartAnnotation,

    #[token(":")]
    Colon,

    #[token(";")]
    Semicolon,

    #[token(",")]
    Comma,

    #[token("=")]
    Equals,

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("{")]
    OpenCurlyParen,

    #[token("}")]
    CloseCurlyParen,

    #[token("[")]
    OpenSquareParen,

    #[token("]")]
    CloseSquareParen,

    #[token(">", priority = 10)]
    Greater,

    #[token("<", priority = 10)]
    Less,

    #[token(">=", priority = 10)]
    GreaterEquals,

    #[token("<=", priority = 10)]
    LessEquals,

    #[token("->")]
    #[token("=>")]
    Forward,

    #[token("<->")]
    Bidrectional,

    #[token("/*!")]
    StartComment,

    #[token("*/")]
    EndComment,

    #[regex("0x[0-9a-fA-F]+", capture_hex_value)]
    #[regex("[+-]0x[0-9a-fA-F]+", capture_signed_hex_value)]
    #[regex("[+-]?[0-9]+", capture_dec_value)]
    Number(u64),

    #[regex("0b[01_]+", capture_slice)]
    Binary(String),

    #[regex("\"[^\"]*\"", capture_quoted_string)]
    String(String),

    #[regex("[a-zA-Z_><][a-zA-Z0-9_=]+", capture_identifier)]
    #[regex("[a-zA-Z><][a-zA-Z0-9_=]*", capture_identifier)]
    Identifier(String),

    #[regex("'[a-z]", capture_identifier)]
    MetaVariable(String),
}

impl Token {
    pub fn is_metavar(&self) -> bool {
        matches!(self, Token::MetaVariable(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Token::Number(_))
    }
}

fn capture_hex_value(lex: &logos::Lexer<Token>) -> Option<u64> {
    let slice = lex.slice();
    u64::from_str_radix(&slice[2..], 16).ok()
}

fn capture_signed_hex_value(lex: &logos::Lexer<Token>) -> Option<u64> {
    let slice = lex.slice();
    let val = u64::from_str_radix(&slice[3..], 16).ok()?;

    if slice.starts_with("-") {
        let val = val as i64;
        val.checked_neg().map(|v| v as u64)
    } else if slice.starts_with('+') {
        Some(val)
    } else {
        None
    }
}

fn capture_dec_value(lex: &logos::Lexer<Token>) -> Option<u64> {
    match lex.slice().parse::<i64>() {
        Ok(val) => Some(val as u64),
        Err(_) => None,
    }
}

fn capture_quoted_string(lex: &logos::Lexer<Token>) -> String {
    let slice = lex.slice();
    slice[1..slice.len() - 1].to_owned()
}

fn capture_slice(lex: &logos::Lexer<Token>) -> String {
    lex.slice().to_owned()
}

fn capture_identifier(lex: &logos::Lexer<Token>) -> String {
    lex.slice().to_owned()
}
