mod lexer;

use lexer::Lexer;
use std::io;

fn main() -> io::Result<()> {
    let mut parser = Lexer::default();
    parser.parse_tokens("test.md")?;
    println!("Parsed: {:?}", parser);
    Ok(())
}
