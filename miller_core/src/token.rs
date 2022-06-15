#[derive(Debug)]

pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    False,

    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
    // @@@ Drop once error handling
    NotFound,
}

#[derive(Debug)]
pub struct Token {
    pub token: TokenType,
    // pub lexeme: String,
    // pub literal: String,
    pub line: u32,
}

impl Token {
    pub fn new(token: TokenType, line: u32) -> Token {
        Token { token, line }
    }
}
