use std::{
    collections::HashMap,
    num::{ParseFloatError, ParseIntError},
};

use expr::{
    operand::{Operand, OperandType},
    operator::{default_handlers, Operator, OperatorAction},
    parse_expr, to_suffix, ExprToken,
};
use lex::{tokenize, LexToken};

pub mod expr;
pub mod lex;

pub type Integer = i128;
pub type Float = f64;

#[derive(Debug, thiserror::Error, PartialEq, Clone, Default)]
pub enum Error {
    #[default]
    #[error("Invalid token")]
    InvalidToken,
    #[error("{0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("{0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Invalid bit width hint: {0}")]
    InvalidBitWidthHint(String),
    #[error("Not enough operand")]
    NotEnoughOperand,
    #[error("No matching handler for {op}")]
    NoMatchingHandler { op: Operator },
    #[error("Redundant operand: {0}")]
    RedundantOperand(usize),
    #[error("Invalid data type: expected {expected:?}, got {got}")]
    InvalidDataType {
        expected: Vec<OperandType>,
        got: OperandType,
    },
    #[error("{0}")]
    Custom(String),
    #[error("Expect operator, got {0:?}")]
    ExpectOperator(LexToken),
    #[error("Expect operand, got {0:?}")]
    ExpectOperand(LexToken),
    #[error("Expect {0:?}, got {1:?}")]
    ExpectToken(LexToken, LexToken),
    #[error("Expression ends unexpectedly")]
    UnexpectedEnd,
}

pub struct Evaluator {
    operators: HashMap<Operator, OperatorAction>,
}

impl Default for Evaluator {
    fn default() -> Self {
        let operators = HashMap::from_iter(default_handlers());
        Self { operators }
    }
}

impl Evaluator {
    pub fn insert_op_handler(&mut self, op: Operator, handler: OperatorAction) {
        self.operators.insert(op, handler);
    }

    pub fn eval(&self, formula: &str) -> Result<Operand, Error> {
        let lex = tokenize(formula)?;
        let expr_tokens = parse_expr(&mut lex.as_slice().into())?;
        let suffix = to_suffix(&expr_tokens);
        self.consume_operators(&suffix)
    }

    pub fn consume_operators(&self, expr: &[ExprToken]) -> Result<Operand, Error> {
        let mut operands = vec![];
        for token in expr {
            match token {
                ExprToken::Operand(op) => operands.push(op.clone()),
                ExprToken::Operator(op) => self
                    .operators
                    .get(op)
                    .ok_or(Error::NoMatchingHandler { op: op.clone() })?(
                    &mut operands
                )?,
            }
        }

        if operands.len() != 1 {
            Err(Error::RedundantOperand(operands.len()))?;
        }

        operands.into_iter().last().ok_or(Error::NotEnoughOperand)
    }
}

#[cfg(test)]
mod test {
    use expr::operator::Operator;

    use super::*;

    fn test_func(operands: &mut Vec<Operand>) -> Result<(), Error> {
        let c = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()?;
        let b = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()?;
        let a = operands.pop().ok_or(Error::NotEnoughOperand)?.as_int()?;

        operands.push(Operand::Integer(a * b + c));

        Ok(())
    }

    #[test]
    fn test_basic() {
        let mut evaluator = Evaluator::default();
        evaluator.insert_op_handler(Operator::Custom("func".to_string()), Box::new(test_func));
        assert_eq!(
            evaluator.eval("2+(1 - 5)*3/4").unwrap(),
            Operand::Integer(2 + (1 - 5) * 3 / 4)
        );
        assert_eq!(
            evaluator.eval("3.0 / 5 ^^ 0.3").unwrap(),
            Operand::Float(3f64 / 5f64.powf(0.3f64))
        );
        assert_eq!(
            evaluator.eval("1 << 10").unwrap(),
            Operand::Integer(1 << 10)
        );
        assert_eq!(
            evaluator.eval("!0 & 0xF0F00000").unwrap(),
            Operand::Integer(0xF0F00000)
        );
        assert_eq!(
            evaluator.eval("~(0xF<<4)").unwrap(),
            Operand::Integer(0xFFFFFF0F)
        );
        assert_eq!(
            evaluator.eval("~(0xF>>2)").unwrap(),
            Operand::Integer(0xFFFFFFFC)
        );
        assert_eq!(
            evaluator.eval("0b101 b^ b!0").unwrap(),
            Operand::Integer(0b11111010)
        );
        assert_eq!(evaluator.eval("4096 / 1k").unwrap(), Operand::Integer(4));
        assert_eq!(
            evaluator.eval("(2--1)*5+-3+func(((2)),(1+2),4)").unwrap(),
            Operand::Integer(22)
        );
        assert_eq!(
            evaluator.eval(r"1+rev(ascii('_FVH'))+2").unwrap(),
            Operand::Integer(0x48564662)
        )
    }
}
