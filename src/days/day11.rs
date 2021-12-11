use std::cmp::min;

use crate::aoc_base::Day;

pub struct Day11;

fn advance(a: &mut Vec<Vec<i32>>) -> usize {
    let n = a.len();
    let m = a[0].len();

    let mut stk = vec![];
    for i in 0..n { for j in 0..m {
        a[i][j] += 1;
        if a[i][j] == 10 { stk.push([i, j]); }
    }}

    let mut ans = stk.len();

    while let Some([i, j]) = stk.pop() {
        for ni in i.saturating_sub(1)..=min(n-1, i+1) {
            for nj in j.saturating_sub(1)..=min(m-1, j+1) {
                if a[ni][nj] < 10 {
                    a[ni][nj] += 1;
                    if a[ni][nj] == 10 {
                        stk.push([ni, nj]);
                        ans += 1;
                    }
                }
            }
        }
    }

    for i in 0..n { for j in 0..m {
        if a[i][j] == 10 { a[i][j] = 0; }
    }}

    ans
}

impl Day for Day11 {
    type Parsed = Vec<Vec<i32>>;

    type Output1 = usize;

    type Output2 = usize;

    fn num() -> usize {
        11
    }

    fn title() -> &'static str {
        "Dumbo Octopus"
    }

    fn parse(input: &str) -> Self::Parsed {
        input.lines().map(|ln| ln.bytes().map(|x| (x - b'0') as i32).collect()).collect()
    }

    fn part1(mut data: Self::Parsed) -> Self::Output1 {
        (0..100).map(|_| advance(&mut data)).sum()
    }

    fn part2(mut data: Self::Parsed) -> Self::Output2 {
        let num = data.len() * data[0].len();
        (1..).find(|_| advance(&mut data) == num).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;

    use super::*;

    #[test]
    fn gmail() {
        Day11::test(InputSource::gmail(11), Some(1681), Some(276))
    }
}