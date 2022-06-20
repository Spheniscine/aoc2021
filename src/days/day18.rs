use crate::aoc_base::Day;

pub struct Day18;

type SnToken = i32;
type Sn = Vec<SnToken>;

const OPEN: SnToken = -1;
const CLOSE: SnToken = -2;

fn parse_sn(ln: &str) -> Sn {
    let mut sn = Sn::new();

    for ch in ln.bytes() {
        match ch {
            b'[' => sn.push(OPEN),
            b']' => sn.push(CLOSE),
            b'0'..=b'9' => sn.push((ch - b'0') as _),
            _ => ()
        };
    }

    sn
}

fn add_sn(a: &Sn, b: &Sn) -> Sn {
    let mut c = Sn::new();
    c.push(OPEN);
    c.extend(a);
    c.extend(b);
    c.push(CLOSE);

    loop {
        let mut changed = false;
        
        // exploding
        let mut depth = 0;
        let mut res = Sn::new();
        let mut nx = 0;
        for &token in &c {
            res.push(token);
            if token == OPEN {
                depth += 1;
            } else if token == CLOSE {
                depth -= 1;
                if depth >= 4 {
                    changed = true;
                    res.pop();
                    let right = res.pop().unwrap();
                    let left = res.pop().unwrap();
                    res.pop();

                    nx = right;
                    if let Some(i) = (0..res.len()).rev().find(|&i| res[i] >= 0) {
                        res[i] += left;
                    }

                    res.push(0);
                }
            } else {
                let i = res.len() - 1;
                res[i] += nx;
                nx = 0;
            }
        }

        c = res;

        // splitting
        if let Some(i) = c.iter().position(|&x| x >= 10) {
            changed = true;
            let a = c[i] / 2;
            let b = c[i] - a;

            res = Sn::new();
            res.extend(&c[..i]);
            res.extend([OPEN, a, b, CLOSE]);
            res.extend(&c[i+1..]);
            c = res;
        }

        if !changed { break; }
    }

    c
}

fn magnitude(a: &Sn) -> SnToken {
    let mut stk = Vec::new();

    for &token in a {
        if token >= 0 {
            stk.push(token);
        } else if token == CLOSE {
            let b = stk.pop().unwrap();
            let a = stk.pop().unwrap();

            stk.push(3*a + 2*b);
        }
    }

    stk[0]
}

impl Day for Day18 {
    type Parsed = Vec<Sn>;

    type Output1 = SnToken;

    type Output2 = SnToken;

    fn num() -> usize {
        18
    }

    fn title() -> &'static str {
        "Snailfish"
    }

    fn parse(input: &str) -> Self::Parsed {
        input.lines().map(parse_sn).collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        let sum = data.into_iter().reduce(|a, b| add_sn(&a, &b)).unwrap();
        magnitude(&sum)
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let n = data.len();
        (0..n).flat_map(|i| (0..n).map(move|j| (i, j)))
            .filter(|x| x.0 != x.1)
            .map(|(i, j)| magnitude(&add_sn(&data[i], &data[j]))).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;
    use super::*;

    #[test]
    fn gmail() {
        Day18::test(InputSource::gmail(18), Some(3675), Some(4650))
    }
}