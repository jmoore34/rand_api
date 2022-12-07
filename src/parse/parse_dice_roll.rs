use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{opt, recognize},
    multi::{many0, many1},
    sequence::{pair, tuple},
    IResult,
};

use crate::{
    dice::{
        advantage::AdvantageStatus,
        atom::DiceExpressionAtom,
        keepdrop::{HighestOrLowest, KeepDrop, KeepOrDrop},
    },
    Expression,
};

use super::parse_numbers::{parse_signed, parse_u8};

pub fn parse_dice_expression(input: &str) -> IResult<&str, Expression> {
    let (remain, atoms) = many1(parse_dice_expression_atom)(input)?;

    Ok((remain, Expression::DiceExpression(atoms)))
}

fn parse_dice_expression_atom(input: &str) -> IResult<&str, DiceExpressionAtom> {
    alt((parse_dice_roll_atom, parse_constant_atom))(input)
}

fn parse_constant_atom(input: &str) -> IResult<&str, DiceExpressionAtom> {
    let (remain, constant) = parse_signed::<i8>(input)?;
    Ok((remain, DiceExpressionAtom::Constant(constant)))
}

fn parse_dice_roll_atom(input: &str) -> IResult<&str, DiceExpressionAtom> {
    let (remain, ((sign, number_of_dice, _, number_of_sides), modifiers)) = pair(
        // First: quantity and number of sides, e.g. 3d6 (required)
        tuple((
            // Optional +/- sign
            // When it is -, this roll will be subtracted
            opt(alt((tag("+"), tag("-")))),
            // Optional number of dice (defaults to 1)
            opt(parse_u8),
            tag("d"),
            // Number of sides on each dice
            parse_u8,
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
    let (remain, (_, sides)) = pair(tag("r"), parse_u8)(input)?;

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
        parse_u8,
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
