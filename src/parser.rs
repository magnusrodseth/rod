use crate::ast::Expression;
use chumsky::prelude::*;

pub fn parser() -> impl Parser<char, Expression, Error = Simple<char>> {
    let number = text::int(10)
        .map(|s: String| Expression::Number(s.parse().unwrap()))
        .padded();

    // TODO: This will become clear later
    let atom = number;

    let operation = |c| just(c).padded();

    let unary = operation('-')
        .repeated()
        .then(atom)
        .foldr(|_, rhs| Expression::Negation(Box::new(rhs)));

    unary.then_ignore(end())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_number() {
        let src = "123";
        let result = parser().parse(src).expect("Parse error");
        assert_eq!(result, Expression::Number(123.0));
    }

    #[test]
    fn simple_negation() {
        let src = "-123";
        let result = parser().parse(src).expect("Parse error");
        assert_eq!(
            result,
            Expression::Negation(Box::new(Expression::Number(123.0)))
        );
    }

    #[test]
    fn simple_negation_of_negation() {
        let src = "--123";
        let result = parser().parse(src).expect("Parse error");
        assert_eq!(
            result,
            Expression::Negation(Box::new(Expression::Negation(Box::new(
                Expression::Number(123.0)
            ))))
        );
    }
}
