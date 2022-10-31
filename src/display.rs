use std::num::TryFromIntError;

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
  let a_ascii_code: u8 = 'A'
    .try_into()
    .map_err(|e: std::char::TryFromCharError| e.to_string())?;
  let zero_ascii_code: u8 = '0'
    .try_into()
    .map_err(|e: std::char::TryFromCharError| e.to_string())?;

  if base > 36 || base < 2 {
    return Err("Invalid base".into());
  }
  if int == 0 {
    return Ok("0".into());
  }
  let mut result = String::new();
  let mut exp = 0;
  let base: UintType = base.into();
  if int < 0 {
    result.insert(0, '-');
  }
  let mut abs = int
    .abs()
    .try_into()
    .map_err(|e: TryFromIntError| e.to_string())?;
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
    let digit: u8 = (abs / tmp_base)
      .try_into()
      .map_err(|e: TryFromIntError| e.to_string())?;
    if digit >= 10 {
      result.insert(result.len(), (a_ascii_code + (digit - 10)).into());
    } else {
      result.insert(result.len(), (zero_ascii_code + (digit)).into());
    }
    abs = abs - UintType::from(digit) * tmp_base;
    if exp == 0 {
      break;
    } else {
      exp -= 1;
    }
  }
  return Ok(result);
}
