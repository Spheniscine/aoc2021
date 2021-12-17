use std::{fmt::{Display, Debug}};
pub trait Day {
    type Parsed: Clone;
    type Output1: Display + Debug + PartialEq;
    type Output2: Display + Debug + PartialEq;

    fn num() -> usize;
    fn title() -> &'static str;

    fn parse(input: &str) -> Self::Parsed;
    fn part1(data: Self::Parsed) -> Self::Output1;
    fn part2(data: Self::Parsed) -> Self::Output2;

    fn run(source: InputSource) {
        print!("Day {}", Self::num());
        if Self::title().len() > 0 {
            print!(": {}", Self::title());
        }
        println!();

        let input = source.read();
        let parsed = Self::parse(&input);
        
        println!();
        println!("Part 1:");
        let output1 = Self::part1(parsed.clone());
        println!("{}", output1);

        println!();
        println!("Part 2:");
        let output2 = Self::part2(parsed.clone());
        println!("{}", output2);
    }

    #[cfg(test)]
    fn test(source: InputSource, output1: Option<Self::Output1>, output2: Option<Self::Output2>) {
        let input = source.read();
        let parsed = Self::parse(&input);

        if let Some(expected) = output1 {
            let result = Self::part1(parsed.clone());
            assert_eq!(result, expected)
        }

        if let Some(expected) = output2 {
            let result = Self::part2(parsed.clone());
            assert_eq!(result, expected)
        }
    }
}


pub enum InputSource {
    File { path: String },
    Literal(&'static str)
}

impl InputSource {
    pub fn read(&self) -> String {
        match self {
            Self::File { path } => {
                std::fs::read_to_string(path).unwrap()
            },
            Self::Literal(value) => {
                value.to_string()
            }
        }
    }

    pub fn gmail(day_num: usize) -> Self {
        Self::File { path: format!("input/gmail/day{}.txt", day_num) }
    }
}