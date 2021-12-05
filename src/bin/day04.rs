use std::{fmt::Debug, fs};

use anyhow::Result;

#[derive(Debug)]
struct Cell {
    value: u32,
    marked: bool,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Board {
    size: usize,
    cells: Vec<Cell>,
    won: bool,
}

impl Cell {
    fn mark(&mut self) {
        self.marked = true;
    }
}

impl Board {
    fn init(input: Vec<&str>, size: usize) -> Result<Board> {
        let cells = input
            .iter()
            .enumerate()
            .map(|(i, line)| {
                line.split_whitespace()
                    .enumerate()
                    .map(|(j, str)| {
                        let value: u32 = str.parse()?;
                        Ok(Cell {
                            value,
                            marked: false,
                            x: i,
                            y: j,
                        })
                    })
                    .collect::<Vec<Result<Cell>>>()
            })
            .flatten()
            .collect::<Result<Vec<Cell>>>()?;

        Ok(Board {
            size,
            cells,
            won: false,
        })
    }

    fn mark(&mut self, value: u32) -> Option<(usize, usize)> {
        if let Some(cell) = self.cells.iter_mut().find(|cell| cell.value == value) {
            cell.mark();
            return Some((cell.x, cell.y));
        }
        None
    }

    fn is_complete(&self, row: usize, col: usize) -> bool {
        self.is_line_complete(row, |cell| cell.x) || self.is_line_complete(col, |cell| cell.y)
    }

    fn is_line_complete<F>(&self, index: usize, get_cell_coordinate: F) -> bool
    where
        F: Fn(&Cell) -> usize,
    {
        !self
            .cells
            .iter()
            .filter(|cell| get_cell_coordinate(cell) == index)
            .any(|cell| !cell.marked)
    }

    fn calculate_score(&self, draw: u32) -> u32 {
        let sum = self
            .cells
            .iter()
            .filter(|cell| !cell.marked)
            .map(|cell| cell.value)
            .sum::<u32>();
        sum * draw
    }

    fn set_won(&mut self) {
        self.won = true;
    }
}

fn get_winning_scores(input: &str) -> Result<Vec<u32>> {
    let mut iter = input.lines();

    let draws: Vec<u32> = iter
        .next()
        .ok_or(anyhow::anyhow!("Invalid input"))?
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    let mut boards: Vec<Board> = iter
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .chunks(5)
        .map(|window| Board::init(window.to_vec(), 5))
        .collect::<Result<_, _>>()?;

    let winning_scores = draws
        .iter()
        .map(|draw| {
            boards
                .iter_mut()
                .filter(|board| !board.won)
                .map(|board| {
                    if let Some((x, y)) = board.mark(*draw) {
                        if board.is_complete(x, y) {
                            board.set_won();
                            return Some(board.calculate_score(*draw));
                        }
                    }
                    None
                })
                // flatten here unwraps the options and skips none values
                .flatten()
                .collect::<Vec<u32>>()
        })
        .flatten()
        .collect();
    Ok(winning_scores)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day04.txt")?;
    let result = get_winning_scores(&input)?;

    println!("{}", result.first().unwrap());
    println!("{}", result.last().unwrap());

    Ok(())
}

#[cfg(test)]
mod day04_tests {
    use crate::get_winning_scores;

    static TEST_INPUT: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_both_parts() {
        let scores = get_winning_scores(TEST_INPUT).unwrap();
        assert_eq!(*scores.first().unwrap(), 4512);
        assert_eq!(*scores.last().unwrap(), 1924);
    }
}
