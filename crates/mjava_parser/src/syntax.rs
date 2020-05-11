use mjava_scanner::TokenKind;
use core::fmt;

use crate::syntax::SyntaxKind::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxKind {
    GOAL,
    EOF,
    MAIN_CLASS,
    CLASS_DECLARATION,
    METHOD_DECLARATION,
    VAR_DECLARATION,
    RETURN_KW,
    ELSE_KW,
    EXTEND_KW,
    TYPE_INT,
    TYPE_BOOLEAN,
    TYPE_IDENT(String),
    TYPE_INT_ARRAY,
    INT(String),
    IDENT(String),
    TRUE_KW,
    FALSE_KW,
    STAR,
    PLUS,
    MINUS,
    AMP,
    L_ANGLE,
    LENGTH_EXPRESSION,
    QUOTE_EXPRESSION,
    NEW_INT_EXPRESSION,
    NEW_CLASS_EXPRESION,
    EXCL_EXPRESSION,
    LBRACK_EXPRESSION,
    LPAREN_EXPRESSION,
    IF_STATE,
    WHILE_STATE,
    ASSIGN_STATE,
    PRINT_STATE,
    BLOCK_STATE,
    ERROR_STATE(String),
    THIS_KW,
}


impl fmt::Display for SyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GOAL => write!(f, "GOAL"),
            EOF => write!(f, "EOF"),
            MAIN_CLASS => write!(f, "EOF"),
            CLASS_DECLARATION => write!(f, "CLASS_DECLARATION"),
            METHOD_DECLARATION => { write!(f, "METHOD_DECLARATION") }
            VAR_DECLARATION => { write!(f, "VAR_DECLARATION") }
            LENGTH_EXPRESSION => write!(f, "LENGTH_EXPRESSION `length`"),
            QUOTE_EXPRESSION => write!(f, " QUOTE_EXPRESSION `.`"),
            NEW_INT_EXPRESSION => write!(f, " NEW_INT_EXPRESSION `new`"),
            NEW_CLASS_EXPRESION => write!(f, " NEW_OBJECT_EXPRESSION `new`"),
            EXCL_EXPRESSION => write!(f, " EXCL_EXPRESSION `!`"),
            LBRACK_EXPRESSION => write!(f, "LBRACK_EXPRESSION `[`"),
            LPAREN_EXPRESSION => write!(f, "LPAREN_EXPRESSION `(`"),
            STAR => { write!(f, "OP `*`") }
            PLUS => write!(f, "OP `+`"),
            MINUS => write!(f, "OP `-`"),
            AMP => write!(f, "OP `&&`"),
            L_ANGLE => write!(f, "OP `<`"),

            IF_STATE => write!(f, "IF_STATEMENT `if`"),
            WHILE_STATE => write!(f, "WHILE_STATEMENT `while`"),
            ASSIGN_STATE => write!(f, "ASSIGN_STATEMENT `=`"),
            PRINT_STATE => write!(f, "PRINT_STATEMENT `System.out.println`"),
            BLOCK_STATE => write!(f, "BLOCK_STATEMENT `{{`"),

            RETURN_KW => write!(f, "RETURN_SUB_STATEMENT `return`"),
            ELSE_KW => write!(f, "ELSE_SUB_STATEMENT `else`"),
            EXTEND_KW => write!(f, "EXTEND_SUB_STATEMENT `extend`"),

            TYPE_INT => write!(f, "INT_TYPE"),
            TYPE_BOOLEAN => write!(f, "BOOLEAN_TYPE"),
            TYPE_IDENT(s) => write!(f, "{}_TYPE", s),
            TYPE_INT_ARRAY => write!(f, "INT_ARRAY_TYPE"),

            INT(int) => write!(f, "Integer :{}", int),
            IDENT(id) => write!(f, "Identifier :{}", id),
            TRUE_KW => write!(f, "TRUE_KW `true`"),
            FALSE_KW => write!(f, "FALSE_KW `false`"),
            THIS_KW=>write!(f,"THIS_KW `this`"),
            t => {panic!("no such {:?}",t)}
            
        }
    }
}

