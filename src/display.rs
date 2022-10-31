use std::num::TryFromIntError;

use super::*;

pub fn val_to_string(val: &Value, base: u32) -> Result<String, String> {
  Ok(match val {
    Value::String(s) => format!("{}", s),
    Value::Int(result) => {
      format!("{}", convert_to_string(*result, base)?)
    }
    Value::Boolean(b) => format!("{}", b),
    Value::Float(f) => {
      if f.floor() == *f {
        format!("{:.1}", f)
      } else {
        format!("{}", f)
      }
    }
    Value::Empty => format!("None"),
    Value::Tuple(t) => format!("{:?}", t),
  })
}

fn convert_to_string(int: IntType, base: u32) -> Result<String, String> {
  let a_ascii_code: u8 = 'A'
    .try_into()
    .map_err(|e: std::char::TryFromCharError| e.to_string())?;
  let zero_ascii_code: u8 = '0'
    .try_into()
    .map_err(|e: std::char::TryFromCharError| e.to_string())?;

  if base > 36 || base < 2 {
    return Err(format!("Invalid base: {}", base));
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

#[test]
fn test_radix() {
  assert_eq!(convert_to_string(10, 2), Ok("1010".into()));
  assert_eq!(convert_to_string(10, 1), Err("Invalid base: 1".into()));
  assert_eq!(convert_to_string(10, 37), Err("Invalid base: 37".into()));
  assert_eq!(convert_to_string(35, 36), Ok("Z".into()));
  assert_eq!(convert_to_string(36, 36), Ok("10".into()));
  assert_eq!(convert_to_string(71, 36), Ok("1Z".into()));
}
