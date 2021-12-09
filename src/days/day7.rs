use crate::aoc_base::Day;

pub struct Day7;

impl Day for Day7 {
    type Parsed = Vec<i32>;

    type Output1 = i32;

    type Output2 = i64;

    fn num() -> usize {
        7
    }

    fn title() -> &'static str {
        "The Treachery of Whales"
    }

    fn parse(input: &str) -> Self::Parsed {
        input.split(',').map(|x| x.parse().unwrap()).collect()
    }

    fn part1(mut data: Self::Parsed) -> Self::Output1 {
        let n = data.len();
        let med = *data.select_nth_unstable(n / 2).1;
        data.iter().map(|&a| (a - med).abs()).sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        fn cost(a: i32, b: i32) -> i64 {
            let d = (a - b).abs() as i64;
            d * (d + 1) / 2
        }

        let f = |x| data.iter().map(|&a| cost(x, a)).sum::<i64>();

        let mut l = 0;
        let mut r = *data.iter().max().unwrap();

        loop {
            let d = r - l;
            if d < 6 {
                return (l..=r).map(f).min().unwrap();
            }
            let a = l + d/3;
            let b = r - d/3;

            let fa = f(a);
            let fb = f(b);
            if fa >= fb { l = a; }
            if fa <= fb { r = b; }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;

    use super::*;

    #[test]
    fn gmail() {
        Day7::test(InputSource::gmail(7), Some(339321), Some(95476244))
    }
}