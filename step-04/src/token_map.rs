use std::collections::HashMap;
use std::ffi::CStr;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::os::raw::{c_char, c_int};
use std::os::unix::io::AsRawFd;

#[link(name = "lexer")]
extern "C" {
    fn parse(lexer: *mut i32, input_fd: i32);
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum TokenType {
    Palabra,
    Numero,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

#[derive(Debug, Default)]
pub struct TokenMap {
    token_map: HashMap<(usize, usize), Token>,
}

impl TokenMap {
    pub fn parse_tokens(&mut self, filename: &str) -> io::Result<()> {
        let input = File::open(filename)?;
        let fd = input.as_raw_fd();
        unsafe {
            parse(self as *mut TokenMap as *mut _, fd);
        }
        Ok(())
    }

    pub fn get_token_at(&self, yylineno: usize, yycolumn: usize) -> Option<Token> {
        for column in (0..yycolumn).rev() {
            if let Some(token) = self.token_map.get(&(yylineno, column)) {
                return Some(token.clone());
            }
        }
        None
    }

    fn push_token(&mut self, token: Token, yylineno: usize, yycolumn: usize) {
        self.token_map.insert((yylineno, yycolumn), token);
    }
}

#[no_mangle]
pub fn lexer_push_token(
    parser: *mut TokenMap,
    token_type: c_int,
    token_value: *const c_char,
    yylineno: c_int,
    yycolumn: c_int,
) {
    let token_type = match token_type {
        0 => TokenType::Palabra,
        1 => TokenType::Numero,
        _ => return,
    };
    let value = unsafe { CStr::from_ptr(token_value).to_str().unwrap().to_string() };
    let parser = unsafe { &mut *parser as &mut TokenMap };
    let token = Token { token_type, value };
    parser.push_token(token, yylineno as usize, yycolumn as usize);
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            TokenType::Palabra => write!(f, "Palabra"),
            TokenType::Numero => write!(f, "Numero"),
        }
    }
}
