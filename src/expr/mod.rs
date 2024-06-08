use std::{collections::HashMap, fmt::Display};

use operand::Operand;
use operator::{default_handlers, Operator, OperatorAction};

use crate::lex::LexToken;

pub mod operand;
pub mod operator;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Not enough operand")]
    NotEnoughOperand,
    #[error("No matching handler for {op}")]
    NoMatchingHandler { op: Operator },
    #[error("Redundant operand: {0}")]
    RedundantOperand(usize),
    #[error("Invalid data type: {0}")]
    InvalidDataType(String),
    #[error("{0}")]
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum ExprToken {
    Operator(Operator),
    Operand(Operand),
}

impl From<&LexToken> for ExprToken {
    fn from(value: &LexToken) -> Self {
        match value {
            LexToken::Integer(int) => ExprToken::Operand(Operand::Integer(*int)),
            LexToken::Float(float) => ExprToken::Operand(Operand::Float(*float)),
            LexToken::Add => ExprToken::Operator(Operator::Add),
            LexToken::Minus => ExprToken::Operator(Operator::Minus),
            LexToken::Mul => ExprToken::Operator(Operator::Mul),
            LexToken::Div => ExprToken::Operator(Operator::Div),
            LexToken::OpenParenthesis => ExprToken::Operator(Operator::OpenParenthesis),
            LexToken::CloseParenthesis => ExprToken::Operator(Operator::CloseParenthesis),
            LexToken::BitOr => ExprToken::Operator(Operator::BitOr),
            LexToken::BitAnd => ExprToken::Operator(Operator::BitAnd),
            LexToken::BitXor(width) => ExprToken::Operator(Operator::BitXor(*width)),
            LexToken::BitNot(width) => ExprToken::Operator(Operator::BitNot(*width)),
            LexToken::Expo => ExprToken::Operator(Operator::Expo),
            LexToken::Mod => ExprToken::Operator(Operator::Mod),
            LexToken::RightShift => ExprToken::Operator(Operator::RightShift),
            LexToken::LeftShift => ExprToken::Operator(Operator::LeftShift),
        }
    }
}

impl Display for ExprToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprToken::Operand(op) => write!(f, "{op}"),
            ExprToken::Operator(op) => write!(f, "{op}"),
        }
    }
}

pub fn to_suffix(src: &[ExprToken]) -> Vec<ExprToken> {
    let mut list = Vec::with_capacity(src.len());
    let mut op_stack: Vec<Operator> = vec![];

    for token in src {
        match token {
            ExprToken::Operand(_) => list.push(token.clone()),
            ExprToken::Operator(op) => {
                if *op == Operator::CloseParenthesis {
                    while let Some(op) = op_stack.pop() {
                        if op == Operator::OpenParenthesis {
                            break;
                        } else {
                            list.push(ExprToken::Operator(op));
                        }
                    }
                } else if *op == Operator::OpenParenthesis {
                    op_stack.push(op.clone());
                } else {
                    while let Some(top) = op_stack.last() {
                        if top.precedence() > op.precedence() {
                            break;
                        }
                        list.push(ExprToken::Operator(op_stack.pop().unwrap()));
                    }
                    op_stack.push(op.clone());
                }
            }
        }
    }

    while let Some(op) = op_stack.pop() {
        list.push(ExprToken::Operator(op));
    }
    list
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

    pub fn eval(&self, expr: &[ExprToken]) -> Result<Operand, Error> {
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

pub fn print_tokens(tokens: &[ExprToken]) {
    for tk in tokens {
        print!("{tk} ");
    }
    println!("");
}
