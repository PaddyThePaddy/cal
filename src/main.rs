use anyhow::Context;
use rcal::{expr::operand::Operand, Evaluator};
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
    #[arg(short, long, action = clap::ArgAction::Count)]
    pretty: u8,
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
    /// Print floating point number with scientific notation
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
        let num = result.as_int().with_context(|| {
            format!(
                "--hex flag requires the result of the expression to be integer. Got {result:?}",
            )
        })?;
        if args.pretty == 0 {
            println!("{num:X}");
        } else if args.pretty == 1 {
            println!("{num:#X}");
        } else {
            let num_str = format!("{num:X}");
            let mut str_sections = vec![];
            if num_str.len() % 4 != 0 {
                str_sections.push(&num_str[0..num_str.len() % 4]);
            }
            for (s, e) in (num_str.len() % 4..num_str.len())
                .step_by(4)
                .map(|s| (s, s + 4))
            {
                str_sections.push(&num_str[s..e]);
            }

            println!("0x{}", str_sections.join("_"));
        }
    } else if args.format.bin {
        let num = result.as_int().with_context(|| {
            format!(
                "--bin flag requires the result of the expression to be integer. Got {result:?}",
            )
        })?;
        if args.pretty == 0 {
            println!("{num:b}");
        } else if args.pretty == 1 {
            println!("{num:#b}");
        } else {
            let num_str = format!("{num:b}");
            let mut str_sections = vec![];
            if num_str.len() % 4 != 0 {
                str_sections.push(&num_str[0..num_str.len() % 4]);
            }
            for (s, e) in (num_str.len() % 4..num_str.len())
                .step_by(4)
                .map(|s| (s, s + 4))
            {
                str_sections.push(&num_str[s..e]);
            }

            println!("0b{}", str_sections.join("_"));
        }
    } else if args.format.oct {
        let num = result.as_int().with_context(|| {
            format!(
                "--oct flag requires the result of the expression to be integer. Got {result:?}",
            )
        })?;
        if args.pretty == 0 {
            println!("{num:o}");
        } else if args.pretty == 1 {
            println!("{num:#o}");
        } else {
            let num_str = format!("{num:o}");
            let mut str_sections = vec![];
            if num_str.len() % 4 != 0 {
                str_sections.push(&num_str[0..num_str.len() % 4]);
            }
            for (s, e) in (num_str.len() % 4..num_str.len())
                .step_by(4)
                .map(|s| (s, s + 4))
            {
                str_sections.push(&num_str[s..e]);
            }

            println!("0o{}", str_sections.join("_"));
        }
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
        match result {
            Operand::Float(f) => println!("{f}"),
            Operand::String(s) => println!("{s}"),
            Operand::Integer(num) => {
                if args.pretty == 0 {
                    println!("{num}");
                } else {
                    let num_str = format!("{num}");
                    let mut str_sections = vec![];
                    if num_str.len() % 3 != 0 {
                        str_sections.push(&num_str[0..num_str.len() % 3]);
                    }
                    for (s, e) in (num_str.len() % 3..num_str.len())
                        .step_by(3)
                        .map(|s| (s, s + 3))
                    {
                        str_sections.push(&num_str[s..e]);
                    }

                    println!("{}", str_sections.join(","));
                }
            }
        }
    }

    Ok(())
}
