use crate::aoc_base::Day;

pub struct Day20;

fn parse_line(s: &str) -> Vec<bool> {
    s.bytes().map(|x| x == b'#').collect()
}

type State = (Vec<Vec<bool>>, bool);
fn step(state: &State, enh: &[bool]) -> State {
    let n = state.0.len() as i32;
    let m = state.0[0].len() as i32;

    let s = |i: i32, j: i32| {
        if (0..n).contains(&i) && (0..m).contains(&j) {
            state.0[i as usize][j as usize]
        } else {
            state.1
        }
    };

    ((-1..n+1).map(|i| {
        (-1..m+1).map(|j| {
            let mut k = 0;
            for a in i-1 ..= i+1 { for b in j-1 ..= j+1 {
                k = (k << 1) | s(a, b) as usize
            }}
            enh[k]
        }).collect()
    }).collect(), enh[if state.1 {(1 << 9)-1} else {0}])

}

impl Day for Day20 {
    type Parsed = (Vec<bool>, Vec<Vec<bool>>);

    type Output1 = usize;

    type Output2 = usize;

    fn num() -> usize {
        20
    }

    fn title() -> &'static str {
        "Trench Map"
    }

    fn parse(input: &str) -> Self::Parsed {
        let mut it = input.lines();
        let enh = parse_line(it.next().unwrap());
        it.next();
        (enh, it.map(|x| parse_line(x)).collect())
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        let (enh, a) = data;
        let mut a = (a, false);

        a = step(&a, &enh);
        a = step(&a, &enh);
        a.0.iter().map(|l| l.iter().filter(|&&x| x).count()).sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let (enh, a) = data;
        let mut a = (a, false);

        for _ in 0..50 {
            a = step(&a, &enh);
        }
        a.0.iter().map(|l| l.iter().filter(|&&x| x).count()).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;
    use super::*;

    #[test]
    fn gmail() {
        Day20::test(InputSource::gmail(20), Some(5846), Some(21149))
    }
}