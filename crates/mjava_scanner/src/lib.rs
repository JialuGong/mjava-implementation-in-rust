#[macro_use]
pub mod cursor;

pub struct Token {
    kind: TokenKind,
    length: usize,
}
pub struct TokenError {
    length: usize,
    line: usize,
}
impl Token {
    pub fn new(kind: TokenKind, length: usize) -> Token {
        Token { kind, length }
    }
}

impl TokenError {
    pub fn new(length: usize, line: usize) -> TokenError {
        TokenError { length, line }
    }
}

pub enum TokenKind {
    CLASS_KW,
    PUBLIC_KW,
    STATIC_KW,
    VOID_KW,
    MAIN_KW,
    STRING_KW,
    EXTEND_KW,
    RETURN_KW,
    INT_KW,
    BOOLEAN_KW,
    IF_KW,
    ELSE_KW,
    WHILE_KW,
    LENGTH_KW,
    TRUE_KW,
    FALSE_KW,
    THIS_KW,
    NEW_KW,
    SYSTEM_KW,
    L_BRACK,
    R_BRACK,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    COMMA,
    SEMI,
    EQ,
    L_ANGLE,
    PLUS,
    MINUS,
    STAR,
    EXCL,
    AMP,
    IDENT,
    INTER,
    BLANK_BLOCK,
    UNKNOWN,
    KROWN_ID,
}

use crate::cursor::{Cursor, EOF_CHAR};
use crate::TokenKind::*;

pub fn first_token(input: &str) -> Result<Token, TokenError> {
    debug_assert!(!input.is_empty());
    Cursor::new(input).advance_token()
}

pub fn tokenize(mut input: &str) -> impl Iterator<Item = Result<Token, TokenError>> + '_ {
    std::iter::from_fn(move || {
        if input.is_empty() {
            return None;
        }
        let token = first_token(input);
        let len = match &token {
            Ok(o) => o.length,
            Err(e) => e.length,
        };
        input = &input[len..];
        Some(token)
    })
}

impl Cursor<'_> {
    pub fn advance_token(&mut self) -> Result<Token, TokenError> {
        let first_char = self.next_char().unwrap();
        let kind = match first_char {
            //blank block
            ' ' | '\t' => self.blank_block(),
            'S' => self.system_block(),

            //id block contain KW
            c if self.is_id_start(c) => self.id_block(),

            //number block
            '0'..='9' => self.number_block(),

            //sybols
            '[' => L_BRACK,
            ']' => R_BRACK,
            '(' => L_PAREN,
            ')' => R_PAREN,
            '{' => L_CURLY,
            '}' => R_CURLY,
            ',' => COMMA,
            ';' => SEMI,
            '=' => EQ,
            '<' => L_ANGLE,
            '+' => PLUS,
            '-' => MINUS,
            '*' => STAR,
            '!' => EXCL,
            //"&&"
            '&' => match self.first_char() {
                '&' => {
                    self.next_char();
                    AMP
                }
                _ => UNKNOWN,
            },
            _ => UNKNOWN,
        };
        //TODO SOLVE LINE PROBLEM
        match kind {
            UNKNOWN => Err(TokenError::new(self.consum(), self.line())),
            _ => Ok(Token::new(kind, self.consum())),
        }
    }

    fn blank_block(&mut self) -> TokenKind {
        if self.prev()=='\n' {self.line_plus()}
        loop {
            if self.is_black_continue(self.first_char()) {
                if self.first_char()=='\n'{
                    self.line_plus();
                }
                self.next_char();
            } else {
                break;
            }
        }
        BLANK_BLOCK
    }
    fn system_block(&mut self) -> TokenKind {
        //first match system.out.println
        let mut new_string = String::new();
        for i in 0.. {
            let ch = self.nth_char(i);
            if self.is_system_continue(ch) {
                new_string.push(ch)
            } else {
                break;
            }
        }
        if new_string == "ystem.out.println" {
            //consume `ystem.out.println`
            for _i in 0..17 {
                self.next_char();
            }
            SYSTEM_KW
        } else {
            self.id_block()
        }
    }

    fn id_block(&mut self) -> TokenKind {
        let mut id_str = String::new();
        id_str.push(self.prev());
        loop {
            if self.is_id_continue(self.first_char()) {
                self.next_char();
            } else {
                break;
            }
        }
        if id_str.ends_with("_") {
            KROWN_ID
        } else {
            self.keyword_block(&id_str)
        }
    }
    fn number_block(&mut self) -> TokenKind {
        loop {
            if self.is_number_continue(self.first_char()) {
                self.next_char();
            } else {
                break;
            }
        }
        INTER
    }
    fn keyword_block(&self, s: &String) -> TokenKind {
        match &**s {
            "class" => CLASS_KW,
            "public" => PUBLIC_KW,
            "static" => STATIC_KW,
            "void" => VOID_KW,
            "main" => MAIN_KW,
            "String" => STATIC_KW,
            "extend" => EXTEND_KW,
            "return" => RETURN_KW,
            "int" => INT_KW,
            "boolean" => BOOLEAN_KW,
            "if" => IF_KW,
            "else" => ELSE_KW,
            "while" => WHILE_KW,
            "length" => LENGTH_KW,
            "true" => TRUE_KW,
            "false" => FALSE_KW,
            "new" => NEW_KW,
            _ => IDENT,
        }
    }
    fn is_id_start(&self, c: char) -> bool {
        match c {
            'A'..='Z' => true,
            'a'..='z'=>true,
            _ => false,
        }
    }
    fn is_id_continue(&self, c: char) -> bool {
        match c {
            '_' => true,
            'A'..='Z' => true,
            'a'..='z'=>true,
            '0'..='9' => true,
            _ => false,
        }
    }
    fn is_system_continue(&self, c: char) -> bool {
        match c {
            'A'..='z' => true,
            '.' => true,
            _ => false,
        }
    }
    fn is_number_continue(&self, c: char) -> bool {
        if c >= '0' && c <= '9' {
            true
        } else {
            false
        }
    }
    fn is_black_continue(&self, c: char) -> bool {
        match c {
            ' ' | '\t' | '\n' | '\r' => true,
            _ => false,
        }
    }
}
