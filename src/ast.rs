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

pub struct ScopedVariable<'a> {
    name: &'a String,
    value: f64,
}

pub fn eval<'a>(
    expr: &'a Expression,
    vars: &mut Vec<ScopedVariable<'a>>,
    functions: &mut Vec<(&'a String, &'a [String], &'a Expression)>,
) -> Result<f64, String> {
    match expr {
        Expression::Number(x) => Ok(*x),
        Expression::Negation(a) => Ok(-eval(a, vars, functions)?),
        Expression::Add(a, b) => Ok(eval(a, vars, functions)? + eval(b, vars, functions)?),
        Expression::Subtract(a, b) => Ok(eval(a, vars, functions)? - eval(b, vars, functions)?),
        Expression::Multiply(a, b) => Ok(eval(a, vars, functions)? * eval(b, vars, functions)?),
        Expression::Divide(a, b) => Ok(eval(a, vars, functions)? / eval(b, vars, functions)?),
        Expression::Variable(name) => {
            if let Some(scope) = vars.iter().rev().find(|var| var.name == name) {
                Ok(scope.value)
            } else {
                Err(format!("Cannot find variable `{}` in scope", name))
            }
        }
        Expression::Let { name, rhs, then } => {
            let right = eval(rhs, vars, functions)?;
            vars.push(ScopedVariable { name, value: right });
            let output = eval(then, vars, functions);
            vars.pop();
            output
        }
        Expression::Call(name, args) => {
            if let Some((_, arg_names, body)) = functions
                .iter()
                .rev()
                .find(|(var, _, _)| *var == name)
                .copied()
            {
                if !(arg_names.len() == args.len()) {
                    Err(format!(
                        "Wrong number of arguments for function `{}`: expected {}, found {}",
                        name,
                        arg_names.len(),
                        args.len(),
                    ))
                } else {
                    let mut args = args
                        .iter()
                        .map(|arg| eval(arg, vars, functions))
                        .zip(arg_names.iter())
                        .map(|(val, name)| Ok(ScopedVariable { name, value: val? }))
                        .collect::<Result<_, String>>()?;

                    vars.append(&mut args);
                    let output = eval(body, vars, functions);
                    vars.truncate(vars.len() - args.len());

                    output
                }
            } else {
                Err(format!("Cannot find function `{}` in scope", name))
            }
        }
        Expression::Function {
            name,
            arguments,
            body,
            then,
        } => {
            functions.push((name, arguments, body));
            let output = eval(then, vars, functions);
            functions.pop();
            output
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use chumsky::Parser;

    /// This is a macro that makes it easier to assert the evaluation of the abstract syntax tree.
    /// It parses the source code, then evaluates the AST, then compares the result to the expected value.
    macro_rules! assert_eval {
        ($src:expr, $result:expr) => {
            match parser::parser().parse($src) {
                Ok(node) => match eval(&node, &mut Vec::new(), &mut Vec::new()) {
                    Ok(result) => {
                        assert_eq!(result, $result);
                    }
                    Err(err) => panic!("Evaluation error: {}", err),
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
