use std::{ops::RangeInclusive, cmp::max};

use regex::Regex;

use crate::aoc_base::Day;

pub struct Day17;

impl Day for Day17 {
    type Parsed = [RangeInclusive<i32>; 2];

    type Output1 = i32;

    type Output2 = i32;

    fn num() -> usize {
        17
    }

    fn title() -> &'static str {
        "Trick Shot"
    }

    fn parse(input: &str) -> Self::Parsed {
        let re = Regex::new(r"^target area: x=(.+)\.\.(.+), y=(.+)\.\.(.+)$").unwrap();
        let cap = re.captures(input).unwrap();
        [ cap[1].parse().unwrap() ..= cap[2].parse().unwrap(), cap[3].parse().unwrap() ..= cap[4].parse().unwrap() ]
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        let [xr, yr] = data;

        let ymin = *yr.start();
        for yv in (ymin ..= -ymin).rev() {
            for xv in 1 ..= *xr.end() {
                let mut xv = xv; let mut yv = yv;
                let mut x = 0; let mut y = 0;
                let mut ymx = 0;

                loop {
                    if xr.contains(&x) && yr.contains(&y) { return ymx; }
                    if x > *xr.end() || y < *yr.start() { break; }
                    x += xv; y += yv;
                    xv -= xv.signum(); yv -= 1;
                    ymx = max(ymx, y);
                }
            }
        }

        panic!()
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let [xr, yr] = data;

        let mut ans = 0;

        let ymin = *yr.start();
        for yv in (ymin ..= -ymin).rev() {
            for xv in 1 ..= *xr.end() {
                let mut xv = xv; let mut yv = yv;
                let mut x = 0; let mut y = 0;
                let mut ymx = 0;

                loop {
                    if xr.contains(&x) && yr.contains(&y) { ans += 1; break; }
                    if x > *xr.end() || y < *yr.start() { break; }
                    x += xv; y += yv;
                    xv -= xv.signum(); yv -= 1;
                    ymx = max(ymx, y);
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;
    use super::*;

    #[test]
    fn gmail() {
        Day17::test(InputSource::gmail(17), Some(7503), Some(3229))
    }
}