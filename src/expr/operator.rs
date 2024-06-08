use std::{fmt::Display, ops::BitXor};

use crate::Integer;

use super::{operand::Operand, Error};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    Add,
    Minus,
    Mul,
    Div,
    OpenParenthesis,
    CloseParenthesis,
    BitOr,
    BitAnd,
    BitXor(usize),
    BitNot(usize),
    Expo,
    Mod,
    Custom(String),
    RightShift,
    LeftShift,
}

impl Operator {
    // lower number is hight precedence
    pub fn precedence(&self) -> usize {
        match self {
            Self::Custom(_) => 0,
            Self::BitNot(_) => 0,
            Self::Expo => 2,
            Self::Mul | Self::Div | Self::Mod => 3,
            Self::Add | Self::Minus => 4,
            Self::LeftShift | Self::RightShift => 5,
            Self::BitAnd => 6,
            Self::BitXor(_) => 7,
            Self::BitOr => 8,
            Self::OpenParenthesis | Self::CloseParenthesis => 9,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::OpenParenthesis => write!(f, "("),
            Operator::CloseParenthesis => write!(f, ")"),
            Operator::BitOr => write!(f, "|"),
            Operator::BitAnd => write!(f, "&"),
            Operator::BitXor(width) => write!(f, "{width}^"),
            Operator::BitNot(width) => write!(f, "{width}!"),
            Operator::Expo => write!(f, "^"),
            Operator::Mod => write!(f, "%"),
            Operator::Custom(id) => write!(f, "{id}"),
            Operator::RightShift => write!(f, ">>"),
            Operator::LeftShift => write!(f, "<<"),
        }
    }
}

pub type OperatorAction = Box<dyn Fn(&mut Vec<Operand>) -> Result<(), Error>>;

pub fn default_handlers() -> Vec<(Operator, OperatorAction)> {
    vec![
        (Operator::Add, Box::new(op_add)),
        (Operator::Minus, Box::new(op_minus)),
        (Operator::Mul, Box::new(op_mul)),
        (Operator::Div, Box::new(op_div)),
        (Operator::Expo, Box::new(op_exp)),
        (Operator::BitOr, Box::new(op_bit_or)),
        (Operator::BitAnd, Box::new(op_bit_and)),
        (Operator::Mod, Box::new(op_mod)),
        (Operator::LeftShift, Box::new(op_bit_sh_left)),
        (Operator::RightShift, Box::new(op_bit_sh_right)),
        (Operator::BitNot(8), Box::new(op_bit_not_8)),
        (Operator::BitNot(16), Box::new(op_bit_not_16)),
        (Operator::BitNot(32), Box::new(op_bit_not_32)),
        (Operator::BitNot(64), Box::new(op_bit_not_64)),
        (Operator::BitXor(8), Box::new(op_bit_xor_8)),
        (Operator::BitXor(16), Box::new(op_bit_xor_16)),
        (Operator::BitXor(32), Box::new(op_bit_xor_32)),
        (Operator::BitXor(64), Box::new(op_bit_xor_64)),
        (Operator::BitXor(128), Box::new(op_bit_xor_128)),
    ]
}

fn op_add(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let (a, b) = Operand::upgrade_if_need(a, b);

    let result = match a {
        Operand::Integer(a) => {
            let b = b
                .as_int()
                .ok_or(Error::InvalidDataType("Float".to_string()))?;
            Operand::Integer(a + b)
        }
        Operand::Float(a) => {
            let b = b
                .as_float()
                .ok_or(Error::InvalidDataType("Integer".to_string()))?;
            Operand::Float(a + b)
        }
    };
    operands.push(result);

    Ok(())
}

fn op_minus(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let (a, b) = Operand::upgrade_if_need(a, b);

    let result = match a {
        Operand::Integer(a) => {
            let b = b
                .as_int()
                .ok_or(Error::InvalidDataType("Float".to_string()))?;
            Operand::Integer(a - b)
        }
        Operand::Float(a) => {
            let b = b
                .as_float()
                .ok_or(Error::InvalidDataType("Integer".to_string()))?;
            Operand::Float(a - b)
        }
    };
    operands.push(result);

    Ok(())
}

fn op_mul(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let (a, b) = Operand::upgrade_if_need(a, b);

    let result = match a {
        Operand::Integer(a) => {
            let b = b
                .as_int()
                .ok_or(Error::InvalidDataType("Float".to_string()))?;
            Operand::Integer(a * b)
        }
        Operand::Float(a) => {
            let b = b
                .as_float()
                .ok_or(Error::InvalidDataType("Integer".to_string()))?;
            Operand::Float(a * b)
        }
    };
    operands.push(result);

    Ok(())
}

fn op_div(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let (a, b) = Operand::upgrade_if_need(a, b);

    let result = match a {
        Operand::Integer(a) => {
            let b = b
                .as_int()
                .ok_or(Error::InvalidDataType("Float".to_string()))?;
            Operand::Integer(a / b)
        }
        Operand::Float(a) => {
            let b = b
                .as_float()
                .ok_or(Error::InvalidDataType("Integer".to_string()))?;
            Operand::Float(a / b)
        }
    };
    operands.push(result);

    Ok(())
}

fn op_exp(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let (a, b) = Operand::upgrade_if_need(a, b);

    let result = match a {
        Operand::Integer(a) => {
            let b = b
                .as_int()
                .ok_or(Error::InvalidDataType("Float".to_string()))?;
            if b < u32::MIN as Integer || b > u32::MAX as Integer {
                Err(Error::Custom(
                    "Exp operation for integers only allows u32 as parameter".to_string(),
                ))?;
            }
            Operand::Integer(a.pow(b as u32))
        }
        Operand::Float(a) => {
            let b = b
                .as_float()
                .ok_or(Error::InvalidDataType("Integer".to_string()))?;
            Operand::Float(a.powf(b))
        }
    };
    operands.push(result);

    Ok(())
}

fn op_bit_or(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))?;
    let a = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))?;

    operands.push(Operand::Integer(a | b));

    Ok(())
}

fn op_bit_and(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))?;
    let a = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))?;

    operands.push(Operand::Integer(a & b));

    Ok(())
}

fn op_bit_sh_right(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))?;
    let a = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))?;

    operands.push(Operand::Integer(a >> b));

    Ok(())
}

fn op_bit_sh_left(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))?;
    let a = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))?;

    operands.push(Operand::Integer(a << b));

    Ok(())
}

fn op_mod(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))?;
    let a = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))?;

    operands.push(Operand::Integer(a % b));

    Ok(())
}

fn op_bit_not_8(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let a = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))? as u8;
    operands.push(Operand::Integer((!a) as Integer));

    Ok(())
}

fn op_bit_not_16(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let a = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))? as u16;
    operands.push(Operand::Integer((!a) as Integer));

    Ok(())
}

fn op_bit_not_32(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let a = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))? as u32;
    operands.push(Operand::Integer((!a) as Integer));

    Ok(())
}

fn op_bit_not_64(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let a = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))? as u64;
    operands.push(Operand::Integer((!a) as Integer));

    Ok(())
}

fn op_bit_xor_8(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))? as u8;
    let a = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))? as u8;
    operands.push(Operand::Integer((a.bitxor(b)) as Integer));

    Ok(())
}

fn op_bit_xor_16(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))? as u16;
    let a = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))? as u16;
    operands.push(Operand::Integer((a.bitxor(b)) as Integer));

    Ok(())
}

fn op_bit_xor_32(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))? as u32;
    let a = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))? as u32;
    operands.push(Operand::Integer((a.bitxor(b)) as Integer));

    Ok(())
}

fn op_bit_xor_64(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))? as u64;
    let a = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))? as u64;
    operands.push(Operand::Integer((a.bitxor(b)) as Integer));

    Ok(())
}

fn op_bit_xor_128(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))? as u128;
    let a = operands
        .pop()
        .ok_or(Error::NotEnoughOperand)?
        .as_int()
        .ok_or(Error::InvalidDataType("Float".to_string()))? as u128;
    operands.push(Operand::Integer((a.bitxor(b)) as Integer));

    Ok(())
}
