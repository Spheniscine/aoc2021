use shash::SHashMap;

use crate::aoc_base::Day;
use crate::util::permutations::Permutations;

pub struct Day8;

#[derive(Debug, Clone, Default)]
pub struct Entry {
    patterns: [u8; 10],
    output: [u8; 4]
}

fn parse_pattern(s: &str) -> u8 {
    let mut ans = 0;
    for c in s.bytes() { 
        ans |= 1 << (c - b'a');
    }
    ans
}

const ORTH: [u8; 10] = [
    //abcdefg
    0b1110111,
    0b0010010,
    0b1011101,
    0b1011011,
    0b0111010,
    0b1101011,
    0b1101111,
    0b1010010,
    0b1111111,
    0b1111011
];

impl Day for Day8 {
    type Parsed = Vec<Entry>;

    type Output1 = usize;

    type Output2 = usize;

    fn num() -> usize {
        8
    }

    fn title() -> &'static str {
        "Seven Segment Search"
    }

    fn parse(input: &str) -> Self::Parsed {
        input.lines().map(|ln| {
            let mut entry = Entry::default();
            let mut ite = ln.split(' ');

            entry.patterns.fill_with(|| parse_pattern(ite.next().unwrap()));
            ite.next();
            entry.output.fill_with(|| parse_pattern(ite.next().unwrap()));

            entry
        }).collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        data.iter().map(|entry| entry.output.iter().filter(|&&x| [2, 4, 3, 7].contains(&x.count_ones())).count() ).sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let mut perms = Permutations::full(7);
        let mut map = SHashMap::default();
        while let Some(p) = perms.next() {
            let trans = ORTH.map(|x| {
                let mut y = 0u8;

                for i in 0..7 {
                    if (x >> i) & 1 == 1 {
                        y |= 1 << p[i];
                    }
                }

                y
            });

            let mut sorted = trans;
            sorted.sort_unstable();
            map.insert(sorted, trans);
        }

        data.iter().map(|entry| {
            let mut patterns = entry.patterns;
            patterns.sort_unstable();

            let trans = map[&patterns];

            let mut ans = 0;

            for &pat in &entry.output {
                let d = trans.iter().position(|&t| t == pat).unwrap();
                ans = ans * 10 + d;
            }

            ans
        }).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;

    use super::*;

    #[test]
    fn gmail() {
        Day8::test(InputSource::gmail(8), Some(390), Some(1011785))
    }
}