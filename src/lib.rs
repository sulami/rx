use anyhow::Result;

use crate::output::Output;
use crate::parser::parse;

pub mod expr;
pub mod output;
pub mod parser;

pub fn convert(input: &str, output: &dyn Output) -> Result<String> {
    if let Ok((_, expr)) = parse(input) {
        Ok(output.output(&expr)?)
    } else {
        Err(anyhow::anyhow!("Failed to parse input"))
    }
}
