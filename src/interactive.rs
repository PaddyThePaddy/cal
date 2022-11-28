use super::*;
use regex::{Captures, Regex};
use rustyline::Result as RustyResult;
use rustyline::{
  completion::{Candidate, Completer},
  highlight::Highlighter,
  hint::Hinter,
  line_buffer::LineBuffer,
  validate::Validator,
  Config, Context, Editor,
};

lazy_static! {
  static ref BASE_REGEX: Regex = Regex::new(r"(?i)_(?:base|b)\s*=?\(?\s*(\d+)\s*\)?").unwrap();
  static ref MEM_REGEX: Regex = Regex::new(r"\$(-)?(\d+)").unwrap();
  static ref SEP_REGEX: Regex = Regex::new(r"\s").unwrap();
}

#[derive(Debug, Clone)]
struct Completion {
  str: String,
  triggers: Vec<String>,
}

impl Completion {
  pub fn with_triggers<T: std::convert::Into<String>>(s: T, triggers: Vec<T>) -> Completion {
    Completion {
      str: s.into(),
      triggers: triggers.into_iter().map(|tr| tr.into()).collect(),
    }
  }
  pub fn starts_with_lower(&self, other: &str) -> bool {
    self.triggers.iter().any(|s| s.starts_with(other))
  }
}

impl Candidate for Completion {
  fn display(&self) -> &str {
    return &self.str;
  }
  fn replacement(&self) -> &str {
    return &self.str;
  }
}

impl<T> std::convert::From<T> for Completion
where
  T: std::convert::Into<String>,
{
  fn from(s: T) -> Completion {
    let s: String = s.into();
    Completion {
      triggers: vec![s.to_lowercase()],
      str: s,
    }
  }
}

struct Helper {
  completions: Vec<Completion>,
}

impl Helper {
  fn new<T>(candidates: Vec<T>) -> Helper
  where
    T: std::convert::Into<Completion>,
  {
    Helper {
      completions: candidates.into_iter().map(|i| i.into()).collect(),
    }
  }

  fn add<T>(&mut self, candidates: T)
  where
    T: std::convert::Into<Completion>,
  {
    self.completions.push(candidates.into());
  }
}

impl Completer for Helper {
  type Candidate = Completion;

  fn complete(
    &self,
    line: &str,
    pos: usize,
    _: &Context<'_>,
  ) -> RustyResult<(usize, Vec<Self::Candidate>)> {
    let start = (&line[..pos]).rfind(' ').map(|n| n + 1).unwrap_or(0);
    let end = (&line[pos..]).find(' ').unwrap_or(line.len());
    let current_word = line[start..end].to_lowercase();

    if current_word.len() == 0 {
      return Ok((start, Vec::new()));
    }

    return Ok((
      start,
      self
        .completions
        .iter()
        .filter(|c| c.starts_with_lower(&current_word))
        .map(|c| c.clone())
        .collect(),
    ));
  }

  fn update(&self, line: &mut LineBuffer, start: usize, elected: &str) {
    let word_start = start;
    let remaining = &line[line.pos()..];
    let word_end = line.pos()
      + remaining
        .chars()
        .position(|c| c == ' ')
        .unwrap_or(remaining.len());
    line.replace(word_start..word_end, elected);
  }
}

impl Hinter for Helper {
  type Hint = String;
}

impl Highlighter for Helper {}

impl Validator for Helper {}

impl rustyline::Helper for Helper {}

fn add_built_in_fn(helper: &mut Helper) {
  [
    ("min", vec!["min"]),
    ("max", vec!["max"]),
    ("len", vec!["len"]),
    ("floor", vec!["floor"]),
    ("round", vec!["round"]),
    ("ceil", vec!["ceil"]),
    ("if", vec!["if"]),
    ("typeof", vec!["typeof"]),
    ("math::is_nan", vec!["math::is_nan", "is_nan"]),
    ("math::is_finite", vec!["math::is_finite", "is_finite"]),
    (
      "math::is_infinite",
      vec!["math::is_infinite", "is_infinite"],
    ),
    ("math::is_normal", vec!["math::is_normal", "is_normal"]),
    ("math::ln", vec!["math::ln", "ln"]),
    ("math::log", vec!["math::log", "log"]),
    ("math::log2", vec!["math::log2", "log2"]),
    ("math::log10", vec!["math::log10", "log10"]),
    ("math::exp", vec!["math::exp", "exp"]),
    ("math::exp2", vec!["math::exp2", "exp2"]),
    ("math::pow", vec!["math::pow", "pow"]),
    ("math::cos", vec!["math::cos", "cos"]),
    ("math::acos", vec!["math::acos", "acos"]),
    ("math::cosh", vec!["math::cosh", "cosh"]),
    ("math::acosh", vec!["math::acosh", "acosh"]),
    ("math::sin", vec!["math::sin", "sin"]),
    ("math::asin", vec!["math::asin", "asin"]),
    ("math::sinh", vec!["math::sinh", "sinh"]),
    ("math::asinh", vec!["math::asinh", "asinh"]),
    ("math::tan", vec!["math::tan", "tan"]),
    ("math::atan", vec!["math::atan", "atan"]),
    ("math::atan2", vec!["math::atan2", "atan2"]),
    ("math::tanh", vec!["math::tanh", "tanh"]),
    ("math::atanh", vec!["math::atanh", "atanh"]),
    ("math::sqrt", vec!["math::sqrt", "sqrt"]),
    ("math::cbrt", vec!["math::cbrt", "cbrt"]),
    ("math::hypot", vec!["math::hypot", "hypot"]),
    (
      "str::regex_matches",
      vec!["str::regex_matches", "regex_matches"],
    ),
    (
      "str::regex_replace",
      vec!["str::regex_replace", "regex_replace"],
    ),
    (
      "str::to_lowercase",
      vec!["str::to_lowercase", "to_lowercase"],
    ),
    (
      "str::to_uppercase",
      vec!["str::to_uppercase", "to_uppercase"],
    ),
    ("str::trim", vec!["str::trim", "trim"]),
    ("str::from", vec!["str::from", "from"]),
    ("bitand", vec!["bitand"]),
    ("bitor", vec!["bitor"]),
    ("bitxor", vec!["bitxor"]),
    ("bitnot", vec!["bitnot"]),
    ("shl", vec!["shl"]),
    ("shr", vec!["shr"]),
    ("random", vec!["random"]),
  ]
  .into_iter()
  .for_each(|(name, triggers)| helper.add(Completion::with_triggers(name, triggers)))
}

pub fn interactive(mut base: u32, context: &mut HashMapContext) {
  let mut rl =
    Editor::<Helper>::with_config(Config::builder().auto_add_history(true).build()).unwrap();
  let mut memory: Vec<Value> = Vec::new();
  let mut echo = false;
  let mut completions = vec!["_base", "_echo", "_memlen", "_memval", "exit", "BIT"];

  completions.extend(custom_fn::get_custom_fn().into_iter().map(|(n, _)| n));
  let mut helper = Helper::new(completions);
  add_built_in_fn(&mut helper);
  rl.set_helper(Some(helper));

  'control: loop {
    let mut input = match rl.readline("cal> ") {
      Ok(s) => s,
      Err(_) => break,
    };
    match input.trim() {
      "exit" => break,
      "_memlen" => {
        println!("{}\n", memory.len());
        continue 'control;
      }
      "_memval" => {
        memory
          .iter()
          .enumerate()
          .for_each(|(index, v)| println!("{}: {:?}", index, v));
        println!();
        continue 'control;
      }
      "_echo" => {
        echo = !echo;
        println!("echo: {}\n", echo);
        continue 'control;
      }
      _ => {}
    }
    if let Some(cap) = BASE_REGEX.captures(&input) {
      let new_base = match cap.get(1).unwrap().as_str().parse::<u32>() {
        Ok(i) => i,
        Err(e) => {
          println!("Convert to int failed: {}", e);
          continue;
        }
      };
      if new_base < 2 || new_base > 36 {
        println!(
          "Invalid radix base: {}. Only 2-36 is supported.\n",
          new_base
        );
        continue 'control;
      }
      base = new_base;
      println!("new base = {}\n", base);
      continue;
    }

    let mut break_flag = false;
    input = MEM_REGEX
      .replace_all(&input, |cap: &Captures| {
        if memory.len() == 0 {
          println!("No memory at the moment.\n");
          break_flag = true;
          return "".to_string();
        }
        let index = match cap.get(2).unwrap().as_str().parse::<usize>() {
          Ok(i) => i,
          Err(_) => {
            println!("Convert {}'s index failed.\n", cap.get(0).unwrap().as_str());
            break_flag = true;
            return "".to_string();
          }
        };
        if cap.get(1).is_some() {
          if index > memory.len() || index == 0 {
            println!(
              "{} exceed valid memory slots. Valid range is from 1 to {}.\n",
              cap.get(0).unwrap().as_str(),
              memory.len()
            );
            break_flag = true;
            return "".to_string();
          }
        } else {
          if index >= memory.len() {
            println!(
              "{} exceed valid memory slots. Valid range is from 0 to {}.\n",
              cap.get(0).unwrap().as_str(),
              memory.len()
            );
            break_flag = true;
            return "".to_string();
          }
        }
        if cap.get(1).is_some() {
          return memory[memory.len() - index].to_string();
        } else {
          return memory[index].to_string();
        }
      })
      .into_owned();
    if break_flag {
      continue 'control;
    }

    input = pre_processor::pre_process(&input);
    if echo {
      println!("echo: {}", &input);
    }
    match eval_with_context_mut(&input, context) {
      Ok(result) => {
        match display::val_to_string(&result, base) {
          Ok(r) => {
            if let Some(s) = r {
              println!("{}", s);
              memory.push(result)
            }
          }
          Err(s) => println!("{}", s),
        };
      }
      Err(e) => println!("{}", e),
    }
    println!();
  }
}
