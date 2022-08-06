use nom::{branch::alt, IResult};
use nom::bytes::complete::{tag, take_while1, take_while};
use nom::character::complete::{one_of, digit0, digit1, multispace0};
use nom::combinator::{all_consuming, map, opt, recognize, value};
use nom::error::{ErrorKind, ParseError};
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, pair, separated_pair, tuple};
use rand::{thread_rng, Rng};

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    CoinFlip,
    IntRange(i32, i32),
    FloatRange(f32, f32)
}

fn parse_coin_flip(input: &str) -> IResult<&str, Expression> {
    value(Expression::CoinFlip, tag("coin"))
    (input)
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit() || c == '_'
}

// parse 0 or more digits (including underscore)
fn parse_digit0(input: &str) -> IResult<&str, &str> {
    take_while(is_digit)
    (input)
}

// parse 1 or more digits (including underscore)
fn parse_digit1(input: &str) -> IResult<&str, &str> {
    take_while1(is_digit)
    (input)
}

fn parse_signed_integer(input: &str) -> IResult<&str, &str> {
    recognize(
        pair(
            opt(alt((
                tag("+"),
                tag("-")
            ))),
            parse_digit1
        )
    )
    (input)
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    let (remain, raw_int) = parse_signed_integer(input)?;

    match raw_int.parse::<i64>() {
        Ok(i) => Ok((remain, i)),
        Err(_) => todo!(),
    }
}

fn parse_fraction_part(input: &str) -> IResult<&str, &str> {
    recognize(
        pair(
            tag("."),
            parse_digit0 // allow e.g. "5."
        )
    )
    (input)
}

fn parse_exponent(input: &str) -> IResult<&str, &str> {
    recognize(
        pair(
            alt((
                tag("e"),
                tag("E")
            )), 
            parse_signed_integer
        )
    )
    (input)   
}

// parse a float
// must contain decimal point and/or exponent
fn parse_float(input: &str) -> IResult<&str, &str> {
    recognize(
        tuple((
            opt(alt((
                tag("+"),
                tag("-")
            ))),
            parse_digit0,
            alt((
                parse_exponent,
                recognize(pair(
                    parse_fraction_part,
                    opt(parse_exponent)
                ))
            ))
        ))
    )
    (input)
}


fn parse_f32(input: &str) -> IResult<&str, f32> {
    let (remain, raw_float) = parse_float(input)?;

    match raw_float.parse::<f32>() {
        Ok(i) => Ok((remain, i)),
        Err(_) => todo!(),
    }
}

fn parse_float_range(input: &str) -> IResult<&str, Expression> {
    let (remain, (min, max)) = separated_pair(
        parse_f32,
        tag("-"),
        parse_f32
    )
    (input)?;

    Ok((remain, Expression::FloatRange(min, max)))
}

#[cfg(test)]
mod tests {
    use crate::{parse_coin_flip, Expression, parse_f32, parse_i64, parse_float_range};

    #[test]
    fn test_coin_flip() {
        assert_eq!(parse_coin_flip("coin"), Ok(("", Expression::CoinFlip)))
    }

    #[test]
    fn test_int() {
        assert_eq!(parse_i64("5"), Ok(("", 5i64)));
        assert_eq!(parse_i64("+5"), Ok(("", 5i64)));
    }

    #[test]
    fn test_float() {
        assert_eq!(parse_f32("5.0"), Ok(("", 5.0)));
        assert_eq!(parse_f32("5."), Ok(("", 5.0)));
        assert_eq!(parse_f32("5e1"), Ok(("", 50.0)));
        assert_eq!(parse_f32("+0052.0052e-1"), Ok(("", 52.0052e-1)));
        assert_eq!(parse_f32(".5"), Ok(("", 0.5)));
        assert_eq!(parse_f32(".5e+0"), Ok(("", 0.5)));
        assert_eq!(parse_f32("+.5e+0"), Ok(("", 0.5)));
    }

    #[test]
    fn test_float_range() {
        assert_eq!(parse_float_range("5.0-10.0"), Ok(("", Expression::FloatRange(5.0, 10.0))));

    }
}
