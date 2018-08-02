extern crate regex;
use self::regex::Regex;

pub struct Token{
    char_type: TokenNames,
    text: String
}

enum TokenNames{
    N_A,
    EOF,
    NAME,
    COMMA,
    L_BRACKET,
    R_BRACKET
}

#[derive(Debug)]
pub struct Lexer {
    text: String,
    position: u32,
    pub theChar: Option<char>,
}

impl Lexer {

    pub fn consume(&mut self) -> &mut Lexer{
            self.position += 1u32;
            self.theChar = Some(self.text.chars().nth(self.position as usize).unwrap());
            self
    }

    pub fn match_char(&mut self, reqChar: char) -> Result<&mut Lexer, String>{
        if reqChar == self.theChar.unwrap(){
            Ok(self.consume())
        }else{
            Err("error".to_string())
        }
    }

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
    
    fn name_token(&self) -> Token{
        let mut strV = "";
        while self.is_letter() {
            strV = &[strV, &self.theChar.unwrap().to_string()].concat();
            self.consume();
        }
        Token{char_type: TokenNames::NAME, text: strV.to_string()}
    }

    pub fn next_token(&self) -> Result<Token, String>{
        while self.theChar!=None {
            match self.theChar {
                Some('\n')|Some('\t')|Some('\n')|Some('\r')
                => {
                    self.white_space();
                    continue;
                },
                Some(',') => {
                    self.consume();
                    Ok(Token{char_type: TokenNames::COMMA, text: ",".to_string()});
                },
                Some('[') => {
                    self.consume();
                    Ok(Token{char_type: TokenNames::L_BRACKET, text: "[".to_string()});
                },
                Some(']') => {
                    self.consume();
                    Ok(Token{char_type: TokenNames::R_BRACKET, text: "]".to_string()});
                },
                Some(_) => {
                    if self.is_letter(){
                        Ok(self.name_token());
                    }else{
                        Err("EuntimeError".to_string());
                    }
                }
            }
        }
        Ok(Token{char_type: TokenNames::EOF, text: "<EOF>".to_string()})
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