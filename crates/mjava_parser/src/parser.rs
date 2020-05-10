//FIXME ADD MORE PANIC
use crate::cursor::Lexer;
use crate::tree_node::TreeNode;
use mjava_scanner::{Token, TokenKind, TokenKind::*};
use std::cell::{RefCell};
use std::panic;
use std::rc::Rc;
use std::borrow::BorrowMut;
use crate::syntax::SyntaxKind;

macro_rules! match_token {
    ($v:expr, $p:pat) => (
        if let $p = $v { true } else { panic!("you are ,shout be, missing") }
    );
}

// THIS PART IS A MACRO FOR POWER-BINDING
fn infix_binding_power(p: TokenKind) -> Option<(u8, u8)> {
    match p {
        TokenKind::AMP | TokenKind::L_ANGLE => Some((1u8, 2u8)),
        TokenKind::PLUS | TokenKind::MINUS => Some((3u8, 4u8)),
        TokenKind::STAR => Some((5u8, 6u8)),
        TokenKind::DOT => Some((10u8, 9u8)),
        _ => None,
    }
}

fn prefix_binding_power(p: TokenKind) -> ((), u8) {
    match p {
        TokenKind::EXCL => ((), 7u8),
        _ => panic!("bad token")
    }
}

fn postfix_binding_power(p: TokenKind) -> Option<(u8, ())> {
    match p {
        TokenKind::L_BRACK => Some((8u8,())),
        _ => None
    }
}
pub fn parser(tokens: Vec<Token>) -> Option<Box<TreeNode>> {
    let mut lexer = Lexer::new(tokens);
    build_goal(&mut lexer)
}
pub fn token2syntax(lexer:&mut Lexer)->SyntaxKind{
    match lexer.next_token(){
        TRUE_KW=>SyntaxKind::TRUE_KW,
        FALSE_KW=>SyntaxKind::FALSE_KW,
        ELSE_KW=>SyntaxKind::ELSE_KW,
        RETURN_KW=>SyntaxKind::RETURN_KW,
        EXTENDS_KW=>SyntaxKind::EXTEND_KW,
        STAR=>SyntaxKind::STAR,
        PLUS=>SyntaxKind::PLUS,
        MINUS=>SyntaxKind::MINUS,
        AMP=>SyntaxKind::AMP,
        L_ANGLE=>SyntaxKind::L_ANGLE,
        LENGTH_KW=>SyntaxKind::LENGTH_EXPRESSION,
        EXCL=>SyntaxKind::EXCL_EXPRESSION,
        IF_KW=>SyntaxKind::IF_STATE,
        WHILE_KW=>SyntaxKind::WHILE_STATE,
        EQ=>SyntaxKind::ASSIGN_STATE,
        SYSTEM_KW=>SyntaxKind::PRINT_STATE,
        L_CURLY=>SyntaxKind::BLOCK_STATE,
        L_PAREN=>SyntaxKind::LPAREN_EXPRESSION,
        L_BRACK=>SyntaxKind::LBRACK_EXPRESSION,
        _=>panic!("No such token!")
    }

}

fn build_goal(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    let mut root = Box::new(TreeNode::new(SyntaxKind::GOAL));
    root.add_child(build_main_class(lexer));
    match lexer.next_token() {
        L_CURLY => loop {
            if let TokenKind::R_CURLY = lexer.nth_token(1) {
                lexer.next_token();
                break;
            }
            root.add_child(build_classes(lexer));
        },
        _ => panic!("Missing `left Brack`"),
    }
    match_token!(lexer.next_token(),EOF );
    Some(root)
}

fn build_main_class(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    let mut root = Box::new(TreeNode::new(SyntaxKind::MAIN_CLASS));
    match_token!(lexer.next_token(),CLASS_KW);
    match lexer.next_token() {
        IDENT(id) => {
            root
                .add_syntax_child(SyntaxKind::IDENT(id));
        }
        _ => { panic!("you are {:?},missing Ident token", lexer.get_prev()) }
    }
    match_token!(lexer.next_token(),L_CURLY);
    match_token!(lexer.next_token(),PUBLIC_KW);
    match_token!(lexer.next_token(),STATIC_KW);
    match_token!(lexer.next_token(),VOID_KW);
    match_token!(lexer.next_token(),MAIN_KW);
    match_token!(lexer.next_token(),L_PAREN);
    match_token!(lexer.next_token(),STRING_KW);
    match_token!(lexer.next_token(),L_BRACK);
    match_token!(lexer.next_token(),R_BRACK);
    match_token!(lexer.nth_token(1),IDENT(..));
    root
        .add_syntax_child(atom_token_kind(lexer));
    match_token!(lexer.next_token(),R_PAREN);
    match_token!(lexer.next_token(),L_CURLY);
    root.add_child(build_statement(lexer));
    match_token!(lexer.next_token(),R_CURLY);
    Some(root)
}

//FIXME 简化class,考虑有Ｎone
fn build_classes(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    if let TokenKind::CLASS_KW = lexer.nth_token(1) {
        let mut root = build_class(lexer);
        loop {
            let class_node = build_class(lexer);
            if class_node.is_none() {
                break;
            }
            &root.as_ref().unwrap().add_sibling(class_node);
        }
        root
    } else {
        None
    }
}

fn build_class(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    if let TokenKind::CLASS_KW = lexer.nth_token(1) {
        lexer.next_token();
        let mut root = Box::new(TreeNode::new(SyntaxKind::CLASS_DECLARATION));
        match_token!(lexer.nth_token(1),IDENT(..));
        root
            .add_syntax_child(atom_token_kind(lexer));

        //for extend part
        match_token!(lexer.nth_token(1),EXTENDS_KW);
        if let EXTENDS_KW = lexer.nth_token(1) {
            let mut extends_root = Box::new(TreeNode::new(atom_token_kind(
                lexer,
            )));
            match_token!(lexer.nth_token(1),IDENT(..));
            extends_root
                
                .add_syntax_child(atom_token_kind(lexer));
            root.add_child(Some(extends_root));
        }
        match_token!(lexer.next_token(),L_CURLY);
        root.add_child(build_vardeclarations(lexer));
        root.add_child(build_methods(lexer));
        match_token!(lexer.next_token(),R_CURLY);
        Some(root)
    } else {
        None
    }
}

fn build_vardeclarations(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    let mut root = build_vardeclaration(lexer);
    if root.is_some() {
        loop {
            let node = build_vardeclaration(lexer);
            if node.is_none() {
                break;
            }
            &root.as_ref().unwrap().add_sibling(node);
        }
    }
    root
}

fn build_methods(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    let mut root = build_method(lexer);
    if root.is_some() {
        loop {
            let node = build_method(lexer);
            if node.is_none() {
                break;
            }
            &root.as_ref().unwrap().add_sibling(node);
        }
    }
    root
}

fn build_vardeclaration(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    let root = match lexer.nth_token(1) {
        BOOLEAN_KW | INT_KW => Some(Box::new(TreeNode::new(
            SyntaxKind::VAR_DECLARATION,
        ))),
        IDENT(_id) => Some(Box::new(TreeNode::new(
            SyntaxKind::VAR_DECLARATION,
        ))),
        _ => None,
    };
    if root.is_none() {
        return root;
    }
    
    ptr.as_ref()
        .unwrap()
        .add_syntax_child(type_token_kind(lexer));

    match_token!(lexer.nth_token(1),IDENT(..));
    ptr.as_ref()
        .unwrap()
        
        .add_syntax_child(atom_token_kind(lexer));
    match_token!(lexer.next_token(),SEMI);
    root
}

fn build_method(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    if let PUBLIC_KW = lexer.nth_token(1) {
        lexer.next_token();
        let mut root = Box::new(TreeNode::new(SyntaxKind::METHOD_DECLARATION));
        root.add_child(build_param(lexer));
        match_token!(lexer.next_token(),L_PAREN);
        root.add_child(build_params(lexer));
        match_token!(lexer.next_token(),R_PAREN);
        match_token!(lexer.next_token(),L_CURLY);
        root.add_child(build_statement(lexer));
        root.add_child(build_return(lexer));
        match_token!(lexer.next_token(),SEMI);
        match_token!(lexer.next_token(),R_CURLY);
        Some(root)
    } else {
        None
    }
}

fn build_params(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    let mut root = build_param(lexer);
    if root.is_none() {
        return None;
    } else {
        loop {
            let node = build_param(lexer);
            if node.is_none() {
                break;
            }
            &root.as_ref().unwrap().add_sibling(node);
        }
        return root;
    }
}

fn build_param(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    let root = match lexer.nth_token(1) {
        BOOLEAN_KW | INT_KW => Some(Box::new(TreeNode::new(token2syntax(lexer)))),
        IDENT(_id) => Some(Box::new(TreeNode::new(token2syntax(lexer)))),
        _ => None,
    };
    if root.is_none() {
        return root;
    }
    match_token!(lexer.nth_token(1),IDENT(..));
    &root
        .as_ref()
        .unwrap()
        
        .add_syntax_child(atom_token_kind(lexer));
    root
}

fn build_return(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    match_token!(lexer.nth_token(1),RETURN);
    let mut root = Box::new(TreeNode::new(token2syntax(
        lexer
    )));
    root.add_child(build_expression(lexer));
    Some(root)
}

fn build_statement(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    match lexer.nth_token(1) {
        L_CURLY => build_block_statement(lexer),
        IF_KW => build_if_statement(lexer),
        WHILE_KW => build_while_statement(lexer),
        SYSTEM_KW => build_print_statement(lexer),
        IDENT(_id) => build_assign_statement(lexer),
        _ => None,
    }
}

fn build_block_statement(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    match_token!(lexer.nth_token(1),L_CURLY);
    let mut root = Box::new(TreeNode::new(token2syntax(lexer)));
    root.add_child(build_statement(lexer));
    match_token!(lexer.next_token(),R_CURLY);
    Some(root)
}

fn build_if_statement(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    match_token!(lexer.nth_token(1),IF_KW);
    let mut root = Box::new(TreeNode::new(token2syntax(lexer)));
    match_token!(lexer.next_token(),L_PAREN);
    let mut expression_node = build_expression(lexer);
    &expression_node
        .as_ref()
        .unwrap()
        
        .add_child(build_statement(lexer));
    root.add_child(expression_node);
    match_token!(lexer.next_token(),R_PAREN);
    match_token!(lexer.nth_token(1),ELSE_KW);
    let mut else_node = Some(Box::new(TreeNode::new(token2syntax(
        lexer,
    ))));
    &else_node
        .as_ref()
        .unwrap()
        
        .add_child(build_statement(lexer));
    root.add_child(else_node);
    Some(root)
}

fn build_while_statement(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    match_token!(lexer.nth_token(1),WHILE_KW);
    let mut root = Box::new(TreeNode::new(token2syntax(
        lexer,
    )));
    match_token!(lexer.next_token(),L_PAREN);
    root.add_child(build_expression(lexer));
    match_token!(lexer.next_token(),R_PAREN);
    root.add_child(build_statement(lexer));
    Some(root)
}

fn build_print_statement(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    match_token!(lexer.nth_token(1),SYSTEM_KW);
    let mut root = Box::new(TreeNode::new(token2syntax(
        lexer,
    )));

    match_token!(lexer.next_token(),L_PAREN);
    root.add_child(build_expression(lexer));
    match_token!(lexer.next_token(),R_PAREN);
    match_token!(lexer.next_token(),SEMI);
    Some(root)
}

//FIXME WHIE IS SYSTEM_KW
fn build_assign_statement(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    match_token!(lexer.nth_token(1),IDENT(..));
    let id = Box::new(TreeNode::new(token2syntax(
        lexer,
    )));
    let expression_1 = match lexer.nth_token(1) {
        L_BRACK => {
            match_token!(lexer.next_token(),L_PAREN);
            let tmp = build_expression(lexer);
            match_token!(lexer.next_token(),R_PAREN);
            tmp
        }
        _ => None,
    };
    match_token!(lexer.nth_token(1),EQ);
    let mut root = Box::new(TreeNode::new(token2syntax(lexer)));
    let expression_2 = build_expression(lexer);
    root.add_child(Some(id));
    root.add_child(expression_1);
    root.add_child(expression_2);
    match_token!(lexer.next_token(),SEMI);
    Some(root)
}

//for Expression part
//This part using pratt parsing
fn build_expression(lexer: &mut Lexer) -> Option<Box<TreeNode>> {
    expression_bp(lexer, 0u8)
}

fn expression_bp(lexer: &mut Lexer, min_bp: u8) -> Option<Box<TreeNode>> {
    let mut lhs = match lexer.nth_token(1) {
        //FOR ATOM NODE
        TRUE_KW  => {
            Some(Box::new(TreeNode::new(token2syntax(lexer))))
        }
        FALSE_KW=>{
            Some(Box::new(TreeNode::new(token2syntax(lexer))))
        }
        IDENT(_id) => {
            Some(Box::new(TreeNode::new(token2syntax(lexer))))
        }
        INTER(int) => {
            Some(Box::new(TreeNode::new(token2syntax(lexer))))
        }

        //FOR TWO NEW INT EXPRESSION PART
        NEW_KW => {
            lexer.next_token();
            match lexer.next_token() {
                IDENT(id) => {
                    let mut tmp = Box::new(TreeNode::new(SyntaxKind::NEW_CLASS_EXPRESION));
                    tmp.add_child(Some(Box::new(TreeNode::new(SyntaxKind::IDENT(id)))));
                    match_token!(lexer.next_token(),L_PAREN);
                    match_token!(lexer.next_token(),R_PAREN);
                    Some(tmp)
                }
                INT_KW => {
                    let mut tmp = Box::new(TreeNode::new(SyntaxKind::NEW_INT_EXPRESSION));
                    match_token!(lexer.next_token(),L_BRACK);
                    tmp.add_child(expression_bp(lexer, 0));
                    match_token!(lexer.next_token(),R_BRACK);
                    Some(tmp)
                }
                _ => panic!("Error")
            }
        }
        AMP | L_ANGLE | PLUS | MINUS | EXCL | STAR | L_BRACK | DOT => {
            let kind = lexer.next_token();
            let ((), r_bp) = prefix_binding_power(kind);
            let mut tmp = Box::new(TreeNode::new(SyntaxKind::EXCL_EXPRESSION));
            tmp.add_child(expression_bp(lexer, r_bp));
            Some(tmp)
        }
        L_PAREN => {
            let mut mhs = Box::new(TreeNode::new(SyntaxKind::LPAREN_EXPRESSION));
            let rhs = expression_bp(lexer, 0u8);
            mhs.add_child(rhs);
            Some(mhs)
        }
        _ => { None }
    };
    if lhs.is_none() {
        return None;
    }
    loop {
        let op_token = match lexer.nth_token(1) {
            EOF => break,
            AMP | L_ANGLE | PLUS | MINUS | EXCL | STAR | L_BRACK | L_PAREN | DOT => lexer.nth_token(1),
            _ => { panic!("") }
        };
        if let Some((l_bp, ())) = postfix_binding_power(op_token.clone()) {
            if l_bp < min_bp {
                break;
            }
            lexer.next_token();
            let mut mhs = Box::new(TreeNode::new(SyntaxKind::LBRACK_EXPRESSION));
            match_token!(lexer.next_token(),R_BRACK);
            let rhs = expression_bp(lexer, l_bp);
            mhs.add_child(lhs);
            mhs.add_child(rhs);
            lhs = Some(mhs);
        }
        if let Some((l_bp, r_bp)) = infix_binding_power(op_token) {
            if l_bp < min_bp {
                break;
            }
            if let DOT = lexer.nth_token(1) {
                lexer.next_token();
                match lexer.next_token() {
                    IDENT(id) => {
                        let mut mhs = Box::new(TreeNode::new(SyntaxKind::QUOTE_EXPRESSION));
                        mhs.add_child(lhs);
                        match_token!(lexer.next_token(),L_PAREN);
                        //FIXME 改变c_bp的值
                        let c_bp = 8u8;
                        let rhs = expression_bp(lexer, c_bp);
                        match_token!(lexer.next_token(),R_PAREN);
                        mhs.add_child(rhs);
                        lhs = Some(mhs);
                    }
                    LENGTH_KW => {
                        let mut mhs =Box::new(TreeNode::new(SyntaxKind::LENGTH_EXPRESSION));
                        mhs.add_child(lhs);
                        lhs = Some(mhs);
                    }
                    _ => { panic!("missing token!") }
                }
            } else if let COMA = lexer.nth_token(1) {
                lexer.next_token();
                let sibling = expression_bp(lexer, r_bp);
                &lhs.as_ref().unwrap().add_sibling(sibling);
            } else {
                let rhs = expression_bp(lexer, r_bp);
                let mut mhs = Box::new(TreeNode::new(token2syntax(lexer)));
                mhs.add_child(lhs);
                mhs.add_child(rhs);
                lhs = Some(mhs);
            }
        }
    }
    lhs
}


fn atom_token_kind(lexer: &mut Lexer) -> SyntaxKind {
    token2syntax(lexer)
}

fn type_token_kind(lexer: &mut Lexer) -> SyntaxKind {
    match lexer.next_token() {
        INT_KW => {
            if let L_BRACK = lexer.nth_token(1) {
                lexer.next_token();
                match_token!(lexer.next_token(), R_BRACK);
                SyntaxKind::TYPE_INT_ARRAY
            } else {
                SyntaxKind::TYPE_INT
            }
        }
        BOOLEAN_KW => SyntaxKind::TYPE_BOOLEAN,
        IDENT(id) => SyntaxKind::TYPE_IDENT(id),
        _ => panic!("missing type"),
    }
}
