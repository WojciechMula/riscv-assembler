use crate::generator::rust::format_bitvector;
use crate::generator::rust::rust_constructor;
use crate::model::FunctionInvocation;
use crate::model::Value;
use crate::model::rewrite;

pub struct Condition {
    /// A rust code fragment using `Context` fileds/methods
    pub rust_expr: String,

    /// An error message when the condition is not met
    pub error_msg: String,
}

pub fn resolve_arch_condition(cond: &Value) -> Option<Condition> {
    let FunctionInvocation { name, args } = cond.as_fn_call().ok()?;

    fn value(rust_expr: String, error_msg: String) -> Option<Condition> {
        Some(Condition {
            rust_expr,
            error_msg,
        })
    }

    match (name.as_str(), args.len()) {
        ("eq_int", 2) => {
            if is_symbol(&args[0], "xlen") {
                match args[1].as_integer().unwrap_or(0) {
                    32 => {
                        return value(
                            "ctx.is_32bit()".into(),
                            "available only in 32-bit mode".into(),
                        );
                    }
                    64 => {
                        return value(
                            "ctx.is_64bit()".into(),
                            "available only in 64-bit mode".into(),
                        );
                    }
                    _ => (),
                }
            }
        }
        ("gteq_int", 2) => {
            if is_symbol(&args[0], "xlen") {
                match args[1].as_integer().unwrap_or(0) {
                    64 => {
                        return value(
                            "ctx.is_64bit()".into(),
                            "available only in 64-bit mode".into(),
                        );
                    }
                    _ => (),
                }
            }
        }
        ("currentlyEnabled", 1) => {
            if let Value::Symbol(sym) = &args[0] {
                let name = sym.strip_prefix("Ext_").unwrap_or(sym);
                return value(
                    format!("ctx.extensions.{sym}"),
                    format!("requried extension {name}"),
                );
            }
        }
        ("haveSingleFPU", 0) => {
            return value(
                "ctx.extensions.Ext_F || ctx.extensions.Ext_Zfinx".into(),
                "required extension F or Zfinx".into(),
            );
        }
        ("haveDoubleFPU", 0) => {
            return value(
                "ctx.extensions.Ext_D || ctx.extensions.Ext_Zdinx".into(),
                "required extension D or Zdinx".into(),
            );
        }
        ("haveHalfMin", 0) => {
            return value(
                "ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx || ctx.extensions.Ext_Zfhmin || ctx.extensions.Ext_Zhinxmin".into(),
                "required extension Zfh or Zhinx or Zfhmin or Zhinxmin".into(),
            );
        }
        ("haveHalfFPU", 0) => {
            return value(
                "ctx.extensions.Ext_Zfh || ctx.extensions.Ext_Zhinx".into(),
                "required extension Zfh or Zhinx".into(),
            );
        }
        _ => (),
    }

    None
}

pub fn resolve_runtime_cond(cond: &Value) -> Option<Condition> {
    let FunctionInvocation { name, args } = cond.as_fn_call().ok()?;

    fn value(rust_expr: String, error_msg: String) -> Option<Condition> {
        Some(Condition {
            rust_expr,
            error_msg,
        })
    }

    match (name.as_str(), args.len()) {
        ("neq_bits", 2) => {
            if let Value::Symbol(variable) = &args[0] {
                if let Value::BitVector(bv) = &args[1] {
                    return value(
                        format!("{variable} != {}", rust_constructor(&args[1])),
                        format!(
                            "argument `{variable}` must not be equal {}",
                            format_bitvector(bv)
                        ),
                    );
                }
            }
        }
        ("neq_anything", 2) => {
            if let Value::Symbol(variable) = &args[0] {
                if let Value::Symbol(constant) = &args[1] {
                    let name = if constant == "zreg" { "x0" } else { constant };

                    return value(
                        format!("{variable} != {constant}"),
                        format!("argument `{variable}` must not be equal {name}"),
                    );
                }
            }
        }
        _ => (),
    }

    None
}

fn is_symbol(v: &Value, expected: &str) -> bool {
    if let Ok(name) = v.as_symbol() {
        name == expected
    } else {
        false
    }
}

fn is_extension_condition(cond: &Value) -> bool {
    let Ok(FunctionInvocation { name, .. }) = cond.as_fn_call() else {
        return false;
    };

    matches!(
        name.as_str(),
        "currentlyEnabled" | "haveSingleFPU" | "haveDoubleFPU" | "haveHalfMin" | "haveHalfFPU"
    )
}

/**
    Function simplifies condition having mixed preconditions
    regarding ISA extensions and others.
*/
pub fn simplify_condition(cond: &Value) -> Value {
    rewrite(cond, &|v| simplify_aux(v))
}

fn simplify_aux(v: &Value) -> Option<Value> {
    if is_extension_condition(v) {
        return Some(Value::Unit);
    }

    let FunctionInvocation { name, args } = v.as_fn_call().ok()?;
    if !matches!(name.as_str(), "and_bool" | "or_bool") {
        return None;
    }

    let mut newargs: Vec<Value> = args.iter().filter(|v| !v.is_unit()).cloned().collect();

    match newargs.len() {
        0 => Some(Value::Unit),
        1 => Some(newargs.pop().unwrap()),
        _ => Some(Value::FunctionInvocation(FunctionInvocation {
            name: name.clone(),
            args: newargs,
        })),
    }
}
