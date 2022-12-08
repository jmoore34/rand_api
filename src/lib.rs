use dice::atom::DiceExpressionAtom;
use evaluate::evaluate_expression;
use parse::parse_expression;


mod parse;
mod evaluate;
mod dice;

#[derive(Clone, Debug, PartialEq)]

pub enum Expression {
    CoinFlip,
    IntRange(i64, i64),
    FloatRange(f32, f32),
    DiceExpression(Vec<DiceExpressionAtom>)
}

pub fn evaluate(expression: &str) -> String {
    match parse_expression(expression) {
        Ok((_remainder, expression)) => evaluate_expression(expression),
        Err(e) => format!("Error: {e}"),
    }
}
