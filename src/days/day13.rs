use shash::SHashSet;

use crate::{aoc_base::Day, util::display_dots::display_dots};

pub struct Day13;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldKind { X, Y }
type Instruction = (FoldKind, i32);

#[derive(Debug, Clone)]
pub struct Data {
    dots: SHashSet<[i32; 2]>,
    instructions: Vec<Instruction>
}

fn do_fold(dots: &SHashSet<[i32; 2]>, instruction: Instruction) -> SHashSet<[i32; 2]> {
    let dim: usize = if instruction.0 == FoldKind::X { 0 } else { 1 };

    dots.iter().map(|dot| {
        let mut dot = *dot;
        if dot[dim] > instruction.1 { 
            dot[dim] = instruction.1 - (dot[dim] - instruction.1);
        }
        dot
    }).collect()
}

impl Day for Day13 {
    type Parsed = Data;

    type Output1 = usize;

    type Output2 = String;

    fn num() -> usize {
        13
    }

    fn title() -> &'static str {
        "Transparent Origami"
    }

    fn parse(input: &str) -> Self::Parsed {
        let mut sections = input.split("\n\n");
        let dots = sections.next().unwrap().lines().map(|ln| {
            let ln = ln.split_once(',').unwrap();
            [ln.0.parse().unwrap(), ln.1.parse().unwrap()]
        }).collect();
        let instructions = sections.next().unwrap().lines().map(|ln| {
            let ln = ln.rsplit_once(' ').unwrap().1.split_once('=').unwrap();
            (if ln.0 == "x" {FoldKind::X} else {FoldKind::Y}, ln.1.parse().unwrap())
        }).collect();

        Data { dots, instructions }
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        let res = do_fold(&data.dots, data.instructions[0]);
        res.len()
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let mut dots = data.dots;
        for &instruction in &data.instructions {
            dots = do_fold(&dots, instruction)
        }

        display_dots(&dots)
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;
    use indoc::indoc;
    use super::*;

    #[test]
    fn gmail() {
        let part2 = indoc! {"
            #.....##..#..#.####..##..#..#.####...##
            #....#..#.#..#.#....#..#.#..#.#.......#
            #....#....####.###..#....#..#.###.....#
            #....#.##.#..#.#....#.##.#..#.#.......#
            #....#..#.#..#.#....#..#.#..#.#....#..#
            ####..###.#..#.####..###..##..####..##.
        "}.to_string();
        Day13::test(InputSource::gmail(13), Some(807), Some(part2))
    }
}