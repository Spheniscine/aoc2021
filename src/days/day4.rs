use crate::aoc_base::Day;

pub struct Day4;

const BOARD_SIZE: usize = 5;
type Board = [[usize; BOARD_SIZE]; BOARD_SIZE];
type Marks = [[bool; BOARD_SIZE]; BOARD_SIZE];

#[derive(Debug, Clone)]
pub struct Bingo {
    draws: Vec<usize>,
    boards: Vec<Board>
}

impl Bingo {
    fn positions(&self) -> Vec<Vec<[usize; 3]>> {
        let lim = self.draws.iter().max().copied().unwrap_or(0);
        let mut res = vec![vec![]; lim+1];

        for i in 0..self.boards.len() {
            let board = &self.boards[i];
            for j in 0..BOARD_SIZE { for k in 0..BOARD_SIZE {
                res[board[j][k]].push([i, j, k]);
            }}
        }

        res
    }
}

fn check_won(marks: &Marks, board: &Board, j: usize, k: usize) -> Option<usize> {
    if (0..BOARD_SIZE).all(|k| marks[j][k]) || (0..BOARD_SIZE).all(|j| marks[j][k]) {
        let mut unmarked_sum = 0;
        for j in 0..BOARD_SIZE { for k in 0..BOARD_SIZE {
            if !marks[j][k] { unmarked_sum += board[j][k]; }
        }}
        Some(unmarked_sum)
    } else { None }
}

impl Day for Day4 {
    type Parsed = Bingo;

    type Output1 = usize;

    type Output2 = usize;

    fn num() -> usize {
        4
    }

    fn title() -> &'static str {
        "Giant Squid"
    }

    fn parse(input: &str) -> Self::Parsed {
        let mut sections = input.split("\n\n");
        let draws = sections.next().unwrap().split(',').map(|x| x.parse().unwrap()).collect();
        let boards = sections.map(|src| {
            let mut board = Board::default();
            for (i, ln) in src.lines().enumerate() {
                for (j, v) in ln.split_ascii_whitespace().enumerate() {
                    board[i][j] = v.parse().unwrap();
                }
            }
            board
        }).collect();

        Bingo { draws, boards }
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        let positions = data.positions();
        let mut marks = vec![Marks::default(); data.boards.len()];

        for &draw in &data.draws {
            for &[i, j, k] in &positions[draw] {
                marks[i][j][k] = true;
                if let Some(unmarked_sum) = check_won(&marks[i], &data.boards[i], j, k) {
                    return unmarked_sum * draw
                }
            }
        }

        panic!("no winner")
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let positions = data.positions();
        let mut ans = 0;
        let mut marks = vec![Marks::default(); data.boards.len()];
        let mut won = vec![false; data.boards.len()];

        for &draw in &data.draws {
            for &[i, j, k] in &positions[draw] {
                if won[i] { continue; }
                marks[i][j][k] = true;
                if let Some(unmarked_sum) = check_won(&marks[i], &data.boards[i], j, k) {
                    won[i] = true;
                    ans = unmarked_sum * draw;
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
        Day4::test(InputSource::gmail(4), Some(38594), Some(21184))
    }
}