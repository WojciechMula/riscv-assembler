mod generator;
mod sail;

pub mod model;

pub use generator::generate;

pub type Error = Box<dyn std::error::Error + 'static>;
pub type Result<T> = anyhow::Result<T, Error>;

#[macro_export]
macro_rules! errfmt {
    ($($args:expr),*) => {
        format!("{}:{}: {}", file!(), line!(), format!($($args),*))
    };
}

#[macro_export]
macro_rules! err {
    ($($args:expr),*) => {
        Err(format!("{}:{}: {}", file!(), line!(), format!($($args),*)).as_str().into())
    };
}

pub fn assert_equals<T: std::cmp::PartialEq>(a: T, b: T, msg: String) -> crate::Result<()> {
    if a == b { Ok(()) } else { Err(msg.into()) }
}
