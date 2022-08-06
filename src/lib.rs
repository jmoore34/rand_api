use evaluate::evaluate_expression;
use parse::parse_expression;

mod parse;
mod evaluate;

#[derive(Clone, Debug, PartialEq)]

pub enum Expression {
    CoinFlip,
    IntRange(i64, i64),
    FloatRange(f32, f32)
}

pub fn evaluate(expression: &str) -> String {
    match parse_expression(expression) {
        Ok((_remainder, expression)) => evaluate_expression(expression),
        Err(e) => format!("Error: {e}"),
    }
}
