use crate::model::Value;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum Builtin {
    #[default]
    /// Matches "," sorrounded by optional spaces
    Separator,

    /// Matches at least one space (" "+)
    Space,

    /// Matches any number of spaces (" "*)
    OptionalSpace,

    /// Optional signed k-bit number, stored into 'binding'
    OptionalSigned {
        k: usize,
        binding: String,
    },

    /// Parse stack pointer register (sp or x2)
    StackPointer,

    /// Nonzero k-bit constant or 0 if nothing
    MaybeNonzero {
        k: usize,
        binding: String,
    },

    CsrName {
        binding: String,
    },

    SubVector {
        binding: String,
        lo: usize,
        hi: usize,
    },

    Label {
        binding: String,
        bit_width: usize,
    },
}

impl From<Builtin> for Value {
    fn from(v: Builtin) -> Self {
        Value::Builtin(v)
    }
}
