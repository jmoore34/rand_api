use nom::{combinator::value, IResult, bytes::complete::tag};

use crate::Expression;

pub fn parse_coin_flip(input: &str) -> IResult<&str, Expression> {
    value(Expression::CoinFlip, tag("coin"))
    (input)
}

#[cfg(test)]
mod tests {
    use crate::Expression;

    use super::parse_coin_flip;

    #[test]
    fn test_coin_flip() {
        assert_eq!(parse_coin_flip("coin"), Ok(("", Expression::CoinFlip)))
    }
}