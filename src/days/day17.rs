use crate::aoc_base::Day;

pub struct Day17;

impl Day for Day17 {
    type Parsed = ();

    type Output1 = &'static str;

    type Output2 = &'static str;

    fn num() -> usize {
        17
    }

    fn title() -> &'static str {
        ""
    }

    fn parse(input: &str) -> Self::Parsed {
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        "TODO"
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        "TODO"
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;
    use super::*;

    #[test]
    fn gmail() {
        //Day17::test(InputSource::gmail(17), Some(1553), Some(1597))
    }
}