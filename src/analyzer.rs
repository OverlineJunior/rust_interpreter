use crate::{lexer::Literal, parser::Expr};
use std::collections::HashMap;

pub type Env = HashMap<String, Type>;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    String,
}

impl Expr<()> {
    pub fn analyze(&self, env: &mut Env) -> Result<Expr<Type>, String> {
        match self {
            Expr::Lit(lit, _) => match lit {
                Literal::Int(_) => Ok(Expr::Lit(lit.clone(), Type::Int)),
                Literal::String(_) => Ok(Expr::Lit(lit.clone(), Type::String)),
            },

            Expr::Var(name, _) => {
                if let Some(ty) = env.get(name) {
                    Ok(Expr::Var(name.clone(), ty.clone()))
                } else {
                    Err(format!("Undefined variable `{name}`"))
                }
            }

            Expr::Neg(expr, _) => {
                let expr = expr.analyze(env)?;

                if *expr.ty() == Type::Int {
                    Ok(Expr::Neg(Box::new(expr), Type::Int))
                } else {
                    Err(format!(
                        "Cannot apply unary operator `-` to type `{:?}`",
                        expr.ty()
                    ))
                }
            }

            Expr::Add(lhs, rhs, _) => {
                let lhs = lhs.analyze(env)?;
                let rhs = rhs.analyze(env)?;

                if *lhs.ty() == Type::Int && *rhs.ty() == Type::Int {
                    Ok(Expr::Add(Box::new(lhs), Box::new(rhs), Type::Int))
                } else {
                    Err(format!(
                        "Cannot apply binary operator `+` to types `{:?}` and `{:?}`",
                        lhs.ty(),
                        rhs.ty()
                    ))
                }
            }

            Expr::Sub(lhs, rhs, _) => {
                let lhs = lhs.analyze(env)?;
                let rhs = rhs.analyze(env)?;

                if *lhs.ty() == Type::Int && *rhs.ty() == Type::Int {
                    Ok(Expr::Sub(Box::new(lhs), Box::new(rhs), Type::Int))
                } else {
                    Err(format!(
                        "Cannot apply binary operator `-` to types `{:?}` and `{:?}`",
                        lhs.ty(),
                        rhs.ty()
                    ))
                }
            }

            Expr::Mul(lhs, rhs, _) => {
                let lhs = lhs.analyze(env)?;
                let rhs = rhs.analyze(env)?;

                if *lhs.ty() == Type::Int && *rhs.ty() == Type::Int {
                    Ok(Expr::Mul(Box::new(lhs), Box::new(rhs), Type::Int))
                } else {
                    Err(format!(
                        "Cannot apply binary operator `*` to types `{:?}` and `{:?}`",
                        lhs.ty(),
                        rhs.ty()
                    ))
                }
            }

            Expr::Div(lhs, rhs, _) => {
                let lhs = lhs.analyze(env)?;
                let rhs = rhs.analyze(env)?;

                if *lhs.ty() == Type::Int && *rhs.ty() == Type::Int {
                    Ok(Expr::Div(Box::new(lhs), Box::new(rhs), Type::Int))
                } else {
                    Err(format!(
                        "Cannot apply binary operator `/` to types `{:?}` and `{:?}`",
                        lhs.ty(),
                        rhs.ty()
                    ))
                }
            }

            Expr::Let {
                name,
                value,
                then,
                ty: _,
            } => {
                let value = value.analyze(env)?;
                env.insert(name.clone(), value.ty().clone());

                let then = then.analyze(env)?;
                let ty = then.ty().clone();

                // Variable goes out of scope after it's used by `then`.
                env.remove(name);

                Ok(Expr::Let {
                    name: name.clone(),
                    value: Box::new(value),
                    then: Box::new(then),
                    ty,
                })
            }
        }
    }
}
