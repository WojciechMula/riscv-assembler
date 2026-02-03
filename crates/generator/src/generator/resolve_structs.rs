use crate::TypesRepository;
use crate::generator::resolve_enums::type_name;
use crate::model::Type;
use crate::model::Union;
use crate::sail::IdentifierKind;
use crate::sail::Sail;
use log::debug;
use std::collections::BTreeSet;

pub fn resolve_structs(
    types: &mut TypesRepository,
    instructions: &Union,
    sail: &Sail,
) -> crate::Result<()> {
    let mut names = BTreeSet::<String>::new();
    for (name, signature) in &instructions.0 {
        debug!("resolving constructor {name}");
        match signature {
            Type::Tuple(types) => {
                for typ in types {
                    if let Some(name) = type_name(typ) {
                        names.insert(name.clone());
                    };
                }
            }
            _ => {
                if let Some(name) = type_name(signature) {
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

    for struct_type in types.structs.values_mut() {
        for field_type in struct_type.fields.values_mut() {
            if let Type::Ident(symbol) = field_type {
                if sail.what_is(symbol) == IdentifierKind::Enum {
                    *field_type = Type::Enum(symbol.to_string());
                }
            }
        }
    }

    Ok(())
}
