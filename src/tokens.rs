use std::convert::{TryFrom, TryInto};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BFToken {
    Inc,
    Dec,
    Left,
    Right,
    BeginLoop,
    EndLoop,
    Read,
    Write,
}

impl TryFrom<char> for BFToken {
    type Error = ();

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            '+' => Ok(Self::Inc),
            '-' => Ok(Self::Dec),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            '[' => Ok(Self::BeginLoop),
            ']' => Ok(Self::EndLoop),
            ',' => Ok(Self::Read),
            '.' => Ok(Self::Write),
            _ => Err(()),
        }
    }
}

pub fn tokenize(input: &str) -> Vec<BFToken> {
    input.chars().filter_map(|c| c.try_into().ok()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "+-+<<>[abc,+.][";
        let expected = {
            use BFToken::*;
            vec![
                Inc, Dec, Inc, Left, Left, Right, BeginLoop, Read, Inc, Write, EndLoop, BeginLoop,
            ]
        };

        assert_eq!(tokenize(input), expected);
    }
}
