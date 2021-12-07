use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day07.txt")?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<i32>> {
    input
        .split(',')
        .map(|pos| pos.parse().map_err(|_err| anyhow::anyhow!("err")))
        .collect::<Result<_, _>>()
}

fn part1(input: &str) -> Result<i32> {
    let positions: Vec<i32> = parse_input(input)?;

    Ok(positions
        .iter()
        .map(|source| {
            positions
                .iter()
                .map(|target| (target - source).abs())
                .sum::<i32>()
        })
        .min()
        .unwrap())
}

fn part2(input: &str) -> Result<i32> {
    let mut positions: Vec<i32> = parse_input(input)?;
    positions.sort_unstable();

    Ok((*positions.first().unwrap()..*positions.last().unwrap())
        .map(|source| {
            positions
                .iter()
                .map(|target| (target - source).abs())
                .map(|distance| (distance * (distance + 1)) / 2)
                .sum::<i32>()
        })
        .min()
        .unwrap())
}

#[cfg(test)]
mod day07_tests {
    use crate::{part1, part2};

    static TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 168);
    }
}
