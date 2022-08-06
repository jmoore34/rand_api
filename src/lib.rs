mod parse;


#[derive(Clone, Debug, PartialEq)]

pub enum Expression {
    CoinFlip,
    IntRange(i64, i64),
    FloatRange(f32, f32)
}


