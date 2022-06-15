use crate::token::{Token, TokenType};

pub fn scan_tokens(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current = 0;
    let mut start = 0;

    while current <= source.len() - 1 {
        start = current;
        let char = source.chars().nth(current).unwrap();
        current += 1;

        if current == source.len() {
            tokens.push(Token::new(TokenType::Eof, 33));
            break;
        }

        match char {
            '(' => tokens.push(Token::new(TokenType::LeftParen, 32)),
            ')' => tokens.push(Token::new(TokenType::RightParen, 32)),
            '{' => tokens.push(Token::new(TokenType::LeftBrace, 32)),
            '}' => tokens.push(Token::new(TokenType::RightBrace, 32)),
            ';' => tokens.push(Token::new(TokenType::Semicolon, 32)),
            ',' => tokens.push(Token::new(TokenType::Comma, 32)),
            '.' => tokens.push(Token::new(TokenType::Dot, 32)),
            '-' => tokens.push(Token::new(TokenType::Minus, 32)),
            '+' => tokens.push(Token::new(TokenType::Plus, 32)),
            '*' => tokens.push(Token::new(TokenType::Star, 32)),
            _ => tokens.push(Token::new(TokenType::NotFound, 32)),
        }
    }

    tokens
}

// @@@@@ need to cleanup end of line error
fn peak(source: &String, current: &usize) -> Result<char, ()> {
    if current - 1 == source.len() {
        Err(())
    } else {
        Ok(source.chars().nth(current + 1).unwrap())
    }
}
