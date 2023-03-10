use std::fmt::{Debug, Display};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    CONST,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LEFT_PAREN => write!(f, "{}", "("),
            TokenType::RIGHT_PAREN => write!(f, "{}", ")"),
            TokenType::LEFT_BRACE => write!(f, "{}", "{"),
            TokenType::RIGHT_BRACE => write!(f, "{}", "}"),
            TokenType::COMMA => write!(f, "{}", ","),
            TokenType::DOT => write!(f, "{}", "."),
            TokenType::MINUS => write!(f, "{}", "-"),
            TokenType::PLUS => write!(f, "{}", "+"),
            TokenType::SEMICOLON => write!(f, "{}", ";"),
            TokenType::SLASH => write!(f, "{}", "/"),
            TokenType::STAR => write!(f, "{}", "*"),

            // One or two character tokens.
            TokenType::BANG => write!(f, "{}", "!"),
            TokenType::BANG_EQUAL => write!(f, "{}", "!="),
            TokenType::EQUAL => write!(f, "{}", "="),
            TokenType::EQUAL_EQUAL => write!(f, "{}", "=="),
            TokenType::GREATER => write!(f, "{}", ">"),
            TokenType::GREATER_EQUAL => write!(f, "{}", ">="),
            TokenType::LESS => write!(f, "{}", "<"),
            TokenType::LESS_EQUAL => write!(f, "{}", "<="),

            // Literals.
            TokenType::IDENTIFIER => write!(f, "{}", "<var>"),
            TokenType::STRING => write!(f, "{}", "<string>"),
            TokenType::NUMBER => write!(f, "{}", "<number>"),

            // Keywords.
            TokenType::AND => write!(f, "{}", "and"),
            TokenType::CLASS => write!(f, "{}", "class"),
            TokenType::CONST => write!(f, "{}", "const"),
            TokenType::ELSE => write!(f, "{}", "else"),
            TokenType::FALSE => write!(f, "{}", "false"),
            TokenType::FUN => write!(f, "{}", "fun"),
            TokenType::FOR => write!(f, "{}", "for"),
            TokenType::IF => write!(f, "{}", "if"),
            TokenType::NIL => write!(f, "{}", "nil"),
            TokenType::OR => write!(f, "{}", "or"),
            TokenType::PRINT => write!(f, "{}", "print"),
            TokenType::RETURN => write!(f, "{}", "return"),
            TokenType::SUPER => write!(f, "{}", "super"),
            TokenType::THIS => write!(f, "{}", "this"),
            TokenType::TRUE => write!(f, "{}", "true"),
            TokenType::VAR => write!(f, "{}", "var"),
            TokenType::WHILE => write!(f, "{}", "while"),

            TokenType::EOF => write!(f, "{}", "eof"),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Token<'a> {
    pub(super) token_type: TokenType,
    pub literal: &'a [u8],
    pub line: u32,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, literal: &'a [u8], line: u32) -> Self {
        Token {
            token_type,
            literal,
            line,
        }
    }
}

impl<'a> Debug for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: ... {:?} {}...",
            self.line,
            self.token_type,
            String::from_utf8_lossy(self.literal)
        )
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.literal))
    }
}
