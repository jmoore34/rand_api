use nom::{
    IResult,
    error::ParseError,
    sequence::delimited, character::complete::space0,
  };

  /// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
  /// trailing whitespace, returning the output of `inner`.
  /// Adapted w modifications from https://github.com/Geal/nom/blob/main/doc/nom_recipes.md#wrapper-combinators-that-eat-whitespace-before-and-after-a-parser
  pub fn spaced<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
  {
    delimited(
      space0,
      inner,
      space0
    )
  }