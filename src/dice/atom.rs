use super::{advantage::AdvantageStatus, keepdrop::KeepDrop};

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
