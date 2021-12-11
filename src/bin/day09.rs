use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use advent_of_code_2021::common::Grid;
use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day09.txt")?;
    println!("{}", part1(&input));
    println!("{}", part2(&input));
    Ok(())
}

fn part1(input: &str) -> u32 {
    let grid = parse_input(input);
    low_point_iter(&grid).map(|(_, val)| val + 1).sum()
}

fn part2(input: &str) -> u32 {
    let grid = parse_input(input);

    let mut basin_sizes = low_point_iter(&grid)
        .map(|low_point| basin_size(&grid, low_point.0))
        .collect::<Vec<u32>>();

    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn basin_size(grid: &Grid, low_point: (usize, usize)) -> u32 {
    let mut visited = HashSet::new();
    visited.insert(low_point);

    let mut stack = VecDeque::new();
    stack.push_back(low_point);

    while !stack.is_empty() {
        let point = stack.pop_front().unwrap();
        for (x, y) in grid
            .iter_adjacent(point.0, point.1, false)
            .filter(|(x, y)| grid.cells[*x][*y] != 9)
        {
            if grid.cells[x][y] > grid.cells[point.0][point.1] {
                visited.insert((x, y));
                stack.push_back((x, y));
            }
        }
    }
    visited.len() as u32
}

fn low_point_iter(grid: &Grid) -> impl Iterator<Item = ((usize, usize), &u32)> {
    grid.iter_cells().filter(|((x, y), &val)| {
        grid.iter_adjacent(*x, *y, false)
            .all(|(a, b)| val < grid.cells[a][b])
    })
}

fn parse_input(input: &str) -> Grid {
    let cells = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    Grid { cells }
}

#[cfg(test)]
mod day09_tests {
    use crate::{part1, part2};

    static TEST_INPUT: &str = r"2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 1134);
    }
}
