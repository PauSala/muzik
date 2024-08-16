#[derive(Debug, Clone, PartialEq, Eq)]

/// Duration of a note
pub enum Duration {
    Whole,
    Half,
    Quarter,
    Eight,
}

/// Rest duration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rest {
    Whole,
    Half,
    Quarter,
    Eight,
}

/// Token types for the comping generator
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Chord(String),
    Rest(Rest),
    Duration(Duration),
    Eof,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Token {
        Token { token_type }
    }
}
