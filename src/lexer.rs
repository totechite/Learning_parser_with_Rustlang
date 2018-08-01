#[derive(Debug)]

pub struct Lexer {
    text: String,
    position: u32,
    pub theChar: char,
}

impl Lexer {

    pub fn consume(&mut self) -> &mut Lexer{
            self.position += 1u32;
            self.theChar = self.text.chars().nth(self.position as usize).unwrap();
            self
    }

    pub fn match_char(&mut self, reqChar: char) -> Result<&mut Lexer, String>{
        if reqChar == self.theChar{
            Ok(self.consume())
        }else{
            Err("error".to_string())
        }
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
            theChar: self.text.clone().chars().next().unwrap()
        }
    }
}