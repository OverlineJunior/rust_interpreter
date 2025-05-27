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

    Let {
        name: String,
        value: Box<Expr<Ty>>,
        then: Box<Expr<Ty>>,
        ty: Ty,
    },
}

impl<Ty> Expr<Ty> {
    pub fn ty(&self) -> &Ty {
        match self {
            Expr::Lit(_, ty) => ty,
            Expr::Var(_, ty) => ty,
            Expr::Neg(_, ty) => ty,
            Expr::Add(_, _, ty) => ty,
            Expr::Sub(_, _, ty) => ty,
            Expr::Mul(_, _, ty) => ty,
            Expr::Div(_, _, ty) => ty,
            Expr::Let { ty, .. } => ty,
        }
    }
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
        // atom -> int | string | var
        let atom = (select! {
            Token::Int(lit) => Expr::Lit(lit, ()),
            Token::String(lit) => Expr::Lit(lit, ()),
        })
        .or(name.map(|name| Expr::Var(name, ())));

        // unary -> atom | '-' unary
        let unary = just(Token::Sub)
            .repeated()
            .foldr(atom, |_op, rhs| Expr::Neg(Box::new(rhs), ()));

        // product -> unary ( '*' unary | '/' unary )*
        let product = unary.clone().foldl(
            choice((
                just(Token::Mul).to(Expr::Mul as fn(_, _, _) -> Expr<()>),
                just(Token::Div).to(Expr::Div as fn(_, _, _) -> Expr<()>),
            ))
            .then(unary)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs), ()),
        );
        // sum -> product ( '+' product | '-' product )*
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

    let decl = recursive(|decl| {
        // let -> 'let' ident '=' expr 'in' decl
        let let_ = just(Token::Let)
			.ignore_then(name)
			.then_ignore(just(Token::Assign))
			.then(expr.clone())
            .then_ignore(just(Token::In))
            .then(decl)
			.map(|((name, value), then)| Expr::Let {
                name,
                value: Box::new(value),
                then: Box::new(then),
                ty: (),
            });

        // let -> let | expr
        let_.or(expr)
    });

    decl
}
