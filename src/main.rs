use std::fs;
use std::io::{stdout, Write, BufWriter, prelude::*};

pub mod lexer;
use lexer::Lexer;
use lexer::LexerBuilder;


fn main(){
    // let string: String = "[a,b,[abc,ab,a]]".to_string();
    let mut file = fs::File::open("sample.md").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let mut lexer: Lexer = LexerBuilder::new().text(contents).build();
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let mut frag = true;
    while frag{
        let tc = lexer.next_token().unwrap();
        writeln!(out, "{:?}", &tc).unwrap();
        frag = tc.text != "<EOF>";
    }
}