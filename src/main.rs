use std::io::{stdout, Write, BufWriter};

pub mod lexer;
use lexer::Lexer;
use lexer::LexerBuilder;


fn main(){
    let string: String = "[a,b]".to_string();
    let mut lexer: Lexer = LexerBuilder::new().text(string).build();

    // let tc = &lexer.next_token().unwrap();

    // let out = stdout();
    // let mut out = BufWriter::new(out.lock());
    // writeln!(out, "{:?}", tc.theChar).unwrap();
}