use std::{cmp::Ordering, fs};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day03.txt")?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut counts = vec![0; lines[0].len()];
    lines.iter().for_each(|line| {
        line.chars()
            .enumerate()
            .filter(|(_i, c)| *c == '1')
            .for_each(|(i, _c)| counts[i] += 1)
    });
    let len = lines.len();
    let gamma_str: String = counts
        .iter()
        .map(|c| if c > &(len / 2) { '1' } else { '0' })
        .collect();
    let epsilon_str: String = counts
        .iter()
        .map(|c| if c > &(len / 2) { '0' } else { '1' })
        .collect();
    let gamma = u32::from_str_radix(&gamma_str, 2)?;
    let epsilon = u32::from_str_radix(&epsilon_str, 2)?;
    Ok(gamma * epsilon)
}

fn part2(input: &str) -> Result<u32> {
    let o2_rating_str = get_rating(input, retain_marker_for_o2_rating)?;
    let co2_rating_str = get_rating(input, retain_marker_for_co2_rating)?;

    let o2_rating = u32::from_str_radix(&o2_rating_str, 2)?;
    let co2_rating = u32::from_str_radix(&co2_rating_str, 2)?;

    Ok(o2_rating * co2_rating)
}

fn get_rating(input: &str, get_retain_marker: fn(u32, u32) -> char) -> Result<&str> {
    let mut lines: Vec<&str> = input.lines().collect();
    for i in 0..lines[0].len() {
        let count_of_ones = lines
            .iter()
            .filter_map(|line| line.chars().nth(i))
            .fold(0, |acc, c| if c == '1' { acc + 1 } else { acc });
        let retain_marker = get_retain_marker(count_of_ones, lines.len() as u32);
        lines.retain(|line| line.chars().nth(i).map_or(false, |c| c == retain_marker));
        if lines.len() == 1 {
            return Ok(lines[0]);
        }
    }
    Err(anyhow::anyhow!("Could not get rating"))
}

fn retain_marker_for_o2_rating(count_of_ones: u32, lines_count: u32) -> char {
    let count_of_zeros = lines_count - count_of_ones;
    match count_of_ones.cmp(&count_of_zeros) {
        Ordering::Less => '0',
        Ordering::Greater | Ordering::Equal => '1',
    }
}

fn retain_marker_for_co2_rating(count_of_ones: u32, lines_count: u32) -> char {
    let count_of_zeros = lines_count - count_of_ones;
    match count_of_ones.cmp(&count_of_zeros) {
        Ordering::Less => '1',
        Ordering::Greater | Ordering::Equal => '0',
    }
}

#[cfg(test)]
mod day03_tests {
    use crate::{part1, part2};

    static TEST_INPUT: &str = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 198);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 230);
    }
}
