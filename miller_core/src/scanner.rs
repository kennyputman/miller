use crate::token::{self, Token, TokenType};

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    line: i32,
    current: usize,
    start: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            line: 1,
            current: 0,
            start: 0,
        }
    }
    pub fn scan_tokens(&mut self) {
        while self.current <= self.source.len() - 1 {
            self.start = self.current;
            let char = self.source.chars().nth(self.current).unwrap();
            self.current += 1;

            if self.current == self.source.len() {
                self.make_token(TokenType::Eof);
                break;
            }

            match char {
                '(' => self.make_token(TokenType::LeftParen),
                ')' => self.make_token(TokenType::RightParen),
                '{' => self.make_token(TokenType::LeftBrace),
                '}' => self.make_token(TokenType::RightBrace),
                ';' => self.make_token(TokenType::Semicolon),
                ',' => self.make_token(TokenType::Comma),
                '.' => self.make_token(TokenType::Dot),
                '-' => self.make_token(TokenType::Minus),
                '+' => self.make_token(TokenType::Plus),
                '*' => self.make_token(TokenType::Star),
                _ => self.make_token(TokenType::NotFound),
            }
        }
    }

    fn make_token(&mut self, token_type: TokenType) {
        let lexeme = &self.source[self.start..self.current];
        let new_token = Token::new(
            token_type,
            lexeme.to_string(),
            lexeme.to_string(),
            self.line,
        );
        self.tokens.push(new_token);
    }
    // @@@@@ need to cleanup end of line error
    fn peak(source: &String, current: &usize) -> Result<char, ()> {
        if current - 1 == source.len() {
            Err(())
        } else {
            Ok(source.chars().nth(current + 1).unwrap())
        }
    }
}
