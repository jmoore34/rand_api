/// This module is responsible for parsing expressions

// We need to allow dead code because many parsing functions
// are referenced but not called w/ parentheses
#[allow(dead_code)]
mod parse_coin_flip;
#[allow(dead_code)]
mod parse_numbers;
#[allow(dead_code)]
mod parse_ranges;
mod parse_expression;
mod parse_dice_roll;

pub use parse_expression::parse_expression;