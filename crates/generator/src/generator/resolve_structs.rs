use crate::generator::ResolvedTypes;
use crate::generator::resolve_enums::resolve_type;
use crate::model::Type;
use crate::model::Union;
use crate::sail::IdentifierKind;
use crate::sail::Sail;
use log::debug;
use std::collections::BTreeSet;

pub fn resolve_structs(
    types: &mut ResolvedTypes,
    instructions: &Union,
    sail: &Sail,
) -> crate::Result<()> {
    let mut names = BTreeSet::<String>::new();
    for (name, signature) in &instructions.0 {
        debug!("resolving constructor {name}");
        match signature {
            Type::Tuple(types) => {
                for typ in types {
                    if let Some(name) = resolve_type(typ) {
                        names.insert(name.clone());
                    };
                }
            }
            _ => {
                if let Some(name) = resolve_type(signature) {
                    names.insert(name.clone());
                }
            }
        }
    }

    for name in names {
        debug!("struct `{name}`");
        match sail.what_is(&name) {
            IdentifierKind::Enum => {}
            IdentifierKind::Struct => {
                let val = sail.get_struct(&name)?;
                types.structs.insert(name.clone(), val);
            }
            _ => panic!("type `{name}` cannot be identified"),
        }
    }

    Ok(())
}
