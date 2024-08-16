#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Duration {
    Half,
    Quarter,
    Eight,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rest {
    HalfRest,
    QuarterRest,
    EightRest,
}
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
