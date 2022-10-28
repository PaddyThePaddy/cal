use super::*;

pub fn print_val(val: &Value, base: u32) {
  match val {
    Value::String(s) => println!("{}", s),
    Value::Int(result) => {
      println!(
        "{}",
        convert_to_string(*result, base).expect("Convert int to string failed")
      )
    }
    Value::Boolean(b) => println!("{}", b),
    Value::Float(f) => {
      if f.floor() == *f {
        println!("{:.1}", f);
      } else {
        println!("{}", f)
      }
    }
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
