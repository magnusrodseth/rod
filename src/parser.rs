use crate::ast::Expression;
use chumsky::prelude::*;

pub fn parser() -> impl Parser<char, Expression, Error = Simple<char>> {
    let number = text::int(10)
        .map(|s: String| Expression::Number(s.parse().unwrap()))
        .padded();

    number.then_ignore(end())
}
