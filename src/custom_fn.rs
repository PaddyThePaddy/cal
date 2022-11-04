use std::num::TryFromIntError;

use super::*;

pub fn add_custom_function(context: &mut HashMapContext) {
  context
    .set_function("or8".into(), Function::new(or8))
    .unwrap();
  context
    .set_function("and8".into(), Function::new(and8))
    .unwrap();
  context
    .set_function("xor8".into(), Function::new(xor8))
    .unwrap();
  context
    .set_function("not8".into(), Function::new(not8))
    .unwrap();
  context
    .set_function("or16".into(), Function::new(or16))
    .unwrap();
  context
    .set_function("and16".into(), Function::new(and16))
    .unwrap();
  context
    .set_function("xor16".into(), Function::new(xor16))
    .unwrap();
  context
    .set_function("not16".into(), Function::new(not16))
    .unwrap();
  context
    .set_function("or32".into(), Function::new(or32))
    .unwrap();
  context
    .set_function("and32".into(), Function::new(and32))
    .unwrap();
  context
    .set_function("xor32".into(), Function::new(xor32))
    .unwrap();
  context
    .set_function("not32".into(), Function::new(not32))
    .unwrap();
  context
    .set_function("or64".into(), Function::new(or64))
    .unwrap();
  context
    .set_function("and64".into(), Function::new(and64))
    .unwrap();
  context
    .set_function("xor64".into(), Function::new(xor64))
    .unwrap();
  context
    .set_function("not64".into(), Function::new(not64))
    .unwrap();
  context
    .set_function("bits".into(), Function::new(bits))
    .unwrap();
  context
    .set_function("bits_t".into(), Function::new(bits_t))
    .unwrap();
  context
    .set_function("float".into(), Function::new(float))
    .unwrap();
  context
    .set_function("ascii".into(), Function::new(ascii))
    .unwrap();
  context
    .set_function("rev".into(), Function::new(rev))
    .unwrap();
  context
    .set_function("com".into(), Function::new(merge))
    .unwrap();
  context
    .set_function("combine".into(), Function::new(merge))
    .unwrap();
  context
    .set_function("mer".into(), Function::new(merge))
    .unwrap();
  context
    .set_function("merge".into(), Function::new(merge))
    .unwrap();
  context
    .set_function("bytes".into(), Function::new(bytes))
    .unwrap();
}

fn to_u8(val: &Value) -> EvalexprResult<u8> {
  if let Value::Int(int) = val {
    if *int < 0 || *int > u8::MAX.into() {
      return Err(EvalexprError::CustomMessage(
        "Value exceed 8 bit width".into(),
      ));
    }
    return (*int).try_into().to_eval_result();
  } else {
    return Err(EvalexprError::ExpectedInt {
      actual: val.clone(),
    });
  }
}

fn float(val: &Value) -> EvalexprResult<Value> {
  if val.is_float() {
    return Ok(val.clone());
  } else if let Value::Int(int) = val {
    // f64 can only be converted from i32 without data loss
    let f = i32::try_from(*int).to_eval_result()?.into();
    return Ok(Value::Float(f));
  } else {
    return Err(EvalexprError::ExpectedInt {
      actual: val.clone(),
    });
  }
}

fn to_u16(val: &Value) -> EvalexprResult<u16> {
  if let Value::Int(int) = val {
    if *int < 0 || *int > u16::MAX.into() {
      return Err(EvalexprError::CustomMessage(
        "Value exceed 16 bit width".into(),
      ));
    }
    return (*int).try_into().to_eval_result();
  } else {
    return Err(EvalexprError::ExpectedInt {
      actual: val.clone(),
    });
  }
}

fn to_u32(val: &Value) -> EvalexprResult<u32> {
  if let Value::Int(int) = val {
    if *int < 0 || *int > u32::MAX.into() {
      return Err(EvalexprError::CustomMessage(
        "Value exceed 32 bit width".into(),
      ));
    }
    return (*int).try_into().to_eval_result();
  } else {
    return Err(EvalexprError::ExpectedInt {
      actual: val.clone(),
    });
  }
}

fn to_u64(val: &Value) -> EvalexprResult<u64> {
  if let Value::Int(int) = val {
    if *int < 0 || *int > u64::MAX.into() {
      return Err(EvalexprError::CustomMessage(
        "Value exceed 36 bit width".into(),
      ));
    }
    return (*int).try_into().to_eval_result();
  } else {
    return Err(EvalexprError::ExpectedInt {
      actual: val.clone(),
    });
  }
}

fn not8(val: &Value) -> EvalexprResult<Value> {
  return Ok(Value::Int(IntType::from(!(to_u8(val)?))));
}

fn or8(val: &Value) -> EvalexprResult<Value> {
  if let Value::Tuple(args) = val {
    if args.len() != 2 {
      return Err(EvalexprError::WrongFunctionArgumentAmount {
        expected: 2,
        actual: args.len(),
      });
    }
    let a = to_u8(&args[0])?;
    let b = to_u8(&args[1])?;
    return Ok(Value::Int((a | b).into()));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn and8(val: &Value) -> EvalexprResult<Value> {
  if let Value::Tuple(args) = val {
    if args.len() != 2 {
      return Err(EvalexprError::WrongFunctionArgumentAmount {
        expected: 2,
        actual: args.len(),
      });
    }
    let a = to_u8(&args[0])?;
    let b = to_u8(&args[1])?;
    return Ok(Value::Int((a & b).into()));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn xor8(val: &Value) -> EvalexprResult<Value> {
  if let Value::Tuple(args) = val {
    if args.len() != 2 {
      return Err(EvalexprError::WrongFunctionArgumentAmount {
        expected: 2,
        actual: args.len(),
      });
    }
    let a = to_u8(&args[0])?;
    let b = to_u8(&args[1])?;
    return Ok(Value::Int((a ^ b).into()));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn not16(val: &Value) -> EvalexprResult<Value> {
  return Ok(Value::Int(IntType::from(!(to_u16(val)?))));
}

fn or16(val: &Value) -> EvalexprResult<Value> {
  if let Value::Tuple(args) = val {
    if args.len() != 2 {
      return Err(EvalexprError::WrongFunctionArgumentAmount {
        expected: 2,
        actual: args.len(),
      });
    }
    let a = to_u16(&args[0])?;
    let b = to_u16(&args[1])?;
    return Ok(Value::Int((a | b).into()));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn and16(val: &Value) -> EvalexprResult<Value> {
  if let Value::Tuple(args) = val {
    if args.len() != 2 {
      return Err(EvalexprError::WrongFunctionArgumentAmount {
        expected: 2,
        actual: args.len(),
      });
    }
    let a = to_u16(&args[0])?;
    let b = to_u16(&args[1])?;
    return Ok(Value::Int((a & b).into()));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn xor16(val: &Value) -> EvalexprResult<Value> {
  if let Value::Tuple(args) = val {
    if args.len() != 2 {
      return Err(EvalexprError::WrongFunctionArgumentAmount {
        expected: 2,
        actual: args.len(),
      });
    }
    let a = to_u16(&args[0])?;
    let b = to_u16(&args[1])?;
    return Ok(Value::Int((a ^ b).into()));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn not32(val: &Value) -> EvalexprResult<Value> {
  return Ok(Value::Int(IntType::from(!(to_u32(val)?))));
}

fn or32(val: &Value) -> EvalexprResult<Value> {
  if let Value::Tuple(args) = val {
    if args.len() != 2 {
      return Err(EvalexprError::WrongFunctionArgumentAmount {
        expected: 2,
        actual: args.len(),
      });
    }
    let a = to_u32(&args[0])?;
    let b = to_u32(&args[1])?;
    return Ok(Value::Int((a | b).into()));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn and32(val: &Value) -> EvalexprResult<Value> {
  if let Value::Tuple(args) = val {
    if args.len() != 2 {
      return Err(EvalexprError::WrongFunctionArgumentAmount {
        expected: 2,
        actual: args.len(),
      });
    }
    let a = to_u32(&args[0])?;
    let b = to_u32(&args[1])?;
    return Ok(Value::Int((a & b).into()));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn xor32(val: &Value) -> EvalexprResult<Value> {
  if let Value::Tuple(args) = val {
    if args.len() != 2 {
      return Err(EvalexprError::WrongFunctionArgumentAmount {
        expected: 2,
        actual: args.len(),
      });
    }
    let a = to_u32(&args[0])?;
    let b = to_u32(&args[1])?;
    return Ok(Value::Int((a ^ b).into()));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn not64(val: &Value) -> EvalexprResult<Value> {
  return Ok(Value::Int(IntType::from(!(to_u64(val)?))));
}

fn or64(val: &Value) -> EvalexprResult<Value> {
  if let Value::Tuple(args) = val {
    if args.len() != 2 {
      return Err(EvalexprError::WrongFunctionArgumentAmount {
        expected: 2,
        actual: args.len(),
      });
    }
    let a = to_u64(&args[0])?;
    let b = to_u64(&args[1])?;
    return Ok(Value::Int((a | b).into()));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn and64(val: &Value) -> EvalexprResult<Value> {
  if let Value::Tuple(args) = val {
    if args.len() != 2 {
      return Err(EvalexprError::WrongFunctionArgumentAmount {
        expected: 2,
        actual: args.len(),
      });
    }
    let a = to_u64(&args[0])?;
    let b = to_u64(&args[1])?;
    return Ok(Value::Int((a & b).into()));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn xor64(val: &Value) -> EvalexprResult<Value> {
  if let Value::Tuple(args) = val {
    if args.len() != 2 {
      return Err(EvalexprError::WrongFunctionArgumentAmount {
        expected: 2,
        actual: args.len(),
      });
    }
    let a = to_u64(&args[0])?;
    let b = to_u64(&args[1])?;
    return Ok(Value::Int((a ^ b).into()));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn count_bits(val: UintType) -> Result<Vec<IntType>, TryFromIntError> {
  let mut result = Vec::new();
  let bit_width = (std::mem::size_of::<UintType>() * 8).try_into()?;

  for i in 0..bit_width {
    if val & (1 << i) != 0 {
      result.push(i.try_into()?);
    }
  }
  return Ok(result);
}

fn bits(val: &Value) -> EvalexprResult<Value> {
  return count_bits(val.as_int()?.try_into().to_eval_result()?)
    .to_eval_result()
    .map(|v| {
      Value::String(
        v.iter()
          .map(|i| i.to_string())
          .collect::<Vec<String>>()
          .join(", "),
      )
    });
}

fn bits_t(val: &Value) -> EvalexprResult<Value> {
  return count_bits(val.as_int()?.try_into().to_eval_result()?)
    .to_eval_result()
    .map(|v| Value::Tuple(v.iter().map(|i| Value::Int(*i)).collect()));
}

fn ascii(val: &Value) -> EvalexprResult<Value> {
  match val {
    Value::Int(int) => {
      let mut ascii = String::new();
      let mut uint: UintType = (*int).try_into().to_eval_result()?;
      while uint != 0 {
        let ch = char::from(u8::try_from(uint & 0xFF).to_eval_result()?);
        ascii.insert(0, ch);
        uint = uint >> 8;
      }
      return Ok(Value::String(ascii));
    }
    Value::Tuple(vec) => {
      let mut result = String::new();
      for v in vec.into_iter() {
        result.push(char::from(to_u8(&v)?));
      }
      return Ok(Value::String(result));
    }
    Value::String(s) => {
      let mut result = Vec::new();
      for c in s.chars() {
        if !c.is_ascii() {
          return Err(EvalexprError::CustomMessage(
            "Only ascii characters are allowed".into(),
          ));
        }
        result.push(Value::Int(IntType::from(u8::try_from(c).to_eval_result()?)));
      }
      return Ok(Value::Tuple(result));
    }
    _ => {
      return Err(EvalexprError::ExpectedInt {
        actual: val.clone(),
      })
    }
  }
}

fn rev(val: &Value) -> EvalexprResult<Value> {
  match val {
    Value::String(s) => Ok(Value::String(String::from_iter(s.chars().rev()))),
    Value::Int(n) => {
      let mut uint: UintType = (*n).try_into().to_eval_result()?;
      let mut output: UintType = 0;
      while uint > 0 {
        output = (output << 8) + (uint & 0xFF);
        uint >>= 8;
      }
      return Ok(Value::Int(IntType::try_from(output).to_eval_result()?));
    }
    Value::Tuple(vec) => Ok(Value::Tuple(
      vec.into_iter().map(|v| v.to_owned()).rev().collect(),
    )),
    _ => Err(EvalexprError::ExpectedInt {
      actual: val.clone(),
    }),
  }
}

fn merge(val: &Value) -> EvalexprResult<Value> {
  match val {
    Value::Tuple(vec) => {
      let mut output: UintType = 0;
      if vec.len() > std::mem::size_of::<UintType>() {
        return Err(
          format!(
            "Array length exceed current data width ({}).",
            std::mem::size_of::<UintType>()
          )
          .to_eval_error(),
        );
      }
      for v in vec.iter() {
        output = (output << 8) + UintType::from(to_u8(v)?);
      }
      return Ok(Value::Int(IntType::try_from(output).to_eval_result()?));
    }
    _ => {
      return Err(EvalexprError::ExpectedTuple {
        actual: val.clone(),
      })
    }
  }
}

fn bytes(val: &Value) -> EvalexprResult<Value> {
  match val {
    Value::Int(n) => {
      let mut uint: UintType = (*n).try_into().to_eval_result()?;
      let mut result = Vec::new();
      while uint > 0 {
        result.insert(
          0,
          Value::Int(IntType::try_from(uint & 0xFF).to_eval_result()?),
        );
        uint >>= 8;
      }
      return Ok(Value::Tuple(result));
    }
    _ => Err(EvalexprError::ExpectedInt {
      actual: val.clone(),
    }),
  }
}
