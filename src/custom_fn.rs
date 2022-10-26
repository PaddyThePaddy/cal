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
}

fn to_u8(val: &Value) -> EvalexprResult<u8> {
  if let Value::Int(int) = val {
    if *int < 0 || *int > u8::MAX as i64 {
      return Err(EvalexprError::CustomMessage(
        "Value exceed 8 bit width".into(),
      ));
    }
    return Ok(*int as u8);
  } else {
    return Err(EvalexprError::CustomMessage("Value is not int".into()));
  }
}

fn to_u16(val: &Value) -> EvalexprResult<u16> {
  if let Value::Int(int) = val {
    if *int < 0 || *int > u16::MAX as i64 {
      return Err(EvalexprError::CustomMessage(
        "Value exceed 16 bit width".into(),
      ));
    }
    return Ok(*int as u16);
  } else {
    return Err(EvalexprError::CustomMessage("Value is not int".into()));
  }
}

fn to_u32(val: &Value) -> EvalexprResult<u32> {
  if let Value::Int(int) = val {
    if *int < 0 || *int > u32::MAX as i64 {
      return Err(EvalexprError::CustomMessage(
        "Value exceed 32 bit width".into(),
      ));
    }
    return Ok(*int as u32);
  } else {
    return Err(EvalexprError::CustomMessage("Value is not int".into()));
  }
}

fn not8(val: &Value) -> EvalexprResult<Value> {
  return Ok(Value::Int(!(to_u8(val)?) as i64));
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
    return Ok(Value::Int((a | b) as i64));
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
    return Ok(Value::Int((a & b) as i64));
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
    return Ok(Value::Int((a ^ b) as i64));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn not16(val: &Value) -> EvalexprResult<Value> {
  return Ok(Value::Int(!(to_u16(val)?) as i64));
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
    return Ok(Value::Int((a | b) as i64));
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
    return Ok(Value::Int((a & b) as i64));
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
    return Ok(Value::Int((a ^ b) as i64));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}

fn not32(val: &Value) -> EvalexprResult<Value> {
  return Ok(Value::Int(!(to_u32(val)?) as i64));
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
    return Ok(Value::Int((a | b) as i64));
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
    return Ok(Value::Int((a & b) as i64));
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
    return Ok(Value::Int((a ^ b) as i64));
  } else {
    return Err(EvalexprError::WrongFunctionArgumentAmount {
      expected: 2,
      actual: 1,
    });
  }
}
