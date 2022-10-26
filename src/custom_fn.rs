use evalexpr::*;

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
}

fn to_u8(val: &Value) -> EvalexprResult<u8> {
  if let Value::Int(int) = val {
    if *int < 0 || *int > u8::MAX as IntType {
      return Err(EvalexprError::CustomMessage(
        "Value exceed 8 bit width".into(),
      ));
    }
    return Ok(*int as u8);
  } else {
    return Err(EvalexprError::CustomMessage(format!(
      "Value {:?} is not int",
      val
    )));
  }
}

fn to_u16(val: &Value) -> EvalexprResult<u16> {
  if let Value::Int(int) = val {
    if *int < 0 || *int > u16::MAX as IntType {
      return Err(EvalexprError::CustomMessage(
        "Value exceed 16 bit width".into(),
      ));
    }
    return Ok(*int as u16);
  } else {
    return Err(EvalexprError::CustomMessage(format!(
      "Value {:?} is not int",
      val
    )));
  }
}

fn to_u32(val: &Value) -> EvalexprResult<u32> {
  if let Value::Int(int) = val {
    if *int < 0 || *int > u32::MAX as IntType {
      return Err(EvalexprError::CustomMessage(
        "Value exceed 32 bit width".into(),
      ));
    }
    return Ok(*int as u32);
  } else {
    return Err(EvalexprError::CustomMessage(format!(
      "Value {:?} is not int",
      val
    )));
  }
}

fn to_u64(val: &Value) -> EvalexprResult<u64> {
  if let Value::Int(int) = val {
    if *int < 0 || *int > u32::MAX as IntType {
      return Err(EvalexprError::CustomMessage(
        "Value exceed 32 bit width".into(),
      ));
    }
    return Ok(*int as u64);
  } else {
    return Err(EvalexprError::CustomMessage(format!(
      "Value {:?} is not int",
      val
    )));
  }
}

fn not8(val: &Value) -> EvalexprResult<Value> {
  return Ok(Value::Int(!(to_u8(val)?) as IntType));
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
    return Ok(Value::Int((a | b) as IntType));
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
    return Ok(Value::Int((a & b) as IntType));
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
    return Ok(Value::Int((a ^ b) as IntType));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn not16(val: &Value) -> EvalexprResult<Value> {
  return Ok(Value::Int(!(to_u16(val)?) as IntType));
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
    return Ok(Value::Int((a | b) as IntType));
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
    return Ok(Value::Int((a & b) as IntType));
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
    return Ok(Value::Int((a ^ b) as IntType));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn not32(val: &Value) -> EvalexprResult<Value> {
  return Ok(Value::Int(!(to_u32(val)?) as IntType));
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
    return Ok(Value::Int((a | b) as IntType));
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
    return Ok(Value::Int((a & b) as IntType));
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
    return Ok(Value::Int((a ^ b) as IntType));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn not64(val: &Value) -> EvalexprResult<Value> {
  return Ok(Value::Int(!(to_u64(val)?) as IntType));
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
    return Ok(Value::Int((a | b) as IntType));
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
    return Ok(Value::Int((a & b) as IntType));
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
    return Ok(Value::Int((a ^ b) as IntType));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn bits(val: &Value) -> EvalexprResult<Value> {
  if let Value::Int(int) = val {
    let int = *int as u128;
    let mut result: Vec<String> = Vec::new();
    for i in 0..128 {
      if int & 1 << i != 0 {
        result.push(i.to_string());
      }
    }

    return Ok(Value::String(result.join(", ")));
  } else {
    return Err(EvalexprError::CustomMessage(format!(
      "Value {:?} is not int",
      val
    )));
  }
}

fn bits_t(val: &Value) -> EvalexprResult<Value> {
  if let Value::Int(int) = val {
    let int = *int as u128;
    let mut result = Vec::new();
    for i in 0..128 {
      if int & 1 << i != 0 {
        result.push(Value::Int(i.into()));
      }
    }

    return Ok(Value::Tuple(result));
  } else {
    return Err(EvalexprError::CustomMessage(format!(
      "Value {:?} is not int",
      val
    )));
  }
}
