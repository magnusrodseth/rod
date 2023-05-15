mod ast;
mod parser;
use chumsky::Parser;

fn main() {
    // TODO: Use Clap for command line arguments
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    println!("{:?}", parser::parser().parse(src));
}
