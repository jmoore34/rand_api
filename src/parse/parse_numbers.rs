use nom::{IResult, bytes::complete::{take_while, tag, take_while1}, combinator::{recognize, opt}, sequence::{pair, tuple}, branch::alt};

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

pub fn parse_signed_integer(input: &str) -> IResult<&str, &str> {
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

pub fn parse_i64(input: &str) -> IResult<&str, i64> {
    let (remain, raw_int) = parse_signed_integer(input)?;

    match raw_int.parse::<i64>() {
        Ok(i) => Ok((remain, i)),
        Err(_) => todo!(),
    }
}

// parse an i64 (i.e., no decimal point or exponent)
// and cast it to a f32
pub fn parse_i64_as_f32(input: &str) -> IResult<&str, f32> {
    let (remain, raw_int) = parse_signed_integer(input)?;

    match raw_int.parse::<f32>() {
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

// strict f32: must have dot and/or exponent
pub fn parse_f32(input: &str) -> IResult<&str, f32> {
    let (remain, raw_float) = parse_float(input)?;

    match raw_float.parse::<f32>() {
        Ok(i) => Ok((remain, i)),
        Err(_) => todo!(),
    }
}

pub fn flexible_parse_f32(input: &str) -> IResult<&str, f32> {
    let (remain, raw_float) = alt((
        parse_float,
        parse_signed_integer
    ))(input)?;

    match raw_float.parse::<f32>() {
        Ok(i) => Ok((remain, i)),
        Err(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::parse_numbers::{parse_i64, parse_f32};

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
}