use std::io::{stdout, Write, BufWriter};

pub mod lexer;
use lexer::Lexer;
use lexer::LexerBuilder;


fn main(){
    let string: String = "[a,b,[abc,ab,a]]".to_string();
    let mut lexer: Lexer = LexerBuilder::new().text(string).build();
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let mut frag = true;
    while frag{
        let tc = lexer.next_token().unwrap();
        writeln!(out, "{:?}", &tc).unwrap();
        frag = tc.text != "<EOF>";
    }
}