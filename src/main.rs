#[macro_use]
extern crate lazy_static;

use evalexpr::*;
use regex::Regex;
mod custom_fn;
mod interactive;
mod pre_processor;

const HELP_MSG: &str = r#"A cli calculator highly depends on crate https://github.com/ISibboI/evalexpr.
Check readme on its github page for the expression syntax.

Note: Add a . after integers to convert them to float (like "5."), that will force the evalexpr module use normal calculation rules.
Or it will use programming calculation rules by default (like 3 / 2 == 1)"#;

fn build_arg() -> clap::ArgMatches {
  clap::Command::new("cal")
    .author("paddythepaddy@duck.com")
    .version(git_version::git_version!())
    .about(HELP_MSG)
    .arg(
      clap::Arg::new("output_base")
        .short('B')
        .long("base")
        .action(clap::ArgAction::Set)
        .default_value("10")
        .help("Change outputs radix. Only affects integer results"),
    )
    .arg(
      clap::Arg::new("hex")
        .short('x')
        .long("hex")
        .action(clap::ArgAction::SetTrue)
        .help("Short hand of --base 16"),
    )
    .arg(
      clap::Arg::new("bin")
        .short('b')
        .long("bin")
        .action(clap::ArgAction::SetTrue)
        .help("Short hand of --base 2"),
    )
    .arg(
      clap::Arg::new("oct")
        .short('o')
        .long("oct")
        .action(clap::ArgAction::SetTrue)
        .help("Short hand of --base 8"),
    )
    .arg(clap::Arg::new("formula").action(clap::ArgAction::Append))
    .group(
      clap::ArgGroup::new("base")
        .args(["output_base", "hex", "bin", "oct"])
        .multiple(false),
    )
    .get_matches()
}

fn main() {
  let args = build_arg();
  let mut context = HashMapContext::new();
  custom_fn::add_custom_function(&mut context);
  let base: u32 = if args.get_flag("hex") {
    16
  } else if args.get_flag("oct") {
    8
  } else if args.get_flag("bin") {
    2
  } else {
    args
      .get_one::<String>("output_base")
      .expect("Get output base failed")
      .parse::<u32>()
      .expect("Invalid base")
  };
  if !args.contains_id("formula") {
    if atty::is(atty::Stream::Stdin) {
      println!("base: {}", base);
    }
    interactive::interactive(base, &mut context);
  } else {
    let items: Vec<String> = args
      .get_many::<String>("formula")
      .expect("Get args failed")
      .map(|s| s.into())
      .collect();
    let formula = pre_processor::pre_process(&items.join(" "));
    let result = eval_with_context_mut(&formula, &mut context)
      .unwrap_or_else(|e| Value::String(format!("{}", e)));
    print_val(result, base);
  }
}

fn print_val(val: Value, base: u32) {
  match val {
    Value::String(s) => println!("{}", s),
    Value::Int(result) => {
      println!(
        "{}",
        convert_to_string(result, base).expect("Convert int to string failed")
      )
    }
    Value::Boolean(b) => println!("{}", b),
    Value::Float(f) => println!("float: {}", f),
    Value::Empty => println!("()"),
    Value::Tuple(t) => println!("{:?}", t),
  }
}

fn convert_to_string(int: IntType, base: u32) -> Result<String, String> {
  if base > 36 || base < 2 {
    return Err("Invalid base".into());
  }
  if int == 0 {
    return Ok("0".into());
  }
  let mut result = String::new();
  let mut exp = 0;
  let int = int as i128;
  let base = base as u128;
  if int < 0 {
    result.insert(0, '-');
  }
  let mut abs = int.abs() as u128;
  loop {
    exp += 1;
    let tmp = base.pow(exp);
    if tmp > abs {
      break;
    }
  }
  exp -= 1;
  loop {
    let tmp_base = base.pow(exp);
    let digit: u8 = (abs / tmp_base) as u8;
    if digit >= 10 {
      result.insert(result.len(), ('A' as u8 + (digit - 10)) as char);
    } else {
      result.insert(result.len(), ('0' as u8 + (digit)) as char);
    }
    abs -= digit as u128 * tmp_base;
    if exp == 0 {
      break;
    } else {
      exp -= 1;
    }
  }
  return Ok(result);
}
