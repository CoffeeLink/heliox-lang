use crate::instructions::Instruction;
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum LiteralKind {
    Integer, // "10" or "0xFF" or "0b0000_0000" numbers
    Register, // "r4" or "R4"
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum TokenKind {
    Literal(LiteralKind), // values like "10" or "0b0000" or "r10"
    Identifier, // constants and label names
    Label, // Label starts
    Constant, // Const Define
    Directive, // The .<name> <arg> thing
    Instruction(Instruction),
    EndStatement // The ";" so "call main; mov r10 r1". after ";" | "\n"
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Token<'a> {
    kind: TokenKind,
    value: &'a str,

    //debug info
    line : usize,
    char : usize
}

pub struct Lexer<'a> {
    source: &'a str,
    current_char : usize, // for lexing

    current_line : usize, // for debug in tokens so 31:x is possible
    current_char_on_line : usize, // for debug in tokens so line:10 is possible
}
impl <'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, current_char: 0, current_line: 1, current_char_on_line: 1 }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.source.chars().nth(self.current_char)
    }

    pub fn consume(&mut self) {
        self.current_char += 1;
    }

    fn parse_str(&mut self) -> String {
        let mut buffer = String::new();

        buffer
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens : Vec<Token> = Vec::new();

        while self.peek().is_some() {
            let base_char = self.peek().unwrap();

            match base_char {
                '\n'  => {
                    self.current_line += 1;
                    println!("line: {}", self.current_line);
                    },
                ';' => {  },
                _ => {}
            }

            self.consume()
        }

        tokens
    }
}