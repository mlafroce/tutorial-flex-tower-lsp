use std::collections::HashMap;
use std::ffi::CStr;
use std::fs::File;
use std::io;
use std::os::raw::{c_char, c_int};
use std::os::unix::io::AsRawFd;

#[link(name = "lexer")]
extern "C" {
    fn parse(lexer: *mut i32, input_fd: i32);
}

#[derive(Debug)]
#[repr(C)]
enum TokenType {
    Palabra,
    Numero,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: String,
}

#[derive(Debug, Default)]
pub struct Lexer {
    token_map: HashMap<(usize, usize), Token>,
    cur_token: usize,
}

impl Lexer {
    pub fn parse_tokens(&mut self, filename: &str) -> io::Result<()> {
        let input = File::open(filename)?;
        let fd = input.as_raw_fd();
        unsafe {
            parse(self as *mut Lexer as *mut _, fd);
        }
        Ok(())
    }

    fn push_token(&mut self, token: Token, yylineno: usize, yycolumn: usize) {
        self.token_map.insert((yylineno, yycolumn), token);
    }
}

#[no_mangle]
pub fn lexer_push_token(
    parser: *mut Lexer,
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
    let parser = unsafe { &mut *parser as &mut Lexer };
    let token = Token { token_type, value };
    parser.push_token(token, yylineno as usize, yycolumn as usize);
}
