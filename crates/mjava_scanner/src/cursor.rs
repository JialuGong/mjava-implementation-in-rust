//The curosor is a mini  rust-cursor
use std::str::Chars;

///The cursor is a `iterator` of a char sequence
///
///
///
/// # Examples
/// ```rust
/// //create a new cursor
///let cursor=Cursor::new("abc");
/// assert_eq!('a',cursor.first_char());
/// assert_eq!('b',cursor.scond_char());
///
/// ```
///
///
pub struct Cursor<'a> {
    input: Chars<'a>,
    prev: char,
    initital_len: usize,
}
pub const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    ///create a `Scanner`
    pub fn new(init_input: &'a str)-> Cursor {
        Cursor {
            input: init_input.chars(),
            #[cfg(debug_assertions)]
            prev: EOF_CHAR,
            initital_len: init_input.len(),    
        }
    }

    pub fn prev(&self) -> char {
        #[cfg(debug_assertions)]
        {
            self.prev
        }
        #[cfg(not(debug_assertions))]
        {
            '\0'
        }
    }

    pub fn nth_char(&self, n: usize) -> char {
        self.chars().nth(n).unwrap_or(EOF_CHAR)
    }

    fn chars(&self) -> Chars<'a> {
        self.input.clone()
    }

    pub fn first_char(&self) -> char {
        self.nth_char(0)
    }

    pub fn second_char(&self) -> char {
        self.nth_char(1)
    }

    pub fn is_eof(&self) -> bool {
        self.input.as_str().is_empty()
    }
    pub fn next_char(&mut self) -> Option<char> {
        let c = self.input.next()?;
        self.prev = c;
        Some(c)
    }
    pub fn consum(&self) -> usize {
        self.initital_len - self.input.as_str().len()
    }
}
