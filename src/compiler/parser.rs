use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{
    errors::err::ErrTrait,
    instructions::{
        binary::{Binary, BinaryOp},
        call::Call,
        chunk::Chunk,
        constant::Constant,
        define::{Define, DefinitionScope, Override, Resolve},
        instructions::{Instruction, None, Pop},
        jump::{ForceJump, Jump},
        print::Print,
        properties::{Get, Set},
        return_inst::Return,
        unary::{Unary, UnaryOp},
    },
    values::{obj::Class, values::Value},
};

use super::{
    compiler::{Compiler, FunctionType},
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
    pub compiler: RefCell<&'a mut Compiler<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(
        scanner: &'a Scanner<'a>,
        chunk: &'a mut Chunk,
        compiler: &'a mut Compiler<'a>,
    ) -> Result<Self, Box<dyn ErrTrait>> {
        let current = RefCell::new(scanner.next()?);
        Ok(Parser {
            scanner,
            current,
            previous: RefCell::new(None),
            chunk: RefCell::new(chunk),
            compiler: RefCell::new(compiler),
        })
    }

    fn check(&self, type_: TokenType) -> bool {
        self.current.borrow().token_type == type_
    }

    fn match_(&'a self, type_: TokenType) -> Result<bool, Box<dyn ErrTrait>> {
        if !self.check(type_) {
            return Ok(false);
        }
        self.advance()?;
        Ok(true)
    }

    fn advance(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        let next = self.scanner.next()?;
        self.previous
            .replace_with(|_| Some(self.current.replace(next)));

        Ok(())
    }

    fn consume(&'a self, expected: TokenType) -> Result<(), Box<dyn ErrTrait>> {
        let token = self.current.borrow().clone();
        if token.token_type == expected {
            self.advance()?;
            return Ok(());
        }
        let scan_line = self.scanner.line();
        return Err(Box::new(ParserErr::new(
            format!("Expected {} but found {}", expected, token),
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

    fn push(&self, inst: impl Instruction + 'static) -> Result<(), Box<dyn ErrTrait>> {
        let mut chunk = self.chunk.borrow_mut();
        chunk.write_to_chunk(Box::new(inst), self.scanner.line().number)?;
        Ok(())
    }

    fn start_scope(&'a self) {
        self.compiler.borrow_mut().start_scope();
    }

    fn end_scope(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        self.compiler
            .borrow_mut()
            .end_scope(&mut self.chunk.borrow_mut(), self.scanner.line().number)?;
        Ok(())
    }

    fn escape_scope(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        let mut brace_pair_count: u32 = 1;
        loop {
            if brace_pair_count == 0 {
                break;
            }
            if self.match_(TokenType::LEFT_BRACE)? {
                brace_pair_count += 1;
                continue;
            }
            if self.match_(TokenType::RIGHT_BRACE)? {
                brace_pair_count -= 1;
                continue;
            }
            self.advance()?;
            if self.match_(TokenType::EOF)? {
                let token = self.get_previous()?;
                let scan_line = self.scanner.line();
                return Err(Box::new(ParserErr::new(
                    "Unexpected EOF".to_string(),
                    String::from_utf8_lossy(token.literal).to_string(),
                    scan_line.number,
                    scan_line.offset,
                )));
            }
        }
        Ok(())
    }

    pub fn number(&self) -> Result<(), Box<dyn ErrTrait>> {
        let token = self.get_previous()?;
        let val = match String::from_utf8_lossy(token.literal).parse::<f64>() {
            Ok(float) => float,
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
                return Err(Box::new(ParserErr::new(
                    format!(
                        "Expected literal [true | false| nil] found {}",
                        String::from_utf8_lossy(token.literal)
                    ),
                    self.scanner.line_to_string(),
                    scan_line.number,
                    scan_line.offset,
                )));
            }
        };

        self.push(Constant::new(val))?;
        Ok(())
    }

    pub fn var(&'a self, can_assign: bool) -> Result<(), Box<dyn ErrTrait>> {
        let token = self.get_previous()?;

        // we need to find the relvant scope for the identifier before we
        // build any instructions
        let is_const = self.compiler.borrow().check_const_from_token(&token);
        let scope = match self.compiler.borrow().resolve(&token) {
            Some(scope_val) => scope_val,
            None => {
                let scan_line = self.scanner.line();
                return Err(Box::new(ParserErr::new(
                    format!(
                        "Can not access or overwrite undefined variable: `{}`",
                        token
                    ),
                    self.scanner.line_to_string(),
                    scan_line.number,
                    scan_line.offset,
                )));
            }
        };

        let match_ = self.match_(TokenType::EQUAL)?;
        if match_ && can_assign && !is_const {
            self.expression()?;
            return self.push(Override::new(format!("{}", token), scope));
        }
        if match_ && !can_assign {
            let scan_line = self.scanner.line();
            return Err(Box::new(ParserErr::new(
                "Invalid assignment target. Can only assign to previously defined variables."
                    .to_string(),
                self.scanner.line_to_string(),
                scan_line.number,
                scan_line.offset,
            )));
        }
        if match_ && is_const {
            let scan_line = self.scanner.line();
            return Err(Box::new(ParserErr::new(
                format!(
                    "Invalid assignment target. Can not assign to `const` `{}`",
                    token
                ),
                self.scanner.line_to_string(),
                scan_line.number,
                scan_line.offset,
            )));
        }
        self.push(Resolve::new(format!("{}", token), scope))
    }

    pub fn or(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        let origin = self.chunk.borrow().code.len();
        self.push(None::new())?;
        self.push(Pop::new())?;

        self.parse_expr(Precendence::Or)?;

        let dest = self.chunk.borrow().code.len();
        self.push(Jump::new(dest, false))?;

        self.chunk.borrow_mut().swap_instructions(origin, dest)?;
        Ok(())
    }

    pub fn and(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        let origin = self.chunk.borrow().code.len();
        self.push(None::new())?;
        self.push(Pop::new())?;

        self.parse_expr(Precendence::And)?;

        let dest = self.chunk.borrow().code.len();
        self.push(Jump::new(dest, true))?;

        self.chunk.borrow_mut().swap_instructions(origin, dest)?;

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
        self.parse_expr(Precendence::Unary)?;
        self.push(Unary::new(op))?;
        Ok(())
    }

    pub fn binary(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        let token = self.get_previous()?;
        let rule = construct_rule(token.token_type);
        self.parse_expr(rule.precedence.next()?)?;
        let mut after_push_hook: fn(&Parser) -> Result<(), Box<dyn ErrTrait>> = |_| Ok(());
        let op = match token.token_type {
            TokenType::PLUS => BinaryOp::ADD,
            TokenType::MINUS => BinaryOp::SUBTRACT,
            TokenType::STAR => BinaryOp::MULTIPLY,
            TokenType::SLASH => BinaryOp::DIVIDE,
            TokenType::EQUAL_EQUAL => BinaryOp::EQUAL,
            TokenType::GREATER => BinaryOp::GREATER,
            TokenType::LESS => BinaryOp::LESS,
            TokenType::BANG_EQUAL => {
                after_push_hook = |parser| parser.push(Unary::new(UnaryOp::Bang));
                BinaryOp::EQUAL
            }
            TokenType::GREATER_EQUAL => {
                after_push_hook = |parser| parser.push(Unary::new(UnaryOp::Bang));
                BinaryOp::LESS
            }
            TokenType::LESS_EQUAL => {
                after_push_hook = |parser| parser.push(Unary::new(UnaryOp::Bang));
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

    fn parse_expr(&'a self, prec: Precendence) -> Result<(), Box<dyn ErrTrait>> {
        let prefix_not_found_err = || {
            println!("Parser [Prefix not found]]: {}", self);
            let scan_line = self.scanner.line();
            Box::new(ParserErr::new(
                format!(
                    "Expected expression, found {}",
                    self.previous.borrow().as_ref().unwrap()
                ),
                self.scanner.line_to_string(),
                scan_line.number,
                scan_line.offset,
            ))
        };

        let infix_not_found_err = || {
            println!("Parser [Infix not found]]: {}", self);
            let scan_line = self.scanner.line();
            Box::new(ParserErr::new(
                format!(
                    "Expected expression, found {}",
                    self.previous.borrow().as_ref().unwrap()
                ),
                self.scanner.line_to_string(),
                scan_line.number,
                scan_line.offset,
            ))
        };

        self.advance()?;
        let prefix_rule = construct_rule(self.get_previous()?.token_type);
        let can_assign = prec as u8 <= Precendence::Assignment as u8;
        match prefix_rule.prefix {
            Some(method) => method(self, can_assign)?,
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
                Some(method) => method(self, can_assign)?,
                None => return Err(infix_not_found_err()),
            }
        }
        Ok(())
    }

    fn expression(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        self.parse_expr(Precendence::Assignment)
    }

    pub fn call(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        let mut args_len: usize = 0;
        if !self.check(TokenType::RIGHT_PAREN) {
            loop {
                self.expression()?;
                args_len += 1;
                if !self.match_(TokenType::COMMA)? {
                    break;
                }
            }
        }
        self.consume(TokenType::RIGHT_PAREN)?;

        let line = self.scanner.line();
        self.push(Call::new(
            args_len,
            line.number,
            self.scanner.line_to_string(),
        ))?;
        Ok(())
    }

    pub fn dot(&'a self, can_assign: bool) -> Result<(), Box<dyn ErrTrait>> {
        self.consume(TokenType::IDENTIFIER)?;
        let id = self.previous.borrow().as_ref().unwrap().clone();
        if can_assign && self.match_(TokenType::EQUAL)? {
            self.expression()?;
            let line = self.scanner.line();
            self.push(Set::new(
                format!("{}", id),
                line.number,
                self.scanner.line_to_string(),
            ))?;
        } else {
            let line = self.scanner.line();
            self.push(Get::new(
                format!("{}", id),
                line.number,
                self.scanner.line_to_string(),
            ))?;
        }

        Ok(())
    }

    fn function(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        self.start_scope();
        self.consume(TokenType::LEFT_PAREN)?;
        if !self.check(TokenType::RIGHT_PAREN) {
            loop {
                self.consume(TokenType::IDENTIFIER)?;
                let id = self.previous.borrow().as_ref().unwrap().clone();

                let scope = self.var_decl_inner(false, id.clone())?;

                self.push(Define::new(scope, format!("{}", id)))?;

                // marks the new var as initialized
                self.compiler.borrow().mark_latest_init();

                if !self.match_(TokenType::COMMA)? {
                    break;
                }
            }
        }
        self.consume(TokenType::RIGHT_PAREN)?;
        self.consume(TokenType::LEFT_BRACE)?;
        self.block()?;
        self.end_scope()?;

        Ok(())
    }

    fn print(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        self.expression()?;
        self.consume(TokenType::SEMICOLON)?;
        self.push(Print::new())?;
        Ok(())
    }

    fn block(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        while !self.check(TokenType::RIGHT_BRACE) && !self.check(TokenType::EOF) {
            self.declaration()?;
        }
        self.consume(TokenType::RIGHT_BRACE)?;
        Ok(())
    }

    fn expr_stmt(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        self.expression()?;
        self.consume(TokenType::SEMICOLON)?;
        self.push(Pop::new())?;
        Ok(())
    }

    fn var_decl_inner(
        &'a self,
        const_: bool,
        id: Token<'a>,
    ) -> Result<DefinitionScope, Box<dyn ErrTrait>> {
        // we need to check that this isn't a redefinition
        // in the same scope
        let scope_depth = self.compiler.borrow().scope();
        if scope_depth > 0 {
            match self.compiler.borrow().resolve_in_scope(&id) {
                Some(_) => {
                    let scan_line = self.scanner.line();
                    return Err(Box::new(ParserErr::new(
                        format!("Can not redefine `{}` in the same scope", id),
                        self.scanner.line_to_string(),
                        scan_line.number,
                        scan_line.offset,
                    )));
                }
                None => {}
            }
        }

        let scope = self
            .compiler
            .borrow_mut()
            .add_local(format!("{}", id), const_);

        Ok(scope)
    }

    fn var_decl(&'a self, const_: bool) -> Result<(), Box<dyn ErrTrait>> {
        self.consume(TokenType::IDENTIFIER)?;
        let id = self.previous.borrow().as_ref().unwrap().clone();

        let scope = self.var_decl_inner(const_, id.clone())?;

        if self.match_(TokenType::EQUAL)? {
            self.expression()?;
        } else {
            self.push(Constant::new(Value::Nil))?;
        }

        self.consume(TokenType::SEMICOLON)?;
        self.push(Define::new(scope, format!("{}", id)))?;

        // marks the new var as initialized
        self.compiler.borrow().mark_latest_init();

        Ok(())
    }

    fn if_stmt(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        self.consume(TokenType::LEFT_PAREN)?;
        self.expression()?;
        self.consume(TokenType::RIGHT_PAREN)?;

        // current instruction index + 1, where I expect the
        // call to jump to be
        let dest = self.chunk.borrow().code.len();
        // creating a none instruction so we can swap the jump
        // with something other than a valid instruction
        self.push(None::new())?;

        self.statement()?;

        // just incase we do execute the if clause
        // we need to force jump any possible else clause
        let force_jump_dest = self.chunk.borrow().code.len();
        self.push(None::new())?;

        // create the jump instruction pointing to where
        // the None instruction will eventually end up
        // on the call stack
        let origin = self.chunk.borrow().code.len();
        self.push(Jump::new(origin, true))?;

        //swap the None instruction with the jump instruction
        self.chunk.borrow_mut().swap_instructions(origin, dest)?;

        if self.match_(TokenType::ELSE)? {
            self.statement()?;
            // replicates the jump semantics for else
            let origin = self.chunk.borrow().code.len();
            self.push(ForceJump::new(origin))?;
            self.chunk
                .borrow_mut()
                .swap_instructions(origin, force_jump_dest)?;
        }

        self.push(Pop::new())?;
        Ok(())
    }

    /// Syntactic sugar for while loops
    /// Its strictly a for(decl/assignment; cond: incr)
    /// format, if for(;;) or any other variation is needed
    /// use while
    fn for_stmt(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        // the initial decl/assignment section
        self.consume(TokenType::LEFT_PAREN)?;
        if self.match_(TokenType::VAR)? {
            self.var_decl(false)?;
        } else {
            self.expr_stmt()?;
        }

        let jump_position = self.chunk.borrow().code.len();

        // the loop condition
        self.expression()?;
        self.consume(TokenType::SEMICOLON)?;

        let pre_expr_pos = self.chunk.borrow().code.len();
        self.push(None::new())?;
        self.push(Pop::new())?;
        let force_jump_pos = self.chunk.borrow().code.len();
        self.push(None::new())?;

        // the loop incr
        let pre_incr_pos = self.chunk.borrow().code.len();
        self.expression()?;
        self.consume(TokenType::RIGHT_PAREN)?;

        self.push(Pop::new())?;
        // jumps back to check the condition
        self.push(ForceJump::new(jump_position))?;

        // co-ordinates skipping over the incr expr
        let body_start_pos = self.chunk.borrow().code.len();
        self.push(ForceJump::new(body_start_pos))?;
        self.chunk
            .borrow_mut()
            .swap_instructions(force_jump_pos, body_start_pos)?;

        self.statement()?;

        // jumps back to the incr after the body
        self.push(ForceJump::new(pre_incr_pos))?;

        // condition jump for the loop break
        let post_for_clause = self.chunk.borrow().code.len();
        self.push(Jump::new(post_for_clause, true))?;
        self.chunk
            .borrow_mut()
            .swap_instructions(pre_expr_pos, post_for_clause)?;

        self.push(Pop::new())?;
        Ok(())
    }

    fn while_stmt(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        let jump_position = self.chunk.borrow().code.len();

        self.consume(TokenType::LEFT_PAREN)?;
        self.expression()?;
        self.consume(TokenType::RIGHT_PAREN)?;

        let origin = self.chunk.borrow().code.len();
        self.push(None::new())?;
        self.push(Pop::new())?;

        self.statement()?;

        // jump position can be pre-determined so we don't need to swap
        // with a none
        self.push(ForceJump::new(jump_position))?;

        let dest = self.chunk.borrow().code.len();
        self.push(Jump::new(dest, true))?;
        self.chunk.borrow_mut().swap_instructions(origin, dest)?;

        self.push(Pop::new())?;
        Ok(())
    }

    fn func_decl(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        self.consume(TokenType::IDENTIFIER)?;
        let id = self.previous.borrow().as_ref().unwrap().clone();

        // mark the new fun as init
        let scope = self
            .compiler
            .borrow_mut()
            .add_local(format!("{}", id), true);
        self.compiler.borrow().mark_latest_init();

        // function decl semantics
        let mut func = Compiler::compile(
            self.scanner.src_vec_from_current(),
            FunctionType::Function(format!("{}", id), self.scanner.line().number as u32),
            self.compiler.borrow().globals(),
            Some(*self.compiler.borrow()),
            self.compiler.borrow().upvalues.clone(),
        )?;

        // skip over function
        let mut arity: usize = 0;
        self.consume(TokenType::LEFT_PAREN)?;

        if !self.check(TokenType::RIGHT_PAREN) {
            loop {
                self.advance()?;
                arity += 1;
                if !self.match_(TokenType::COMMA)? {
                    break;
                }
            }
        }
        self.consume(TokenType::RIGHT_PAREN)?;
        self.consume(TokenType::LEFT_BRACE)?;
        self.escape_scope()?;

        func.set_arity(arity);

        // push fun instructions
        self.push(Constant::new(Value::Func(Rc::new(func))))?;
        self.push(Define::new(scope, format!("{}", id)))?;

        Ok(())
    }

    fn return_(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        if !self.check(TokenType::SEMICOLON) {
            self.expression()?;
        }
        self.consume(TokenType::SEMICOLON)?;

        self.push(Return::new())?;
        Ok(())
    }

    fn class_decl(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        self.consume(TokenType::IDENTIFIER)?;
        let id = self.previous.borrow().as_ref().unwrap().clone();

        let scope = self
            .compiler
            .borrow_mut()
            .add_local(format!("{}", id), true);
        self.compiler.borrow().mark_latest_init();

        self.push(Constant::new(Value::Class(Rc::new(Class::new(format!(
            "{}",
            id
        ))))))?;
        self.push(Define::new(scope, format!("{}", id)))?;
        Ok(())
    }

    fn statement(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        if self.match_(TokenType::PRINT)? {
            return self.print();
        }
        if self.match_(TokenType::LEFT_BRACE)? {
            self.start_scope();
            let res = self.block();
            self.end_scope()?;
            return res;
        }

        self.expr_stmt()
    }

    fn declaration(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        if self.match_(TokenType::VAR)? {
            return self.var_decl(false);
        }
        if self.match_(TokenType::CONST)? {
            return self.var_decl(true);
        }
        if self.match_(TokenType::IF)? {
            return self.if_stmt();
        }
        if self.match_(TokenType::WHILE)? {
            return self.while_stmt();
        }
        if self.match_(TokenType::FOR)? {
            return self.for_stmt();
        }
        if self.match_(TokenType::FUN)? {
            return self.func_decl();
        }
        if self.match_(TokenType::RETURN)? {
            return self.return_();
        }
        if self.match_(TokenType::CLASS)? {
            return self.class_decl();
        }
        self.statement()
    }

    pub fn parse(&'a self) -> Result<(), Box<dyn ErrTrait>> {
        let compiler_type = self.compiler.borrow().type_.clone();
        match compiler_type {
            FunctionType::Function(_, _) => return self.function(),
            FunctionType::Script => loop {
                if self.scanner.is_at_end() {
                    break;
                }
                self.declaration()?;
            },
        }
        Ok(())
    }
}

impl<'a> Display for Parser<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
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
",
            self.scanner,
            self.current.borrow(),
            self.previous.borrow(),
            self.chunk.borrow()
        )
    }
}
