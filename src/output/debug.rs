use crate::expr::Expr;
use crate::output::{Output, OutputError};

#[derive(Default)]
pub struct DebugOutput {}

impl Output for DebugOutput {
    fn output(&self, expr: &Expr) -> Result<String, OutputError> {
        Ok(format!("{expr:#?}"))
    }
}
