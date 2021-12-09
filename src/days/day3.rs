use crate::aoc_base::Day;

#[derive(Debug, Clone)]
pub struct Input {
    num_bits: u32,
    vals: Vec<u16>
}

pub struct Day3;

impl Day for Day3 {
    type Parsed = Input;

    type Output1 = u32;

    type Output2 = u32;

    fn num() -> usize {
        3
    }

    fn title() -> &'static str {
        "Binary Diagnostic"
    }

    fn parse(input: &str) -> Self::Parsed {
        let num_bits = input.lines().next().unwrap().len() as u32;
        let vals = input.lines().map(|ln| u16::from_str_radix(ln, 2).unwrap()).collect();
        Input { num_bits, vals }
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        let mut gamma = 0u32;
        let n = data.vals.len();
        for s in 0..data.num_bits {
            if data.vals.iter().filter(|&&x| x >> s & 1 == 1).nth(n/2).is_some() {
                gamma |= 1 << s;
            }
        }
        let epsilon = !gamma & (1 << data.num_bits) - 1;
        gamma * epsilon
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let mut work = [data.vals.clone(), data.vals.clone()];

        for s in (0..data.num_bits).rev() {
            for ty in 0..2 { if work[ty].len() > 1 {
                let (a, b) = work[ty].iter().copied().partition::<Vec<_>, _>(|&x| x >> s & 1 == 0);
                let mut parts = [a, b];
                if parts[0].len() > parts[1].len() { parts.swap(0, 1); }
                work[ty] = parts.into_iter().nth(ty).unwrap();
            }}
        }

        debug_assert!(work[0].len() == 1 && work[1].len() == 1);
        let oxygen_rating = work[1][0];
        let co2_rating = work[0][0];
        oxygen_rating as u32 * co2_rating as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;

    use super::*;

    #[test]
    fn gmail() {
        Day3::test(InputSource::gmail(3), Some(4191876), Some(3414905))
    }
}