use crate::aoc_base::Day;
pub struct Day2;

impl Day for Day2 {
    type Parsed = Vec<[i64; 2]>;

    type Output1 = i64;

    type Output2 = i64;

    fn num() -> usize {
        2
    }

    fn title() -> &'static str {
        "Dive!"
    }

    fn parse(input: &str) -> Self::Parsed {
        input.lines().map(|line| {
            let mut line = line.split(' ');
            let dir = line.next().unwrap();
            let x: i64 = line.next().unwrap().parse().unwrap();

            match dir {
                "forward" => [x, 0],
                "down" => [0, x],
                "up" => [0, -x],
                _ => panic!()
            }
        }).collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        let [x, y] = data.iter().copied().reduce(|a, b| [a[0] + b[0], a[1] + b[1]]).unwrap();
        x * y
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let mut aim = 0;
        let mut cx = 0;
        let mut cy = 0;
        for &[x, y] in &data {
            aim += y;
            cx += x;
            cy += x * aim;
        }

        cx * cy
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;

    use super::*;

    #[test]
    fn gmail() {
        Day2::test(InputSource::gmail(2), Some(1427868), Some(1568138742))
    }
}