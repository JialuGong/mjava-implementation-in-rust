//FIXME ADD MORE PANIC
use crate::cursor::Lexer;
use crate::syntax::{SyntaxKind, TypeKind, StatementKind, OpType, ExpressionKind, BridgeKind};

use crate::tree_node::TreeNode;
use mjava_scanner::{Token, TokenKind, TokenKind::*};
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;


macro_rules! match_token {
    ($v:expr, $p:pat) => (
        if let $p = $v { true } else { panic!("you are ,shout be, missing") }
    );
}

// THIS PART IS A MACRO FOR POWER-BINDING
macro_rules! infix_binding_power {
    ($p:pat) => {
        match $p {
          TokenKind::AMP||TokenKind::L_ANGEL=>(1u8,2u8),
          TokenKind::PLUS|TokenKind::MINUS=>(3u8,4u8),
          TokenKind::STAR=>(5u8,6u8),
           TokenKind::DOT=>(8u8,7u8),
          _=>panic!("Not op token!"),
        }
    };
}
macro_rules! prefix_binding_power{
  ($p:pat)=>{
    match $p{
      TokenKind::EXCL=>((),5u8),
      _=>panic!("bad token")
    }
  }
}

macro_rules! postfix_binding_power {
  ($p:pat) => {
    match $p{
      TokenKind::L_PAREN=>((),7U8),
      TokenKind::L_BRACK=>((),7U8)
    }

  };
}
macro_rules! token2syntax {
    ($v:expr) => (
      match $v{
            TokenKind::IDENT(id) => SyntaxKind::IDENT(id),
            TokenKind::INTER(int) => SyntaxKind::INT(int),
            TokenKind::EXTENDS_KW => SyntaxKind::BRIDGE(BridgeKind::EXTEND_KW),
            // TokenKind::INT_KW => SyntaxKind::TYPE(TypeKind::INT_KW),
            // TokenKind::BOOLEAN_KW => SyntaxKind::TYPE(TypeKind::BOOLEAN_KW),
            TokenKind::RETURN_KW => SyntaxKind::BRIDGE(BridgeKind::RETURN_KW),
            TokenKind::IF_KW => SyntaxKind::STATEMENT(StatementKind::IF_STATE),
            TokenKind::ELSE_KW => SyntaxKind::BRIDGE(BridgeKind::EXTEND_KW),
            TokenKind::WHILE_KW => SyntaxKind::STATEMENT(StatementKind::WHILE_STATE),
            TokenKind::SYSTEM_KW => SyntaxKind::STATEMENT(StatementKind::PRINT_STATE),
            TokenKind::LENGTH_KW => SyntaxKind::EXPRESSION(ExpressionKind::LENGTH_EXPRESSION),
            //FIXME: SOLVE NEW PROBLEM
            // TokenKind::NEW_KW => SyntaxKind::EXPRESSION,
            TokenKind::EQ => SyntaxKind::STATEMENT(StatementKind::ASSIGN_STATE),
            TokenKind::PLUS => SyntaxKind::EXPRESSION(ExpressionKind::OP_EXPRESSION(OpType::PLUS)),
            TokenKind::L_ANGLE => SyntaxKind::EXPRESSION(ExpressionKind::OP_EXPRESSION(OpType::L_ANGLE)),
            TokenKind::MINUS => SyntaxKind::EXPRESSION(ExpressionKind::OP_EXPRESSION(OpType::MINUS)),
            TokenKind::STAR => SyntaxKind::EXPRESSION(ExpressionKind::OP_EXPRESSION(OpType::STAR)),
            TokenKind::AMP => SyntaxKind::EXPRESSION(ExpressionKind::OP_EXPRESSION(OpType::AMP)),
            TokenKind::DOT => SyntaxKind::EXPRESSION(ExpressionKind::QUOTE_EXPRESSION),
            TokenKind::EXCL => SyntaxKind::EXPRESSION(ExpressionKind::EXCL_EXPRESSION),
            TokenKind::L_CURLY => SyntaxKind::STATEMENT(StatementKind::BLOCK_STATE),
            _ => { panic!("token2syntax no such token") }
      }
    );
}

pub fn parser(tokens: Vec<Token>) -> Option<Rc<RefCell<TreeNode>>> {

    let mut lexer = Lexer::new(tokens);
    build_goal(&mut lexer)
}

fn build_goal(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = Rc::new(RefCell::new(TreeNode::new(SyntaxKind::GOAL)));
    root.borrow_mut().add_child(build_main_class(lexer));
    match lexer.next_token() {
        L_CURLY => loop {
            if let TokenKind::R_CURLY = lexer.nth_token(1) {
                lexer.next_token();
                break;
            }
            root.borrow_mut().add_child(build_classes(lexer));
        },
        _ => panic!("Missing `left Brack`"),
    }
    match_token!(lexer.next_token(),EOF );
    Some(root)
}

fn build_main_class(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = Rc::new(RefCell::new(TreeNode::new(SyntaxKind::MAIN_CLASS)));
    match_token!(lexer.next_token(),CLASS_KW);
    match lexer.next_token() {
        IDENT(id) => {
            root.borrow_mut()
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
    root.borrow_mut()
        .add_syntax_child(atom_token_kind(lexer));
    match_token!(lexer.next_token(),R_PAREN);
    match_token!(lexer.next_token(),L_CURLY);
    root.borrow_mut().add_child(build_statement(lexer));
    match_token!(lexer.next_token(),R_CURLY);
    Some(root)
}

//FIXME 简化class,考虑有Ｎone
fn build_classes(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    if let TokenKind::CLASS_KW = lexer.nth_token(1) {
        let mut root = build_class(lexer);
        loop {
            let class_node = build_class(lexer);
            if class_node.is_none() {
                break;
            }
            &root.as_ref().unwrap().borrow_mut().add_sibling(class_node);
        }
        root
    } else {
        None
    }
}

fn build_class(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    if let TokenKind::CLASS_KW = lexer.nth_token(1) {
        lexer.next_token();
        let mut root = Rc::new(RefCell::new(TreeNode::new(SyntaxKind::CLASS_DECLARATION)));
        match_token!(lexer.nth_token(1),IDENT(..));
        root.borrow_mut()
            .add_syntax_child(atom_token_kind(lexer));

        //for extend part
        match_token!(lexer.nth_token(1),EXTENDS_KW);
        if let EXTENDS_KW = lexer.nth_token(1) {
            let mut extends_root = Rc::new(RefCell::new(TreeNode::new(atom_token_kind(
                lexer,
            ))));
            match_token!(lexer.nth_token(1),IDENT(..));
            extends_root
                .borrow_mut()
                .add_syntax_child(atom_token_kind(lexer));
            root.borrow_mut().add_child(Some(extends_root));
        }
        match_token!(lexer.next_token(),L_CURLY);
        root.borrow_mut().add_child(build_vardeclarations(lexer));
        root.borrow_mut().add_child(build_methods(lexer));
        match_token!(lexer.next_token(),R_CURLY);
        Some(root)
    } else {
        None
    }
}

fn build_vardeclarations(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = build_vardeclaration(lexer);
    if root.is_some() {
        loop {
            let node = build_vardeclaration(lexer);
            if node.is_none() {
                break;
            }
            &root.as_ref().unwrap().borrow_mut().add_sibling(node);
        }
    }
    root
}

fn build_methods(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = build_method(lexer);
    if root.is_some() {
        loop {
            let node = build_method(lexer);
            if node.is_none() {
                break;
            }
            &root.as_ref().unwrap().borrow_mut().add_sibling(node);
        }
    }
    root
}

fn build_vardeclaration(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let root = match lexer.nth_token(1) {
        BOOLEAN_KW | INT_KW => Some(Rc::new(RefCell::new(TreeNode::new(
            SyntaxKind::VAR_DECLARATION,
        )))),
        IDENT(_id) => Some(Rc::new(RefCell::new(TreeNode::new(
            SyntaxKind::VAR_DECLARATION,
        )))),
        _ => None,
    };
    if root.is_none() {
        return root;
    }
    let ptr = &root;
    ptr.as_ref()
        .unwrap()
        .borrow_mut()
        .add_syntax_child(type_token_kind(lexer));

    match_token!(lexer.nth_token(1),IDENT(..));
    ptr.as_ref()
        .unwrap()
        .borrow_mut()
        .add_syntax_child(atom_token_kind(lexer));
    match_token!(lexer.next_token(),SEMI);
    root
}

fn build_method(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    if let PUBLIC_KW = lexer.nth_token(1) {
        lexer.next_token();
        let mut root = Rc::new(RefCell::new(TreeNode::new(SyntaxKind::METHOD_DECLARATION)));
        root.borrow_mut().add_child(build_param(lexer));
        match_token!(lexer.next_token(),L_PAREN);
        root.borrow_mut().add_child(build_params(lexer));
        match_token!(lexer.next_token(),R_PAREN);
        match_token!(lexer.next_token(),L_CURLY);
        root.borrow_mut().add_child(build_statement(lexer));
        root.borrow_mut().add_child(build_return(lexer));
        match_token!(lexer.next_token(),SEMI);
        match_token!(lexer.next_token(),R_CURLY);
        Some(root)
    } else {
        None
    }
}

fn build_params(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = build_param(lexer);
    if root.is_none() {
        return None;
    } else {
        loop {
            let node = build_param(lexer);
            if node.is_none() {
                break;
            }
            &root.as_ref().unwrap().borrow_mut().add_sibling(node);
        }
        return root;
    }
}

fn build_param(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let root = match lexer.nth_token(1) {
        BOOLEAN_KW | INT_KW => Some(Rc::new(RefCell::new(TreeNode::new(type_token_kind(lexer))))),
        IDENT(_id) => Some(Rc::new(RefCell::new(TreeNode::new(type_token_kind(lexer))))),
        _ => None,
    };
    if root.is_none() {
        return root;
    }
    match_token!(lexer.nth_token(1),IDENT(..));
    &root
        .as_ref()
        .unwrap()
        .borrow_mut()
        .add_syntax_child(atom_token_kind(lexer));
    root
}

fn build_return(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    match_token!(lexer.nth_token(1),RETURN);
    let mut root = Rc::new(RefCell::new(TreeNode::new(atom_token_kind(
        lexer,
    ))));
    root.borrow_mut().add_child(build_expression(lexer));
    Some(root)
}

fn build_statement(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    match lexer.nth_token(1) {
        L_CURLY => build_block_statement(lexer),
        IF_KW => build_if_statement(lexer),
        WHILE_KW => build_while_statement(lexer),
        SYSTEM_KW => build_print_statement(lexer),
        IDENT(_id) => build_assign_statement(lexer),
        _ => None,
    }
}

fn build_block_statement(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    match_token!(lexer.nth_token(1),L_CURLY);
    let mut root = Rc::new(RefCell::new(TreeNode::new(atom_token_kind(lexer))));
    root.borrow_mut().add_child(build_statement(lexer));
    match_token!(lexer.next_token(),R_CURLY);
    Some(root)
}

fn build_if_statement(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    match_token!(lexer.nth_token(1),IF_KW);
    let mut root = Rc::new(RefCell::new(TreeNode::new(atom_token_kind(lexer))));
    match_token!(lexer.next_token(),L_PAREN);
    let mut expression_node = build_expression(lexer);
    &expression_node
        .as_ref()
        .unwrap()
        .borrow_mut()
        .add_child(build_statement(lexer));
    root.borrow_mut().add_child(expression_node);
    match_token!(lexer.next_token(),R_PAREN);
    match_token!(lexer.nth_token(1),ELSE_KW);
    let mut else_node = Some(Rc::new(RefCell::new(TreeNode::new(atom_token_kind(
        lexer,
    )))));
    &else_node
        .as_ref()
        .unwrap()
        .borrow_mut()
        .add_child(build_statement(lexer));
    root.borrow_mut().add_child(else_node);
    Some(root)
}

fn build_while_statement(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    match_token!(lexer.nth_token(1),WHILE_KW);
    let mut root = Rc::new(RefCell::new(TreeNode::new(atom_token_kind(
        lexer,
    ))));
    match_token!(lexer.next_token(),L_PAREN);
    root.borrow_mut().add_child(build_expression(lexer));
    match_token!(lexer.next_token(),R_PAREN);
    root.borrow_mut().add_child(build_statement(lexer));
    Some(root)
}

fn build_print_statement(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    match_token!(lexer.nth_token(1),SYSTEM_KW);
    let mut root = Rc::new(RefCell::new(TreeNode::new(atom_token_kind(
        lexer,
    ))));

    match_token!(lexer.next_token(),L_PAREN);
    root.borrow_mut().add_child(build_expression(lexer));
    match_token!(lexer.next_token(),R_PAREN);
    match_token!(lexer.next_token(),SEMI);
    Some(root)
}

//FIXME WHIE IS SYSTEM_KW
fn build_assign_statement(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    match_token!(lexer.nth_token(1),IDENT(..));
    let id = Rc::new(RefCell::new(TreeNode::new(atom_token_kind(
        lexer,
    ))));
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
    let mut root = Rc::new(RefCell::new(TreeNode::new(atom_token_kind(lexer))));
    let expression_2 = build_expression(lexer);
    root.borrow_mut().add_child(Some(id));
    root.borrow_mut().add_child(expression_1);
    root.borrow_mut().add_child(expression_2);
    match_token!(lexer.next_token(),SEMI);
    Some(root)
}

fn build_expression(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    //TODO
}


fn atom_token_kind(lexer: &mut Lexer) -> SyntaxKind {
    token2syntax!(lexer.next_token())
}

fn type_token_kind(lexer: &mut Lexer) -> SyntaxKind {
    match lexer.next_token() {
        INT_KW => {
            if let L_BRACK = lexer.nth_token(1) {
                lexer.next_token();
                match_token!(lexer.next_token(), R_BRACK);
                SyntaxKind::TYPE(TypeKind::INT_ARRAY)
            } else {
                SyntaxKind::TYPE(TypeKind::INT)
            }
        }
        BOOLEAN_KW => SyntaxKind::TYPE(TypeKind::BOOLEAN),
        IDENT(id) => SyntaxKind::TYPE(TypeKind::IDENT(id)),
        _ => panic!("missing type"),
    }
}
