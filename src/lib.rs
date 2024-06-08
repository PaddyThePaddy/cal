use expr::{operand::Operand, to_suffix, Evaluator, ExprToken};
use lex::LexToken;
use logos::Logos;

pub mod expr;
pub mod lex;

pub type Integer = i128;
pub type Float = f64;

pub fn tokenize(formula: &str) -> Result<Vec<ExprToken>, lex::Error> {
    let lexer = LexToken::lexer(formula);
    Ok(lexer
        .collect::<Result<Vec<_>, lex::Error>>()?
        .iter()
        .map(ExprToken::from)
        .collect())
}

pub fn eval(formula: &str) -> Result<Operand, anyhow::Error> {
    let tokens = tokenize(formula)?;
    let suffix = to_suffix(&tokens);
    let calculator = Evaluator::default();
    Ok(calculator.eval(&suffix)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            eval("2+(1 - 5)*3/4").unwrap(),
            Operand::Integer(2 + (1 - 5) * 3 / 4)
        );
        assert_eq!(
            eval("3.0 / 5 ^^ 0.3").unwrap(),
            Operand::Float(3f64 / 5f64.powf(0.3f64))
        );
        assert_eq!(eval("1 << 10").unwrap(), Operand::Integer(1 << 10));
        assert_eq!(
            eval("!0 & 0xF0F00000").unwrap(),
            Operand::Integer(0xF0F00000)
        );
        assert_eq!(eval("~(0xF<<4)").unwrap(), Operand::Integer(0xFFFFFF0F));
        assert_eq!(eval("~(0xF>>2)").unwrap(), Operand::Integer(0xFFFFFFFC));
        assert_eq!(eval("0b101 b^ b!0").unwrap(), Operand::Integer(0b11111010));
        assert_eq!(eval("4096 / 1k").unwrap(), Operand::Integer(4));
    }
}
