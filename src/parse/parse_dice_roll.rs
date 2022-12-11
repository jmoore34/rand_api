use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{opt, recognize},
    multi::{many0, many1},
    sequence::{pair, tuple, preceded},
    IResult, character::complete::space0,
};

use crate::{
    dice::{
        advantage::AdvantageStatus,
        atom::DiceExpressionAtom,
        keepdrop::{HighestOrLowest, KeepDrop, KeepOrDrop},
    },
    Expression,
};

use super::{parse_numbers::{parse_signed_integer, parse_unsigned_integer}, parse_whitespace::spaced};

pub fn parse_dice_expression(input: &str) -> IResult<&str, Expression> {
    let (remain, atoms) = many1(parse_dice_expression_atom)(input)?;

    Ok((remain, Expression::DiceExpression(atoms)))
}

fn parse_dice_expression_atom(input: &str) -> IResult<&str, DiceExpressionAtom> {
    preceded(
        // Optional leading whitespace
        space0,
        alt((parse_dice_roll_atom, parse_constant_atom))
    )(input)
}

fn parse_constant_atom(input: &str) -> IResult<&str, DiceExpressionAtom> {
    let (remain, constant) = parse_signed_integer::<i8>(input)?;
    Ok((remain, DiceExpressionAtom::Constant(constant)))
}

fn parse_dice_roll_atom(input: &str) -> IResult<&str, DiceExpressionAtom> {
    let (remain, ((sign, _, number_of_dice, _, number_of_sides), modifiers)) = pair(
        // First: quantity and number of sides, e.g. 3d6 (required)
        tuple((
            // Optional +/- sign, with possible whitespace after it
            // When it is -, this roll will be subtracted
            opt(alt((tag("+"), tag("-")))),
            space0,
            // Optional number of dice (defaults to 1)
            opt(parse_unsigned_integer::<u8>),
            tag("d"),
            // Number of sides on each dice
            parse_unsigned_integer::<u8>,
        )),
        // Second: parse modifiers (rerolls, advantage, etc) -- optional
        many0(parse_roll_modifier),
    )(input)?;
    let number_of_dice = number_of_dice.unwrap_or(1);
    let subtracted = match sign {
        Some("-") => true,
        // If plus sign, or sign omitted, then the roll is not subtracted
        _ => false,
    };
    let (advantage_status, reroll, drop_keep) = {
        let mut advantage_status = AdvantageStatus::None;
        let mut reroll = None;
        let mut keep_drop = vec![];

        for modifier in modifiers {
            match modifier {
                RollModifier::Reroll(r) => {
                    // todo: throw error if reroll already set
                    // or set to the higher of the existing and the new one
                    reroll = Some(r)
                }
                RollModifier::Disadvantage => {
                    // todo: throw error if advantage status already set
                    advantage_status = AdvantageStatus::Disadvantage
                }
                RollModifier::Advantage => advantage_status = AdvantageStatus::Advantage,
                RollModifier::KeepDrop(kd) => keep_drop.push(kd),
            }
        }
        (advantage_status, reroll, keep_drop)
    };

    Ok((
        remain,
        DiceExpressionAtom::Roll {
            number_of_dice,
            number_of_sides,
            advantage_status,
            keep_drop: drop_keep,
            reroll,
            subtracted,
        },
    ))
}

#[derive(Debug)]
enum RollModifier {
    Reroll(u8),
    Disadvantage,
    Advantage,
    KeepDrop(KeepDrop),
}

fn parse_roll_modifier(input: &str) -> IResult<&str, RollModifier> {
    alt((
        parse_reroll_modifier,
        parse_keep_drop,
        parse_advantage_modifier,
        parse_disadvantage_modifier,
    ))(input)
}

fn parse_reroll_modifier(input: &str) -> IResult<&str, RollModifier> {
    let (remain, (_, sides)) = pair(tag("r"), parse_unsigned_integer::<u8>)(input)?;

    Ok((remain, RollModifier::Reroll(sides)))
}

/// Parse "a", "adv", "advantage"
fn parse_advantage_modifier(input: &str) -> IResult<&str, RollModifier> {
    let (remain, _) = recognize(pair(tag("a"), opt(pair(tag("dv"), opt(tag("antage"))))))(input)?;

    Ok((remain, RollModifier::Advantage))
}

/// Parse "d", "dis", "disadv", "disadvantage"
fn parse_disadvantage_modifier(input: &str) -> IResult<&str, RollModifier> {
    let (remain, _) = recognize(pair(
        tag("d"),
        opt(pair(tag("is"), opt(parse_advantage_modifier))),
    ))(input)?;

    Ok((remain, RollModifier::Disadvantage))
}

fn parse_keep_drop(input: &str) -> IResult<&str, RollModifier> {
    let (remain, (keep_or_drop, highest_or_lowest, amount)) = tuple((
        alt((tag("k"), tag("d"))),
        opt(alt((tag("h"), tag("l")))),
        parse_unsigned_integer::<u8>,
    ))(input)?;

    let keep_or_drop = match keep_or_drop {
        "k" => KeepOrDrop::Keep,
        "d" => KeepOrDrop::Drop,
        _ => unreachable!(),
    };

    let highest_or_lowest = match highest_or_lowest {
        // If highest or lowest is provided, we use it
        Some("h") => HighestOrLowest::Highest,
        Some("l") => HighestOrLowest::Lowest,
        Some(_) => unreachable!(),

        // Else, if it's  not provided, we need to find the default.
        // The default for keep is highest, and the default for
        // drop is lowest.
        None => match keep_or_drop {
            KeepOrDrop::Keep => HighestOrLowest::Highest,
            KeepOrDrop::Drop => HighestOrLowest::Lowest,
        },
    };

    Ok((
        remain,
        RollModifier::KeepDrop(KeepDrop {
            keep_or_drop,
            amount,
            highest_or_lowest,
        }),
    ))
}

#[cfg(test)]
mod tests {
    use crate::{DiceExpressionAtom::*, parse::parse_dice_roll::*, dice::{
        advantage::*, keepdrop::*
    }};
    #[test]
    fn constant() {
        assert_eq!(parse_dice_expression_atom("-7"), Ok(("", DiceExpressionAtom::Constant(-7))));
        assert_eq!(parse_dice_expression_atom("+7"), Ok(("", DiceExpressionAtom::Constant(7))));
    }

    #[test]
    fn kitchen_sink() {
        assert_eq!(parse_dice_expression("-6d6r2d2dh2+d20a-5"), Ok(("",
        Expression::DiceExpression(vec![
            Roll {
                number_of_dice: 6,
                number_of_sides: 6,
                advantage_status: AdvantageStatus::None,
                keep_drop: vec![
                    KeepDrop {
                        keep_or_drop: KeepOrDrop::Drop,
                        amount: 2,
                        highest_or_lowest: HighestOrLowest::Lowest
                    },
                    KeepDrop {
                        keep_or_drop: KeepOrDrop::Drop,
                        amount: 2,
                        highest_or_lowest: HighestOrLowest::Highest
                    }
                ],
                reroll: Some(2),
                subtracted: true
            },
            Roll {
                number_of_dice: 1,
                number_of_sides: 20,
                advantage_status: AdvantageStatus::Advantage,
                keep_drop: vec![],
                reroll: None,
                subtracted: false
            },
            Constant(-5)
        ]))));
    }

    #[test]
    fn kitchen_sink_whitespace() {
        assert_eq!(parse_dice_expression("-6d6r2d2dh2 + d20a -5"), Ok(("",
        Expression::DiceExpression(vec![
            Roll {
                number_of_dice: 6,
                number_of_sides: 6,
                advantage_status: AdvantageStatus::None,
                keep_drop: vec![
                    KeepDrop {
                        keep_or_drop: KeepOrDrop::Drop,
                        amount: 2,
                        highest_or_lowest: HighestOrLowest::Lowest
                    },
                    KeepDrop {
                        keep_or_drop: KeepOrDrop::Drop,
                        amount: 2,
                        highest_or_lowest: HighestOrLowest::Highest
                    }
                ],
                reroll: Some(2),
                subtracted: true
            },
            Roll {
                number_of_dice: 1,
                number_of_sides: 20,
                advantage_status: AdvantageStatus::Advantage,
                keep_drop: vec![],
                reroll: None,
                subtracted: false
            },
            Constant(-5)
        ]))));
    }
}