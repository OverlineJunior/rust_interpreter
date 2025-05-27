use crate::lexer::{Literal, Token};
use chumsky::{input::ValueInput, prelude::*};

#[derive(Debug)]
pub enum Expr<Ty> {
    Lit(Literal, Ty),
    Var(String, Ty),

    Neg(Box<Expr<Ty>>, Ty),
    Add(Box<Expr<Ty>>, Box<Expr<Ty>>, Ty),
    Sub(Box<Expr<Ty>>, Box<Expr<Ty>>, Ty),
    Mul(Box<Expr<Ty>>, Box<Expr<Ty>>, Ty),
    Div(Box<Expr<Ty>>, Box<Expr<Ty>>, Ty),

    Let(String, Box<Expr<Ty>>, Ty),
    Block(Vec<Expr<Ty>>, Ty),
}

#[allow(clippy::let_and_return)]
pub fn parser<'a, I>() -> impl Parser<'a, I, Expr<()>, extra::Err<Rich<'a, Token>>>
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    let name = select! {
        Token::Name(name) => name,
    };

    let expr = {
        let atom = (select! {
            Token::Int(lit) => Expr::Lit(lit, ()),
        })
        .or(name.map(|name| Expr::Var(name, ())));

        let unary = just(Token::Sub)
            .repeated()
            .foldr(atom, |_op, rhs| Expr::Neg(Box::new(rhs), ()));

        let product = unary.clone().foldl(
            choice((
                just(Token::Mul).to(Expr::Mul as fn(_, _, _) -> Expr<()>),
                just(Token::Div).to(Expr::Div as fn(_, _, _) -> Expr<()>),
            ))
            .then(unary)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs), ()),
        );

        let sum = product.clone().foldl(
            choice((
                just(Token::Add).to(Expr::Add as fn(_, _, _) -> Expr<()>),
                just(Token::Sub).to(Expr::Sub as fn(_, _, _) -> Expr<()>),
            ))
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs), ()),
        );

        sum
    };

    let decl = {
        let let_ = just(Token::Let)
			.ignore_then(name)
			.then_ignore(just(Token::Assign))
			.then(expr.clone())
			.map(|(name, value)| Expr::Let(name, Box::new(value), ()));

        let_.or(expr)
    };

    let program = decl
        .repeated()
        .collect()
        .map(|decls| Expr::Block(decls, ()))
        .then_ignore(end());

    program
}
