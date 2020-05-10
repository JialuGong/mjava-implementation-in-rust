extern crate mjava_scanner;

use crate::syntax::SyntaxKind;
use std::rc::Rc;
use std::fmt;
use std::cell::RefCell;

pub struct TreeNode {
    pub children: Vec<Option<Rc<RefCell<TreeNode>>>>,
    pub siblings: Vec<Option<Rc<RefCell<TreeNode>>>>,
    pub kind: SyntaxKind,
}

impl TreeNode {
    pub fn new(syntax_kind: SyntaxKind) -> TreeNode {
        TreeNode {
            children: Vec::new(),
            siblings: Vec::new(),
            kind: syntax_kind,
        }
    }
    pub fn add_syntax_child(&mut self, child_kind: SyntaxKind) {
        self.children.push(Some(Rc::new(RefCell::from(TreeNode::new(child_kind)))));
    }
    pub fn add_child(&mut self, child: Option<Rc<RefCell<TreeNode>>>) {
        self.children.push(child);
    }
    pub fn add_sibling(&mut self, sibling: Option<Rc<RefCell<TreeNode>>>) {
        self.siblings.push(sibling);
    }
    pub fn print(root:Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut stack = vec![(root.clone(), 0, false)];
        let mut ans = String::new();
        while !stack.is_empty() {
            let (mut tree_node, layer, flag) = stack.pop().unwrap();
            if let Some(rc) = tree_node.clone() {
               if flag{
                   ans=format!("{}{}{}\n",ans,rc.borrow().kind," ".repeat(layer));
               }
                else{
                    for child in &rc.borrow().children {
                        if let Some(child_rc) = (*child).clone() {
                            stack.push((child.clone(),layer+1,false));
                        }
                    }
                    stack.push((tree_node.clone(),layer,true));
                }
            }
        }
        ans
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ans = format!("{}", self.kind);
        write!(f, "{}", ans)
    }
}
