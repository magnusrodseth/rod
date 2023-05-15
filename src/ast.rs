/// This is the language's abtract syntax tree (AST).
/// In this language, everything is an expression. Each expression may itself contain sub-expressions.
#[derive(Debug)]
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
