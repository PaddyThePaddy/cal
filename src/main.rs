use evalexpr::*;
use regex::Regex;
#[macro_use]
extern crate lazy_static;

lazy_static! {
  static ref BIT_REGEX: Regex = Regex::new(r"(?i)BIT(\d+)").unwrap();
  static ref KB_REGEX: Regex = Regex::new(r"(?i)(\d+)KB").unwrap();
  static ref MB_REGEX: Regex = Regex::new(r"(?i)(\d+)MB").unwrap();
  static ref GB_REGEX: Regex = Regex::new(r"(?i)(\d+)GB").unwrap();
  static ref TB_REGEX: Regex = Regex::new(r"(?i)(\d+)TB").unwrap();
  static ref PB_REGEX: Regex = Regex::new(r"(?i)(\d+)PB").unwrap();
}

fn build_arg() -> clap::ArgMatches {
  clap::Command::new("cal")
    .author("paddythepaddy@duck.com")
    .version(git_version::git_version!())
    .about("Cli calculator for myself")
    .arg(
      clap::Arg::new("output_base")
        .short('b')
        .long("base")
        .action(clap::ArgAction::Set)
        .default_value("10")
        .help("Only affect integer results"),
    )
    .arg(clap::Arg::new("formula").action(clap::ArgAction::Append))
    .get_matches()
}

fn main() {
  let args = build_arg();
  let base: u32 = args
    .get_one::<String>("output_base")
    .expect("Get output base failed")
    .parse::<u32>()
    .expect("Invalid base");
  println!("base: {}", base);
  if !args.contains_id("formula") {
    interactive(base);
  } else {
    let items: Vec<String> = args
      .get_many::<String>("formula")
      .expect("Get args failed")
      .map(|s| s.into())
      .collect();
    let formula = replace_vars(&items.join(" "));
    let result = eval_int(&formula).unwrap();
    println!(
      "{}",
      convert_to_string(result, base).expect("Convert to string failed")
    );
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
  // vars.iter().for_each(|(key, val)| {
  //     result = result.replace(key, val);
  // });
  return result;
}

fn interactive(base: u32) {
  let stdin = std::io::stdin();
  let mut context = HashMapContext::new();
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
    input = replace_vars(&input);
    match eval_with_context_mut(&input, &mut context) {
      Ok(result) => match result {
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
      },
      Err(e) => println!("{}", e),
    }
  }
}

fn convert_to_string(int: i64, base: u32) -> Result<String, String> {
  if base > 36 || base < 2 {
    return Err("Invalid base".into());
  }
  if int == 0 {
    return Ok("0".into());
  }
  let mut result = String::new();
  let mut exp = 0;
  if int < 0 {
    result.insert(0, '-');
  }
  let mut abs = int.abs() as u32;
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
    abs -= digit as u32 * tmp_base;
    if exp == 0 {
      break;
    } else {
      exp -= 1;
    }
  }
  return Ok(result);
}
