use crate::aoc_base::Day;

pub struct Day9;

impl Day for Day9 {
    type Parsed = ();

    type Output1 = ();

    type Output2 = ();

    fn num() -> usize {
        9
    }

    fn title() -> &'static str {
        ""
    }

    fn parse(input: &str) -> Self::Parsed {
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;

    use super::*;

    #[test]
    fn gmail() {
        //Day9::test(InputSource::gmail(9), Some(1553), Some(1597))
    }
}