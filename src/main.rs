use anyhow::Context;
use cal::Evaluator;
use clap::{Args, Parser};

/// A cli calculator
#[derive(Debug, Parser)]
#[command(version=git_version::git_version!(
    prefix = "git:",
    cargo_prefix = "cargo",
    fallback = "wtf"
))]
struct Cli {
    #[command(flatten)]
    format: PrintFormat,
    #[arg(action = clap::ArgAction::Append)]
    expr: Vec<String>,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct PrintFormat {
    /// Output with hexadecimal format
    /// (the result of the expression must be integer)
    #[arg(long, short = 'x')]
    hex: bool,
    /// Output with octal format
    /// (the result of the expression must be integer)
    #[arg(long, short)]
    oct: bool,
    /// Output with binary format
    /// (the result of the expression must be integer)
    #[arg(long, short)]
    bin: bool,
    /// Print floating point number with scienfic notation
    #[arg(long, short)]
    exp: bool,
    /// Output list of 1 bits in the result
    /// (the result of the expression must be integer)
    #[arg(long)]
    bits: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let expr = args.expr.join(" ");
    let evaluator = Evaluator::default();
    let result = evaluator.eval(&expr)?;
    if args.format.hex {
        println!(
            "{:X}",
            result.as_int().with_context(|| format!(
                "--hex flag requires the result of the expression to be integer. Got {result:?}",
            ))?
        );
    } else if args.format.bin {
        println!(
            "{:b}",
            result.as_int().with_context(|| format!(
                "--bin flag requires the result of the expression to be integer. Got {result:?}",
            ))?
        );
    } else if args.format.oct {
        println!(
            "{:o}",
            result.as_int().with_context(|| format!(
                "--oct flag requires the result of the expression to be integer. Got {result:?}",
            ))?
        );
    } else if args.format.bits {
        let result = result.as_int().with_context(|| {
            format!(
                "--bits flag requires the result of the expression to be integer. Got {result:?}"
            )
        })?;
        let mut list = vec![];
        for i in 0..127 {
            if result & (1 << i) != 0 {
                list.push(format!("{i}"));
            }
        }
        println!("{}", list.join(", "));
    } else if args.format.exp {
        let result = result.as_float().with_context(|| {
            format!(
                "--exp flag requires the result of the expression to be floating point number. Got {result:?}"
            )
        })?;
        println!("{result:e}");
    } else {
        println!("{result}");
    }

    Ok(())
}
