#[macro_use]
pub mod scanner;

/**
 * This is the Token enum
 *
 */
#[derive(Debug)]
pub enum Token {
    IntegerLitera(String),
    Identifier(String),
    ReservedWord(String),
    Symbol(String),
}

/**
 * This is the TokenError Struct
 */
#[derive(Debug)]
pub struct TokenError {
    pub value: String,
}
