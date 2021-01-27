use crate::tokens::BFToken;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub enum BFSyntax {
    Root(Vec<BFSyntax>),
    Inc(u8),
    Dec(u8),
    Left(usize),
    Right(usize),
    Loop(Vec<BFSyntax>),
    Read,
    Write,
}

impl TryFrom<Vec<BFToken>> for BFSyntax {
    type Error = &'static str;

    fn try_from(input: Vec<BFToken>) -> Result<Self, Self::Error> {
        let mut pos = 0;
        return Ok(Self::Root(Self::parse(&mut pos, &input)?));
    }
}

impl BFSyntax {
    fn parse(pos: &mut usize, input: &[BFToken]) -> Result<Vec<Self>, &'static str> {
        let mut result = vec![];

        while *pos < input.len() {
            let new_item = match input[*pos] {
                BFToken::Inc => Self::Inc(1),
                BFToken::Dec => Self::Dec(1),
                BFToken::Left => Self::Left(1),
                BFToken::Right => Self::Right(1),
                BFToken::Read => Self::Read,
                BFToken::Write => Self::Write,
                BFToken::BeginLoop => {
                    *pos = *pos + 1;
                    Self::Loop(Self::parse(pos, input)?)
                }
                BFToken::EndLoop => {
                    return Ok(result);
                }
            };
            result.push(new_item);
            *pos = *pos + 1;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tokens::tokenize;

    #[test]
    fn test_simple_program() {
        assert_eq!(
            BFSyntax::try_from(tokenize("++-->><<")).unwrap(),
            BFSyntax::Root(vec![
                BFSyntax::Inc(1),
                BFSyntax::Inc(1),
                BFSyntax::Dec(1),
                BFSyntax::Dec(1),
                BFSyntax::Right(1),
                BFSyntax::Right(1),
                BFSyntax::Left(1),
                BFSyntax::Left(1),
            ])
        )
    }

    #[test]
    fn test_single_loop() {
        assert_eq!(
            BFSyntax::try_from(tokenize("++[-]")).unwrap(),
            BFSyntax::Root(vec![
                BFSyntax::Inc(1),
                BFSyntax::Inc(1),
                BFSyntax::Loop(vec![BFSyntax::Dec(1)])
            ])
        )
    }

    #[test]
    fn test_nested_loop() {
        assert_eq!(
            BFSyntax::try_from(tokenize("++[-[>]]")).unwrap(),
            BFSyntax::Root(vec![
                BFSyntax::Inc(1),
                BFSyntax::Inc(1),
                BFSyntax::Loop(vec![
                    BFSyntax::Dec(1),
                    BFSyntax::Loop(vec![BFSyntax::Right(1)])
                ])
            ])
        )
    }

    #[test]
    fn test_loop_with_follower() {
        assert_eq!(
            BFSyntax::try_from(tokenize("++[-]+")).unwrap(),
            BFSyntax::Root(vec![
                BFSyntax::Inc(1),
                BFSyntax::Inc(1),
                BFSyntax::Loop(vec![BFSyntax::Dec(1),]),
                BFSyntax::Inc(1),
            ])
        )
    }
}
