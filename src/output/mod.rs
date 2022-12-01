use crate::expr::Expr;

pub mod debug;
pub mod javascript;
pub mod pcre;

pub trait Output {
    fn output(&self, expr: &Expr) -> Result<String, OutputError>;
}

#[derive(Debug)]
pub enum OutputError {
    FeatureNotSupported,
}

impl std::error::Error for OutputError {}

impl std::fmt::Display for OutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FeatureNotSupported => write!(f, "feature is not supported by output format"),
        }
    }
}
