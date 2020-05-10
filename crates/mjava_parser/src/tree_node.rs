extern crate mjava_scanner;

use crate::syntax::SyntaxKind;
use std::fmt;
use std::borrow::BorrowMut;

 #[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
     children: Vec<Option<Box<TreeNode>>>,
     siblings: Vec<Option<Box<TreeNode>>>,
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
        self.children.push(Some(Box::new(TreeNode::new(child_kind))))
    }
    pub fn add_child(& mut self, child: Option<Box<TreeNode>>) {
        self.children.push(child);
    }
    pub fn add_sibling(& mut self, sibling: Option<Box<TreeNode>>) {
        self.siblings.push(sibling);
    }
    pub fn print(root:Option<Box<TreeNode>>) -> String {
        let mut stack = vec![(root.clone(), 0, false)];
        let mut ans = String::new();
        while !stack.is_empty() {
            let (mut tree_node, layer, flag) = stack.pop().unwrap();
            if let Some(rc) = tree_node.clone() {
               if flag{
                   ans=format!("{}{}{}\n",ans,rc.kind," ".repeat(layer));
               }
                else{
                    for child in &rc.children {
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
    // pub fn parent_add_child( parent:&mut Option<Rc<RefCell<TreeNode>>>,child:Option<Rc<RefCell<TreeNode>>>)->bool{
    //     let k=Rc::new(RefCell::new(TreeNode::new(SyntaxKind::LBRACK_EXPRESSION)));
    //     k.borrow_mut().
    //     true
    // }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ans = format!("{}", self.kind);
        write!(f, "{}", ans)
    }
}
