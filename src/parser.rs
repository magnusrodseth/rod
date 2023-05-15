use crate::ast::Expression;
use chumsky::prelude::*;

pub fn parser() -> impl Parser<char, Expression, Error = Simple<char>> {
    let identifier = text::ident().padded();

    let expression = recursive(|expression| {
        let number = text::int(10)
            .map(|s: String| Expression::Number(s.parse().unwrap()))
            .padded();

        let call = identifier
            .then(
                expression
                    .clone()
                    .separated_by(just(','))
                    .allow_trailing()
                    .delimited_by(just('('), just(')')),
            )
            .map(|(f, args)| Expression::Call(f, args));

        // We call things that behave like single values 'atoms' by convention.
        let atom = number
            .or(expression.delimited_by(just('('), just(')')))
            .or(call)
            .or(identifier.map(Expression::Variable))
            .padded();

        let op = |c| just(c).padded();

        let unary = op('-')
            .repeated()
            .then(atom)
            .foldr(|_, rhs| Expression::Negation(Box::new(rhs)));

        let product = unary
            .clone()
            .then(
                op('*')
                    .to(Expression::Multiply as fn(_, _) -> _)
                    .or(op('/').to(Expression::Divide as fn(_, _) -> _))
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum = product
            .clone()
            .then(
                op('+')
                    .to(Expression::Add as fn(_, _) -> _)
                    .or(op('-').to(Expression::Subtract as fn(_, _) -> _))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        sum
    });

    let declaration = recursive(|declaration| {
        let r#let = text::keyword("let")
            .ignore_then(identifier)
            .then_ignore(just('='))
            .then(expression.clone())
            .then_ignore(just(";"))
            .then(declaration.clone())
            .map(|((name, rhs), then)| Expression::Let {
                name,
                rhs: Box::new(rhs),
                then: Box::new(then),
            });

        let r#fn = text::keyword("fn")
            .ignore_then(identifier)
            .then(identifier.repeated())
            .then_ignore(just('='))
            .then(expression.clone())
            .then_ignore(just(';'))
            .then(declaration)
            .map(|(((name, arguments), body), then)| Expression::Function {
                name,
                arguments,
                body: Box::new(body),
                then: Box::new(then),
            });

        // To avoid the parser accidentally deciding that "let" is a variable,
        // we place r#let earlier in the or chain than expression so that
        // it prioritises the correct interpretation.
        r#let.or(r#fn).or(expression).padded()
    });

    declaration.then_ignore(end())
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

    #[test]
    fn simple_addition() {
        let src = "1 + 2";
        let result = parser().parse(src).expect("Parse error");
        assert_eq!(
            result,
            Expression::Add(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )
        );
    }

    #[test]
    fn simple_subtraction() {
        let src = "1 - 2";
        let result = parser().parse(src).expect("Parse error");
        assert_eq!(
            result,
            Expression::Subtract(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )
        );
    }

    #[test]
    fn simple_multiplication() {
        let src = "1 * 2";
        let result = parser().parse(src).expect("Parse error");
        assert_eq!(
            result,
            Expression::Multiply(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )
        );
    }

    #[test]
    fn simple_division() {
        let src = "1 / 2";
        let result = parser().parse(src).expect("Parse error");
        assert_eq!(
            result,
            Expression::Divide(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0))
            )
        );
    }

    #[test]
    fn add_and_subtract() {
        let src = "2 + 3 - 7 + 5";
        let result = parser().parse(src).expect("Parse error");
        assert_eq!(
            result,
            Expression::Add(
                Box::new(Expression::Subtract(
                    Box::new(Expression::Add(
                        Box::new(Expression::Number(2.0)),
                        Box::new(Expression::Number(3.0))
                    )),
                    Box::new(Expression::Number(7.0))
                )),
                Box::new(Expression::Number(5.0))
            )
        );
    }
}
