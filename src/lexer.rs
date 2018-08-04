extern crate regex;
use self::regex::Regex;

use token::Token;

#[derive(Debug)]
pub struct Lexer {
    text: String,
    position: u32,
    pub theChar: Option<char>,
}

impl Lexer {

    pub fn consume(&mut self) -> &mut Lexer{
            self.position += 1u32;
            self.theChar = self.text.chars().nth(self.position as usize);
            self
    }

    // pub fn match_char(&mut self, reqChar: char) -> Result<&mut Lexer, String>{
    //     if reqChar == self.theChar.unwrap(){
    //         Ok(self.consume())
    //     }else{
    //         Err("error".to_string())
    //     }
    // }

    fn is_letter(&self) -> bool{
        let re = Regex::new(r"[a-z]|[A-Z]").unwrap();
        re.is_match(&self.theChar.unwrap().to_string())
    }

    fn white_space(&mut self){
        let re = Regex::new("[:space:]").unwrap();
        while re.is_match(&self.theChar.unwrap().to_string()) {
            self.consume();
        }
    }
    
    fn name_token(&mut self) -> Token{
        let mut strV = "".to_string();
        while self.is_letter() {
            strV += &self.theChar.unwrap().to_string();
            self.consume();
        }
        Token::NAME(strV.to_string())
    }

    pub fn next_token(&mut self) -> Result<Token, String>{
        while self.theChar!=None {
            match self.theChar.unwrap() {
                '\n'|'\t'|'\r'
                => {
                    self.white_space();
                    continue;
                    },
                ',' => {
                    self.consume();
                    return Ok(Token::COMMA);
                },
                '[' => {
                    self.consume();
                    return Ok(Token::L_BRACKET);
                },
                ']' => {
                    self.consume();
                    return Ok(Token::R_BRACKET);
                },
                _ => {
                    if self.is_letter(){
                        return Ok(self.name_token());
                    }else{
                        return Err("RuntimeError".to_string());
                    }
                }
            }
        }
        Ok(Token::EOF)
    }
}

pub struct LexerBuilder{
    text: String,
    position: u32,
    theChar: char,
}

impl LexerBuilder{
    pub fn new() -> LexerBuilder{
        LexerBuilder{
            text: "".to_string(),
            position: 0,
            theChar: ' ',
        }
    }

    pub fn text(&mut self, elem: String) -> &mut LexerBuilder{
        self.text = elem;
        self
    }

    pub fn position(&mut self, elem: u32) -> &mut LexerBuilder{
        self.position = elem;
        self
    }

    pub fn build(&self) -> Lexer{
        Lexer{
            text: self.text.clone(),
            position: self.position,
            theChar: Some(self.text.clone().chars().next().unwrap())
        }
    }
}