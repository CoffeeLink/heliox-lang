use heliox_asm::lexer::Lexer;
use std::fs;

fn main() {
    let src = fs::read_to_string("test_scripts/simple_test.hla").unwrap();
    let mut lex = Lexer::new(&src);

    lex.tokenize();
}
