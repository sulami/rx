use crate::expr::Expr;

pub mod debug;
pub mod javascript;
pub mod pcre;
pub mod pcre2;

pub trait Output {
    fn output(&self, expr: &Expr) -> Result<String, OutputError>;
}

#[derive(Debug)]
pub enum OutputError {
    FeatureNotSupported(&'static str),
}

impl std::error::Error for OutputError {}

impl std::fmt::Display for OutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FeatureNotSupported(feat) => {
                write!(f, "feature is not supported by output format: {feat}")
            }
        }
    }
}
