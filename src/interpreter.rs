use std::collections::HashMap;

use crate::{analyzer::Type, lexer::Literal, parser::Expr};

pub type Env = HashMap<String, Literal>;

const UNREACHABLE_AFTER_ANALYZER: &str = "Should be unreachable after semantic analysis";

impl Expr<Type> {
    pub fn eval(&self, env: &mut Env) -> Result<Literal, String> {
        match self {
            Expr::Lit(lit, _) => Ok(lit.clone()),

            Expr::Var(name, _) => env
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable `{name}`")),

            Expr::Neg(expr, _) => match expr.eval(env)? {
                Literal::Int(n) => Ok(Literal::Int(-n)),
                _ => panic!("{UNREACHABLE_AFTER_ANALYZER}"),
            },

            Expr::Add(lhs, rhs, _) => match (lhs.eval(env)?, rhs.eval(env)?) {
                (Literal::Int(lhs), Literal::Int(rhs)) => Ok(Literal::Int(lhs + rhs)),
                _ => panic!("{UNREACHABLE_AFTER_ANALYZER}"),
            },

			Expr::Sub(lhs, rhs, _) => match (lhs.eval(env)?, rhs.eval(env)?) {
				(Literal::Int(lhs), Literal::Int(rhs)) => Ok(Literal::Int(lhs - rhs)),
				_ => panic!("{UNREACHABLE_AFTER_ANALYZER}"),
			},

			Expr::Mul(lhs, rhs, _) => match (lhs.eval(env)?, rhs.eval(env)?) {
				(Literal::Int(lhs), Literal::Int(rhs)) => Ok(Literal::Int(lhs * rhs)),
				_ => panic!("{UNREACHABLE_AFTER_ANALYZER}"),
			},

			Expr::Div(lhs, rhs, _) => match (lhs.eval(env)?, rhs.eval(env)?) {
				(Literal::Int(lhs), Literal::Int(rhs)) => {
					if rhs == 0 {
						Err("Attempt to divide by zero".to_string())
					} else {
						Ok(Literal::Int(lhs / rhs))
					}
				}
				_ => panic!("{UNREACHABLE_AFTER_ANALYZER}"),
			},

			Expr::Let {
				name,
				value,
				then,
				..
			} => {
				let value = value.eval(env)?;
				env.insert(name.clone(), value.clone());

				let then = then.eval(env)?;

				// Variable goes out of scope after it's used by `then`.
				env.remove(name);

				Ok(then)
			},
        }
    }
}
