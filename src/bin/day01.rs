use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input: Vec<u32> = fs::read_to_string("input/day01.txt")?
        .lines()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));
    Ok(())
}

fn part1(input: &[u32]) -> u32 {
    count_increases_in_window(input, 1)
}

fn part2(input: &[u32]) -> u32 {
    count_increases_in_window(input, 3)
}

fn count_increases_in_window(input: &[u32], window_size: usize) -> u32 {
    let mut count = 0;
    for i in window_size..input.len() {
        if input[i] > input[i - window_size] {
            count += 1
        }
    }
    count
}

#[cfg(test)]
mod day01_tests {
    use crate::{part1, part2};

    fn test_input() -> Vec<u32> {
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&test_input()), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&test_input()), 5);
    }
}
