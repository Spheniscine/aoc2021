use crate::{aoc_base::Day, util::{dijk_heap::DijkHeap, gridhelper::GridHelper}};

pub struct Day15;

impl Day for Day15 {
    type Parsed = Vec<Vec<i32>>;

    type Output1 = i32;

    type Output2 = i32;

    fn num() -> usize {
        15
    }

    fn title() -> &'static str {
        "Chiton"
    }

    fn parse(input: &str) -> Self::Parsed {
        input.lines().map(|ln| ln.bytes().map(|x| (x - b'0') as _).collect() ).collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        let n = data.len();
        let m = data[0].len();

        let dest = n*m - 1;

        let mut heap = DijkHeap::<i32>::new(n * m);
        heap.promote(0, 0);
        let gh = GridHelper::new(n, m);

        while let Some(x) = heap.pop() {
            let cost = *heap.cost(x);
            if x == dest { return cost; }
            let (i, j) = (x/m, x%m);
            for (ni, nj) in gh.neighbors(i, j) {
                heap.promote(ni * m + nj, cost + data[ni][nj]);
            }
        }

        -1
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let n0 = data.len();
        let m0 = data[0].len();

        let factor = 5;
        let n = n0 * factor;
        let m = m0 * factor;

        let dest = n*m - 1;

        let mut heap = DijkHeap::<i32>::new(n * m);
        heap.promote(0, 0);
        let gh = GridHelper::new(n, m);

        while let Some(x) = heap.pop() {
            let cost = *heap.cost(x);
            if x == dest { return cost; }
            let (i, j) = (x/m, x%m);
            for (ni, nj) in gh.neighbors(i, j) {
                let i0 = ni % n0;
                let ti = (ni / n0) as i32;
                let j0 = nj % m0;
                let tj = (nj / n0) as i32;
                let risk = (data[i0][j0] - 1 + ti + tj) % 9 + 1;
                heap.promote(ni * m + nj, cost + risk);
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;
    use super::*;

    #[test]
    fn gmail() {
        Day15::test(InputSource::gmail(15), Some(487), Some(2821))
    }
}