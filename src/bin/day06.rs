use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day06.txt")?;
    println!("{}", simulate(&input, 80)?);
    println!("{}", simulate(&input, 256)?);
    Ok(())
}

fn simulate(input: &str, days: u32) -> Result<u64> {
    let input: Vec<usize> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    let mut counts: Vec<u64> = vec![0; 9];

    for i in input {
        counts[i] += 1;
    }
    for _ in 0..days {
        let zeros = counts[0];
        for j in 0..8 {
            counts[j] = counts[j + 1];
        }
        counts[6] += zeros;
        counts[8] = zeros;
    }
    Ok(counts.iter().sum())
}

mod day06_tests {
    use crate::simulate;

    #[test]
    fn test_simulate() {
        let test_input = "3,4,3,1,2";
        assert_eq!(simulate(&test_input, 18).unwrap(), 26);
        assert_eq!(simulate(&test_input, 80).unwrap(), 5934);
        assert_eq!(simulate(&test_input, 256).unwrap(), 26984457539);
    }
}
