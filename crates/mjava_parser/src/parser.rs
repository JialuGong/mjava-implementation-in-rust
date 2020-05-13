//FIXME ADD MORE PANIC
use crate::cursor::Lexer;
use crate::syntax::SyntaxKind;
use crate::tree_node::TreeNode;
use mjava_scanner::{Token, TokenKind, TokenKind::*};
use std::cell::RefCell;
use std::rc::Rc;

///# macro eat_node
/// consuming the next and
macro_rules! eat_node {
    ($b:ident,$l:ident,$k:pat,$ke:expr,$root:expr) => {
        if let $k = $l.next_token() {
            let prev = $l.get_prev();
            match get_syntax_node(prev) {
                Some(syntax_kind) => {
                    $root
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .add_child(Some(Rc::new(RefCell::new(TreeNode::new(syntax_kind)))));
                }
                None => {}
            }
        } else {
            let prev = $l.get_prev();
            let err = match prev {
                IDENT(id) => format!(
                    "In {} part,Need [{}],You are [Identifier `{}`]",
                    $b, $ke, id
                ),
                INTER(int) => format!("In {} part,Need [{}],You are [Integer `{}`]", $b, $ke, int),
                _ => format!("In {} part,Need [{}],You are [{}]", $b, $ke, prev),
            };
            $root
                .as_ref()
                .unwrap()
                .borrow_mut()
                .add_child(Some(Rc::new(RefCell::new(TreeNode::new(
                    SyntaxKind::ERROR_STATE(err),
                )))));
        }
    };
}

fn get_syntax_node(token: TokenKind) -> Option<SyntaxKind> {
    match token {
        IDENT(id) => Some(SyntaxKind::IDENT(id)),
        INTER(int) => Some(SyntaxKind::INT(int)),
        THIS_KW => Some(SyntaxKind::THIS_KW),
        TRUE_KW => Some(SyntaxKind::TRUE_KW),
        FALSE_KW => Some(SyntaxKind::FALSE_KW),
        // ELSE_KW => Some(SyntaxKind::ELSE_KW),
        RETURN_KW => Some(SyntaxKind::RETURN_KW),
        EXTENDS_KW => Some(SyntaxKind::EXTEND_KW),
        STAR => Some(SyntaxKind::STAR),
        PLUS => Some(SyntaxKind::PLUS),
        MINUS => Some(SyntaxKind::MINUS),
        AMP => Some(SyntaxKind::AMP),
        L_ANGLE => Some(SyntaxKind::L_ANGLE),
        LENGTH_KW => Some(SyntaxKind::LENGTH_EXPRESSION),
        EXCL => Some(SyntaxKind::EXCL_EXPRESSION),
        IF_KW => Some(SyntaxKind::IF_STATE),
        WHILE_KW => Some(SyntaxKind::WHILE_STATE),
        // EQ => Some(SyntaxKind::ASSIGN_STATE),
        SYSTEM_KW => Some(SyntaxKind::PRINT_STATE),
        L_CURLY => Some(SyntaxKind::BLOCK_STATE),
        L_PAREN => Some(SyntaxKind::LPAREN_EXPRESSION),
        L_BRACK => Some(SyntaxKind::LBRACK_EXPRESSION),
        _ => None,
    }
}

fn infix_binding_power(p: TokenKind) -> Option<(u8, u8)> {
    match p {
        TokenKind::AMP | TokenKind::L_ANGLE => Some((3u8, 4u8)),
        TokenKind::PLUS | TokenKind::MINUS => Some((5u8, 6u8)),
        TokenKind::STAR => Some((7u8, 8u8)),
        TokenKind::DOT => Some((12u8, 11u8)),
        TokenKind::COMMA => Some((1u8, 2u8)),
        _ => None,
    }
}

fn prefix_binding_power(p: TokenKind) -> Option<((), u8)> {
    match p {
        TokenKind::EXCL => Some(((), 9u8)),
        _ => None,
    }
}

fn postfix_binding_power(p: TokenKind) -> Option<(u8, ())> {
    match p {
        TokenKind::L_BRACK => Some((10u8, ())),
        _ => None,
    }
}
pub fn parser(tokens: Vec<Token>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut lexer = Lexer::new(tokens);
    build_goal(&mut lexer)
}
pub fn token2syntax(lexer: &mut Lexer) -> SyntaxKind {
    match lexer.next_token() {
        IDENT(id) => SyntaxKind::IDENT(id),
        INTER(int) => SyntaxKind::INT(int),
        THIS_KW => SyntaxKind::THIS_KW,
        TRUE_KW => SyntaxKind::TRUE_KW,
        FALSE_KW => SyntaxKind::FALSE_KW,
        ELSE_KW => SyntaxKind::ELSE_KW,
        RETURN_KW => SyntaxKind::RETURN_KW,
        EXTENDS_KW => SyntaxKind::EXTEND_KW,
        STAR => SyntaxKind::STAR,
        PLUS => SyntaxKind::PLUS,
        MINUS => SyntaxKind::MINUS,
        AMP => SyntaxKind::AMP,
        L_ANGLE => SyntaxKind::L_ANGLE,
        LENGTH_KW => SyntaxKind::LENGTH_EXPRESSION,
        EXCL => SyntaxKind::EXCL_EXPRESSION,
        IF_KW => SyntaxKind::IF_STATE,
        WHILE_KW => SyntaxKind::WHILE_STATE,
        EQ => SyntaxKind::ASSIGN_STATE,
        SYSTEM_KW => SyntaxKind::PRINT_STATE,
        L_CURLY => SyntaxKind::BLOCK_STATE,
        L_PAREN => SyntaxKind::LPAREN_EXPRESSION,
        L_BRACK => SyntaxKind::LBRACK_EXPRESSION,
        t => {
            println! {"No such token"};
            SyntaxKind::ERROR_STATE(format!("wrong {:?}", t))
        }
    }
}

fn build_goal(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let block = "Goal";
    let root = Rc::new(RefCell::new(TreeNode::new(SyntaxKind::GOAL)));

    root.borrow_mut().add_child(build_main_class(lexer));
    match lexer.nth_token(1) {
        CLASS_KW => loop {
            let class_node = build_class(lexer);
            &root.borrow_mut().add_child(class_node.clone());
            if class_node.is_none() || TreeNode::check_bp(&class_node) {
                break;
            }
        },
        _ => {
            root.borrow_mut()
                .add_syntax_child(SyntaxKind::ERROR_STATE(format!("No Class Declaration!")));
        }
    }
    if let EOF = lexer.next_token() {
    } else {
        let err = format!("In goal part,missing EOF ,you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
    }
    Some(root)
}

fn build_main_class(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(SyntaxKind::MAIN_CLASS))));
    let block = "MainClass";
    let id = String::from(" ");
    let a = format!("{}", CLASS_KW);
    eat_node!(block, lexer, CLASS_KW, CLASS_KW, &root);
    eat_node!(block, lexer, IDENT(..), IDENT(id.clone()), &root);
    eat_node!(block, lexer, L_CURLY, L_CURLY, &root);
    eat_node!(block, lexer, PUBLIC_KW, PUBLIC_KW, &root);
    eat_node!(block, lexer, STATIC_KW, STATIC_KW, &root);
    eat_node!(block, lexer, VOID_KW, VOID_KW, &root);
    eat_node!(block, lexer, MAIN_KW, MAIN_KW, &root);
    eat_node!(block, lexer, L_PAREN, L_PAREN, &root);
    eat_node!(block, lexer, STRING_KW, STRING_KW, &root);
    eat_node!(block, lexer, L_BRACK, L_BRACK, &root);
    eat_node!(block, lexer, R_BRACK, R_BRACK, &root);
    eat_node!(block, lexer, IDENT(..), IDENT(id.clone()), &root);
    eat_node!(block, lexer, R_PAREN, R_PAREN, &root);
    eat_node!(block, lexer, L_CURLY, L_CURLY, &root);
    //build declartions
    loop {
        let node = build_vardeclaration(lexer);
        &root.as_ref().unwrap().borrow_mut().add_child(node.clone());
        if node.is_none() || TreeNode::check_bp(&node) {
            break;
        }
    }
    //build statement
    loop {
        let node = build_statement(lexer);
        root.as_ref().unwrap().borrow_mut().add_child(node.clone());
        if node.is_none() || TreeNode::check_bp(&node) {
            break;
        }
    }
    for _i in 0..2 {
        eat_node!(block, lexer, R_CURLY, R_CURLY, &root);
    }
    root
}

fn build_class(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let block = "Class";
    let id = "".to_string();
    if let TokenKind::CLASS_KW = lexer.nth_token(1) {
        lexer.next_token();
        let root = Some(Rc::new(RefCell::new(TreeNode::new(
            SyntaxKind::CLASS_DECLARATION,
        ))));
        eat_node!(block, lexer, IDENT(..), IDENT(id.clone()), &root);
        //for extend part
        if let EXTENDS_KW = lexer.nth_token(1) {
            let extends_root = Some(Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer)))));
            eat_node!(block, lexer, IDENT(..), IDENT(id.clone()), &extends_root);
            &root.as_ref().unwrap().borrow_mut().add_child(extends_root);
        }
        eat_node!(block, lexer, L_CURLY, L_CURLY, &root);
        //build declartions
        loop {
            let node = build_vardeclaration(lexer);
            &root.as_ref().unwrap().borrow_mut().add_child(node.clone());
            if node.is_none() || TreeNode::check_bp(&node) {
                break;
            }
        }
        //build methods
        loop {
            let node = build_method(lexer);
            &root.as_ref().unwrap().borrow_mut().add_child(node.clone());
            if node.is_none() || TreeNode::check_bp(&node) {
                break;
            }
        }
        eat_node!(block, lexer, R_CURLY, R_CURLY, &root);
        root
    } else {
        None
    }
}

//TODO change for the macro version
fn build_vardeclaration(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let root = match lexer.nth_token(1) {
        BOOLEAN_KW => {
            if let SEMI = lexer.nth_token(3) {
                lexer.next_token();
                Some(Rc::new(RefCell::new(TreeNode::new(
                    SyntaxKind::TYPE_BOOLEAN,
                ))))
            } else {
                None
            }
        }
        INT_KW => {
            lexer.next_token();
            if let L_BRACK = lexer.nth_token(1) {
                lexer.next_token();
                if let R_BRACK = lexer.next_token() {
                    Some(Rc::new(RefCell::new(TreeNode::new(
                        SyntaxKind::TYPE_INT_ARRAY,
                    ))))
                } else {
                    let err = format!("in VarDeclaration,missing ],you are {}", lexer.get_prev());
                    return Some(Rc::new(RefCell::new(TreeNode::new(
                        SyntaxKind::ERROR_STATE(err),
                    ))));
                }
            } else {
                Some(Rc::new(RefCell::new(TreeNode::new(SyntaxKind::TYPE_INT))))
            }
        }
        IDENT(id) => {
            match lexer.nth_token(2) {
                L_BRACK | EQ => {
                    return None;
                }
                _ => {}
            }
            lexer.next_token();
            Some(Rc::new(RefCell::new(TreeNode::new(
                SyntaxKind::TYPE_IDENT(id),
            ))))
        }
        _ => None,
    };

    if root.is_none() {
        return root;
    }

    if let IDENT(id) = lexer.next_token() {
        &root
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_syntax_child(SyntaxKind::IDENT(id));
    } else {
        let err = format!(
            "in VarDeclaration,missing ident,you are {}",
            lexer.get_prev()
        );
        &root
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
        return root;
    }

    if let SEMI = lexer.next_token() {
    } else {
        let err = format!(
            "in VarDeclaration,missing SEMIN,you are {};",
            lexer.get_prev()
        );
        &root
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
        return root;
    }
    root
}

fn build_method(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let block = "Method";
    if let PUBLIC_KW = lexer.nth_token(1) {
        lexer.next_token();
        let root = Some(Rc::new(RefCell::new(TreeNode::new(
            SyntaxKind::METHOD_DECLARATION,
        ))));
        root.as_ref()
            .unwrap()
            .borrow_mut()
            .add_child(build_param(lexer));
        eat_node!(block, lexer, L_PAREN, L_PAREN, &root);
        let mut node = build_param(lexer);
        loop {
            root.as_ref().unwrap().borrow_mut().add_child(node.clone());
            if node.is_some() && !TreeNode::check_bp(&node) {
                if let COMMA = lexer.nth_token(1) {
                    lexer.next_token();
                    node = build_param(lexer);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        eat_node!(block, lexer, R_PAREN, R_PAREN, &root);
        eat_node!(block, lexer, L_CURLY, L_CURLY, &root);

        //build declartions
        loop {
            let node = build_vardeclaration(lexer);
            root.as_ref().unwrap().borrow_mut().add_child(node.clone());
            if node.is_none() || TreeNode::check_bp(&node) {
                break;
            }
        }

        //build statement
        loop {
            let node = build_statement(lexer);
            root.as_ref().unwrap().borrow_mut().add_child(node.clone());
            if node.is_none() || TreeNode::check_bp(&node) {
                break;
            }
        }
        if let RETURN_KW = lexer.nth_token(1) {
            &root
                .as_ref()
                .unwrap()
                .borrow_mut()
                .add_child(build_return(lexer));
        }
        eat_node!(block, lexer, R_CURLY, R_CURLY, &root);
        root
    } else {
        None
    }
}

fn build_param(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let block = "Method params";
    let root = match lexer.nth_token(1) {
        BOOLEAN_KW => {
            lexer.next_token();
            Some(Rc::new(RefCell::new(TreeNode::new(
                SyntaxKind::TYPE_BOOLEAN,
            ))))
        }
        INT_KW => {
            lexer.next_token();
            if let L_BRACK = lexer.nth_token(1) {
                lexer.next_token();
                let tmp = Some(Rc::new(RefCell::new(TreeNode::new(
                    SyntaxKind::TYPE_INT_ARRAY,
                ))));
                eat_node!(block, lexer, R_BRACK, R_BRACK, &tmp);
                tmp
            } else {
                Some(Rc::new(RefCell::new(TreeNode::new(SyntaxKind::TYPE_INT))))
            }
        }
        IDENT(id) => {
            lexer.next_token();
            Some(Rc::new(RefCell::new(TreeNode::new(
                SyntaxKind::TYPE_IDENT(id),
            ))))
        }
        _ => return None,
    };
    eat_node!(block, lexer, IDENT(..), IDENT(String::new()), &root);
    root
}

fn build_return(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    lexer.next_token();
    let block = "Method Return";
    let root = Some(Rc::new(RefCell::new(TreeNode::new(SyntaxKind::RETURN_KW))));
    &root
        .as_ref()
        .unwrap()
        .borrow_mut()
        .add_child(build_expression(lexer));
    eat_node!(block, lexer, SEMI, SEMI, &root);
    root
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
    let block = "Block statement";
    let root = Some(Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer)))));
    loop {
        let node = match lexer.nth_token(1) {
            IF_KW | WHILE_KW | SYSTEM_KW => build_statement(lexer),
            IDENT(_id) => build_statement(lexer),
            _ => break,
        };
        &root.as_ref().unwrap().borrow_mut().add_child(node.clone());
        if node.is_none() || TreeNode::check_bp(&node) {
            break;
        }
    }
    eat_node!(block, lexer, R_CURLY, R_CURLY, &root);
    root
}

fn build_if_statement(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let block = "If state";
    let root = Some(Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer)))));
    eat_node!(block, lexer, L_PAREN, L_PAREN, &root);
    let expression_node = build_expression(lexer);
    eat_node!(block, lexer, R_PAREN, R_PAREN, &root);
    &expression_node
        .as_ref()
        .unwrap()
        .borrow_mut()
        .add_child(build_statement(lexer));
    &root
        .as_ref()
        .unwrap()
        .borrow_mut()
        .add_child(expression_node);
    let else_node = Some(Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer)))));
    &else_node
        .as_ref()
        .unwrap()
        .borrow_mut()
        .add_child(build_statement(lexer));
    &root.as_ref().unwrap().borrow_mut().add_child(else_node);
    root
}

fn build_while_statement(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let block = "While statement";
    let root = Some(Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer)))));
    eat_node!(block, lexer, L_PAREN, L_PAREN, &root);
    &root
        .as_ref()
        .unwrap()
        .borrow_mut()
        .add_child(build_expression(lexer));
    eat_node!(block, lexer, R_PAREN, R_PAREN, &root);
    root.as_ref()
        .unwrap()
        .borrow_mut()
        .add_child(build_statement(lexer));
    root
}

fn build_print_statement(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let block = "Print statment";
    let root = Some(Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer)))));
    eat_node!(block, lexer, L_PAREN, L_PAREN, &root);
    &root
        .as_ref()
        .unwrap()
        .borrow_mut()
        .add_child(build_expression(lexer));
    eat_node!(block, lexer, R_PAREN, R_PAREN, &root);
    eat_node!(block, lexer, SEMI, SEMI, &root);
    root
}

//FIXME WHIE IS SYSTEM_KW
fn build_assign_statement(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let block = "Assign statement";

    let ident = match lexer.next_token() {
        IDENT(id) => SyntaxKind::IDENT(id),
        _ => SyntaxKind::ERROR_STATE(format!(
            "In assign statement ,missing ident,you are {}",
            lexer.get_prev()
        )),
    };
    let expression_1 = match lexer.nth_token(1) {
        L_BRACK => {
            lexer.next_token();
            let tmp = build_expression(lexer);
            eat_node!(block, lexer, R_BRACK, R_BRACK, &tmp);
            tmp
        }
        _ => None,
    };
    let root = if let EQ = lexer.next_token() {
        Some(Rc::new(RefCell::new(TreeNode::new(
            SyntaxKind::ASSIGN_STATE,
        ))))
    } else {
        let err = format!("In {},missing [=],you are {}", block, lexer.get_prev());
        Some(Rc::new(RefCell::new(TreeNode::new(
            SyntaxKind::ERROR_STATE(err),
        ))))
    };
    let expression_2 = build_expression(lexer);
    &root.as_ref().unwrap().borrow_mut().add_syntax_child(ident);
    root.as_ref().unwrap().borrow_mut().add_child(expression_1);
    root.as_ref().unwrap().borrow_mut().add_child(expression_2);
    eat_node!(block, lexer, SEMI, SEMI, &root);

    root
}

//for Expression part
//This part using pratt parsing
fn build_expression(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let ans = expression_bp(lexer, 0u8);
    println!("come_hear");
    ans
}

fn expression_bp(lexer: &mut Lexer, min_bp: u8) -> Option<Rc<RefCell<TreeNode>>> {
    let mut lhs = match lexer.nth_token(1) {
        //FOR ATOM NODE
        TRUE_KW | FALSE_KW => Some(Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer))))),
        THIS_KW => Some(Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer))))),
        IDENT(id) => Some(Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer))))),
        INTER(int) => Some(Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer))))),

        //FOR TWO NEW INT EXPRESSION PART
        NEW_KW => {
            let block = "New expression";
            lexer.next_token();
            match lexer.next_token() {
                IDENT(id) => {
                    let tmp = Some(Rc::new(RefCell::new(TreeNode::new(
                        SyntaxKind::NEW_CLASS_EXPRESION,
                    ))));
                    &tmp.as_ref()
                        .unwrap()
                        .borrow_mut()
                        .add_child(Some(Rc::new(RefCell::new(TreeNode::new(
                            SyntaxKind::IDENT(id),
                        )))));
                    eat_node!(block, lexer, L_PAREN, L_PAREN, &tmp);
                    eat_node!(block, lexer, R_PAREN, R_BRACK, &tmp);
                    tmp
                }
                INT_KW => {
                    let tmp = Some(Rc::new(RefCell::new(TreeNode::new(SyntaxKind::NEW_INT_EXPRESSION))));
                    eat_node!(block, lexer, L_BRACK, L_BRACK, &tmp);  
                    &tmp.as_ref().unwrap().borrow_mut().add_child(expression_bp(lexer, 0));
                    eat_node!(block, lexer, R_BRACK, R_BRACK, &tmp);
                    tmp
                }
                _ => {
                    let err = format!(
                        "In Expression,missing int_kw or ident,you are {} ",
                        lexer.get_prev()
                    );
                    Some(Rc::new(RefCell::new(TreeNode::new(
                        SyntaxKind::ERROR_STATE(err),
                    ))))
                }
            }
        }

        AMP | L_ANGLE | PLUS | MINUS | EXCL | STAR | L_BRACK | DOT => {
            let kind = lexer.next_token();
            if let Some(((), r_bp)) = prefix_binding_power(kind) {
                let tmp = Rc::new(RefCell::new(TreeNode::new(SyntaxKind::EXCL_EXPRESSION)));
                tmp.borrow_mut().add_child(expression_bp(lexer, r_bp));
                Some(tmp)
            } else {
                let err = format!("In expression,missing !,you are {}", lexer.get_prev());
                Some(Rc::new(RefCell::new(TreeNode::new(
                    SyntaxKind::ERROR_STATE(err),
                ))))
            }
        }
        L_PAREN => {
            lexer.next_token();
            let mhs = Rc::new(RefCell::new(TreeNode::new(SyntaxKind::LPAREN_EXPRESSION)));
            let rhs = expression_bp(lexer, 0u8);
            if let R_PAREN = lexer.next_token() {
                mhs.borrow_mut().add_child(rhs);
                Some(mhs)
            } else {
                let err = format!("In expression,missing ),you are {}", lexer.get_prev());
                Some(Rc::new(RefCell::new(TreeNode::new(
                    SyntaxKind::ERROR_STATE(err),
                ))))
            }
        }
        _ => None,
    };
    if lhs.is_none() {
        return None;
    }
    loop {
        let op_token = match lexer.nth_token(1) {
            AMP | L_ANGLE | PLUS | MINUS | EXCL | STAR | L_BRACK | L_PAREN | DOT => {
                lexer.nth_token(1)
            }
            _ => break,
        };

        if let Some((l_bp, ())) = postfix_binding_power(op_token.clone()) {
            if l_bp < min_bp {
                break;
            }
            lexer.next_token();
            let mhs = Rc::new(RefCell::new(TreeNode::new(SyntaxKind::LBRACK_EXPRESSION)));
            if let R_BRACK = lexer.next_token() {
            } else {
                let err = format!("In Epression,missing [R_BRACK]");
                return Some(Rc::new(RefCell::new(TreeNode::new(
                    SyntaxKind::ERROR_STATE(err),
                ))));
            }
            let rhs = expression_bp(lexer, l_bp);
            mhs.borrow_mut().add_child(lhs);
            mhs.borrow_mut().add_child(rhs);
            lhs = Some(mhs);
            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(op_token) {
            if l_bp < min_bp {
                break;
            }
            if let DOT = lexer.nth_token(1) {
                let block="Quete Expression";
                lexer.next_token();
                match lexer.next_token() {
                    IDENT(id) => {
                        let mhs =
                            Rc::new(RefCell::new(TreeNode::new(SyntaxKind::QUOTE_EXPRESSION)));
                        mhs.borrow_mut().add_child(lhs.clone());
                        mhs.borrow_mut().add_syntax_child(SyntaxKind::IDENT(id));
                        if let L_PAREN = lexer.next_token() {
                        } else {
                            let err =
                                format!("INEXPRESION,missing ( ,you are {}", lexer.get_prev());
                            mhs.borrow_mut()
                                .add_syntax_child(SyntaxKind::ERROR_STATE(err));
                            break;
                        }
                        // let c_bp = 10u8;
                        let rhs = expression_bp(lexer, 0);
                        mhs.borrow_mut().add_child(rhs);
                        loop {
                            if let TokenKind::COMMA = lexer.nth_token(1) {
                                lexer.next_token();
                                let child = expression_bp(lexer, r_bp);
                                mhs.borrow_mut().add_child(child.clone());
                                if child.is_none() || TreeNode::check_bp(&child) {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        if let R_PAREN = lexer.next_token() {
                        } else {
                            let err =
                                format!("INEXPRESION,missing ) ,you are {}", lexer.get_prev());
                            mhs.borrow_mut()
                                .add_syntax_child(SyntaxKind::ERROR_STATE(err));
                            break;
                        }
                        lhs = Some(mhs);
                    }
                    LENGTH_KW => {
                        let mhs =
                            Rc::new(RefCell::new(TreeNode::new(SyntaxKind::LENGTH_EXPRESSION)));
                        mhs.borrow_mut().add_child(lhs);
                        lhs = Some(mhs);
                    }
                    _ => {
                        let err = format!(
                            "In expression qute|length,missing [ident]|[LENGTH_KW],you are {}",
                            lexer.get_prev()
                        );
                        lhs = Some(Rc::new(RefCell::new(TreeNode::new(
                            SyntaxKind::ERROR_STATE(err),
                        ))));
                    }
                }
            } else {
                let mhs = Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer))));
                let rhs = expression_bp(lexer, r_bp);
                mhs.borrow_mut().add_child(lhs);
                mhs.borrow_mut().add_child(rhs);
                lhs = Some(mhs);
            }
            continue;
        }
        break;
    }
    lhs
}
