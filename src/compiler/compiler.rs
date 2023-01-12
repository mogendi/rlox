use std::{cell::RefCell, fmt::{Debug, Display}, path::Prefix, borrow::Borrow};

use crate::{
    errors::err::ErrTrait,
    instructions::{
        binary::{Binary, BinaryOp},
        chunk::Chunk,
        constant::Constant,
        instructions::Instruction,
        unary::{Unary, UnaryOp},
        values::values::Value,
    },
};

use super::{
    err::ParserErr,
    rules::{construct_rule, Precendence},
    scanner::Scanner,
    token::{Token, TokenType},
};

/// Scans and parses the lox language
/// as defined at: https://craftinginterpreters.com/contents.html
///
/// Precendence:
/// ---------------
/// Equality [== !=]       -> Left
/// Comparison [< > <= >=] -> Left
/// Term [+ -]             -> Left
/// Factor [/ *]           -> Left
/// Unary [! -]            -> Right
///
/// Supported CFG:
/// --------------
/// program     -> declaration* EOF
/// declaration -> varDecl | statement | funDecl | classDecl
/// classDecl   -> class IDENTIFIER ( "<" IDENTIFIER )? "{" function* "}"
/// funDecl     -> "fun" function
/// function    -> IDENTIFIER "(" parameters? ")" block
/// paramters   -> IDENTIFIER ("," IDENTIFIER)*
/// varDecl     -> "var" IDENTIFIER ( "=" expression )? ";"
/// statement   -> exprStmt | printStmt | block | ifStmt | whileStmt | forStmt | returnStmt
/// returnStmt  -> "return" expression? ";"
/// forStmt     -> "for" "(" (declaration | exprStmt)? ";" expression? ";" expression? ";"
/// whileStmt   -> "while" expression statement
/// ifStmt      -> if "(" expression ")" statement ("else" statement)?
/// block       -> "{" declaration* "}"
/// exprStmt    -> expression ";"
/// printStmt   -> "print" expression ";"
/// expression  -> assignment
/// assignment  -> (call ".") IDENTIFIER '=' assignment | logic_or
/// logic_or    -> logic_or ("or" logic_and)*
/// logic_and   -> equality ("and" equality)*
/// equality    -> comparison ( (!= | ==) comparison )*
/// comparison  -> term ( (> | >= | < | <=) term)*
/// term        -> factor ( (-|+) factor)*
/// factor      -> unary ( (/|*) unary )*
/// unary       -> ( (!|-) )unary | call
/// call        -> primary ( "(" arguments? ")" | "." IDENTIFIER )*
/// arguments   -> expresion ("," expression)*
/// primary     -> number | string | "true" | "false" | "nil" |
///                 "(" expression ")" | IDENTIFIER |
///                 "super" "." IDENTIFIER

#[derive(Debug)]
pub struct Parser<'a> {
    scanner: &'a Scanner<'a>,
    current: RefCell<Token<'a>>,
    previous: RefCell<Option<Token<'a>>>,
    chunk: RefCell<&'a mut Chunk>,
}

impl<'a> Parser<'a> {
    pub fn new(scanner: &'a Scanner, chunk: &'a mut Chunk) -> Result<Self, Box<dyn ErrTrait>> {
        let current = RefCell::new(scanner.next()?);
        Ok(Parser {
            scanner,
            current,
            previous: RefCell::new(None),
            chunk: RefCell::new(chunk),
        })
    }

    fn advance(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        let next = self.scanner.next()?;
        self.previous
            .replace_with(|_| Some(self.current.replace(next)));

        Ok(())
    }

    fn consume(&'a self, expected: TokenType) -> Result<(), Box<dyn ErrTrait>> {
        let token = self.current.borrow().clone();
        println!("Got: {}, expecting {}", token, expected);
        if token.token_type == expected {
            self.advance()?;
            return Ok(());
        }
        let scan_line = self.scanner.line();
        return Err(Box::new(ParserErr::new(
            format!("Expected {} found {}", expected, token),
            self.scanner.line_to_string(),
            scan_line.number,
            scan_line.offset,
        )));
    }

    fn get_previous(&self) -> Result<Token, Box<dyn ErrTrait>> {
        let prev = &*self.previous.borrow();
        match prev {
            Some(token) => Ok(token.clone()),
            None => {
                let scan_line = self.scanner.line();
                return Err(Box::new(ParserErr::new(
                    format!(
                        "Interpreter Error:: Could consume previous number, Parser dump: {}",
                        self
                    ),
                    self.scanner.line_to_string(),
                    scan_line.number,
                    scan_line.offset,
                )));
            }
        }
    }

    fn push(&self, inst: impl Instruction + 'static) -> Result<(), Box<dyn ErrTrait>>{
        let mut chunk = self.chunk.borrow_mut();
        chunk.write_to_chunk(Box::new(inst), self.scanner.line().number)?;
        Ok(())
    }

    pub fn number(&self) -> Result<(), Box<dyn ErrTrait>> {
        let token = self.get_previous()?;
        let val = match String::from_utf8_lossy(token.literal).parse::<u64>() {
            Ok(float) => float as f64,
            Err(err) => {
                let scan_line = self.scanner.line();
                return Err(Box::new(ParserErr::new(
                    format!(
                        "Expected Number: couldn't convert {} to a valid Number, {}",
                        String::from_utf8_lossy(token.literal),
                        err.to_string()
                    ),
                    self.scanner.line_to_string(),
                    scan_line.number,
                    scan_line.offset,
                )));
            }
        };
        self.push(Constant::new(Value::Number(val)))?;
        return Ok(());
    }

    pub fn literal(&self) -> Result<(), Box<dyn ErrTrait>> {
        let token = self.get_previous()?;
        let val = match token.token_type {
            TokenType::TRUE => Value::Bool(true),
            TokenType::FALSE => Value::Bool(false),
            TokenType::NIL => Value::Nil,
            TokenType::STRING => Value::String(String::from_utf8_lossy(token.literal).to_string()),
            _ => {
                let scan_line = self.scanner.line();
                return Err(
                    Box::new(
                        ParserErr::new(
                            format!(
                                "Expected literal [true | false| nil] found {}",
                                String::from_utf8_lossy(token.literal)
                            ),
                            self.scanner.line_to_string(),
                            scan_line.number,
                            scan_line.offset,
                        )
                    )
                )
            }
        };

        self.push(Constant::new(val))?;
        Ok(())
    }

    pub fn unary(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        let token = self.get_previous()?;
        let op = match token.token_type {
            TokenType::MINUS => UnaryOp::Negate,
            TokenType::BANG => UnaryOp::Bang,
            _ => {
                let scan_line = self.scanner.line();
                return Err(Box::new(ParserErr::new(
                    format!("Invalid unary operator: {}", token.token_type),
                    self.scanner.line_to_string(),
                    scan_line.number,
                    scan_line.offset,
                )));
            }
        };
        self.parse(Precendence::Unary)?;
        self.push(Unary::new(op))?;
        Ok(())
    }

    pub fn binary(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        let token = self.get_previous()?;
        let rule = construct_rule(token.token_type);
        self.parse(rule.precedence.next()?)?;
        let mut after_push_hook: fn(&Parser) -> Result<(), Box<dyn ErrTrait>> = |_| {Ok(())};
        let op = match token.token_type {
            TokenType::PLUS => BinaryOp::ADD,
            TokenType::MINUS => BinaryOp::SUBTRACT,
            TokenType::STAR => BinaryOp::MULTIPLY,
            TokenType::SLASH => BinaryOp::DIVIDE,
            TokenType::EQUAL_EQUAL => BinaryOp::EQUAL,
            TokenType::GREATER => BinaryOp::GREATER,
            TokenType::LESS => BinaryOp::LESS,
            TokenType::BANG_EQUAL => {
                after_push_hook = |parser| { parser.push(Unary::new(UnaryOp::Bang)) };
                BinaryOp::EQUAL
            }
            TokenType::GREATER_EQUAL => {
                after_push_hook = |parser| { parser.push(Unary::new(UnaryOp::Bang)) };
                BinaryOp::LESS
            }
            TokenType::LESS_EQUAL => {
                after_push_hook = |parser| { parser.push(Unary::new(UnaryOp::Bang)) };
                BinaryOp::GREATER
            }
            _ => {
                let scan_line = self.scanner.line();
                return Err(Box::new(ParserErr::new(
                    format!("Invalid Binary operator: {}", token.token_type),
                    self.scanner.line_to_string(),
                    scan_line.number,
                    scan_line.offset,
                )));
            }
        };
        self.push(Binary::new(op))?;
        after_push_hook(self)?;
        Ok(())
    }

    pub fn grouping(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        self.expression()?;
        self.consume(TokenType::RIGHT_PAREN)?;
        Ok(())
    }

    fn parse(&'a self, prec: Precendence) -> Result<(), Box<dyn ErrTrait>> {
        let prefix_not_found_err = || {
            println!("Parser [Prefix not found]]: {}", self);
            let scan_line = self.scanner.line();
            Box::new(ParserErr::new(
                "Expected expression".to_string(),
                self.scanner.line_to_string(),
                scan_line.number,
                scan_line.offset,
            ))
        };

        let infix_not_found_err = || {
            println!("Parser [Infix not found]]: {}", self);
            let scan_line = self.scanner.line();
            Box::new(ParserErr::new(
                "Expected expression".to_string(),
                self.scanner.line_to_string(),
                scan_line.number,
                scan_line.offset,
            ))
        };

        self.advance()?;
        let prefix_rule = construct_rule(self.get_previous()?.token_type);
        match prefix_rule.prefix {
            Some(method) => method(self)?,
            None => return Err(prefix_not_found_err()),
        }

        loop {
            let current_rule = construct_rule(self.current.borrow().token_type);
            if prec as u8 > current_rule.precedence as u8 {
                break;
            }
            self.advance()?;
            let infix_rule = construct_rule(self.get_previous()?.token_type);
            match infix_rule.infix {
                Some(method) => method(self)?,
                None => return Err(infix_not_found_err()),
            }
        }
        Ok(())
    }

    fn expression(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        self.parse(Precendence::Assignment)
    }
}

impl<'a> Display for Parser<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
"
Scanner: {}
-----------------------------------------
Current: {}
-----------------------------------------
Previous: {:?}
-----------------------------------------
Chunk:
======

{}
", self.scanner, self.current.borrow(), self.previous.borrow(), self.chunk.borrow())
    }
}

pub struct Compiler {}

impl Compiler {
    pub fn compile(src: Vec<u8>) -> Result<Chunk, Box<dyn ErrTrait>> {
        let scanner = Scanner::new(src);
        let mut chunk = Chunk::new();
        let parser = Parser::new(&scanner, &mut chunk)?;
        parser.expression()?;
        print!("{}", parser);
        Ok(chunk)
    }
}
