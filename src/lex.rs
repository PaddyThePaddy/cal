use logos::{Lexer, Logos};

use std::{
    num::{ParseFloatError, ParseIntError},
    str::FromStr as _,
};

use crate::{Float, Integer};

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone, Default)]
pub enum Error {
    #[error("{0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("{0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Invalid bit width hint: {0}")]
    InvalidBitWidthHint(String),
    #[error("Invalid token")]
    #[default]
    Invalid,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(error=Error)]
pub enum LexToken {
    #[regex(r"(?i)\d+[kmgtp]?", dec_number, priority = 3)]
    #[regex(r"0x[\da-fA-F]+", hex_number)]
    #[regex(r"0o[0-7]+", oct_number)]
    #[regex(r"0b[01]+", bin_number)]
    Integer(Integer),
    #[regex(
        r"(?:[1-9]\d*|\.\d+|\d+\.\d+)(?:[eE][-+]?(?:\d+|\.\d+|\d+\.\d+))?",
        science_notation,
        priority = 2
    )]
    Float(Float),
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("(")]
    OpenParenthesis,
    #[token(")")]
    CloseParenthesis,
    #[token("|")]
    BitOr,
    #[token("&")]
    BitAnd,
    #[regex(r"(b|w|dw|l|ll)?\^", bit_width)]
    BitXor(usize),
    #[regex(r"(b|w|dw|l)?!", bit_width)]
    #[regex(r"(b|w|dw|l)?~", bit_width)]
    BitNot(usize),
    #[token("^^")]
    Expo,
    #[token("%")]
    Mod,
    #[regex("[a-zA-Z]\\w*", store_identifier)]
    Custom(String),
    #[token(">>")]
    RightShift,
    #[token("<<")]
    LeftShift,
    #[token(",")]
    Comma,
    #[regex(r#""([^"]|\\")*""#, store_string)]
    #[regex(r#"'([^']|\\')*'"#, store_string)]
    String(String),
}

impl LexToken {
    /// lower is higher
    pub fn precedence(&self) -> Option<usize> {
        match self {
            Self::Expo | Self::BitNot(_) => Some(0),
            Self::Mul
            | Self::Mod
            | Self::Div
            | Self::BitAnd
            | Self::BitXor(_)
            | Self::LeftShift
            | Self::RightShift => Some(1),
            Self::Plus | Self::Minus | Self::BitOr => Some(2),
            _ => None,
        }
    }
}

fn dec_number(lex: &mut Lexer<LexToken>) -> Result<Integer, Error> {
    let mut token = lex.slice();
    let unit = if let Some(remain) = token.strip_suffix(['k', 'K']) {
        token = remain;
        1024
    } else if let Some(remain) = token.strip_suffix(['m', 'M']) {
        token = remain;
        1024 * 1024
    } else if let Some(remain) = token.strip_suffix(['g', 'G']) {
        token = remain;
        1024 * 1024 * 1024
    } else if let Some(remain) = token.strip_suffix(['t', 'T']) {
        token = remain;
        1024 * 1024 * 1024 * 1024
    } else if let Some(remain) = token.strip_suffix(['p', 'P']) {
        token = remain;
        1024 * 1024 * 1024 * 1024 * 1024
    } else {
        1
    };
    Integer::from_str_radix(token, 10)
        .map(|n| n * unit)
        .map_err(Error::from)
}

fn hex_number(lex: &mut Lexer<LexToken>) -> Result<Integer, Error> {
    lex.slice()
        .strip_prefix("0x")
        .ok_or(Error::Invalid)
        .and_then(|s| Integer::from_str_radix(s, 16).map_err(Error::from))
}

fn oct_number(lex: &mut Lexer<LexToken>) -> Result<Integer, Error> {
    lex.slice()
        .strip_prefix("0o")
        .ok_or(Error::Invalid)
        .and_then(|s| Integer::from_str_radix(s, 8).map_err(Error::from))
}

fn bin_number(lex: &mut Lexer<LexToken>) -> Result<Integer, Error> {
    lex.slice()
        .strip_prefix("0b")
        .ok_or(Error::Invalid)
        .and_then(|s| Integer::from_str_radix(s, 2).map_err(Error::from))
}

fn store_identifier(lex: &mut Lexer<LexToken>) -> Option<String> {
    Some(lex.slice().to_string())
}

fn bit_width(lex: &mut Lexer<LexToken>) -> Result<usize, Error> {
    lex.slice()
        .strip_suffix(['^', '~', '!'])
        .ok_or(Error::Invalid)
        .and_then(|s| match s {
            "b" => Ok(8),
            "w" => Ok(16),
            "dw" => Ok(32),
            "l" => Ok(64),
            "ll" => Ok(128),
            "" => Ok(32),
            _ => Err(Error::InvalidBitWidthHint(s.to_string())),
        })
}

fn science_notation(lex: &mut Lexer<LexToken>) -> Result<Float, Error> {
    Float::from_str(lex.slice()).map_err(Error::from)
}

fn store_string(lex: &mut Lexer<LexToken>) -> Result<String, Error> {
    Ok(lex
        .slice()
        .strip_prefix(['\'', '"'])
        .ok_or(Error::Invalid)?
        .strip_suffix(['\'', '"'])
        .ok_or(Error::Invalid)?
        .to_string())
}
