mod lexer;
mod parser;
mod analyzer;
mod interpreter;

use std::collections::HashMap;
use chumsky::input::Stream;
use logos::Logos;
use lexer::Token;
use parser::parser;
use chumsky::prelude::*;

const SOURCE: &str = r#"
    let x = 5 in
    let y = 10 in
    x + y
"#;

fn main() {
    let token_iter = Token::lexer(SOURCE)
        .spanned()
        .map(|(tok, span)| match tok {
            Ok(tok) => (tok, span.into()),
            Err(()) => panic!("Lexer error"),
        });

    let token_stream = Stream::from_iter(token_iter)
        .map((0..SOURCE.len()).into(), |(t, s): (_, _)| (t, s));

    let ast = parser().parse(token_stream).unwrap();

    println!("Untyped AST:\n{:#?}\n", ast);

    let typed_ast = ast.analyze(&mut HashMap::new()).unwrap();

    println!("Typed AST:\n{:#?}\n", typed_ast);

    let result = typed_ast.eval(&mut HashMap::new()).unwrap();
    println!("Result: {:?}", result);
}
