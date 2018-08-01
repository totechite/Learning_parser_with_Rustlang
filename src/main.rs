use std::io::{stdout, Write, BufWriter};

pub mod lexer;
use lexer::Lexer;
use lexer::LexerBuilder;

fn main(){
    let string: String = "[a,b]".to_string();
    let mut lexer: Lexer = LexerBuilder::new().text(string).build();
    let tc  = &lexer.match_char('[').unwrap();
    // for _ in 0..10_000_000{
    //     println!("{:?}", tc.theChar);
    // }
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for _ in 0..10_000_000{
        writeln!(out, "{:?}", tc.theChar).unwrap();
    }

}