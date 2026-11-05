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

    /// Parse CSR name or number
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

    /// Represents a reference to CSR of given name
    NamedCsr {
        name: String,
    },

    /// 5-bit signed immediate encoded from -15..16 range (used only in pseudo-ops)
    Imm5Plus1 {
        binding: String,
    },
}

impl From<Builtin> for Value {
    fn from(v: Builtin) -> Self {
        Value::Builtin(v)
    }
}
