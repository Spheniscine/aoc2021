use crate::{aoc_base::Day, util::{gridhelper::GridHelper, dsu::Dsu}};

pub struct Day9;

impl Day for Day9 {
    type Parsed = Vec<Vec<i32>>;

    type Output1 = i32;

    type Output2 = u64;

    fn num() -> usize {
        9
    }

    fn title() -> &'static str {
        "Smoke Basin"
    }

    fn parse(input: &str) -> Self::Parsed {
        input.lines().map(|ln| ln.bytes().map(|a| (a - b'0') as i32).collect() ).collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        let n = data.len();
        let m = data[0].len();
        let gh = GridHelper::new(n, m);

        let mut ans = 0;
        for i in 0..n { for j in 0..m {
            let a = data[i][j];
            if gh.neighbors(i, j).all(|(ni, nj)| a < data[ni][nj]) {
                ans += a + 1;
            }
        }}

        ans
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let n = data.len();
        let m = data[0].len();
        let gh = GridHelper::new(n, m);

        let mut dsu = Dsu::new(n*m);
        let idx = |i: usize, j: usize| i * m + j;

        for i in 0..n { for j in 0..m {
            let a = data[i][j];
            if a != 9 {
                for (ni, nj) in gh.neighbors(i, j) {
                    if data[ni][nj] < a { dsu.join(idx(i, j), idx(ni, nj)); }
                }
            }
        }}

        let mut basins = vec![];

        for i in 0..n { for j in 0..m {
            let a = data[i][j];
            if gh.neighbors(i, j).all(|(ni, nj)| a < data[ni][nj]) {
                basins.push(dsu.size(idx(i, j)) as u64);
            }
        }}
        
        basins.sort_unstable();
        basins.iter().rev().take(3).product()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;

    use super::*;

    #[test]
    fn gmail() {
        Day9::test(InputSource::gmail(9), Some(566), Some(891684))
    }
}