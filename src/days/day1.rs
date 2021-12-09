use crate::aoc_base::Day;

pub struct Day1;

impl Day for Day1 {
    type Parsed = Vec<i32>;

    type Output1 = usize;

    type Output2 = usize;

    fn num() -> usize {
        1
    }

    fn title() -> &'static str {
        "Sonar Sweep"
    }

    fn parse(input: &str) -> Self::Parsed {
        input.lines().map(|x| x.parse().unwrap()).collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        data.windows(2).filter(|x| x[1] > x[0]).count()
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        data.windows(4).filter(|x| x[3] > x[0]).count()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;

    use super::*;

    #[test]
    fn gmail() {
        Day1::test(InputSource::gmail(1), Some(1553), Some(1597))
    }
}