use super::{advantage::AdvantageStatus, keepdrop::KeepDrop, roll_modifier::RollModifier};

/// A dice roll atom is the smallest unit in a dice roll expression.
/// For example, in "3d8 + 2d6r1 + 8", the atoms are "3d8", "2d6r1", and "8"
#[derive(Debug, PartialEq, Clone)]
pub enum DiceExpressionAtom {
    /// An signed integer constant,
    /// e.g. the "-1" in "d20 - 1"
    /// A.k.a. a "modifier", but that
    /// term was avoided to evade confusion
    /// with roll modifiers (e.g. advantage, rerolls, etc)
    Constant(i8),

    Roll {
        /// How many dice are rolled,
        /// e.g. 3 in "3d6"
        number_of_dice: u8,
        /// How many sides each dice has,
        /// e.g. 6 in "3d6"
        number_of_sides: u8,
        /// Whether to roll with advantage,
        /// disadvantage, or neither
        advantage_status: AdvantageStatus,
        /// An optional modifier that lets you drop the
        /// highest or lowest <some number> rolls, or drop
        /// everything and only keep the highest/lowest
        /// <some number> rolls
        keep_drop: Vec<KeepDrop>,
        /// An optional modifier to reroll all dice
        /// (once) that land on the given number or lower.
        /// E.g., reroll all dice in 3d6r2 that land on a
        /// 1 or 2.
        reroll: Option<u8>,
        /// Whether this atom is subtracted, e.g. in
        /// "3d6 - d4", the d4 is subtracted
        subtracted: bool,
    },
}

impl DiceExpressionAtom {
    pub fn new(number_of_dice: u8, number_of_sides: u8, subtracted: bool, modifiers: Vec<RollModifier>) -> Self {
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

        DiceExpressionAtom::Roll {
            number_of_dice,
            number_of_sides,
            advantage_status,
            keep_drop: drop_keep,
            reroll,
            subtracted,
        }
    }
}