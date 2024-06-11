use std::fmt::Display;

use crate::{Float, Integer};

use super::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OperandType {
    Integer,
    Float,
    String,
}

impl Display for OperandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Integer(Integer),
    Float(Float),
    String(String),
}

impl Operand {
    pub fn data_type(&self) -> OperandType {
        match self {
            Operand::Float(_) => OperandType::Float,
            Operand::Integer(_) => OperandType::Integer,
            Operand::String(_) => OperandType::String,
        }
    }
    pub fn to_float(self) -> Result<Self, Error> {
        match self {
            Operand::Float(_) => Ok(self),
            Operand::Integer(int) => Ok(Operand::Float(int as Float)),
            _ => Err(Error::InvalidDataType {
                expected: vec![OperandType::Float, OperandType::Integer],
                got: self.data_type(),
            }),
        }
    }

    pub fn floor_to_int(self) -> Result<Self, Error> {
        match self {
            Operand::Float(float) => Ok(Operand::Integer(float.floor() as Integer)),
            Operand::Integer(_) => Ok(self),
            _ => Err(Error::InvalidDataType {
                expected: vec![OperandType::Float, OperandType::Integer],
                got: self.data_type(),
            }),
        }
    }

    pub fn upgrade_if_need(a: Self, b: Self) -> Result<(Self, Self), Error> {
        if a.data_type() == OperandType::Float || b.data_type() == OperandType::Float {
            Ok((a.to_float()?, b.to_float()?))
        } else {
            Ok((a, b))
        }
    }

    pub fn as_int(&self) -> Result<Integer, Error> {
        match self {
            Operand::Integer(int) => Ok(*int),
            _ => Err(Error::InvalidDataType {
                expected: vec![OperandType::Integer],
                got: self.data_type(),
            }),
        }
    }

    pub fn as_float(&self) -> Result<Float, Error> {
        match self {
            Operand::Float(float) => Ok(*float),
            _ => Err(Error::InvalidDataType {
                expected: vec![OperandType::Float],
                got: self.data_type(),
            }),
        }
    }

    pub fn as_string(&self) -> Result<&str, Error> {
        match self {
            Operand::String(s) => Ok(s.as_str()),
            _ => Err(Error::InvalidDataType {
                expected: vec![OperandType::String],
                got: self.data_type(),
            }),
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Float(float) => write!(f, "{float}"),
            Operand::Integer(int) => write!(f, "{int}"),
            Operand::String(s) => write!(f, "{s}"),
        }
    }
}
