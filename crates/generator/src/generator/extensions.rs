use crate::err;
use crate::generator::collect_mappings;
use crate::model::Type;
use crate::sail::Sail;
use std::collections::HashMap;

macro_rules! w {
    ($f:expr, $($args:expr),*) => {
        $f.push_str(format!($($args),*).as_str());
        $f.push('\n')
    };
    ($target:expr) => {
        $f.push('\n')
    };
}

const ENUM_NAME: &str = "extension";

type EnumerationLabel = String;

#[derive(Default)]
pub struct Extensions {
    extensions: Vec<(EnumerationLabel, String)>,
}

impl Extensions {
    pub fn new(sail: &Sail) -> crate::Result<Self> {
        let enumeration = sail.get_enum(ENUM_NAME)?;
        let mut mappings = collect_mappings(sail, |signature| {
            let Type::Ident(name) = &signature.lhs else {
                return false;
            };

            if name != ENUM_NAME {
                return false;
            }

            signature.rhs.is_string()
        });

        let n = mappings.len();
        if n != 1 {
            return err!(
                "expected exactly one mapping {ENUM_NAME} <-> string, found {n} mapping(s)"
            );
        }

        let mapping = mappings.pop().unwrap();
        let mut tmp = HashMap::<String, String>::new();
        for pair in &mapping.pairs {
            let label = pair.lhs.as_symbol()?;
            let name = pair.rhs.as_string()?;

            tmp.insert(label.clone(), name.clone());
        }

        let mut result = Self::default();
        for label in enumeration.labels {
            let Some(name) = tmp.remove(&label) else {
                return err!("no name for extension '{label}'");
            };

            result.extensions.push((label, name));
        }

        Ok(result)
    }

    pub fn rust(&self) -> String {
        let mut f = String::new();

        let typename = "Extensions";

        w!(f, "#[derive(Default, Debug, Clone)]");
        w!(f, "#[allow(non_snake_case)]");
        w!(f, "pub struct {typename} {{");
        for (field, _) in &self.extensions {
            w!(f, "pub {field}: bool,");
        }
        w!(f, "}}");

        w!(f, "impl {typename} {{");
        {
            w!(
                f,
                "pub fn new(extensions: &[&String]) -> crate::Result<Self> {{"
            );
            w!(f, "let mut res = Self::default();");
            w!(f, "for ext in extensions {{");
            w!(f, "match ext.to_lowercase().as_str() {{");
            for (field, name) in &self.extensions {
                w!(f, "\"{name}\" => res.{field} = true,");
            }
            w!(f, "_ => return err!(\"unknown extension '{{ext}}'\"),");
            w!(f, "}}");
            w!(f, "}}");
            w!(f, "Ok(res)");
            w!(f, "}}");
        }

        {
            w!(f, "pub const fn all() -> Self {{");
            w!(f, "Self {{");
            for (key, _) in &self.extensions {
                w!(f, "{key}: true,");
            }
            w!(f, "}}");
            w!(f, "}}");
        }
        w!(f, "}}");

        f
    }
}
