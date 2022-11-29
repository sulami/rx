use anyhow::Result;
use clap::{Parser, ValueEnum};

use rx::convert;
use rx::output::Output;
use rx::output::{debug::DebugOutput, pcre::PCREOutput};

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

pub fn main() -> Result<()> {
    let args = Args::parse();

    let out: &dyn Output = match args.output {
        OutputFormat::Debug => &DebugOutput {},
        OutputFormat::PCRE => &PCREOutput {},
    };

    print!("{}", convert(&args.expression, out)?);
    Ok(())
}
