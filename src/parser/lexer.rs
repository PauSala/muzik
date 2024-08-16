use super::tokens::{Duration, Rest, Token, TokenType};
use std::{iter::Peekable, str::Chars};

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    current: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            source: String::new(),
            tokens: Vec::new(),
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn set_source(&mut self, source: &str) {
        source.clone_into(&mut self.source);
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type));
    }

    fn advance(&mut self, chars: &mut Peekable<Chars>) -> Option<char> {
        self.current += 1;
        chars.next()
    }

    fn is_chord_character(&self, c: &char) -> bool {
        *c != '\\' && *c != '.' && *c != '_' && *c != ',' && *c != '|'
    }

    pub fn scan_tokens(&mut self, source: &str) -> Vec<Token> {
        self.set_source(source);
        let source = self.source.clone();
        let mut iter = source.chars().peekable();
        while !self.is_at_end() {
            self.scan_token(&mut iter);
        }
        self.add_token(TokenType::Eof);
        let res = self.tokens.clone();
        self.source = String::new();
        self.tokens.clear();
        self.current = 0;
        res
    }

    fn scan_token(&mut self, chars: &mut Peekable<Chars>) {
        let c = self.advance(chars);
        match c {
            None => (),
            Some(c) => match c {
                '\\' => self.add_token(TokenType::Duration(Duration::Quarter)),
                '_' => self.add_token(TokenType::Rest(Rest::QuarterRest)),
                ',' => self.add_token(TokenType::Duration(Duration::Eight)),
                '.' => self.add_token(TokenType::Rest(Rest::EightRest)),
                '|' => (),
                _ => {
                    let mut chord = String::from(c);
                    let p = chars.peek();
                    let mut cond = p.is_some() && self.is_chord_character(p.unwrap());
                    while cond {
                        let c = self.advance(chars).unwrap();
                        chord.push(c);
                        let p = chars.peek();
                        cond = p.is_some() && self.is_chord_character(p.unwrap());
                    }
                    self.add_token(TokenType::Chord(chord))
                }
            },
        }
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}
