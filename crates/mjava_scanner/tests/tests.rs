#[cfg(test)]
mod tests {
    use mjava_scanner::TokenKind::*;
    use std::env;
    use std::fs;
    fn print_answer(test_path: &str, result_path: &str) {
        let mut line: usize = 1;
        let content = fs::read_to_string(test_path).expect(&*format!("Open {} file failed!",test_path));
        let answer = mjava_scanner::get_tokens(&*content)
            .iter()
            .map(|x| match x {
                Ok(t) => {
                    let kind = &t.kind;
                    match kind {
                        IDENT(s) => format!("IDENTIFIER: `{}`", s),
                        INTER(s) => format!("INTERGER: `{}`", s),
                        CLASS_KW => format!("KEYWORD: CLASS_KW `class`"),
                        PUBLIC_KW => format!("KEYWORD: PUBLIC_KW `public`"),
                        STRING_KW => format!("KEYWORD: STRING_KW `String`"),
                        STATIC_KW => format!("KEYWORD: STATIC_KW `static`"),
                        VOID_KW => format!("KEYWORD: VOID_KW `void`"),
                        MAIN_KW => format!("KEYWORD: MAIN_KW `main`"),
                        EXTENDS_KW => format!("KEYWORD: EXTENDS_KW `extends`"),
                        RETURN_KW => format!("KEYWORD: RETRUN_KW `return`"),
                        INT_KW => format!("KEYWORD: INT_KW `int`"),
                        BOOLEAN_KW => format!("KEYWORD: BOOLEAN_KW `boolean`"),
                        IF_KW => format!("KEYWORD: IF_KW `if`"),
                        ELSE_KW => format!("KEYWORD: ELSE_KW `else`"),
                        WHILE_KW => format!("KEYWORD: WHILE_KW `while`"),
                        LENGTH_KW => format!("KEYWORD: LENGTH_KW `length`"),
                        TRUE_KW => format!("KEYWORD: TRUE_KW `true`"),
                        FALSE_KW => format!("KEYWORD: FALSE_KW `false`"),
                        THIS_KW => format!("KEYWORD: THIS_KW `this`"),
                        NEW_KW => format!("KEYWORD: NEW_KW `new`"),
                        SYSTEM_KW => format!("KEYWORD: SYSTEM_KW `System.out.println`"),
                        L_BRACK => format!("SYMBOL: L_BRACK `[`"),
                        R_BRACK => format!("SYMBOL: R_BRACK `]`"),
                        L_PAREN => format!("SYMBOL: L_PAREN `(`"),
                        R_PAREN => format!("SYMBOL: R_PAREN`)`"),
                        L_CURLY => format!("SYMBOL: L_CURLY `{{`"),
                        R_CURLY => format!("SYMBOL: R_CURLY `}}`"),
                        COMMA => format!("SYMBOL: COMMA `,`"),
                        SEMI => format!("SYMBOL: SEMI `;`"),
                        EQ => format!("SYMBOL: EQ `=`"),
                        L_ANGLE => format!("SYMBOL: L_ANGLE `<`"),
                        PLUS => format!("SYMBOL: PLUS `+`"),
                        MINUS => format!("SYMBOL: MINUS `-`"),
                        STAR => format!("SYMBOL: STAR `*`"),
                        EXCL => format!("SYMBOL: EXCL `!`"),
                        AMP => format!("SYMBOL: AMP `&&`"),
                        DOT => format!("SYMBOL : DOT `.`"),
                        ENTER_BLOCK => {
                            line += 1;
                            "".to_string()
                        }
                        _ => "".to_string(),
                    }
                }
                Err(e) => {
                    let kind = &e.kind;
                    match kind {
                        UNKNOWN(s) => format!("line {} :unknown character `{}`", line,s),
                        WRONG_ID(s) => format!("line {} :wrong ID `{}`", line, s),
                        _ => "".to_string(),
                    }
                }
            })
            .collect::<Vec<String>>()
            .join("\n");
        fs::write(result_path, answer).expect("Some thing wrong when write the file!");
    }

    #[test]
    fn it_works() {
        let root=env!("CARGO_MANIFEST_DIR");
        for i in 1..8{
            let test_path=format!("{}/test_sources/test{}.txt",root,i);
            let result_path=format!("{}/test_sources/result{}.txt",root,i);
            print_answer(&*test_path,&*result_path);
        }

        //self test
        let self_list=["keyword","ident","int","symbol"];
        for i in self_list.iter(){
            let test_path=format!("{}/test_sources/self_{}_test.txt",root,i);
            let result_path=format!("{}/test_sources/self_{}_result.txt",root,i);
            print_answer(&*test_path, &*result_path);
        }
        
       
    }
}
