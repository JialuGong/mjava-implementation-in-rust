#[cfg(test)]
mod tests {
    use mjava_scanner::TokenKind::*;
    use std::env;
    use std::fs;
    fn print_answer(test_path: &str, result_path: &str) {
        let mut line: usize = 1;
        let content = fs::read_to_string(test_path).expect("Open test file failed!");
        let answer = mjava_scanner::get_tokens(&*content)
            .iter()
            .map(|x| match x {
                Ok(t) => {
                    let kind = &t.kind;
                    match kind {
                        IDENT(s) => format!("IDENTIFIER: {}", s),
                        INTER(s) => format!("INTERGER: {}", s),
                        CLASS_KW => format!("KEYWORD: class"),
                        PUBLIC_KW => format!("KEYWORD: public"),
                        STRING_KW => format!("KEYWORD: String"),
                        STATIC_KW => format!("KEYWORD: static"),
                        VOID_KW => format!("KEYWORD: void"),
                        MAIN_KW => format!("KEYWORD: main"),
                        EXTENDS_KW => format!("KEYWORD: extends"),
                        RETURN_KW => format!("KEYWORD: return"),
                        INT_KW => format!("KEYWORD: int"),
                        BOOLEAN_KW => format!("KEYWORD: boolean"),
                        IF_KW => format!("KEYWORD: if"),
                        ELSE_KW => format!("KEYWORD: else"),
                        WHILE_KW => format!("KEYWORD: while"),
                        LENGTH_KW => format!("KEYWORD: length"),
                        TRUE_KW => format!("KEYWORD: true"),
                        FALSE_KW => format!("KEYWORD: false"),
                        THIS_KW => format!("KEYWORD: this"),
                        NEW_KW => format!("KEYWORD: new"),
                        SYSTEM_KW => format!("KEYWORD: System.out.println"),
                        L_BRACK => format!("SYMBOL: ["),
                        R_BRACK => format!("SYMBOL: ]"),
                        L_PAREN => format!("SYMBOL: ("),
                        R_PAREN => format!("SYMBOL: )"),
                        L_CURLY => format!("SYMBOL: {{"),
                        R_CURLY => format!("SYMBOL: }}"),
                        COMMA => format!("SYMBOL: ,"),
                        SEMI => format!("SYMBOL: ;"),
                        EQ => format!("SYMBOL: ="),
                        L_ANGLE => format!("SYMBOL: <"),
                        PLUS => format!("SYMBOL: +"),
                        MINUS => format!("SYMBOL: -"),
                        STAR => format!("SYMBOL: *"),
                        EXCL => format!("SYMBOL: !"),
                        AMP => format!("SYMBOL: &&"),
                        DOT => format!("SYMBOL : ."),
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
                        UNKNOWN(s) => format!("UNKNOWN_ERROR : {},line {}", s, line),
                        KROWN_ID(s) => format!("KROWN_ID: {},line: {}", s, line),
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
    }
}
