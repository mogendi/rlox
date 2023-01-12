use std::{cell::RefCell, marker::PhantomData, fmt::Display};

use crate::errors::err::ErrTrait;

use super::{
    err::ScannerErr,
    token::{Token, TokenType},
};

pub struct Line {
    pub number: usize,
    pub offset: usize,
}

const FORWARD: u8 = 0;
const BACKWARD: u8 = 1;

#[derive(Debug)]
pub struct Scanner<'a> {
    input_stream: Vec<u8>,
    current: RefCell<usize>,
    start: RefCell<usize>,
    line: RefCell<usize>,
    // Forcing Scanner to have 'a lifetime
    phantom: PhantomData<&'a ()>,
}

impl<'a> Scanner<'a> {
    pub fn new(stream: Vec<u8>) -> Self {
        Scanner {
            input_stream: stream,
            current: RefCell::new(0),
            start: RefCell::new(0),
            line: RefCell::new(1),
            phantom: PhantomData,
        }
    }

    fn is_alpha(c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => return true,
            _ => return false,
        }
    }

    fn is_digit(c: char) -> bool {
        match c {
            '0'..='9' => return true,
            _ => return false,
        }
    }

    fn seek(&self, c: char, direction: u8, offset: Option<usize>) -> usize {
        let get_next_index = |index: usize| {
            if direction == FORWARD {
                return index + 1;
            } else {
                return index - 1;
            }
        };

        let mut index = match offset {
            Some(idx) => *self.current.borrow() - idx,
            None => *self.current.borrow(),
        };
        while index > 0 && index < self.input_stream.len() && self.input_stream[index] as char != c
        {
            index = get_next_index(index)
        }
        index
    }

    pub(super) fn line(&self) -> Line {
        Line {
            number: *self.line.borrow(),
            offset: *self.current.borrow() - self.seek('\n', BACKWARD, None),
        }
    }

    pub(super) fn line_to_string(&self) -> String {
        let curr = match *self.current.borrow() >= self.input_stream.len() {
            true => self.input_stream.len() - 1,
            false => *self.current.borrow()
        };
        let offset = match self.input_stream[curr] as char {
            '\n' => Some(1),
            _ => None,
        };
        let mut start_index = self.seek('\n', BACKWARD, offset);
        if start_index > 0 {
            start_index += 1;
        }
        let end_index = self.seek('\n', FORWARD, offset);
        String::from_utf8_lossy(&self.input_stream[start_index..=end_index]).to_string()
    }

    fn is_at_end(&self) -> bool {
        *self.current.borrow() >= self.input_stream.len() - 1
    }

    fn current_to_string(&self) -> String {
        String::from_utf8_lossy(&self.input_stream[*self.start.borrow()..*self.current.borrow()])
            .to_string()
    }

    fn advance(&self) {
        if *self.current.borrow() < self.input_stream.len() {
            self.current.replace_with(|&mut old| old + 1);
        }
    }

    fn peek(&self) -> char {
        self.input_stream[*self.current.borrow()] as char
    }

    fn peek_next(&self) -> char {
        self.input_stream[*self.current.borrow() + 1] as char
    }

    fn match_(&self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != expected {
            return false;
        }
        self.advance();
        true
    }

    fn match_next(&self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek_next() != expected {
            return false;
        }
        self.advance();
        true
    }

    fn check_keyword(
        &self,
        offset: usize,
        expected: &[u8],
        token_type: TokenType,
    ) -> Result<TokenType, Box<dyn ErrTrait>> {
        if self.input_stream.len() > *self.current.borrow() + offset {
            if expected
                == &self.input_stream[*self.current.borrow()..=*self.current.borrow() + offset]
            {
                return Ok(token_type);
            }
        }
        Ok(TokenType::IDENTIFIER)
    }

    fn skip_whitespace(&self) {
        loop {
            let current_char = self.input_stream[*self.current.borrow()] as char;
            match current_char {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        loop {
                            if self.peek() != '\n' && !self.is_at_end() {
                                self.advance()
                            } else {
                                if self.peek() == '\n' {
                                    self.advance()
                                }
                                break;
                            }
                        }
                    }
                }
                '\n' => {
                    self.line.replace_with(|&mut old| old + 1);
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn make_token(&'a self, token_type: TokenType) -> Token<'a> {
        self.advance();
        Token::new(
            token_type,
            &self.input_stream[*self.start.borrow()..*self.current.borrow()],
            *self.line.borrow() as u32,
        )
    }

    fn number(&'a self) -> Result<Token<'a>, Box<dyn ErrTrait>> {
        loop { 
            if Self::is_digit(self.peek_next()) && !self.is_at_end(){
                self.advance();
            } else {
                break
            }
        }
        self.skip_whitespace();
        Ok(self.make_token(TokenType::NUMBER))
    }

    fn string(&'a self) -> Result<Token<'a>, Box<dyn ErrTrait>> {
        let mut at_begining = true;
        while self.peek_next() != '"' && !self.is_at_end() {
            if self.peek_next() == '\n' {
                self.line.replace_with(|&mut old| old + 1);
            }
            if at_begining {
                let current_start = *self.start.borrow();
                self.start.replace(current_start + 1);
                at_begining = false;
            }
            self.advance();
        }
        if self.peek_next() != '"' && self.is_at_end() {
            return Err(Box::new(ScannerErr::new(
                "Unterminated string".to_string(),
                self.line_to_string(),
                *self.line.borrow(),
                *self.current.borrow() - self.seek('\n', BACKWARD, None),
            )));
        }
        let token = self.make_token(TokenType::STRING);
        self.advance();
        Ok(token)
    }

    fn identifier(&'a self) -> Result<Token<'a>, Box<dyn ErrTrait>> {
        let token_type: TokenType = match self.peek() {
            'a' => self.check_keyword(2, &['a' as u8, 'n' as u8, 'd' as u8], TokenType::AND)?,
            'c' => self.check_keyword(
                4,
                &['c' as u8, 'l' as u8, 'a' as u8, 's' as u8, 's' as u8],
                TokenType::CLASS,
            )?,
            'e' => self.check_keyword(
                3,
                &['e' as u8, 'l' as u8, 's' as u8, 'e' as u8],
                TokenType::ELSE,
            )?,
            'f' => match self.peek_next() {
                'a' => self.check_keyword(
                    4,
                    &['f' as u8, 'a' as u8, 'l' as u8, 's' as u8, 'e' as u8],
                    TokenType::FALSE,
                )?,
                'o' => self.check_keyword(4, &['f' as u8, 'o' as u8, 'r' as u8], TokenType::FOR)?,
                'u' => self.check_keyword(4, &['f' as u8, 'u' as u8, 'n' as u8], TokenType::FUN)?,
                _ => TokenType::IDENTIFIER,
            },
            'i' => self.check_keyword(1, &['i' as u8, 'f' as u8], TokenType::IF)?,
            'n' => self.check_keyword(2, &['n' as u8, 'i' as u8, 'l' as u8], TokenType::NIL)?,
            'o' => self.check_keyword(1, &['o' as u8, 'r' as u8], TokenType::OR)?,
            'p' => self.check_keyword(
                4,
                &['p' as u8, 'r' as u8, 'i' as u8, 'n' as u8, 't' as u8],
                TokenType::PRINT,
            )?,
            'r' => self.check_keyword(
                5,
                &[
                    'r' as u8, 'e' as u8, 't' as u8, 'u' as u8, 'r' as u8, 'n' as u8,
                ],
                TokenType::RETURN,
            )?,
            's' => self.check_keyword(
                4,
                &['s' as u8, 'u' as u8, 'p' as u8, 'e' as u8, 'r' as u8],
                TokenType::SUPER,
            )?,
            't' => match self.peek_next() {
                'h' => self.check_keyword(
                    3,
                    &['t' as u8, 'h' as u8, 'i' as u8, 's' as u8],
                    TokenType::THIS,
                )?,
                'r' => self.check_keyword(
                    3,
                    &['t' as u8, 'r' as u8, 'u' as u8, 'e' as u8],
                    TokenType::TRUE,
                )?,
                _ => TokenType::IDENTIFIER,
            },
            'v' => self.check_keyword(2, &['v' as u8, 'a' as u8, 'r' as u8], TokenType::VAR)?,
            'w' => self.check_keyword(
                2,
                &['w' as u8, 'h' as u8, 'i' as u8, 'l' as u8, 'e' as u8],
                TokenType::WHILE,
            )?,
            _ => TokenType::IDENTIFIER,
        };
        while Self::is_alpha(self.peek_next()) || Self::is_digit(self.peek_next()) {
            self.advance()
        }
        Ok(self.make_token(token_type))
    }

    fn scan(&'a self) -> Result<Token<'a>, Box<dyn ErrTrait>> {
        self.skip_whitespace();
        *self.start.borrow_mut() = *self.current.borrow();
        if self.is_at_end() {
            return Ok(Token::new(
                TokenType::EOF,
                &['e' as u8, 'o' as u8, 'f' as u8],
                *self.line.borrow() as u32,
            ));
        }

        let current_char = self.input_stream[*self.current.borrow()] as char;

        if Self::is_alpha(current_char) {
            return self.identifier();
        }

        if Self::is_digit(current_char) {
            return self.number();
        }

        let res = match current_char {
            '(' => Ok(self.make_token(TokenType::LEFT_PAREN)),
            ')' => Ok(self.make_token(TokenType::RIGHT_PAREN)),
            '{' => Ok(self.make_token(TokenType::LEFT_BRACE)),
            '}' => Ok(self.make_token(TokenType::RIGHT_BRACE)),
            ';' => Ok(self.make_token(TokenType::SEMICOLON)),
            ',' => Ok(self.make_token(TokenType::COMMA)),
            '.' => Ok(self.make_token(TokenType::DOT)),
            '-' => Ok(self.make_token(TokenType::MINUS)),
            '+' => Ok(self.make_token(TokenType::PLUS)),
            '/' => Ok(self.make_token(TokenType::SLASH)),
            '*' => Ok(self.make_token(TokenType::STAR)),
            '!' => {
                let token;
                if self.match_next('=') {
                    token = Ok(self.make_token(TokenType::BANG_EQUAL))
                } else {
                    token = Ok(self.make_token(TokenType::BANG))
                }
                token
            }
            '=' => {
                let token;
                if self.match_next('=') {
                    token = Ok(self.make_token(TokenType::EQUAL_EQUAL))
                } else {
                    token = Ok(self.make_token(TokenType::EQUAL))
                }
                token
            }
            '<' => {
                let token;
                if self.match_next('=') {
                    token = Ok(self.make_token(TokenType::LESS_EQUAL))
                } else {
                    token = Ok(self.make_token(TokenType::LESS))
                }
                token
            }
            '>' => {
                let token;
                if self.match_next('=') {
                    token = Ok(self.make_token(TokenType::GREATER_EQUAL))
                } else {
                    token = Ok(self.make_token(TokenType::GREATER))
                }
                token
            }
            '"' => self.string(),

            _ => {
                self.advance();
                return Err(Box::new(ScannerErr::new(
                    format!(
                        "unexpected token on line {}: {:?}",
                        *self.line.borrow(),
                        self.current_to_string()
                    ),
                    self.line_to_string(),
                    *self.line.borrow(),
                    *self.current.borrow() - self.seek('\n', BACKWARD, None),
                )));
            }
        };

        return res;
    }

    // This is necessary since the iter trait doesn't allow
    // lifetime defs on the refrence to next
    pub fn next(&'a self) -> Result<Token<'a>, Box<dyn ErrTrait>> {
        if self.is_at_end() {
            return Ok(self.make_token(TokenType::EOF));
        }
        self.scan()
    }
}

impl<'a> Display for Scanner<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "> {}", self.line_to_string())
    }
}
