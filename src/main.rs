mod ast;
mod parser;
use ast::eval;
use chumsky::Parser;

fn main() {
    // TODO: Use Clap for command line arguments
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    match parser::parser().parse(src) {
        Ok(node) => match eval(&node) {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("Eval error: {}", err),
        },
        Err(errors) => errors
            .into_iter()
            .for_each(|err| println!("Parse error: {}", err)),
    }
}
