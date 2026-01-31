mod assembler;
mod bitvector;
mod context;
mod error;
mod helpers;
mod label_resolver_trait;
mod parser;
mod sail_functions;
mod sail_types;

pub type Result<T> = std::result::Result<T, Error>;

pub use assembler::assemble;
pub use context::Context;
pub use error::Error;
pub use label_resolver_trait::LabelResolverTrait;

#[macro_export]
macro_rules! err {
    ($($args:expr),*) => {
        Err(format!($($args),*).into())
    };
}
