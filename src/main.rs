#[macro_use]
extern crate lazy_static;

use evalexpr::*;
mod base;
mod custom_fn;
mod display;
mod interactive;
mod pre_processor;
use base::*;

const HELP_MSG: &str = r#"A cli calculator highly depends on crate https://github.com/ISibboI/evalexpr.
Check readme on its github page for the expression syntax.

Note: Add a . after integers to convert them to float (like "5."), that will force the evalexpr module use normal calculation rules.
Or it will use programming calculation rules by default (like 3 / 2 == 1)"#;

fn build_arg() -> clap::ArgMatches {
  clap::Command::new("cal")
    .author("paddythepaddy@duck.com")
    .version(git_version::git_version!())
    .about(HELP_MSG)
    .after_long_help(std::include_str!("../readme.md"))
    .after_help("Detail on github: https://github.com/PaddyThePaddy/cal")
    .arg(
      clap::Arg::new("output_base")
        .short('B')
        .long("base")
        .action(clap::ArgAction::Set)
        .value_parser(clap::value_parser!(u32).range(2..37))
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
    *args.get_one::<u32>("output_base").expect("Invalid base")
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
    match display::val_to_string(&result, base) {
      Ok(result) => println!("{}", result),
      Err(msg) => println!("{}", msg),
    };
  }
}
