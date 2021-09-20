use std::process::Command;

/// Compilamos dentro de target para que `cargo clean` limpie todo lo generado
fn main() {
    Command::new("lex")
        .args(&["-otarget/lex.yy.c", "lexer/barba-lang.l"])
        .status()
        .unwrap();
    cc::Build::new()
        .file("target/lex.yy.c")
        .include("lexer")
        .flag("-Wno-unused-function")
        .compile("lexer");
}
