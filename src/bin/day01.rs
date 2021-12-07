use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day01.txt")?
        .lines()
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()?;

    println!("{}", count_increases_in_window(&input, 1));
    println!("{}", count_increases_in_window(&input, 3));
    Ok(())
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
    use crate::count_increases_in_window;

    fn get_test_input() -> Vec<u32> {
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    }

    #[test]
    fn test_part1() {
        assert_eq!(count_increases_in_window(&get_test_input(), 1), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(count_increases_in_window(&get_test_input(), 3), 5);
    }
}
