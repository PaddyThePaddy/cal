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

#[derive(Debug)]
struct Completion {
  str: String,
}

impl Candidate for Completion {
  fn display(&self) -> &str {
    return &self.str;
  }
  fn replacement(&self) -> &str {
    return &self.str;
  }
}

struct Helper {
  completions: Vec<String>,
}

impl Helper {
  fn new() -> Helper {
    Helper {
      completions: custom_fn::get_custom_fn()
        .into_iter()
        .map(|(name, _)| name.to_owned())
        .collect(),
    }
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
    let start = (&line[..pos]).rfind(' ').unwrap_or(0);
    let end = (&line[pos..]).find(' ').unwrap_or(line.len());
    let current_word = &line[start..end];
    if current_word.len() == 0 {
      return Ok((start, Vec::new()));
    }
    return Ok((
      start,
      self
        .completions
        .iter()
        .filter(|c| c.starts_with(current_word))
        .map(|s| Completion { str: s.clone() })
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

pub fn interactive(mut base: u32, context: &mut HashMapContext) {
  let mut rl =
    Editor::<Helper>::with_config(Config::builder().auto_add_history(true).build()).unwrap();
  let mut memory: Vec<Value> = Vec::new();
  let mut echo = false;

  rl.set_helper(Some(Helper::new()));

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
