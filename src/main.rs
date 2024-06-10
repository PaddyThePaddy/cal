use anyhow::Context;
use cal::eval;
use clap::{command, Arg, ArgAction, ArgGroup, ArgMatches};

fn get_args() -> ArgMatches {
    command!()
        .args([
            Arg::new("expr").action(ArgAction::Append).required(true),
            Arg::new("hex")
                .long("hex")
                .short('x')
                .action(ArgAction::SetTrue)
                .help("Output with hexadecimal format (the result of the expression must be integer)"),
            Arg::new("bits").long("bits").action(ArgAction::SetTrue)
                .help("Output list of 1 bits in the result (the result of the expression must be integer)"),
            Arg::new("bin")
                .long("bin")
                .short('b')
                .action(ArgAction::SetTrue)
                .help("Output with binary format (the result of the expression must be integer)"),
            Arg::new("oct")
                .long("oct")
                .short('o')
                .action(ArgAction::SetTrue)
                .help("Output with octal format (the result of the expression must be integer)"),
        ]).group(ArgGroup::new("format").args(["oct", "bin", "hex", "bits"]).multiple(false).required(false))
        .get_matches()
}

fn main() -> anyhow::Result<()> {
    let args = get_args();
    let expr = args
        .get_many::<String>("expr")
        .context("Get argument failed")?
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ");
    let result = eval(&expr, None)?;
    if args.get_flag("hex") {
        println!(
            "{:X}",
            result.as_int().with_context(|| format!(
                "--hex flag requires the result of the expression to be integer. Got {}",
                &result
            ))?
        );
    } else if args.get_flag("bin") {
        println!(
            "{:b}",
            result.as_int().with_context(|| format!(
                "--bin flag requires the result of the expression to be integer. Got {}",
                &result
            ))?
        );
    } else if args.get_flag("oct") {
        println!(
            "{:o}",
            result.as_int().with_context(|| format!(
                "--oct flag requires the result of the expression to be integer. Got {}",
                &result
            ))?
        );
    } else if args.get_flag("bits") {
        let result = result.as_int().with_context(|| {
            format!(
                "--bits flag requires the result of the expression to be integer. Got {}",
                &result
            )
        })?;
        let mut list = vec![];
        for i in 0..127 {
            if result & (1 << i) != 0 {
                list.push(format!("{i}"));
            }
        }
        println!("{}", list.join(", "));
    } else {
        println!("{result}");
    }

    Ok(())
}
