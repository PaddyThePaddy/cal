use std::{fmt::Display, ops::BitXor};

use crate::{lex::LexToken, Integer};

use super::{
    operand::{Operand, OperandType},
    Error,
};

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
    Negate,
    Postive,
}

impl TryFrom<LexToken> for Operator {
    type Error = Error;
    fn try_from(value: LexToken) -> Result<Self, Self::Error> {
        match value {
            LexToken::BitAnd => Ok(Operator::BitAnd),
            LexToken::BitNot(w) => Ok(Operator::BitNot(w)),
            LexToken::BitOr => Ok(Operator::BitOr),
            LexToken::BitXor(w) => Ok(Operator::BitXor(w)),
            LexToken::CloseParenthesis => Ok(Operator::CloseParenthesis),
            LexToken::Div => Ok(Operator::Div),
            LexToken::Expo => Ok(Operator::Expo),
            LexToken::LeftShift => Ok(Operator::LeftShift),
            LexToken::Minus => Ok(Operator::Minus),
            LexToken::Mod => Ok(Operator::Mod),
            LexToken::Mul => Ok(Operator::Mul),
            LexToken::OpenParenthesis => Ok(Operator::OpenParenthesis),
            LexToken::Plus => Ok(Operator::Add),
            LexToken::RightShift => Ok(Operator::RightShift),
            _ => todo!(),
        }
    }
}

impl Operator {
    /// lower number is hight precedence
    /// unary operators should have a precedence 0
    pub fn precedence(&self) -> usize {
        match self {
            Self::Custom(_) | Self::BitNot(_) | Self::Negate | Self::Postive => 0,
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
            Operator::Negate => write!(f, "-"),
            Operator::Postive => write!(f, "+"),
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
        (Operator::Custom("ascii".to_string()), Box::new(ascii)),
        (Operator::Custom("rev".to_string()), Box::new(rev)),
        (Operator::Negate, Box::new(neg)),
        (Operator::Postive, Box::new(noop)),
    ]
}

fn noop(_operands: &mut Vec<Operand>) -> Result<(), Error> {
    Ok(())
}

fn neg(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let token = operands.pop().ok_or(Error::NotEnoughOperand)?;
    match token {
        Operand::Float(f) => operands.push(Operand::Float(f * -1.0)),
        Operand::Integer(i) => operands.push(Operand::Integer(-i)),
        _ => Err(Error::InvalidDataType {
            expected: vec![OperandType::Integer, OperandType::Float],
            got: token.data_type(),
        })?,
    }
    Ok(())
}

fn op_add(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let (a, b) = Operand::upgrade_if_need(a, b)?;

    let result = match a {
        Operand::Integer(a) => {
            let b = b.as_int()?;
            Operand::Integer(a + b)
        }
        Operand::Float(a) => {
            let b = b.as_float()?;
            Operand::Float(a + b)
        }
        _ => Err(Error::InvalidDataType {
            expected: vec![OperandType::Integer, OperandType::Float],
            got: a.data_type(),
        })?,
    };
    operands.push(result);

    Ok(())
}

fn op_minus(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let (a, b) = Operand::upgrade_if_need(a, b)?;

    let result = match a {
        Operand::Integer(a) => {
            let b = b.as_int()?;
            Operand::Integer(a - b)
        }
        Operand::Float(a) => {
            let b = b.as_float()?;
            Operand::Float(a - b)
        }
        _ => Err(Error::InvalidDataType {
            expected: vec![OperandType::Integer, OperandType::Float],
            got: a.data_type(),
        })?,
    };
    operands.push(result);

    Ok(())
}

fn op_mul(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let (a, b) = Operand::upgrade_if_need(a, b)?;

    let result = match a {
        Operand::Integer(a) => {
            let b = b.as_int()?;
            Operand::Integer(a * b)
        }
        Operand::Float(a) => {
            let b = b.as_float()?;
            Operand::Float(a * b)
        }
        _ => Err(Error::InvalidDataType {
            expected: vec![OperandType::Integer, OperandType::Float],
            got: a.data_type(),
        })?,
    };
    operands.push(result);

    Ok(())
}

fn op_div(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let (a, b) = Operand::upgrade_if_need(a, b)?;

    let result = match a {
        Operand::Integer(a) => {
            let b = b.as_int()?;
            Operand::Integer(a / b)
        }
        Operand::Float(a) => {
            let b = b.as_float()?;
            Operand::Float(a / b)
        }
        _ => Err(Error::InvalidDataType {
            expected: vec![OperandType::Integer, OperandType::Float],
            got: a.data_type(),
        })?,
    };
    operands.push(result);

    Ok(())
}

fn op_exp(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?;
    let (a, b) = Operand::upgrade_if_need(a, b)?;

    let result = match a {
        Operand::Integer(a) => {
            let b = b.as_int()?;
            if b < u32::MIN as Integer || b > u32::MAX as Integer {
                Err(Error::Custom(
                    "Exp operation for integers only allows u32 as parameter".to_string(),
                ))?;
            }
            Operand::Integer(a.pow(b as u32))
        }
        Operand::Float(a) => {
            let b = b.as_float()?;
            Operand::Float(a.powf(b))
        }
        _ => Err(Error::InvalidDataType {
            expected: vec![OperandType::Integer, OperandType::Float],
            got: a.data_type(),
        })?,
    };
    operands.push(result);

    Ok(())
}

fn op_bit_or(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()?;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()?;

    operands.push(Operand::Integer(a | b));

    Ok(())
}

fn op_bit_and(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()?;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()?;

    operands.push(Operand::Integer(a & b));

    Ok(())
}

fn op_bit_sh_right(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()?;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()?;

    operands.push(Operand::Integer(a >> b));

    Ok(())
}

fn op_bit_sh_left(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()?;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()?;

    operands.push(Operand::Integer(a << b));

    Ok(())
}

fn op_mod(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()?;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()?;

    operands.push(Operand::Integer(a % b));

    Ok(())
}

fn op_bit_not_8(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()? as u8;
    operands.push(Operand::Integer((!a) as Integer));

    Ok(())
}

fn op_bit_not_16(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()? as u16;
    operands.push(Operand::Integer((!a) as Integer));

    Ok(())
}

fn op_bit_not_32(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()? as u32;
    operands.push(Operand::Integer((!a) as Integer));

    Ok(())
}

fn op_bit_not_64(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()? as u64;
    operands.push(Operand::Integer((!a) as Integer));

    Ok(())
}

fn op_bit_xor_8(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()? as u8;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()? as u8;
    operands.push(Operand::Integer((a.bitxor(b)) as Integer));

    Ok(())
}

fn op_bit_xor_16(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()? as u16;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()? as u16;
    operands.push(Operand::Integer((a.bitxor(b)) as Integer));

    Ok(())
}

fn op_bit_xor_32(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()? as u32;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()? as u32;
    operands.push(Operand::Integer((a.bitxor(b)) as Integer));

    Ok(())
}

fn op_bit_xor_64(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()? as u64;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()? as u64;
    operands.push(Operand::Integer((a.bitxor(b)) as Integer));

    Ok(())
}

fn op_bit_xor_128(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let b = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()? as u128;
    let a = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()? as u128;
    operands.push(Operand::Integer((a.bitxor(b)) as Integer));

    Ok(())
}

fn ascii(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let operand = operands.pop().ok_or(Error::NotEnoughOperand)?;
    match operand {
        Operand::String(s) => {
            let int = s
                .chars()
                .map(|c| c as u8)
                .fold(0, |pre, b| (pre << 8) + b as Integer);
            operands.push(Operand::Integer(int));
        }
        Operand::Integer(mut int) => {
            let mut chars = vec![];
            while int != 0 {
                chars.push((int & 0xFF) as u8 as char);
                int >>= 8;
            }
            chars.reverse();
            operands.push(Operand::String(String::from_iter(chars)))
        }
        _ => Err(Error::InvalidDataType {
            expected: vec![OperandType::String, OperandType::Integer],
            got: operand.data_type(),
        })?,
    }
    Ok(())
}

fn rev(operands: &mut Vec<Operand>) -> Result<(), Error> {
    let operand = operands.pop().ok_or(Error::NotEnoughOperand)?;
    match operand {
        Operand::String(s) => {
            let chars = s.chars().rev().collect::<Vec<_>>();
            operands.push(Operand::String(String::from_iter(chars)))
        }
        Operand::Integer(mut int) => {
            let mut reversed = 0;
            while int != 0 {
                reversed <<= 8;
                reversed += int & 0xFF;
                int >>= 8;
            }
            operands.push(Operand::Integer(reversed))
        }
        _ => Err(Error::InvalidDataType {
            expected: vec![OperandType::String, OperandType::Integer],
            got: operand.data_type(),
        })?,
    }

    Ok(())
}
