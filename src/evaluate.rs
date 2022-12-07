
use rand::{random, thread_rng, Rng};

use crate::Expression;

pub fn evaluate_expression(expression: Expression) -> String {
    match expression {
        Expression::CoinFlip => {
            if random() {
                "Heads"
            } else {
                "Tails"
            }.to_string()
        },
        Expression::IntRange(min, max) => {
            let mut rng = thread_rng();
            rng.gen_range(min..=max).to_string()
        },
        Expression::FloatRange(min, max) => {
            let mut rng = thread_rng();
            rng.gen_range(min..=max).to_string()
        },
        x@Expression::DiceExpression(_) => {
            dbg!(x);
            "".to_owned()
        },

    }
}