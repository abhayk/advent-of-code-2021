use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day02.txt")?;
    println!("{}", part1(&input));
    println!("{}", part2(&input));
    Ok(())
}

fn part1(input: &str) -> i32 {
    let (x, y) = input
        .lines()
        .filter_map(|line| parse_input(line))
        .fold((0, 0), |(x, y), (dx, dy)| (x + dx, y + dy));
    x * y
}

fn part2(input: &str) -> i32 {
    let (x, y, _) = input
        .lines()
        .filter_map(|line| parse_input(line))
        .fold((0, 0, 0), |(x, y, aim), (dx, dy)| {
            (x + dx, y + aim * dx, aim + dy)
        });
    x * y
}

fn parse_input(input: &str) -> Option<(i32, i32)> {
    let mut tokens = input.split_whitespace();
    let direction = tokens.next()?;
    let value: i32 = tokens.next()?.parse().ok()?;
    match direction {
        "forward" => Some((value, 0)),
        "down" => Some((0, value)),
        "up" => Some((0, -value)),
        _ => None,
    }
}

#[cfg(test)]
mod day02_tests {
    use crate::{part1, part2};

    static TEST_INPUT: &str = r"forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 150)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 900)
    }
}
