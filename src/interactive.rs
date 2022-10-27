use super::*;
use rustyline::Editor;

pub fn interactive(mut base: u32, context: &mut HashMapContext) {
  let mut rl = Editor::<()>::new().unwrap();

  loop {
    let mut input = match rl.readline("input> ") {
      Ok(s) => s,
      Err(_) => break,
    };
    rl.add_history_entry(&input);
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
