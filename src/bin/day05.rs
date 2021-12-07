use anyhow::Result;
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

fn generate_points_in_line(a: &Point, b: &Point) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let mut x = a.x;
    let mut y = a.y;
    let dx = get_step(a.x, b.x) as i32;
    let dy = get_step(a.y, b.y) as i32;
    while x != b.x || y != b.y {
        points.push(Point::new(x, y));
        x += dx;
        y += dy;
    }
    points.push(Point::new(b.x, b.y));
    points
}

fn get_step(m: i32, target: i32) -> i8 {
    match m.cmp(&target) {
        Ordering::Less => 1,
        Ordering::Greater => -1,
        Ordering::Equal => 0,
    }
}

fn parse_input(input: &str, format: &Regex) -> Result<(Point, Point)> {
    let items = format
        .captures(input)
        .ok_or(anyhow::anyhow!("Error during captures"))?
        .iter()
        .skip(1)
        .flatten()
        .map(|item| item.as_str().parse())
        .collect::<Result<Vec<i32>, _>>()?;
    Ok((
        Point::new(items[0], items[1]),
        Point::new(items[2], items[3]),
    ))
}

fn calculate_overlap_count(input: &str, include_diagonals: bool) -> Result<usize> {
    let format = Regex::new(r"(\d*),(\d*) -> (\d*),(\d*)")?;

    let input = input
        .lines()
        .map(|line| parse_input(line, &format))
        .collect::<Result<Vec<(Point, Point)>>>()?;

    let overlap_counts = input
        .iter()
        .filter(|(a, b)| include_diagonals || (a.x == b.x || a.y == b.y))
        .map(|(a, b)| generate_points_in_line(a, b))
        .flatten()
        .fold(HashMap::new(), |mut acc, point| {
            *acc.entry(point).or_insert(0) += 1;
            acc
        });

    Ok(overlap_counts.iter().filter(|entry| *entry.1 > 1).count())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day05.txt")?;

    println!("{}", calculate_overlap_count(&input, false)?);
    println!("{}", calculate_overlap_count(&input, true)?);

    Ok(())
}

#[cfg(test)]
mod day05_tests {
    use crate::calculate_overlap_count;

    #[test]
    fn test_both_parts() {
        let input = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(calculate_overlap_count(input, false).unwrap(), 5);
        assert_eq!(calculate_overlap_count(input, true).unwrap(), 12)
    }
}
