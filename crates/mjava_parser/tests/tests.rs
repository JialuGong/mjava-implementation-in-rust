#[cfg(test)]
mod tests {
    use mjava_parser::syntax::{SyntaxKind, SyntaxKind::*};
    use mjava_parser::parser::parser;
    use std::{fmt, io::BufRead};
    use mjava_scanner::{get_tokens, TokenKind, Token};
    use mjava_parser::syntax::ExpressionKind::OP_EXPRESSION;
    use std::cell::RefCell;
    use mjava_parser::tree_node::TreeNode;
    use std::rc::Rc;
    use std::env;
    use std::fs;
    use std::path::Path;
    use std::borrow::Borrow;


    fn string2tree(source: String) -> String {
        let mut scanner_result = get_tokens(&*source);
        let mut tokens=Vec::new();
        for i in 0..scanner_result.len(){
           if let Ok(ok_token)=scanner_result.pop().unwrap(){
               match ok_token.kind.clone(){
                   TokenKind::BLANK_BLOCK|TokenKind::ENTER_BLOCK=>{}
                   _=>{
                       tokens.push(ok_token);
                   }
               }

           }else{
               panic!("wrong tokens!")
           }
        }
        // let mut tokens =
        //     scanner_result
        //         .iter()
        //         .map(|&x.as_ref()|match x {
        //             Ok(token) => token,
        //             Err(e) => panic!("Tokens has wrong token")
        //         }
        //         ).collect::<Vec<_>>();
        let mut root = parser(tokens);
        TreeNode::print(root)
    }


    #[test]
    fn it_works() {
        let root = env!("CARGO_MANIFEST_DIR");
        let mut test_source = &*format!("{}/test_source", root);
        let dirs = match fs::read_dir(test_source) {
            Err(why) => { panic!("{:?}", why.kind()) }
            Ok(paths) => paths
        };
        for dir in dirs {
            let mut paths = match fs::read_dir(dir.unwrap().path()) {
                Err(why) => { panic!("{:?}", why.kind()) }
                Ok(paths) => paths
            };
            for file in paths {
                let file_path=file.unwrap().path();
                let mut source = fs::read_to_string(file_path.clone()).expect("open file failed!");
                let ans = string2tree(source);
                let out_path = format!("{:?}out.txt", file_path);
                fs::write(out_path, ans).expect("Some thing wrong when write the file!");
            }
        }
    }
}
