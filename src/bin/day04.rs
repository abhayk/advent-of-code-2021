use std::fs;

use anyhow::Result;

struct Cell {
    value: u32,
    marked: bool,
    x: usize,
    y: usize,
}

struct Board {
    size: usize,
    cells: Vec<Cell>,
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
                        let value: u32 = str.parse().unwrap();
                        Cell {
                            value,
                            marked: false,
                            x: i,
                            y: j,
                        }
                    })
                    .collect::<Vec<Cell>>()
            })
            .flatten()
            .collect::<Vec<Cell>>();

        Ok(Board { size, cells })
    }

    fn mark(&mut self, value: u32) {
        if let Some(cell) = self.cells.iter_mut().find(|cell| cell.value == value) {
            cell.mark();
        }
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day03.txt")?;
    let mut iter = input.lines();
    let draws: Vec<u32> = iter
        .next()
        .expect("No draws in the input")
        .split(",")
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    let boards_input: Vec<&str> = iter.filter(|line| !line.is_empty()).collect();

    let boards: Vec<Board> = boards_input
        .windows(5)
        .map(|window| Board::init(window.to_vec(), 5))
        .collect::<Result<_, _>>()?;

    for draw in draws {
    }

    Ok(())
}
