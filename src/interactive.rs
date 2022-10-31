use super::*;
use regex::{Captures, Regex};
use rustyline::{Config, Editor};

lazy_static! {
  static ref BASE_REGEX: Regex = Regex::new(r"(?i)base\s*=?\(?\s*(\d+)\s*\)?").unwrap();
  static ref MEM_REGEX: Regex = Regex::new(r"\$(-)?(\d+)").unwrap();
}

pub fn interactive(mut base: u32, context: &mut HashMapContext) {
  let mut rl = Editor::<()>::with_config(Config::builder().auto_add_history(true).build()).unwrap();
  let mut memory: Vec<Value> = Vec::new();
  let mut echo = false;

  'control: loop {
    let mut input = match rl.readline("input> ") {
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
      base = new_base;
      println!("new base = {}\n", base);
      continue;
    }

    let mut break_flag = false;
    input = MEM_REGEX
      .replace(&input, |cap: &Captures| {
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

    let mut new_str: String = String::new();
    let mut pre_end = 0;
    for m in MEM_REGEX.captures_iter(&input) {
      if memory.len() == 0 {
        println!("No memory at the moment.\n");
        continue 'control;
      }
      new_str += &input[pre_end..m.get(0).unwrap().start()];
      let index = match m.get(1).unwrap().as_str().parse::<usize>() {
        Ok(i) => i,
        Err(_) => {
          println!(
            "Convert {}'s index failed. Valid range is from 1 to {}.\n",
            m.get(0).unwrap().as_str(),
            memory.len()
          );
          continue 'control;
        }
      };
      if index > memory.len() || index == 0 {
        println!(
          "{} exceed valid memory slots. Valid range is from 1 to {}.\n",
          m.get(0).unwrap().as_str(),
          memory.len()
        );
        continue 'control;
      }
      new_str = memory[memory.len() - index].to_string();
      pre_end = m.get(0).unwrap().end();
    }
    new_str += &input[pre_end..];
    input = new_str;

    input = pre_processor::pre_process(&input);
    if echo {
      println!("{}", &input);
    }
    match eval_with_context_mut(&input, context) {
      Ok(result) => {
        display::print_val(&result, base);
        memory.push(result)
      }
      Err(e) => println!("{}", e),
    }
    println!();
  }
}
