#[cfg(test)]
mod tests {
    use mjava_scanner::{scanner::get_token_vec, Token};
    use std::fs;
    #[test]
    fn it_works() {
        let file_path="/home/huaiyu/rust-project/mjava/mjava_project/crates/mjava_scanner/tests/test5.txt";
        let write_path="/home/huaiyu/rust-project/mjava/mjava_project/crates/mjava_scanner/tests/result5.txt";
        println!("Test:\t test file name is {}", file_path);

        let content = fs::read_to_string(file_path).expect("Some thing wrong when open the files!");

        let answer = get_token_vec(content);

        let result: String = answer.iter().fold(String::new(), |acc, x| match x {
            Ok(o) => match o {
                Token::Identifier(id) => format!("{} Identifier({}) ", acc, id),
                Token::IntegerLitera(int) => format!("{} IntergerLitera({}) ", acc, int),
                Token::ReservedWord(rw) => format!("{} ReservedWord({}) ", acc, rw),
                Token::Symbol(sy) => format!("{} Symbol({}) ", acc, sy),
            },
            Err(e) => format!("{} ErrorToken({}) ", acc, e.value),
        });

        fs::write(write_path, result).expect("Some thing wrong when write the file!");
    }
}
