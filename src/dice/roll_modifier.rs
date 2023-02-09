use super::keepdrop::KeepDrop;

#[derive(Debug)]
pub enum RollModifier {
    Reroll(u8),
    Disadvantage,
    Advantage,
    KeepDrop(KeepDrop),
}