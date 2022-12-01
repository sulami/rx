use anyhow::Result;
use clap::{Parser, ValueEnum};

use rx::convert;
use rx::output::Output;
use rx::output::{
    debug::DebugOutput, javascript::JavascriptOutput, pcre::PCREOutput, pcre2::PCRE2Output,
};

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
    /// Perl Compatible Regular Expression, version 2
    PCRE2,
    /// Javascript
    JS,
}

pub fn main() -> Result<()> {
    let args = Args::parse();

    let out: &dyn Output = match args.output {
        OutputFormat::Debug => &DebugOutput {},
        OutputFormat::PCRE => &PCREOutput {},
        OutputFormat::PCRE2 => &PCRE2Output {},
        OutputFormat::JS => &JavascriptOutput {},
    };

    print!("{}", convert(&args.expression, out)?);
    Ok(())
}
