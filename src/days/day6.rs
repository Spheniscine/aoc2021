use std::collections::VecDeque;

use crate::aoc_base::Day;

pub struct Day6;

impl Day for Day6 {
    type Parsed = Vec<usize>;

    type Output1 = u64;

    type Output2 = u64;

    fn num() -> usize {
        6
    }

    fn title() -> &'static str {
        "Lanternfish"
    }

    fn parse(input: &str) -> Self::Parsed {
        input.split(',').map(|x| x.parse().unwrap()).collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        simulate(data, 80)
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        simulate(data, 256)
    }
}

fn simulate(data: Vec<usize>, generations: usize) -> u64 {
    let mut cnt = VecDeque::new();
    cnt.extend([0u64; 9]);
    for &a in &data { cnt[a] += 1; }

    for _ in 0..generations {
        let c = cnt.pop_front().unwrap();
        cnt.push_back(c);
        cnt[6] += c;
    }

    cnt.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;

    use super::*;

    #[test]
    fn gmail() {
        Day6::test(InputSource::gmail(6), Some(365131), Some(1650309278600))
    }
}