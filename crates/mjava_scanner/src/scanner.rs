use crate::{Token, TokenError};
//TODO bug to resolve

#[warn(dead_code)]
const ID_GRANPH: [[i32; 3]; 4] = [[1, 3, 3], [1, 1, 2], [1, 1, 3], [3, 3, 3]];

pub fn get_token_vec(s: String) -> Vec<Result<Token, TokenError>> {
    let mut token_vec: Vec<Result<Token, TokenError>> = Vec::new();
    let mut new_string = String::new();
    s.chars().for_each(|x| match x {
        '\t' | '\n' | ' ' => {
            if !new_string.is_empty() {
                token_vec.append(&mut token_to_result_vec(&new_string));
            }
            new_string.clear();
        }
        '[' | ']' | '(' | ')' | '{' | '}' | ',' | ';' | '=' | '<' | '+' | '-' | '*' | '!' => {
            if !new_string.is_empty() {
                token_vec.append(&mut token_to_result_vec(&new_string));
                new_string.clear();
            }
            token_vec.push(Ok(Token::Symbol(x.to_string())));
        }
        '&' => {
            if !new_string.is_empty() {
                if new_string == "&".to_string() {
                    new_string.push('&');
                    token_vec.push(Ok(Token::Symbol(new_string.clone())));
                    new_string.clear();
                } else {
                    token_vec.append(&mut token_to_result_vec(&new_string));
                }
            } else {
                new_string.push('&');
            }
        }
        _ => {
            new_string.push(x);
        }
    });
    token_vec
}

fn token_to_result_vec(token: &String) -> Vec<Result<Token, TokenError>> {
    let mut vec_result: Vec<Result<Token, TokenError>> = Vec::new();
    if token.contains('.') {
        if token == "System.out.println" {
            vec_result.push(Ok(Token::ReservedWord(token.clone())));
        } else {
            token.split('.').for_each(|x| {
                vec_result.push(token_to_result(&x.to_string()));
                vec_result.push(Ok(Token::Symbol('.'.to_string())));
            });
            vec_result.pop();
        }
    } else {
        vec_result.push(token_to_result(token));
    }
    vec_result
}
/**
 * Find token type rand wrap the token
 *
 */
fn token_to_result(token: &String) -> Result<Token, TokenError> {
    let token_clone = token.clone();
    if is_reserved_word(&token) {
        return Ok(Token::ReservedWord(token_clone));
    } else if is_number(&token) {
        return Ok(Token::IntegerLitera(token_clone));
    } else if is_identifier(&token) {
        return Ok(Token::Identifier(token_clone));
    } else {
        return Err(TokenError { value: token_clone });
    }
}

/**
 * To indentify the token is a reserve word
 *
 */
fn is_reserved_word(token: &String) -> bool {
    let token_clone = token.clone();
    match &*token_clone {
        "class" | "public" | "static" | "void" | "main" | "String" | "extends" | "int" => true,
        "boolean" | "if" | "else" | "while" | "length" | "true" | "false" | "this" | "new" => true,
        _ => false,
    }
}

/**
 * To identify the token is a number
 *
 */
fn is_number(token: &String) -> bool {
    for ch in token.chars() {
        if ch > '9' || ch < '0' {
            return false;
        }
    }
    return true;
}

/**
 * To identify the token is a identifier
 *
 */
fn is_identifier(token: &String) -> bool {
    let mut state = 0;
    token.chars().for_each(|x| match x {
        '.' | '_' => {
            state = ID_GRANPH[state as usize][2];
        }
        'A'..='z' => state = ID_GRANPH[state as usize][0],
        '0'..='9' => {
            state = ID_GRANPH[state as usize][1];
        }
        _ => {
            state = 3;
        }
    });

    match state {
        1 => true,
        _ => false,
    }
}
