mod lexer;
mod parser;

use chumsky::input::Stream;
use logos::Logos;
use lexer::Token;
use parser::parser;
use chumsky::prelude::*;

const SOURCE: &str = r"
    let x = 1 + 2 * 3
    let y = x + 2
";

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

    println!("{:#?}", ast);
}
