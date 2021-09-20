#[link(name = "lexer")]
extern "C"{
    fn parse();
}

fn main() {
    unsafe {
        parse();
    }
}
