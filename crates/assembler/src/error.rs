use std::fmt::Display;
use std::fmt::Formatter;

pub type Column = usize;

pub enum Error {
    UnrecognizedInstruction(Column),
    UnrecognizedCsr(Column, String),
    Expected(Column, String),
    Generic(Column, String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Generic(0, s)
    }
}

impl Error {
    pub const fn column(&self) -> Option<Column> {
        match self {
            Self::UnrecognizedInstruction(col) => Some(*col),
            Self::Expected(col, _) => Some(*col),
            Self::UnrecognizedCsr(col, _) => Some(*col),
            Self::Generic(col, _) => Some(*col),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::UnrecognizedInstruction(_) => write!(f, "unrecognized instruction"),
            Self::Expected(_, fragment) => write!(f, "expected '{fragment}'"),
            Self::UnrecognizedCsr(_, name) => write!(f, "unrecognized CSR '{name}'"),
            Self::Generic(_, msg) => write!(f, "{msg}"),
        }
    }
}
