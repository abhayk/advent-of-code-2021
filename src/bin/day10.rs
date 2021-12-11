use std::{collections::VecDeque, fs};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day10.txt")?;
    println!("{}", part1(&input));
    println!("{}", part2(&input));
    Ok(())
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|line| find_corrupted_char(line))
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut scores = input
        .lines()
        .map(|line| calculate_completion_score(line))
        .collect::<Vec<u64>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn find_corrupted_char(line: &str) -> Option<char> {
    let mut stack = VecDeque::new();
    for c in line.chars() {
        match c {
            '[' | '(' | '{' | '<' => stack.push_back(c),
            ']' | ')' | '}' | '>' => {
                let t = stack.pop_back().unwrap();
                if t != find_opening_pair(c) {
                    return Some(c);
                }
            }
            _ => unreachable!(),
        };
    }
    None
}

fn calculate_completion_score(line: &str) -> u64 {
    let mut stack = VecDeque::new();
    for c in line.chars() {
        match c {
            '[' | '(' | '{' | '<' => {
                stack.push_back(c);
            }
            ']' | ')' | '}' | '>' => {
                stack.pop_back();
            }
            _ => unreachable!(),
        };
    }
    stack
        .iter()
        .rev()
        .map(|c| find_closing_pair(*c))
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!(),
        })
        .fold(0, |acc, v| acc * 5 + v)
}

fn find_opening_pair(c: char) -> char {
    match c {
        ']' => '[',
        ')' => '(',
        '}' => '{',
        '>' => '<',
        _ => unreachable!(),
    }
}

fn find_closing_pair(c: char) -> char {
    match c {
        '[' => ']',
        '(' => ')',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod day10_tests {
    use crate::{calculate_completion_score, find_corrupted_char, part1, part2};

    static TEST_INPUT: &str = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 26397);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 288957);
    }

    #[test]
    fn test_find_corrupted_char() {
        assert_eq!(
            find_corrupted_char("{([(<{}[<>[]}>{[]{[(<()>").unwrap(),
            '}'
        );
        assert_eq!(find_corrupted_char("[[<[([]))<([[{}[[()]]]").unwrap(), ')');
        assert_eq!(find_corrupted_char("[{[{({}]{}}([{[{{{}}([]").unwrap(), ']');
        assert_eq!(find_corrupted_char("[<(<(<(<{}))><([]([]()").unwrap(), ')');
        assert_eq!(find_corrupted_char("<{([([[(<>()){}]>(<<{{").unwrap(), '>');
    }

    #[test]
    fn test_calculate_completion_score() {
        assert_eq!(
            calculate_completion_score("[({(<(())[]>[[{[]{<()<>>"),
            288957
        );
        assert_eq!(calculate_completion_score("[(()[<>])]({[<{<<[]>>("), 5566);
        assert_eq!(
            calculate_completion_score("(((({<>}<{<{<>}{[]{[]{}"),
            1480781
        );
        assert_eq!(
            calculate_completion_score("{<[[]]>}<{[{[{[]{()[[[]"),
            995444
        );
        assert_eq!(calculate_completion_score("<{([{{}}[<[[[<>{}]]]>[]]"), 294);
    }
}
