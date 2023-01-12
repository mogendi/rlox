use std::fmt::Debug;

use crate::errors::err::ErrTrait;

use super::{compiler::Parser, err::InterpreterErr, token::TokenType};

#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Precendence {
    None = 0,
    Assignment = 1,
    Or = 2,
    And = 3,
    Equality = 4,
    Comparison = 5,
    Term = 6,
    Factor = 7,
    Unary = 8,
    Call = 9,
    Primary = 10,
}

impl Precendence {
    pub fn next(&self) -> Result<Precendence, Box<dyn ErrTrait>> {
        (*self as u8 + 1).try_into()
    }
}

impl TryFrom<u8> for Precendence {
    type Error = Box<dyn ErrTrait>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Assignment),
            2 => Ok(Self::Or),
            3 => Ok(Self::And),
            4 => Ok(Self::Equality),
            5 => Ok(Self::Comparison),
            6 => Ok(Self::Term),
            7 => Ok(Self::Factor),
            8 => Ok(Self::Unary),
            9 => Ok(Self::Call),
            10 => Ok(Self::Primary),
            _ => Err(Box::new(InterpreterErr::new(format!(
                "Couldn't construct precedence from {}, invalid value passed",
                value
            )))),
        }
    }
}

pub struct ParseRule<'a> {
    pub prefix: Option<Box<dyn FnOnce(&'a Parser<'a>) -> Result<(), Box<dyn ErrTrait>>>>,
    pub infix: Option<Box<dyn FnOnce(&'a Parser<'a>) -> Result<(), Box<dyn ErrTrait>>>>,
    pub precedence: Precendence,
}

impl<'a> ParseRule<'a> {
    pub fn get_prefix(
        &'a self,
    ) -> Result<
        &'a Box<dyn FnOnce(&'a Parser<'a>) -> Result<(), Box<dyn ErrTrait>>>,
        Box<dyn ErrTrait>,
    > {
        match &self.prefix {
            Some(prefix) => Ok(prefix),
            None => Err(Box::new(InterpreterErr::new(
                "Attempted to construct a prefix for rule that doesn't have a prefix".to_string(),
            ))),
        }
    }
}

impl<'a> Debug for ParseRule<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Rule {:?}>", self.precedence)
    }
}

pub fn construct_rule<'a>(token_type: TokenType) -> ParseRule<'a> {
    match token_type {
        TokenType::LEFT_PAREN => ParseRule {
            prefix: Some(Box::new(|parser| parser.grouping())),
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::RIGHT_PAREN => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::LEFT_BRACE => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::RIGHT_BRACE => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::COMMA => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::DOT => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::MINUS => ParseRule {
            prefix: Some(Box::new(|parser| parser.unary())),
            infix: Some(Box::new(|parser| parser.binary())),
            precedence: Precendence::Term,
        },

        TokenType::PLUS => ParseRule {
            prefix: None,
            infix: Some(Box::new(|parser| parser.binary())),
            precedence: Precendence::Term,
        },

        TokenType::SEMICOLON => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::SLASH => ParseRule {
            prefix: None,
            infix: Some(Box::new(|parser| parser.binary())),
            precedence: Precendence::Factor,
        },

        TokenType::STAR => ParseRule {
            prefix: None,
            infix: Some(Box::new(|parser| parser.binary())),
            precedence: Precendence::Factor,
        },

        TokenType::BANG => ParseRule {
            prefix: Some(Box::new(|parser| parser.unary())),
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::BANG_EQUAL => ParseRule {
            prefix: None,
            infix: Some(Box::new(|parser| parser.binary())),
            precedence: Precendence::Equality,
        },

        TokenType::EQUAL => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::EQUAL_EQUAL => ParseRule {
            prefix: None,
            infix: Some(Box::new(|parser| parser.binary())),
            precedence: Precendence::Equality,
        },

        TokenType::GREATER => ParseRule {
            prefix: None,
            infix: Some(Box::new(|parser| parser.binary())),
            precedence: Precendence::Comparison,
        },

        TokenType::GREATER_EQUAL => ParseRule {
            prefix: None,
            infix: Some(Box::new(|parser| parser.binary())),
            precedence: Precendence::Comparison,
        },

        TokenType::LESS => ParseRule {
            prefix: None,
            infix: Some(Box::new(|parser| parser.binary())),
            precedence: Precendence::Comparison,
        },

        TokenType::LESS_EQUAL => ParseRule {
            prefix: None,
            infix: Some(Box::new(|parser| parser.binary())),
            precedence: Precendence::Comparison,
        },

        TokenType::IDENTIFIER => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::STRING => ParseRule {
            prefix: Some(Box::new(|parser| parser.literal())),
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::NUMBER => ParseRule {
            prefix: Some(Box::new(|parser| parser.number())),
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::AND => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::CLASS => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::ELSE => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::FALSE => ParseRule {
            prefix: Some(Box::new(|parser| parser.literal())),
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::FOR => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::FUN => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::IF => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::NIL => ParseRule {
            prefix: Some(Box::new(|parser| parser.literal())),
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::OR => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::PRINT => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::RETURN => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::SUPER => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::THIS => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::TRUE => ParseRule {
            prefix: Some(Box::new(|parser| parser.literal())),
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::VAR => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::WHILE => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },

        TokenType::EOF => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precendence::None,
        },
    }
}
