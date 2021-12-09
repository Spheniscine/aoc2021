use std::{collections::HashMap, cmp::{min, max}};

use crate::aoc_base::Day;

pub struct Day5;

type Line = [[i32; 2]; 2];

impl Day for Day5 {
    type Parsed = Vec<Line>;

    type Output1 = usize;

    type Output2 = usize;

    fn num() -> usize {
        5
    }

    fn title() -> &'static str {
        "Hydrothermal Venture"
    }

    fn parse(input: &str) -> Self::Parsed {
        input.lines().map(|ln| {
            let mut ite = ln.split(" -> ").map(|sp| {
                let mut ite = sp.split(',').map(|x| x.parse::<i32>().unwrap());
                [ite.next().unwrap(), ite.next().unwrap()]
            });

            [ite.next().unwrap(), ite.next().unwrap()]
        }).collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        let mut cnt = HashMap::new();
        for &[[x1, y1], [x2, y2]] in &data {
            if x1 == x2 {
                for y in min(y1, y2)..=max(y1, y2) {
                    *cnt.entry([x1, y]).or_insert(0) += 1;
                }
            } else if y1 == y2 {
                for x in min(x1, x2)..=max(x1, x2) {
                    *cnt.entry([x, y1]).or_insert(0) += 1;
                }
            }
        }

        cnt.iter().filter(|x| *x.1 > 1).count()
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let mut cnt = HashMap::new();
        for &[[mut x, mut y], [x2, y2]] in &data {
            loop {
                *cnt.entry([x, y]).or_insert(0) += 1;
                if x == x2 && y == y2 { break; }
                x += (x2 - x).signum();
                y += (y2 - y).signum();
            }
        }

        cnt.iter().filter(|x| *x.1 > 1).count()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;

    use super::*;

    #[test]
    fn test_input() {
        let inp = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
        "#.trim();
        Day5::test(InputSource::Literal(inp), Some(5), Some(12));
    }

    #[test]
    fn gmail() {
        Day5::test(InputSource::gmail(5), Some(5294), Some(21698))
    }
}