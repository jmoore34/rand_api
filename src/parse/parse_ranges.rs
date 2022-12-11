use nom::{IResult, branch::alt, sequence::separated_pair, bytes::complete::tag};

use crate::Expression;

use super::parse_numbers::{parse_f32, flexible_parse_f32, parse_signed_integer};
use super::parse_whitespace::spaced;

/// parse a float range, e.g. 1.0-5.0
pub fn parse_float_range(input: &str) -> IResult<&str, Expression> {
    let (remain, (min , max)) = alt((
        // handle these cases:
        // <float> - <integer>
        // <float> - <float>
        separated_pair(
            parse_f32,
            spaced(tag("-")),
            flexible_parse_f32
        ),
        // handle this case:
        // <integer> - <float>
        separated_pair(
            parse_signed_integer::<f32>,
            spaced(tag("-")),
            parse_f32
        )
    ))
    (input)?;

    Ok((remain, Expression::FloatRange(min, max)))
}

/// parse an int range, e.g. 1-5
pub fn parse_int_range(input: &str) -> IResult<&str, Expression> {
    let (remain, (min , max)) =
        separated_pair(
            parse_signed_integer::<i64>,
            spaced(tag("-")),
            parse_signed_integer::<i64>
        )
        (input)?;

    Ok((remain, Expression::IntRange(min, max)))
}


#[cfg(test)]
mod tests {
    use crate::{parse::parse_ranges::{parse_float_range, parse_int_range}, Expression};

    #[test]
    fn test_float_range() {
        assert_eq!(parse_float_range("5.0-10.0"), Ok(("", Expression::FloatRange(5.0, 10.0))));
        assert_eq!(parse_float_range("5-10.0"), Ok(("", Expression::FloatRange(5.0, 10.0))));
        assert_eq!(parse_float_range("5.0-10"), Ok(("", Expression::FloatRange(5.0, 10.0))));
        assert!(parse_float_range("5-10").is_err());
    }

    #[test]
    fn test_float_range_whitespace() {
        assert_eq!(parse_float_range("5.0 - 10.0"), Ok(("", Expression::FloatRange(5.0, 10.0))));
        assert_eq!(parse_float_range("5 - 10.0"), Ok(("", Expression::FloatRange(5.0, 10.0))));
        assert_eq!(parse_float_range("5.0 - 10"), Ok(("", Expression::FloatRange(5.0, 10.0))));
        assert!(parse_float_range("5 - 10").is_err());
    }

    #[test]
    fn test_int_range() {
        assert_eq!(parse_int_range("5-10"), Ok(("", Expression::IntRange(5,10))));
        assert!(parse_int_range("5.0-10").is_err());
    }

    #[test]
    fn test_int_range_whitespace() {
        assert_eq!(parse_int_range("5 - 10"), Ok(("", Expression::IntRange(5,10))));
        assert!(parse_int_range("5.0 - 10").is_err());
    }
}