use std::{collections::HashSet, fs};

use advent_of_code_2021::common::Grid;
use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day11.txt")?;
    println!("{}", part1(&input, 100));
    println!("{}", part2(&input));
    Ok(())
}

fn part1(input: &str, steps: u32) -> u32 {
    let mut grid = parse_input(input);
    (0..steps).map(|_| simulate(&mut grid)).sum()
}

fn part2(input: &str) -> u32 {
    let mut grid = parse_input(input);
    let target_flash_count = (grid.max_x() * grid.max_y()) as u32;
    let mut counter = 1;

    while simulate(&mut grid) != target_flash_count {
        counter += 1;
    }
    counter
}

fn simulate(grid: &mut Grid) -> u32 {
    let mut flash_count = 0;

    for i in 0..grid.max_x() {
        for j in 0..grid.max_y() {
            grid.cells[i][j] += 1;
        }
    }
    let mut flashed = true;
    let mut flashed_octos = HashSet::new();
    while flashed {
        flashed = false;
        for i in 0..grid.max_x() {
            for j in 0..grid.max_y() {
                if grid.cells[i][j] > 9 && !flashed_octos.contains(&(i, j)) {
                    flashed_octos.insert((i, j));
                    flashed = true;
                    flash_count += 1;
                    for (ax, ay) in grid.iter_adjacent(i, j, true) {
                        grid.cells[ax][ay] += 1;
                    }
                }
            }
        }
    }
    for (x, y) in flashed_octos {
        grid.cells[x][y] = 0;
    }
    flash_count
}

fn parse_input(input: &str) -> Grid {
    let cells = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    Grid { cells }
}

#[cfg(test)]
mod day11_tests {
    use crate::{part1, part2};

    static TEST_INPUT: &str = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT, 10), 204);
        assert_eq!(part1(TEST_INPUT, 100), 1656);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 195);
    }
}
