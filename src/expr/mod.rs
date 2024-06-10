use std::{collections::HashMap, fmt::Display};

use operand::{Operand, OperandType};
use operator::{default_handlers, Operator, OperatorAction};

use crate::{lex::LexToken, Float, Integer};

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

#[derive(Debug, Clone)]
pub enum ExprToken {
    Operator(Operator),
    Operand(Operand),
}

impl Display for ExprToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprToken::Operand(op) => write!(f, "{op}"),
            ExprToken::Operator(op) => write!(f, "{op}"),
        }
    }
}

pub struct LexTokenIter<'a> {
    inner: &'a [LexToken],
    idx: usize,
}

impl<'a> Iterator for LexTokenIter<'a> {
    type Item = LexToken;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.get(self.idx) {
            Some(item) => {
                self.idx += 1;
                Some(item.clone())
            }
            None => None,
        }
    }
}

impl<'a> From<&'a [LexToken]> for LexTokenIter<'a> {
    fn from(value: &'a [LexToken]) -> Self {
        Self {
            inner: value,
            idx: 0,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
enum ParseState {
    #[default]
    ExpectOperand,
    ExpectOperator,
    ExpectNumber,
}

pub fn parse_expr(tokens: &mut LexTokenIter) -> Result<Vec<ExprToken>, Error> {
    let mut parse_state = ParseState::default();
    let mut ret_list = vec![];
    let mut number_base = 1;

    while let Some(token) = tokens.next() {
        match parse_state {
            ParseState::ExpectOperator => {
                if let LexToken::CloseParenthesis = token {
                    return Ok(ret_list);
                }
                ret_list.push(ExprToken::Operator(token.try_into()?));
                parse_state = ParseState::ExpectOperand;
            }
            ParseState::ExpectOperand => match token {
                LexToken::Float(float) => {
                    ret_list.push(ExprToken::Operand(Operand::Float(
                        float * number_base as Float,
                    )));
                    number_base = 1;
                    parse_state = ParseState::ExpectOperator;
                }
                LexToken::Integer(int) => {
                    ret_list.push(ExprToken::Operand(Operand::Integer(
                        int * number_base as Integer,
                    )));
                    number_base = 1;
                    parse_state = ParseState::ExpectOperator;
                }
                LexToken::Minus => {
                    number_base *= -1;
                    parse_state = ParseState::ExpectNumber;
                }
                LexToken::Plus => {
                    parse_state = ParseState::ExpectNumber;
                }
                LexToken::BitNot(width) => {
                    ret_list.push(ExprToken::Operator(Operator::BitNot(width)));
                }
                LexToken::OpenParenthesis => {
                    ret_list.push(ExprToken::Operator(Operator::OpenParenthesis));
                    ret_list.extend(parse_expr(tokens)?);
                    ret_list.push(ExprToken::Operator(Operator::CloseParenthesis));
                    parse_state = ParseState::ExpectOperator;
                }
                LexToken::Custom(id) => {
                    ret_list.push(ExprToken::Operator(Operator::Custom(id.clone())));
                    ret_list.extend(parse_para(tokens)?);
                    parse_state = ParseState::ExpectOperator;
                }
                LexToken::String(s) => {
                    ret_list.push(ExprToken::Operand(Operand::String(s)));
                    parse_state = ParseState::ExpectOperator;
                }
                _ => Err(Error::ExpectOperand(token.clone()))?,
            },
            ParseState::ExpectNumber => match token {
                LexToken::Float(float) => {
                    ret_list.push(ExprToken::Operand(Operand::Float(
                        float * number_base as Float,
                    )));
                    number_base = 1;
                    parse_state = ParseState::ExpectOperator;
                }
                LexToken::Integer(int) => {
                    ret_list.push(ExprToken::Operand(Operand::Integer(
                        int * number_base as Integer,
                    )));
                    number_base = 1;
                    parse_state = ParseState::ExpectOperator;
                }
                _ => Err(Error::ExpectOperand(token.clone()))?,
            },
        }
    }
    if parse_state != ParseState::ExpectOperator {
        dbg!(parse_state);
        Err(Error::UnexpectedEnd)?;
    }
    Ok(ret_list)
}

pub fn parse_para(tokens: &mut LexTokenIter) -> Result<Vec<ExprToken>, Error> {
    let token = tokens.next().ok_or(Error::UnexpectedEnd)?;
    if token.ne(&LexToken::OpenParenthesis) {
        Err(Error::ExpectToken(LexToken::OpenParenthesis, token.clone()))?;
    }

    let mut ret_list = vec![];
    let mut parenthsis_lvl = 0;
    let mut para_list = vec![];
    loop {
        let token = tokens.next().ok_or(Error::UnexpectedEnd)?;
        match token {
            LexToken::CloseParenthesis => {
                if parenthsis_lvl == 0 {
                    break;
                } else {
                    parenthsis_lvl -= 1;
                    para_list.push(token);
                }
            }
            LexToken::OpenParenthesis => {
                parenthsis_lvl += 1;
                para_list.push(token);
            }
            _ => para_list.push(token),
        }
    }
    para_list
        .split(|tk| *tk == LexToken::Comma).try_for_each(|para_tk| {
            ret_list.extend(parse_expr(&mut para_tk.into())?);
            Ok(())
        })?;

    Ok(ret_list)
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
                        if top.precedence() > op.precedence() || op.precedence() == 0 {
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
    println!();
}
