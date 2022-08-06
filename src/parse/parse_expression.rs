use nom::{IResult, branch::alt};

use crate::Expression;

use super::{parse_ranges::{parse_float_range, parse_int_range}, parse_coin_flip::parse_coin_flip};

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        // there is a separate float range and int range
        // because if the input is 1-5, we don't want floats
        // in that range, only integers
        parse_float_range,
        parse_int_range,
        parse_coin_flip
    ))
    (input)
}