use crate::err;
use crate::generator::rust::bitvector_constructor;
use crate::generator::rust::bitvector_typname;
use crate::model::BitVector;
use crate::model::Pair;
use crate::model::Value;
use crate::sail::Sail;
use std::collections::BTreeMap;

macro_rules! w {
    ($f:expr, $($args:expr),*) => {
        $f.push_str(format!($($args),*).as_str());
        $f.push('\n')
    };
    ($target:expr) => {
        $f.push('\n')
    };
}

const MAPPING_NAME: &str = "csr_name_map";

#[derive(Default)]
pub struct CsrList {
    pub bit_width: usize,
    pub names: BTreeMap<String, u16>,
}

impl CsrList {
    pub fn new(sail: &Sail) -> crate::Result<Self> {
        let mapping = sail.mapping_raw(MAPPING_NAME)?;

        let bit_width = mapping.signature.lhs.as_bitvector()?;
        let max = (1_i64 << bit_width) - 1;
        let range = 0..=max;

        mapping.signature.rhs.assume_string()?;

        let mut names = BTreeMap::<String, u16>::new();
        for Pair { lhs, rhs, .. } in &mapping.pairs {
            let Value::Integer(csr_code) = &lhs else {
                continue;
            };

            let Value::String(name) = &rhs else {
                continue;
            };

            if !range.contains(csr_code) {
                return err!("CSR '{name}' = 0x{csr_code:x}: value outside range 0..2^{bit_width}");
            }

            names.insert(name.clone(), *csr_code as u16);
        }

        Ok(Self { bit_width, names })
    }

    pub fn rust(&self) -> crate::Result<String> {
        let mut f = String::new();

        let bitvector = bitvector_typname(self.bit_width);

        w!(f, "pub fn csr_code(s: &str) -> Option<{bitvector}> {{");
        w!(f, "match s {{");
        for (name, code) in &self.names {
            let bv = BitVector::try_new(*code as u64, self.bit_width)?;
            w!(f, "\"{name}\" => Some({}),", bitvector_constructor(&bv));
        }
        w!(f, "_ => None,");
        w!(f, "}}");
        w!(f, "}}");

        w!(
            f,
            "pub fn {MAPPING_NAME}(p: &mut Parser) -> crate::Result<{bitvector}> {{"
        );
        w!(f, "    let ident = p.expect_alpha_num()?;");
        w!(f, "    match csr_code(ident) {{");
        w!(f, "         Some(code) => Ok(code),");
        w!(f, "         None => {{");
        w!(f, "             let v = crate::parser::parse_u64(ident)?;"); // XXX: rework it!
        w!(f, "             Ok({bitvector}::new(v as u32))");
        w!(f, "         }}");
        w!(f, "    }}");
        w!(f, "}}");

        Ok(f)
    }
}
