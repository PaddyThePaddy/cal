use expr::{operand::Operand, parse_expr, to_suffix, Evaluator};
use lex::LexToken;
use logos::Logos;

pub mod expr;
pub mod lex;

pub type Integer = i128;
pub type Float = f64;

pub fn tokenize(formula: &str) -> Result<Vec<LexToken>, lex::Error> {
    let lexer = LexToken::lexer(formula);
    Ok(lexer
        .collect::<Result<Vec<_>, lex::Error>>()?
        .into_iter()
        .collect())
}

pub fn eval(formula: &str, evaluator: Option<&Evaluator>) -> Result<Operand, anyhow::Error> {
    let lex = tokenize(formula)?;
    let expr_tokens = parse_expr(&mut lex.as_slice().into())?;
    let suffix = to_suffix(&expr_tokens);
    let default_eval = Evaluator::default();
    let calculator = evaluator.unwrap_or(&default_eval);
    Ok(calculator.eval(&suffix)?)
}

#[cfg(test)]
mod test {
    use expr::{operator::Operator, Error};

    use super::*;

    fn test_func(operands: &mut Vec<Operand>) -> Result<(), Error> {
        let c = operands
            .pop()
            .ok_or(Error::NotEnoughOperand)?
            .as_int()
            .ok_or(Error::InvalidDataType("Float".to_string()))?;
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

        operands.push(Operand::Integer(a * b + c));

        Ok(())
    }

    #[test]
    fn test_basic() {
        let mut evaluator = Evaluator::default();
        evaluator.insert_op_handler(Operator::Custom("func".to_string()), Box::new(test_func));
        assert_eq!(
            eval("2+(1 - 5)*3/4", None).unwrap(),
            Operand::Integer(2 + (1 - 5) * 3 / 4)
        );
        assert_eq!(
            eval("3.0 / 5 ^^ 0.3", None).unwrap(),
            Operand::Float(3f64 / 5f64.powf(0.3f64))
        );
        assert_eq!(eval("1 << 10", None).unwrap(), Operand::Integer(1 << 10));
        assert_eq!(
            eval("!0 & 0xF0F00000", None).unwrap(),
            Operand::Integer(0xF0F00000)
        );
        assert_eq!(
            eval("~(0xF<<4)", None).unwrap(),
            Operand::Integer(0xFFFFFF0F)
        );
        assert_eq!(
            eval("~(0xF>>2)", None).unwrap(),
            Operand::Integer(0xFFFFFFFC)
        );
        assert_eq!(
            eval("0b101 b^ b!0", None).unwrap(),
            Operand::Integer(0b11111010)
        );
        assert_eq!(eval("4096 / 1k", None).unwrap(), Operand::Integer(4));
        assert_eq!(
            eval("(2--1)*5+-3+func(((2)),(1+2),4)", Some(&evaluator)).unwrap(),
            Operand::Integer(22)
        );
    }
}
