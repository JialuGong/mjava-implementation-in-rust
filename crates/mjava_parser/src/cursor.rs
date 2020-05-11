extern crate mjava_scanner;
use mjava_scanner::Token;
use mjava_scanner::TokenKind;
pub struct Lexer {
    tokens: Vec<Token>,
    pos: usize,
    prev:TokenKind,
}
impl Lexer {
    pub fn new( mut tokens_init: Vec<Token>) -> Lexer {
         tokens_init.reverse();
        Lexer {
            tokens: tokens_init,
            pos: 0,
            prev:TokenKind::EOF,
        }
    }
    pub fn next_token( &mut self)->TokenKind{
       if let Some(token)=self.tokens.pop(){
           self.prev=token.kind;
           self.prev.clone()
       }else{
           self.prev=TokenKind::EOF;
           TokenKind::EOF
       }
      
    }
    pub fn nth_token(&self,n:usize)->TokenKind{
        if let  Some(token)=self.tokens.get(if self.tokens.len()>=n{self.tokens.len()-n}else{0}){
            token.kind.clone()
        }else{
            TokenKind::EOF
        }
    }
    pub fn get_prev(&self)->TokenKind{
        self.prev.clone()
    }
    // pub fn clear(&mut self){
    //     self.tokens.clear();
    // }
}
