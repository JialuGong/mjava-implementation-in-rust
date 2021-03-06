#[cfg(test)]
mod tests {
    use mjava_parser::parser::parser;
    use mjava_parser::syntax::{SyntaxKind, SyntaxKind::*};
    use mjava_parser::tree_node::TreeNode;
    use mjava_scanner::{get_tokens, Token, TokenKind};
    use std::borrow::Borrow;
    use std::cell::RefCell;
    use std::env;
    use std::fs;
    use std::path::Path;
    use std::rc::Rc;
    use std::{fmt, io::BufRead};

    fn string2tree(source: String) -> String {
        let mut line = 1;
        let mut pos = 0;
        let mut scanner_result = get_tokens(&*source);
        let mut tokens = Vec::new();
        for _i in 0..scanner_result.len() {
            pos += scanner_result.len();
            if let Ok(ok_token) = scanner_result.remove(0) {
                match ok_token.kind.clone() {
                    TokenKind::BLANK_BLOCK => {}
                    TokenKind::ENTER_BLOCK => {
                        line += 1;
                        pos = 0;
                    }
                    _ => {
                        tokens.push(ok_token);
                    }
                }
            } else {
                panic!("wrong tokens! in line :{},row,{}", line, pos);
            }
        }
        let root = parser(tokens);
        TreeNode::print(root)
    }

    #[test]
    // fn work(){
    //     let root=env!("CARGO_MANIFEST_DIR");
    //     let mut test_source = &*format!("{}/test_source", root);
    //     let mut source = fs::read_to_string(file_path.clone()).expect("open file failed!");

    // }
    fn it_works() {
        let root = env!("CARGO_MANIFEST_DIR");
        let mut test_source = format!("{}/test_source", root);
        let mut out_source = format!("{}/output", root);
        for i in 8..24{
            let read_dir = format!("{}/test{}.txt", test_source, i);
            let mut source = fs::read_to_string(read_dir).expect("open failed!");
            let ans = string2tree(source);
            let write_dir = format!("{}/out{}.txt", out_source, i);
            fs::write(write_dir, ans).expect("Some thing wrong when write the file!");
        }

        //     let dirs = match fs::read_dir(test_source) {
        //         Err(why) => { panic!("{:?}", why.kind()) }
        //         Ok(paths) => paths
        //     };
        //     let mut i=1;
        //     for dir in dirs {
        //         let mut paths = match fs::read_dir(dir.unwrap().path()) {
        //             Err(why) => { panic!("{:?}", why.kind()) }
        //             Ok(paths) => paths
        //         };
        //         for file in paths {
        //             let file_path=file.unwrap().path();
        //             let mut source = fs::read_to_string(file_path.clone()).expect("open file failed!");
        //             let ans = string2tree(source);
        //             println!("read path is {:?}",file_path);
        //             let out_path = format!("{}/output/out{}.txt", root,i);
        //             i+=1;
        //             fs::write(out_path, ans).expect("Some thing wrong when write the file!");
        //         }
        //     }
        // }
    }
}
