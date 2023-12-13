use std::{collections::HashSet, error, usize};

use utils::{grid::Grid, string::read_string};

type SolutionType = usize;

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day 13 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt");
    println!("Day 13 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> SolutionType {
    let input = read_string(file_name);
    let mirrors = parse_mirrors(&input);
    mirrors
        .iter()
        .map(|mirror| find_mirror_line(mirror, 0).unwrap())
        .sum()
}

fn part_two(file_name: &str) -> SolutionType {
    let input = read_string(file_name);
    let mirrors = parse_mirrors(&input);
    mirrors
        .iter()
        .map(|mirror: &Grid<char>| find_alternate_mirror_line(mirror))
        .sum()
}

fn parse_mirrors(input: &str) -> Vec<Grid<char>> {
    input.split("\n\n").map(|i| Grid::parse(i)).collect()
}

fn find_alternate_mirror_line(mirror: &Grid<char>) -> usize {
    find_mirror_line(mirror, 1).unwrap()
}

fn find_mirror_line(mirror: &Grid<char>, allowed_errrors: usize) -> Option<usize> {
    if let Some(horizontal_match) = find_horizontal_line(&mirror, allowed_errrors) {
        return Some(horizontal_match * 100);
    }
    if let Some(vertical_match) = find_vertical_line(&mirror, allowed_errrors) {
        return Some(vertical_match);
    }
    None
}

fn find_vertical_line(mirror: &Grid<char>, allowed_errrors: usize) -> Option<usize> {
    for v_line in 1..mirror.width {
        let left_side = (0..v_line)
            .rev()
            .map(|index| mirror.col(index))
            .collect::<Vec<_>>();

        let right_side = (v_line..mirror.width)
            .map(|index| mirror.col(index))
            .collect::<Vec<_>>();

        let cutoff = left_side.len().min(right_side.len());

        let mut errors = 0;

        for (index, line) in left_side[0..cutoff].iter().enumerate() {
            for (char_index, &ch) in line.iter().enumerate() {
                if ch != right_side[index][char_index] {
                    errors += 1
                }
                if errors > allowed_errrors {
                    break;
                }
            }
        }

        println!("Number of errors: {}", errors);

        if errors == allowed_errrors {
            return Some(v_line);
        }
    }
    None
}

fn find_horizontal_line(mirror: &Grid<char>, allowed_errrors: usize) -> Option<usize> {
    for h_line in 1..mirror.height {
        let top_side = (0..h_line)
            .rev()
            .map(|index| mirror.row(index))
            .collect::<Vec<_>>();

        let bottom_side = (h_line..mirror.height)
            .map(|index| mirror.row(index))
            .collect::<Vec<_>>();

        let cutoff = top_side.len().min(bottom_side.len());

        let mut errors = 0;

        for (index, line) in top_side[0..cutoff].iter().enumerate() {
            for (char_index, &ch) in line.iter().enumerate() {
                if ch != bottom_side[index][char_index] {
                    errors += 1
                }
                if errors > allowed_errrors {
                    break;
                }
            }
        }

        if errors == allowed_errrors {
            return Some(h_line);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use utils::grid::Grid;

    use crate::{find_alternate_mirror_line, find_mirror_line, part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(405, part_one("data/example.txt"));
        assert_eq!(34911, part_one("data/input.txt"));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(400, part_two("data/example.txt"));
        assert_eq!(33183, part_two("data/input.txt"));
    }

    #[test]
    fn mirror_line_test() {
        let mirror = Grid {
            width: 9,
            height: 7,
            data: [
                '#', '.', '#', '#', '.', '.', '#', '#', '.', /* formatting */
                '.', '.', '#', '.', '#', '#', '.', '#', '.', /* formatting */
                '#', '#', '.', '.', '.', '.', '.', '.', '#', /* formatting */
                '#', '#', '.', '.', '.', '.', '.', '.', '#', /* formatting */
                '.', '.', '#', '.', '#', '#', '.', '#', '.', /* formatting */
                '.', '.', '#', '#', '.', '.', '#', '#', '.', /* formatting */
                '#', '.', '#', '.', '#', '#', '.', '#', '.', /* formatting */
            ]
            .to_vec(),
        };

        assert_eq!(Some(5), find_mirror_line(&mirror, 0));
    }

    #[test]
    fn alternate_mirror_line_test() {
        let mirror = Grid {
            width: 9,
            height: 7,
            data: [
                '#', '.', '#', '#', '.', '.', '#', '#', '.', /* formatting */
                '.', '.', '#', '.', '#', '#', '.', '#', '.', /* formatting */
                '#', '#', '.', '.', '.', '.', '.', '.', '#', /* formatting */
                '#', '#', '.', '.', '.', '.', '.', '.', '#', /* formatting */
                '.', '.', '#', '.', '#', '#', '.', '#', '.', /* formatting */
                '.', '.', '#', '#', '.', '.', '#', '#', '.', /* formatting */
                '#', '.', '#', '.', '#', '#', '.', '#', '.', /* formatting */
            ]
            .to_vec(),
        };

        assert_eq!(300, find_alternate_mirror_line(&mirror));
    }

    #[test]
    fn second_alternate_mirror_line_test() {
        let mirror = Grid {
            width: 9,
            height: 7,
            data: [
                '#', '.', '.', '.', '#', '#', '.', '.', '#', /* formatting */
                '#', '.', '.', '.', '.', '#', '.', '.', '#', /* formatting */
                '.', '.', '#', '#', '.', '.', '#', '#', '#', /* formatting */
                '#', '#', '#', '#', '#', '.', '#', '#', '.', /* formatting */
                '#', '#', '#', '#', '#', '.', '#', '#', '.', /* formatting */
                '.', '.', '#', '#', '.', '.', '#', '#', '#', /* formatting */
                '#', '.', '.', '.', '.', '#', '.', '.', '#',
            ]
            .to_vec(),
        };

        assert_eq!(100, find_alternate_mirror_line(&mirror));
    }
}
