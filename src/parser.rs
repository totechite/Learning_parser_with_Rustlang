use lexer::Lexer;
use token::Token;

#[derive(Debug)]
pub struct Parser {
    input: Lexer,
    ahead_token: Result<Token, String>
}

impl Parser {

    pub fn new(lexer: Lexer)-> Parser{
        Parser{
            input: lexer,
            ahead_token: Ok(Token::N_A)
        }
    }

    fn match_type(&mut self, t: Token){
        match self.ahead_token {
            Ok(t) => self.consume(),
            _ => {}
        }
    }

    fn consume(&mut self){
        self.ahead_token = self.input.next_token();
    }

    pub fn list(&mut self){
        self.match_type(Token::L_BRACKET);
        self.elements();
        self.match_type(Token::R_BRACKET);
    }

    fn elements(&mut self){
        self.element();
        while self.ahead_token.unwrap()==Token::COMMA{
            self.match_type(Token::COMMA);
            self.element();
        }
    }

    fn element(&mut self){
        match &self.ahead_token{
            Ok(Token::NAME(x)) => self.match_type(self.ahead_token.unwrap()),
            Ok(Token::L_BRACKET) => self.list(),
            _ => {}
        }
    }
}