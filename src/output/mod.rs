use crate::expr::Expr;

pub mod debug;
pub mod pcre;

pub trait Output {
    fn output(&self, expr: &Expr) -> Result<String, OutputError>;
}

#[derive(Debug)]
pub enum OutputError {}

impl std::error::Error for OutputError {}

impl std::fmt::Display for OutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "feature not supported by output format")
    }
}
