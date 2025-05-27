use logos::Logos;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    #[regex("[0-9]+", |lex| Literal::Int(lex.slice().parse::<i64>().unwrap()))]
    Int(Literal),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Name(String),

    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("=")]
    Assign,

    #[token("let")]
    Let,
    #[token("in")]
    In,
}
