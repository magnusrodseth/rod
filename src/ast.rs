/// This is the language's abtract syntax tree (AST).
/// In this language, everything is an expression. Each expression may itself contain sub-expressions.
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(f64),
    Variable(String),

    Negation(Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),

    Call(String, Vec<Expression>),
    Let {
        name: String,
        rhs: Box<Expression>,
        then: Box<Expression>,
    },
    Function {
        name: String,
        arguments: Vec<String>,
        body: Box<Expression>,
        then: Box<Expression>,
    },
}

pub fn eval(expr: &Expression) -> Result<f64, String> {
    match expr {
        Expression::Number(x) => Ok(*x),
        Expression::Negation(a) => Ok(-eval(a)?),
        Expression::Add(a, b) => Ok(eval(a)? + eval(b)?),
        Expression::Subtract(a, b) => Ok(eval(a)? - eval(b)?),
        Expression::Multiply(a, b) => Ok(eval(a)? * eval(b)?),
        Expression::Divide(a, b) => Ok(eval(a)? / eval(b)?),
        _ => todo!(), // We'll handle other cases later
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use chumsky::Parser;

    /// This is a macro that makes it easier to write tests.
    /// It parses the source code, then evaluates the AST, then compares the result to the expected value.
    macro_rules! assert_eval {
        ($src:expr, $result:expr) => {
            match parser::parser().parse($src) {
                Ok(node) => match eval(&node) {
                    Ok(result) => {
                        assert_eq!(result, $result);
                    }
                    Err(err) => panic!("Eval error: {}", err),
                },
                Err(errors) => panic!("Parse error: {:?}", errors),
            }
        };
    }

    #[test]
    fn arithmetic() {
        assert_eval!("3 * 4 + 2", 14.0);
        assert_eval!("3 * (4 + 2)", 18.0);
        assert_eval!("-4 + 2", -2.0);
        assert_eval!("4 + -2", 2.0);
        assert_eval!("-(4 + 2)", -6.0);
    }
}
