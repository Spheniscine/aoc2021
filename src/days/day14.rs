use std::collections::HashMap;

use crate::{aoc_base::Day};

pub struct Day14;

#[derive(Debug, Clone)]
pub struct Data {
    template: Vec<u8>,
    rules: Rules
}

type Counts = HashMap<[u8; 2], u64>;
type Rules = HashMap<[u8; 2], u8>;

fn step(counts: &Counts, rules: &Rules) -> Counts {
    let mut res = Counts::new();

    for (&pair, &num) in counts {
        if let Some(&mid) = rules.get(&pair) {
            *res.entry([pair[0], mid]).or_insert(0) += num;
            *res.entry([mid, pair[1]]).or_insert(0) += num;
        } else {
            *res.entry(pair).or_insert(0) += num;
        }
    }

    res
}

fn get_answer(counts: &Counts, template: &[u8]) -> u64 {
    let mut freq = vec![0u64; 128];
    for (&pair, &num) in counts {
        for ch in pair {
            freq[ch as usize] += num;
        }
    }

    freq[template[0] as usize] += 1;
    freq[*template.last().unwrap() as usize] += 1;
    for e in freq.iter_mut() { *e /= 2; }

    freq.iter().max().unwrap() - freq.iter().filter(|&&x| x != 0).min().unwrap()
}

impl Data {
    fn get_counts(&self) -> Counts {
        let mut res = Counts::new();

        for win in self.template.windows(2) {
            *res.entry([win[0], win[1]]).or_insert(0) += 1;
        }

        res
    }
    // fn step(&mut self) {
    //     let mut next = Vec::new();

    //     for &ch in &self.template {
    //         if let Some(&pr) = next.last() {
    //             if let Some(&mid) = self.rules.get(&[pr, ch]) {
    //                 next.push(mid);
    //             }
    //         }
    //         next.push(ch);
    //     }

    //     self.template = next;
    // }
}

impl Day for Day14 {
    type Parsed = Data;

    type Output1 = u64;

    type Output2 = u64;

    fn num() -> usize {
        14
    }

    fn title() -> &'static str {
        "Extended Polymerization"
    }

    fn parse(input: &str) -> Self::Parsed {
        let mut sections = input.split("\n\n");
        Data {
            template: sections.next().unwrap().as_bytes().to_vec(),
            rules: sections.next().unwrap().lines().map(|ln| {
                let ln = ln.as_bytes();
                ([ln[0], ln[1]], *ln.last().unwrap())
            }).collect()
        }
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        let mut counts = data.get_counts();
        for _ in 0..10 {
            counts = step(&counts, &data.rules);
        }

        get_answer(&counts, &data.template)
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let mut counts = data.get_counts();
        for _ in 0..40 {
            counts = step(&counts, &data.rules);
        }

        get_answer(&counts, &data.template)
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;
    use super::*;

    #[test]
    fn gmail() {
        Day14::test(InputSource::gmail(14), Some(2874), Some(5208377027195))
    }
}