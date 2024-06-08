use std::fmt::Display;

use crate::{Float, Integer};

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Integer(Integer),
    Float(Float),
}

impl Operand {
    pub fn to_float(self) -> Self {
        match &self {
            Operand::Float(_) => self,
            Operand::Integer(int) => Operand::Float(*int as Float),
        }
    }

    pub fn floor_to_int(self) -> Self {
        match &self {
            Operand::Float(float) => Operand::Integer(float.floor() as Integer),
            Operand::Integer(_) => self,
        }
    }

    pub fn is_float(&self) -> bool {
        match &self {
            Operand::Float(_) => true,
            Operand::Integer(_) => false,
        }
    }

    pub fn is_int(&self) -> bool {
        match &self {
            Operand::Float(_) => false,
            Operand::Integer(_) => true,
        }
    }

    pub fn upgrade_if_need(a: Self, b: Self) -> (Self, Self) {
        if a.is_float() || b.is_float() {
            (a.to_float(), b.to_float())
        } else {
            (a, b)
        }
    }

    pub fn as_int(&self) -> Option<Integer> {
        match self {
            Operand::Integer(int) => Some(*int),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<Float> {
        match self {
            Operand::Float(float) => Some(*float),
            _ => None,
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Float(float) => write!(f, "{float}"),
            Operand::Integer(int) => write!(f, "{int}"),
        }
    }
}
