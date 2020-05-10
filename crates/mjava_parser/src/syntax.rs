use mjava_scanner::TokenKind;
use core::fmt;

use crate::syntax::SyntaxKind::{EXPRESSION, STATEMENT, TYPE, INT, BRIDGE};
use crate::syntax::ExpressionKind::OP_EXPRESSION;

pub enum SyntaxKind {
    GOAL,
    EOF,
    MAIN_CLASS,
    CLASS_DECLARATION,
    METHOD_DECLARATION,
    VAR_DECLARATION,
    EXPRESSION(ExpressionKind),
    STATEMENT(StatementKind),
    BRIDGE(BridgeKind),
    TYPE(TypeKind),
    INT(String),
    IDENT(String),
    TRUE_KW,
    FALSE_KW,
}

pub enum ExpressionKind {
    OP_EXPRESSION(OpType),
    LENGTH_EXPRESSION,
    QUOTE_EXPRESSION,
    NEW_INT_EXPRESSION,
    NEW_CLASS_EXPRESION,
    EXCL_EXPRESSION,
}

pub enum StatementKind {
    IF_STATE,
    WHILE_STATE,
    ASSIGN_STATE,
    PRINT_STATE,
    BLOCK_STATE,
}

pub enum OpType {
    STAR,
    PLUS,
    MINUS,
    AMP,
    L_ANGLE,
}

pub enum BridgeKind {
    RETURN_KW,
    ELSE_KW,
    EXTEND_KW,
}

pub enum TypeKind {
    INT,
    BOOLEAN,
    IDENT(String),
    INT_ARRAY,
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
            SyntaxKind::EXPRESSION(expression_kind) => {
                match expression_kind {
                    LENGTH_EXPRESSION => write!(f, "LENGTH_EXPRESSION `length`"),
                    QUOTE_EXPRESSION => write!(f, " QUOTE_EXPRESSION `.`"),
                    NEW_INT_EXPRESSION => write!(f, " NEW_INT_EXPRESSION `new`"),
                    NEW_CLASS_EXPRESION => write!(f, " NEW_OBJECT_EXPRESSION `new`"),
                    EXCL_EXPRESSION => write!(f, " EXCL_EXPRESSION `!`"),
                    OP_EXPRESSION(op_type) => {
                        match op_type {
                            STAR => { write!(f, "OP `*`") }
                            PLUS => write!(f, "OP `+`"),
                            MINUS => write!(f, "OP `-`"),
                            AMP => write!(f, "OP `&&`"),
                            L_ANGLE => write!(f, "OP `<`"),
                        }
                    }
                }
            }
            SyntaxKind::STATEMENT(statement_kind) => {
                match statement_kind {
                    IF_STATE => write!(f, "IF_STATEMENT `if`"),
                    WHILE_STATE => write!(f, "WHILE_STATEMENT `while`"),
                    ASSIGN_STATE => write!(f, "ASSIGN_STATEMENT `=`"),
                    PRINT_STATE => write!(f, "PRINT_STATEMENT `System.out.println`"),
                    BLOCK_STATE => write!(f, "BLOCK_STATEMENT `{{`"),
                }
            }
            SyntaxKind::BRIDGE(bridge_kind) => {
                match bridge_kind {
                    RETURN_KW => write!(f, "RETURN_SUB_STATEMENT `return`"),
                    ELSE_KW => write!(f, "ELSE_SUB_STATEMENT `else`"),
                    EXTEND_KW => write!(f, "EXTEND_SUB_STATEMENT `extend`"),
                }
            }
            TYPE(type_kind) => {
                match type_kind {
                    TypeKind::INT => write!(f, "INT_TYPE"),
                    TypeKind::BOOLEAN => write!(f, "BOOLEAN_TYPE"),
                    TypeKind::IDENT(s) => write!(f, "{}_TYPE", s),
                    TypeKind::INT_ARRAY => write!(f, "INT_ARRAY_TYPE"),
                }
            }
            SyntaxKind::INT(int) => write!(f, "Integer :{}", int),
            SyntaxKind::IDENT(id) => write!(f, "Identifier :{}", id),
            TRUE_KW => write!(f, "TRUE_KW `true`"),
            FALSE_KW => write!(f, "FALSE_KW `false`"),
        }
    }
}

