//! Utilities for keeping or dropping the highest or lowest dice rolls in a set of dice rolls.
//! E.g. "kh3" = keep highest 3 rolls and discard the rest

#[derive(Debug, Clone, PartialEq)]
pub enum KeepOrDrop {
    Keep,
    Drop,
}
#[derive(Debug, Clone, PartialEq)]

pub enum HighestOrLowest {
    Highest,
    Lowest,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeepDrop {
    pub keep_or_drop: KeepOrDrop,
    pub amount: u8,
    pub highest_or_lowest: HighestOrLowest,
}
