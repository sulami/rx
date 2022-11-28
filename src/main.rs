use anyhow::Result;
use clap::{Parser, ValueEnum};

use crate::output::Output;
use crate::output::{debug::DebugOutput, pcre::PCREOutput};

mod expr;
mod output;
mod parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Output format
    #[clap(short, long, value_enum)]
    output: OutputFormat,
    /// The rx expression
    expression: String,
}

#[derive(Copy, Clone, ValueEnum)]
enum OutputFormat {
    /// An unstable, human-readable format for debugging parsing.
    Debug,
    /// Perl Compatible Regular Expression
    PCRE,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let (_, expr) = parser::parse(&args.expression).expect("");

    let out: &dyn Output = match args.output {
        OutputFormat::Debug => &DebugOutput {},
        OutputFormat::PCRE => &PCREOutput {},
    };

    print!("{}", out.output(&expr)?);
    Ok(())
}
