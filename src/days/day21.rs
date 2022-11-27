
use std::cmp::max;

use ndarray::Array5;

use crate::aoc_base::Day;

pub struct Day21;

impl Day for Day21 {
    type Parsed = [i32; 2];

    type Output1 = i64;

    type Output2 = i64;

    fn num() -> usize {
        21
    }

    fn title() -> &'static str {
        "Dirac Dice"
    }

    fn parse(input: &str) -> Self::Parsed {
        let mut it = input.lines().map(|ln| ln.split(' ').last().unwrap().parse().unwrap());
        [it.next().unwrap(), it.next().unwrap()]
    }

    fn part1(mut data: Self::Parsed) -> Self::Output1 {
        let mut score = [0; 2];
        let mut player = 1;
        let mut die = std::iter::repeat(1..=100).flatten();
        for t in 1.. {
            player ^= 1;
            let roll = die.next().unwrap() + die.next().unwrap() + die.next().unwrap();
            let mut pos = (data[player] + roll) % 10;
            if pos == 0 { pos = 10; }

            data[player] = pos;
            score[player] += pos;
            if score[player] >= 1000 {
                return score[player ^ 1] as i64 * t as i64 * 3;
            }
        }
        unreachable!()
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let end_score = 21;
        let board_size = 10;

        let mut mul = [0i64; 10];
        for a in 1..=3 { for b in 1..=3 { for c in 1..=3 {
            mul[a+b+c] += 1;
        }}}

        let mut dp = Array5::<i64>::zeros([end_score, end_score, 2, board_size, board_size]);
        dp[[0, 0, 0, (data[0]-1) as usize, (data[1]-1) as usize]] = 1;
        let mut wins = [0; 2];

        for s0 in 0..end_score { for s1 in 0..end_score { for t in 0..2 { for p0 in 0..board_size { for p1 in 0..board_size {
            let d = dp[[s0, s1, t, p0, p1]];
            if d == 0 { continue; }
            for r in 3..=9 {
                let mut s = [s0, s1];
                let mut p = [p0, p1];

                p[t] = (p[t] + r) % 10;
                s[t] += 1 + p[t];

                if s[t] >= end_score { wins[t] += d * mul[r]; }
                else {
                    dp[[s[0], s[1], t^1, p[0], p[1]]] += d * mul[r];
                }
            }
        }}}}}

        max(wins[0], wins[1])
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(Day21::part1([4, 8]), 739785);
        assert_eq!(Day21::part2([4, 8]), 444356092776315);
    }

    #[test]
    fn gmail() {
        Day21::test(InputSource::gmail(21), Some(797160), Some(27464148626406))
    }
}