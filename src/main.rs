use evalexpr::*;
use regex::Regex;
#[macro_use]
extern crate lazy_static;
mod custom_fn;

lazy_static! {
  static ref BIT_REGEX: Regex = Regex::new(r"(?i)BIT(\d+)").unwrap();
  static ref KB_REGEX: Regex = Regex::new(r"(?i)(\d+)KB").unwrap();
  static ref MB_REGEX: Regex = Regex::new(r"(?i)(\d+)MB").unwrap();
  static ref GB_REGEX: Regex = Regex::new(r"(?i)(\d+)GB").unwrap();
  static ref TB_REGEX: Regex = Regex::new(r"(?i)(\d+)TB").unwrap();
  static ref PB_REGEX: Regex = Regex::new(r"(?i)(\d+)PB").unwrap();
  static ref BASE_REGEX: Regex = Regex::new(r"(?i)base\s*=?\(?\s*(\d+)\s*\)?").unwrap();
  static ref HEX_REGEX1: Regex = Regex::new(r"(?i)0x([a-f0-9]+)").unwrap();
  static ref HEX_REGEX2: Regex = Regex::new(r"(?i)([a-f0-9]+)(?-i)h").unwrap();
  static ref BIN_REGEX1: Regex = Regex::new(r"(?i)0b([01]+)").unwrap();
  static ref BIN_REGEX2: Regex = Regex::new(r"(?i)([01]+)(?-i)b").unwrap();
  static ref OCT_REGEX: Regex = Regex::new(r"(?i)0([0-7]+)").unwrap();
}

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
  println!("base: {}", base);
  if !args.contains_id("formula") {
    interactive(base, &mut context);
  } else {
    let items: Vec<String> = args
      .get_many::<String>("formula")
      .expect("Get args failed")
      .map(|s| s.into())
      .collect();
    let formula = replace_vars(&items.join(" "));
    let result = eval_with_context_mut(&formula, &mut context)
      .unwrap_or_else(|e| Value::String(format!("{}", e)));
    print_val(result, base);
  }
}

fn replace_vars(input: &str /* , vars: &HashMap<String, String>*/) -> String {
  let mut result: String;
  result = BIT_REGEX.replace(input, "shl(1, $1)").into();
  result = KB_REGEX.replace(&result, "($1 * 1024)").into();
  result = MB_REGEX.replace(&result, "($1 * 1024 * 1024)").into();
  result = GB_REGEX
    .replace(&result, "($1 * 1024 * 1024 * 1024)")
    .into();
  result = TB_REGEX
    .replace(&result, "($1 * 1024 * 1024 * 1024 * 1024)")
    .into();
  result = PB_REGEX
    .replace(&result, "($1 * 1024 * 1024 * 1024 * 1024 * 1024)")
    .into();
  let mut new_str: String = String::new();
  let mut pre_end = 0;
  HEX_REGEX1.captures_iter(&result).for_each(|m| {
    new_str += &result[pre_end..m.get(0).unwrap().start()];
    let int = u128::from_str_radix(m.get(1).unwrap().as_str(), 16).unwrap();
    new_str = format!("{}{}", new_str, int);
    pre_end = m.get(0).unwrap().end();
  });
  new_str += &result[pre_end..];
  result = new_str;

  let mut new_str: String = String::new();
  let mut pre_end = 0;
  HEX_REGEX2.captures_iter(&result).for_each(|m| {
    new_str += &result[pre_end..m.get(0).unwrap().start()];
    let int = u128::from_str_radix(m.get(1).unwrap().as_str(), 16).unwrap();
    new_str = format!("{}{}", new_str, int);
    pre_end = m.get(0).unwrap().end();
  });
  new_str += &result[pre_end..];
  result = new_str;

  let mut new_str: String = String::new();
  let mut pre_end = 0;
  BIN_REGEX1.captures_iter(&result).for_each(|m| {
    new_str += &result[pre_end..m.get(0).unwrap().start()];
    let int = u128::from_str_radix(m.get(1).unwrap().as_str(), 2).unwrap();
    new_str = format!("{}{}", new_str, int);
    pre_end = m.get(0).unwrap().end();
  });
  new_str += &result[pre_end..];
  result = new_str;

  let mut new_str: String = String::new();
  let mut pre_end = 0;
  BIN_REGEX2.captures_iter(&result).for_each(|m| {
    new_str += &result[pre_end..m.get(0).unwrap().start()];
    let int = u128::from_str_radix(m.get(1).unwrap().as_str(), 2).unwrap();
    new_str = format!("{}{}", new_str, int);
    pre_end = m.get(0).unwrap().end();
  });
  new_str += &result[pre_end..];
  result = new_str;

  let mut new_str: String = String::new();
  let mut pre_end = 0;
  OCT_REGEX.captures_iter(&result).for_each(|m| {
    new_str += &result[pre_end..m.get(0).unwrap().start()];
    let int = u128::from_str_radix(m.get(1).unwrap().as_str(), 8).unwrap();
    new_str = format!("{}{}", new_str, int);
    pre_end = m.get(0).unwrap().end();
  });
  new_str += &result[pre_end..];
  result = new_str;
  // vars.iter().for_each(|(key, val)| {
  //     result = result.replace(key, val);
  // });
  return result;
}

fn interactive(mut base: u32, context: &mut HashMapContext) {
  let stdin = std::io::stdin();
  loop {
    let mut input = String::new();
    match stdin.read_line(&mut input) {
      Err(e) => {
        eprintln!("{:?}", e);
        break;
      }
      Ok(n) => {
        if n == 0 {
          break;
        }
      }
    };
    if input.trim() == "exit" {
      break;
    }
    if let Some(cap) = BASE_REGEX.captures(&input) {
      let new_base = match cap.get(1).unwrap().as_str().parse::<u32>() {
        Ok(i) => i,
        Err(e) => {
          println!("Convert to int failed: {}", e);
          continue;
        }
      };
      base = new_base;
      println!("new base = {}", base);
      continue;
    }
    input = replace_vars(&input);
    match eval_with_context_mut(&input, context) {
      Ok(result) => print_val(result, base),
      Err(e) => println!("{}", e),
    }
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
