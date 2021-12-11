use std::ops::Not;

use crate::aoc_base::Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    is_open: bool,
    kind: usize
}

impl Not for Token {
    type Output = Token;

    fn not(self) -> Self::Output {
        Token { is_open: !self.is_open, kind: self.kind }
    }
}

impl TryFrom<u8> for Token {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if let Some(i) = b"([{<)]}>".iter().position(|&x| x == value) {
            Ok(Token { is_open: i < 4, kind: i & 3 })
        } else {
            Err(())
        }
    }
}

fn corrupt_score(tokens: &[Token]) -> Option<i64> {
    let mut stk = vec![];
    for &token in tokens {
        if token.is_open {
            stk.push(token)
        } else {
            if stk.last().copied() == Some(!token) {
                stk.pop();
            } else { return Some([3, 57, 1197, 25137][token.kind]); }
        }
    }
    None
}

fn incomplete_score(tokens: &[Token]) -> Option<i64> {
    let mut stk = vec![];
    for &token in tokens {
        if token.is_open {
            stk.push(token)
        } else {
            if stk.last().copied() == Some(!token) {
                stk.pop();
            } else { return None; }
        }
    }
    Some(stk.iter().rfold(0, |acc, x| acc * 5 + (x.kind + 1) as i64))
}

pub struct Day10;

impl Day for Day10 {
    type Parsed = Vec<Vec<Token>>;

    type Output1 = i64;

    type Output2 = i64;

    fn num() -> usize {
        10
    }

    fn title() -> &'static str {
        "Syntax Scoring"
    }

    fn parse(input: &str) -> Self::Parsed {
        input.lines().map(|ln| {
            ln.bytes().map(|x| {
                x.try_into().unwrap()
            }).collect()
        }).collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        data.iter().filter_map(|tokens| corrupt_score(tokens)).sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let mut scores = data.iter().filter_map(|tokens| incomplete_score(tokens)).collect::<Vec<_>>();
        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;

    use super::*;

    #[test]
    fn gmail() {
        Day10::test(InputSource::gmail(10), Some(364389), Some(2870201088))
    }
}