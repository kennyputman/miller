use crate::token::{self, Token};

pub fn scan_tokens(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current = 0;
    let mut start = 0;

    while current <= source.len() - 1 {
        start = current;
        let char = source.chars().nth(current).unwrap();
        current += 1;

        match char {
            '(' => tokens.push(Token {
                token: token::TokenType::LeftBrace,
            }),
            _ => tokens.push(Token {
                token: token::TokenType::And,
            }),
        }
    }

    tokens
}
