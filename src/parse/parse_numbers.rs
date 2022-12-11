use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    combinator::{opt, recognize},
    sequence::{pair, tuple},
    IResult, character::complete::space0,
};

/// Tell if char is digit or underscore
/// so numbers can have underscores in them
/// e.g. 1_000_000
fn is_digit(c: char) -> bool {
    c.is_ascii_digit() || c == '_'
}

/// parse 0 or more digits (including underscore)
fn parse_digit0(input: &str) -> IResult<&str, &str> {
    take_while(is_digit)(input)
}

/// parse 1 or more digits (including underscore)
fn parse_digit1(input: &str) -> IResult<&str, &str> {
    take_while1(is_digit)(input)
}

/// parse an integer with an optional sign (+/-)
/// result is a &str (reference to where the int was found in the input).
/// use another function like parse_i64 for an numeric result
pub fn parse_signed_integer_raw(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        opt(alt((tag("+"), tag("-")))),
        space0,
        parse_digit1
    )))(input)
}

pub fn parse_signed_integer<T: FromStr>(input: &str) -> IResult<&str, T> {
    let (remain, raw_num) = parse_signed_integer_raw(input)?;

    match raw_num.parse::<T>() {
        Ok(i) => Ok((remain, i)),
        Err(err) => todo!(),
    }
}

pub fn parse_unsigned_integer<T: FromStr>(input: &str) -> IResult<&str, T> {
    let (remain, raw_num) = parse_digit1(input)?;

    match raw_num.parse::<T>() {
        Ok(i) => Ok((remain, i)),
        Err(err) => todo!(),
    }
}

/// parse the function part of a float
/// e.g. the .5 in 1.5
fn parse_fraction_part(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        tag("."),
        parse_digit0, // allow e.g. "5."
    ))(input)
}

/// parse the exponent part of a float
/// e.g. the e2 in the 1.5e2
fn parse_exponent(input: &str) -> IResult<&str, &str> {
    recognize(pair(alt((tag("e"), tag("E"))), parse_signed_integer_raw))(input)
}

/// parse a float, returning the part of the str that has the float
/// must contain decimal point and/or exponent
fn parse_float(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        opt(alt((tag("+"), tag("-")))),
        parse_digit0,
        alt((
            parse_exponent,
            recognize(pair(parse_fraction_part, opt(parse_exponent))),
        )),
    )))(input)
}

/// strictly parse f32: must have dot and/or exponent
pub fn parse_f32(input: &str) -> IResult<&str, f32> {
    let (remain, raw_float) = parse_float(input)?;

    match raw_float.parse::<f32>() {
        Ok(i) => Ok((remain, i)),
        Err(_) => todo!(),
    }
}

/// flexibly parse f32: i.e., the input can be an integer
/// w/o decimal or exponent
/// and it will be cast to f32
pub fn flexible_parse_f32(input: &str) -> IResult<&str, f32> {
    let (remain, raw_float) = alt((parse_float, parse_signed_integer_raw))(input)?;

    match raw_float.parse::<f32>() {
        Ok(i) => Ok((remain, i)),
        Err(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::parse_numbers::{parse_f32, parse_signed_integer, parse_unsigned_integer};

    #[test]
    fn test_signed_int() {
        assert_eq!(parse_signed_integer::<i8>("5"), Ok(("", 5i8)));
        assert_eq!(parse_signed_integer::<i8>("+5"), Ok(("", 5i8)));
    }

    #[test]
    fn test_unsigned_int() {
        assert_eq!(parse_unsigned_integer::<u8>("5"), Ok(("", 5u8)));
        assert!(parse_unsigned_integer::<u8>("+5").is_err());
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
}
