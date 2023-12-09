use std::vec;

use itertools::Itertools;
use regex::Regex;
use utils::string::read;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

#[derive(Debug)]
struct Part {
    part_no: u32,
    indices: Vec<Point>,
}

#[derive(Debug)]
struct Symbol {
    idx: Point,
    char: char,
}

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day 3 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt");
    println!("Day 3 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> u32 {
    let lines = read(file_name);
    let (symbols, parts) = parse_schematic(lines);
    let symbol_points: Vec<Point> = symbols.iter().map(|s| s.idx).collect();

    parts
        .iter()
        .filter_map(|part| {
            if neighbours(&part.indices)
                .iter()
                .any(|n| symbol_points.contains(n))
            {
                Some(part.part_no)
            } else {
                None
            }
        })
        .sum::<u32>()
}

fn part_two(file_name: &str) -> u32 {
    let lines = read(file_name);
    let (symbols, parts) = parse_schematic(lines);

    symbols
        .iter()
        .filter(|s| s.char == '*')
        .filter_map(|s| {
            let neighbours = neighbour(&s.idx);

            let viable_parts: Vec<&Part> = parts
                .iter()
                .filter(|&part| neighbours.iter().any(|n| part.indices.contains(n)))
                .collect();

            if viable_parts.len() > 1 {
                Some(viable_parts.iter().fold(1, |acc, part| acc * part.part_no))
            } else {
                None
            }
        })
        .sum()
}

fn parse_schematic(lines: Vec<String>) -> (Vec<Symbol>, Vec<Part>) {
    let number_matcher = Regex::new(r"\d+").unwrap();

    let mut parts: Vec<Part> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    lines.iter().enumerate().for_each(|(idy, line)| {
        number_matcher.find_iter(line).for_each(|caps| {
            let part_no = caps
                .as_str()
                .parse::<u32>()
                .expect("Failed to parse part number!");
            let indices = (caps.start()..caps.end())
                .map(|idx| Point(idx as i32, idy as i32))
                .collect();

            parts.push(Part { part_no, indices });
        });

        line.char_indices().for_each(|(idx, char)| {
            if !char.is_alphanumeric() && char != '.' {
                symbols.push(Symbol {
                    idx: Point(idx as i32, idy as i32),
                    char,
                });
            }
        });
    });

    (symbols, parts)
}

fn neighbours(indices: &Vec<Point>) -> Vec<Point> {
    indices
        .iter()
        .flat_map(|idx| neighbour(idx))
        .unique()
        .collect()
}

fn neighbour(point: &Point) -> Vec<Point> {
    return vec![
        Point(point.0 - 1, point.1 - 1),
        Point(point.0 - 1, point.1),
        Point(point.0 - 1, point.1 + 1),
        Point(point.0, point.1 - 1),
        Point(point.0, point.1),
        Point(point.0, point.1 + 1),
        Point(point.0 + 1, point.1 - 1),
        Point(point.0 + 1, point.1),
        Point(point.0 + 1, point.1 + 1),
    ];
}

#[cfg(test)]
mod test {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(4361, part_one("data/example.txt"))
    }

    #[test]
    fn part_two_test() {
        assert_eq!(467835, part_two("data/example.txt"))
    }
}
