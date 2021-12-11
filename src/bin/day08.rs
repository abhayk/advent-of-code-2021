use std::{collections::HashMap, fs};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day08.txt")?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    let result = input
        .lines()
        .map(|line| line.split(" | ").nth(1).unwrap())
        .flat_map(|str| str.split_whitespace())
        .map(|word| word.len())
        .filter(|len| *len == 2 || *len == 4 || *len == 3 || *len == 7)
        .count();

    Ok(result as u32)
}

#[allow(clippy::many_single_char_names)]
fn part2(input: &str) -> Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut split = line.split(" | ");
        let patterns: Vec<&str> = split.next().unwrap().split_whitespace().collect();
        let output: Vec<&str> = split.next().unwrap().split_whitespace().collect();

        // segments used for displaying each number
        // 0 - abcefg
        // 1 - cf
        // 2 - acdeg
        // 3 - acdfg
        // 4 - bcdf
        // 5 - abdfg
        // 6 - abdefg
        // 7 - acf
        // 8 - abdcefg
        // 9 - abcdfg

        // counts of each segment across the patterns
        // a - 8
        // b - 6
        // c - 8
        // d - 7
        // e - 4
        // f - 9
        // g - 7

        let counts = patterns
            .iter()
            .flat_map(|p| p.chars())
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });

        // b,e,f can be identified since their counts are unique
        let b = *counts.iter().find(|entry| *entry.1 == 6).unwrap().0;
        let e = *counts.iter().find(|entry| *entry.1 == 4).unwrap().0;
        let f = *counts.iter().find(|entry| *entry.1 == 9).unwrap().0;

        // c can be identified from 1 (cf) once f is identified
        let c = filter_pattern(&patterns, 2, vec![f]);

        // a can be identified from 7 (acf) once c and f are identified
        let a = filter_pattern(&patterns, 3, vec![c, f]);

        // d can be identified from 4 (bcdf) once b, c, f are identified
        let d = filter_pattern(&patterns, 4, vec![b, c, f]);

        // g is the last letter. hence the one not equal to any of the other letters.
        let g = filter_pattern(&patterns, 7, vec![a, b, c, d, e, f]);

        let mut segments = HashMap::new();
        segments.insert('a', a);
        segments.insert('b', b);
        segments.insert('c', c);
        segments.insert('d', d);
        segments.insert('e', e);
        segments.insert('f', f);
        segments.insert('g', g);

        // generate the modified segments for each number
        let mut keys = HashMap::new();
        keys.insert(get_key(&segments, "abcefg"), '0');
        keys.insert(get_key(&segments, "cf"), '1');
        keys.insert(get_key(&segments, "acdeg"), '2');
        keys.insert(get_key(&segments, "acdfg"), '3');
        keys.insert(get_key(&segments, "bcdf"), '4');
        keys.insert(get_key(&segments, "abdfg"), '5');
        keys.insert(get_key(&segments, "abdefg"), '6');
        keys.insert(get_key(&segments, "acf"), '7');
        keys.insert(get_key(&segments, "abcdefg"), '8');
        keys.insert(get_key(&segments, "abcdfg"), '9');

        let result = output
            .iter()
            .map(|op| {
                let mut chars = op.chars().collect::<Vec<char>>();
                chars.sort_unstable();
                chars.iter().collect::<String>()
            })
            .map(|k| keys.get(&k).unwrap())
            .collect::<String>()
            .parse::<u32>()?;

        sum += result;
    }
    Ok(sum)
}

fn get_key(segments: &HashMap<char, char>, input: &str) -> String {
    let mut vec: Vec<char> = input.chars().map(|c| *segments.get(&c).unwrap()).collect();
    vec.sort_unstable();
    vec.iter().collect()
}

fn filter_pattern(patterns: &[&str], filter_len: usize, filter_chars: Vec<char>) -> char {
    patterns
        .iter()
        .find(|p| p.len() == filter_len)
        .unwrap()
        .chars()
        .find(|ch| !filter_chars.contains(ch))
        .unwrap()
}

#[cfg(test)]
mod day08_tests {
    use crate::{part1, part2};

    static TEST_INPUT: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT).unwrap(), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT).unwrap(), 61229);
    }
}
