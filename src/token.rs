#[derive(Debug, PartialEq)]
pub enum Token{
    N_A,
    EOF,
    NAME(String),
    COMMA,
    L_BRACKET,
    R_BRACKET
}
