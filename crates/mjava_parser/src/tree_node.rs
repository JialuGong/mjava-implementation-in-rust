extern crate mjava_scanner;


use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;
use crate::syntax::{SyntaxKind::*,SyntaxKind};

 #[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
     children: Vec<Option<Rc<RefCell<TreeNode>>>>,
     siblings: Vec<Option<Rc<RefCell<TreeNode>>>>,
     kind: SyntaxKind,
}

impl TreeNode {
    pub fn new(syntax_kind: SyntaxKind) -> TreeNode {
        TreeNode {
            children: Vec::new(),
            siblings: Vec::new(),
            kind: syntax_kind,
        }
    }
    pub fn add_syntax_child(& mut self, child_kind: SyntaxKind) {
        self.children.push(Some(Rc::new(RefCell::new(TreeNode::new(child_kind)))));
    }
    pub fn add_child(& mut self, child: Option<Rc<RefCell<TreeNode>>>) {
        self.children.push(child);
    }
    pub fn add_sibling(& mut self, sibling: Option<Rc<RefCell<TreeNode>>>) {
        self.siblings.push(sibling);
    }


    pub fn print(root:Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut stack = vec![(root.clone(), 0, false)];
        let mut ans = String::new();
        while !stack.is_empty() {
            let (mut tree_node, layer, flag) = stack.pop().unwrap();
            if let Some(rc) = tree_node.clone() {
               if flag{
                   ans=format!("{}{}{:?}\n",ans,"  ".repeat(layer),rc.borrow().kind);
               }
                else{
                    let mut children=rc.borrow().children.clone();
                    children.reverse();
                    for child in children {
                        if  child.is_some(){
                            stack.push((child.clone(),layer+1,false));
                        }
                    }
                     stack.push((tree_node.clone(),layer,true));
                    //    for sibling in &rc.borrow().siblings{
                    //     if let Some(sibling_rc)=sibling.as_ref().clone(){
                    //         stack.push((sibling.clone(),layer+1,false));
                    //                             }
                    // }
                    
                   
                }
            }
        }
        ans
    }
    pub fn parent_add_child( parent:&mut Option<Rc<RefCell<TreeNode>>>,child:Option<Rc<RefCell<TreeNode>>>)->bool{
        let k=Rc::new(RefCell::new(TreeNode::new(SyntaxKind::LBRACK_EXPRESSION)));
        k.borrow_mut().add_child(None);
        true
    }
}

// impl fmt::Display for TreeNode {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let ans = format!("{:?}", self.kind);
//         write!(f, "{}", ans)
//     }
// }
