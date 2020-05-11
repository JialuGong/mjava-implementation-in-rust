//FIXME ADD MORE PANIC
use crate::cursor::Lexer;
use crate::syntax::SyntaxKind;
use crate::tree_node::TreeNode;
use mjava_scanner::{Token, TokenKind, TokenKind::*};
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

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
    let mut root = Rc::new(RefCell::new(TreeNode::new(SyntaxKind::GOAL)));

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
    let root = Rc::new(RefCell::new(TreeNode::new(SyntaxKind::MAIN_CLASS)));
    if let CLASS_KW = lexer.next_token() {
    } else {
        let err = format!("in MAINCLASS,missing class,you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
        return Some(root);
    }
    match lexer.next_token() {
        IDENT(id) => {
            root.borrow_mut().add_syntax_child(SyntaxKind::IDENT(id));
        }
        _ => {
            let err = format!("in MAINCLASS,missing ident,you are {}", lexer.get_prev());
            root.borrow_mut()
                .add_syntax_child(SyntaxKind::ERROR_STATE(err));
            return Some(root);
        }
    }
    if let L_CURLY = lexer.next_token() {
    } else {
        let err = format!("in MAINCLASS,missing {{,you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
        return Some(root);
    }
    if let PUBLIC_KW = lexer.next_token() {
    } else {
        let err = format!(
            "in MAINCLASS,missing public_kw,you are {}",
            lexer.get_prev()
        );
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
        return Some(root);
    }
    if let STATIC_KW = lexer.next_token() {
    } else {
        let err = format!(
            "in MAINCLASS,missing static_kw,you are {}",
            lexer.get_prev()
        );
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
        return Some(root);
    }
    if let VOID_KW = lexer.next_token() {
    } else {
        let err = format!("in MAINCLASS,missing void_kw,you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
        return Some(root);
    }
    if let MAIN_KW = lexer.next_token() {
    } else {
        let err = format!("in MAINCLASS,missing main_kw,you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
        return Some(root);
    }
    if let L_PAREN = lexer.next_token() {
    } else {
        let err = format!("in MAINCLASS,missing (,you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
        return Some(root);
    }
    if let STRING_KW = lexer.next_token() {
    } else {
        let err = format!(
            "in MAINCLASS,missing String_kw,you are {}",
            lexer.get_prev()
        );
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
        return Some(root);
    }
    if let L_BRACK = lexer.next_token() {
    } else {
        let err = format!("in MAINCLASS,missing [,you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
        return Some(root);
    }
    if let R_BRACK = lexer.next_token() {
    } else {
        let err = format!("in MAINCLASS,missing ],you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
        return Some(root);
    }
    if let IDENT(id) = lexer.next_token() {
        root.borrow_mut().add_syntax_child(SyntaxKind::IDENT(id));
    } else {
        let err = format!("in MAINCLASS,missing ident,you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
        return Some(root);
    }
    if let R_PAREN = lexer.next_token() {
    } else {
        let err = format!("in MAINCLASS,missing ),you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
        return Some(root);
    }
    if let L_CURLY = lexer.next_token() {
    } else {
        let err = format!("in MAINCLASS,missing {{,you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
        return Some(root);
    }
    //build declartions
    loop {
        let node = build_vardeclaration(lexer);
        &root.borrow_mut().add_child(node.clone());
        if node.is_none() || TreeNode::check_bp(&node) {
            break;
        }
    }

    //build statement

    loop {
        let node = build_statement(lexer);
        root.borrow_mut().add_child(node.clone());
        if node.is_none() || TreeNode::check_bp(&node) {
            break;
        }
    }
    for i in 0..2 {
        if let R_CURLY = lexer.next_token() {
        } else {
            let err = format!("in MAINCLASS,missing }},you are {}", lexer.get_prev());
            root.borrow_mut()
                .add_syntax_child(SyntaxKind::ERROR_STATE(err));
            return Some(root);
        }
    }
    Some(root)
}

fn build_class(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    if let TokenKind::CLASS_KW = lexer.nth_token(1) {
        lexer.next_token();
        let mut root = Rc::new(RefCell::new(TreeNode::new(SyntaxKind::CLASS_DECLARATION)));
        if let IDENT(id) = lexer.next_token() {
            root.borrow_mut().add_syntax_child(SyntaxKind::IDENT(id));
        } else {
            let err = format!("in CLASS,missing IDENT,you are {}", lexer.get_prev());
            root.borrow_mut()
                .add_syntax_child(SyntaxKind::ERROR_STATE(err));
            return Some(root);
        };

        //for extend part
        if let EXTENDS_KW = lexer.nth_token(1) {
            let extends_root = Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer))));
            if let IDENT(id) = lexer.nth_token(1) {
                extends_root
                    .borrow_mut()
                    .add_syntax_child(SyntaxKind::IDENT(id));
            } else {
                let err = format!("in CLASS,missing IDENT,you are {}", lexer.get_prev());
                root.borrow_mut()
                    .add_syntax_child(SyntaxKind::ERROR_STATE(err));
                return Some(root);
            }
        }
        if let L_CURLY = lexer.next_token() {
        } else {
            let err = format!("in CLASS,missing IDENT,you are {}", lexer.get_prev());
            root.borrow_mut()
                .add_syntax_child(SyntaxKind::ERROR_STATE(err));
            return Some(root);
        }
        //build declartions
        loop {
            let node = build_vardeclaration(lexer);
            &root.borrow_mut().add_child(node.clone());
            if node.is_none() || TreeNode::check_bp(&node) {
                break;
            }
        }

        //build methods
        loop {
            let node = build_method(lexer);
            &root.borrow_mut().add_child(node.clone());
            if node.is_none() || TreeNode::check_bp(&node) {
                break;
            }
        }

        if let R_CURLY = lexer.next_token() {
        } else {
            let err = format!("in CLASS,missing }},you are {}", lexer.get_prev());
            root.borrow_mut()
                .add_syntax_child(SyntaxKind::ERROR_STATE(err));
            return Some(root);
        }
        Some(root)
    } else {
        None
    }
}

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
    if let PUBLIC_KW = lexer.nth_token(1) {
        lexer.next_token();
        let mut root = Rc::new(RefCell::new(TreeNode::new(SyntaxKind::METHOD_DECLARATION)));
        // let param=build_param(lexer);
        // if !param.is_none(){
        root.borrow_mut().add_child(build_param(lexer));
        // }else{
        //     let err=format!("In method parama,missing type,you are {}",lexer.get_prev());
        //     root.borrow_mut().add_syntax_child(SyntaxKind::ERROR_STATE(err));
        //     return Some(root);
        // }

        if let L_PAREN = lexer.next_token() {
        } else {
            let err = format!("in Method,missing (,you are {}", lexer.get_prev());
            root.borrow_mut()
                .add_syntax_child(SyntaxKind::ERROR_STATE(err));
            return Some(root);
        }

        let mut node = build_param(lexer);
        loop {
            root.borrow_mut().add_child(node.clone());
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

        if let R_PAREN = lexer.next_token() {
        } else {
            let err = format!("in Method,missing ),you are {}", lexer.get_prev());
            root.borrow_mut()
                .add_syntax_child(SyntaxKind::ERROR_STATE(err));
            return Some(root);
        }
        if let L_CURLY = lexer.next_token() {
        } else {
            let err = format!("in Method,missing {{,you are {}", lexer.get_prev());
            root.borrow_mut()
                .add_syntax_child(SyntaxKind::ERROR_STATE(err));
            return Some(root);
        }

        //build declartions
        loop {
            let node = build_vardeclaration(lexer);
            root.borrow_mut().add_child(node.clone());
            if node.is_none() || TreeNode::check_bp(&node) {
                break;
            }
        }

        //build statement
        loop {
            let node = build_statement(lexer);
            root.borrow_mut().add_child(node.clone());
            if node.is_none() || TreeNode::check_bp(&node) {
                break;
            }
        }

        if let RETURN_KW = lexer.nth_token(1) {
            let return_node = build_return(lexer);
            root.borrow_mut().add_child(return_node);
        }
        if let R_CURLY = lexer.next_token() {
        } else {
            let err = format!("in Method,missing }},you are {}", lexer.get_prev());
            root.borrow_mut()
                .add_syntax_child(SyntaxKind::ERROR_STATE(err));
            return Some(root);
        }
        Some(root)
    } else {
        None
    }
}

fn build_param(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
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
                if let R_BRACK = lexer.next_token() {
                } else {
                    let err = format!("in Method params,missing ],you are {}", lexer.get_prev());
                    return Some(Rc::new(RefCell::new(TreeNode::new(
                        SyntaxKind::ERROR_STATE(err),
                    ))));
                }
                Some(Rc::new(RefCell::new(TreeNode::new(
                    SyntaxKind::TYPE_INT_ARRAY,
                ))))
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

    if let IDENT(id) = lexer.next_token() {
        &root
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_syntax_child(SyntaxKind::IDENT(id));
    } else {
        let err = format!("in MethodParam ,missing ident,you are {}", lexer.get_prev());
        &root
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
    }
    root
}

fn build_return(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    lexer.next_token();
    let root = Rc::new(RefCell::new(TreeNode::new(SyntaxKind::RETURN_KW)));
    root.borrow_mut().add_child(build_expression(lexer));
    if let SEMI = lexer.next_token() {
    } else {
        let err = format!(
            "in Method return part ,missing SEMI(;),you are {}",
            lexer.get_prev()
        );
        &root
            .borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
    }
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
    let root = Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer))));
    loop {
        let node = match lexer.nth_token(1) {
             IF_KW | WHILE_KW | SYSTEM_KW => build_statement(lexer),
            IDENT(_id) => build_statement(lexer),
            _ => break,
        };
        root.borrow_mut().add_child(node.clone());
        if node.is_none() || TreeNode::check_bp(&node) {
            break;
        }

        // if let L_CURLY=lexer.nth_token(1){
        //     break;
        // }
        // let node = build_statement(lexer);
        // root.borrow_mut().add_child(node.clone());
        // if node.is_none() || TreeNode::check_bp(&node) {
        //     break;
        // }
      
    }
    if let R_CURLY = lexer.next_token() {
    } else {
        let err = format!("In black statement,missing }},you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
    }

    Some(root)
}

fn build_if_statement(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let root = Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer))));
    if let L_PAREN = lexer.next_token() {
    } else {
        let err = format!("In if statement,missing (,you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
    }
    let mut expression_node = build_expression(lexer);
    if let R_PAREN = lexer.next_token() {
    } else {
        let err = format!("In if statement,missing ),you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
    }
    &expression_node
        .as_ref()
        .unwrap()
        .borrow_mut()
        .add_child(build_statement(lexer));
    root.borrow_mut().add_child(expression_node);
    if let ELSE_KW = lexer.nth_token(1) {
    } else {
        let err = format!(
            "In if statement,missing else_kw,you are {}",
            lexer.get_prev()
        );
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
    }
    let mut else_node = Some(Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer)))));
    &else_node
        .as_ref()
        .unwrap()
        .borrow_mut()
        .add_child(build_statement(lexer));
    root.borrow_mut().add_child(else_node);
    Some(root)
}

fn build_while_statement(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let root = Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer))));
    if let L_PAREN = lexer.next_token() {
    } else {
        let err = format!("In while statement,missing (,you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
    }
    root.borrow_mut().add_child(build_expression(lexer));
    if let R_PAREN = lexer.next_token() {
    } else {
        let err = format!("In if statement,missing ),you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
    }
    root.borrow_mut().add_child(build_statement(lexer));
    Some(root)
}

fn build_print_statement(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
    let root = Rc::new(RefCell::new(TreeNode::new(token2syntax(lexer))));

    if let L_PAREN = lexer.next_token() {
    } else {
        let err = format!("In print statement,missing (,you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
    }
    root.borrow_mut().add_child(build_expression(lexer));
    if let R_PAREN = lexer.next_token() {
    } else {
        let err = format!("In print statement,missing ),you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
    }
    if let SEMI = lexer.next_token() {
    } else {
        let err = format!("In print statement,missing ;,you are {}", lexer.get_prev());
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
    }
    Some(root)
}

//FIXME WHIE IS SYSTEM_KW
fn build_assign_statement(lexer: &mut Lexer) -> Option<Rc<RefCell<TreeNode>>> {
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
            if let R_BRACK = lexer.next_token() {
            } else {
                let err = format!("In assing statment,missing ],you are {}", lexer.get_prev());

                &tmp.as_ref()
                    .unwrap()
                    .borrow_mut()
                    .add_child(Some(Rc::new(RefCell::new(TreeNode::new(
                        SyntaxKind::ERROR_STATE(err),
                    )))));
            }
            tmp
        }
        _ => None,
    };
    let root = if let EQ = lexer.next_token() {
        Rc::new(RefCell::new(TreeNode::new(SyntaxKind::ASSIGN_STATE)))
    } else {
        let err = format!(
            "In assigne statement,missing =,you are {}",
            lexer.get_prev()
        );
        Rc::new(RefCell::new(TreeNode::new(SyntaxKind::ERROR_STATE(err))))
    };
    let expression_2 = build_expression(lexer);
    root.borrow_mut().add_syntax_child(ident);
    root.borrow_mut().add_child(expression_1);
    root.borrow_mut().add_child(expression_2);
    if let SEMI = lexer.next_token() {
    } else {
        let err = format!(
            "In assigne statement,missing ;,you are {}",
            lexer.get_prev()
        );
        root.borrow_mut()
            .add_syntax_child(SyntaxKind::ERROR_STATE(err));
    }
    Some(root)
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
            lexer.next_token();
            match lexer.next_token() {
                IDENT(id) => {
                    let mut tmp =
                        Rc::new(RefCell::new(TreeNode::new(SyntaxKind::NEW_CLASS_EXPRESION)));
                    tmp.borrow_mut()
                        .add_child(Some(Rc::new(RefCell::new(TreeNode::new(
                            SyntaxKind::IDENT(id),
                        )))));
                    if let L_PAREN = lexer.next_token() {
                    } else {
                        let err = format!("In Expression,missing ( ,you are {}", lexer.get_prev());
                        tmp.borrow_mut()
                            .add_child(Some(Rc::new(RefCell::new(TreeNode::new(
                                SyntaxKind::ERROR_STATE(err),
                            )))));
                    }
                    if let R_PAREN = lexer.next_token() {
                    } else {
                        let err = format!("In Expression,missing ) ,you are {}", lexer.get_prev());
                        tmp.borrow_mut()
                            .add_child(Some(Rc::new(RefCell::new(TreeNode::new(
                                SyntaxKind::ERROR_STATE(err),
                            )))));
                    }
                    Some(tmp)
                }
                INT_KW => {
                    let tmp = Rc::new(RefCell::new(TreeNode::new(SyntaxKind::NEW_INT_EXPRESSION)));
                    if let L_BRACK = lexer.next_token() {
                    } else {
                        let err = format!("In Expression,missing [ ,you are {}", lexer.get_prev());
                        tmp.borrow_mut()
                            .add_child(Some(Rc::new(RefCell::new(TreeNode::new(
                                SyntaxKind::ERROR_STATE(err),
                            )))));
                    }
                    tmp.borrow_mut().add_child(expression_bp(lexer, 0));
                    if let R_BRACK = lexer.next_token() {
                    } else {
                        let err = format!("In Expression,missing ] ,you are {}", lexer.get_prev());
                        tmp.borrow_mut()
                            .add_child(Some(Rc::new(RefCell::new(TreeNode::new(
                                SyntaxKind::ERROR_STATE(err),
                            )))));
                    }
                    Some(tmp)
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
                let err = format!("error");
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
                lexer.next_token();
                match lexer.next_token() {
                    IDENT(id) => {
                        let mut mhs =
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
