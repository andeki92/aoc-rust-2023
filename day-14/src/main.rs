use std::collections::{HashMap, VecDeque};

use utils::{grid::Grid, string::read_string};

type SolutionType = usize;

#[derive(Debug)]
enum Direction {
    NORTH,
    WEST,
    SOUTH,
    EAST,
}

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day 14 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt");
    println!("Day 14 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> SolutionType {
    let input = read_string(file_name);
    let grid = Grid::parse(&input);

    let new_grid = tilt(grid, Direction::NORTH);
    calculate_load(&new_grid)
}

fn part_two(file_name: &str) -> SolutionType {
    let input = read_string(file_name);
    let grid = Grid::parse(&input);
    let new_grid = run_cycles(grid, 1_000_000_000);

    calculate_load(&new_grid)
}

fn run_cycles(mut grid: Grid<char>, cycles: usize) -> Grid<char> {
    let mut memo = HashMap::new();
    let mut memo_index = HashMap::new();
    let mut index = 0;

    let start = loop {
        if index > cycles {
            panic!("Exceeded number of cycles")
        }

        let original = grid.clone();
        grid = tilt(grid, Direction::NORTH);
        grid = tilt(grid, Direction::WEST);
        grid = tilt(grid, Direction::SOUTH);
        grid = tilt(grid, Direction::EAST);

        memo.insert(original, (grid.clone(), index));
        memo_index.insert(index, grid.clone());
        index += 1;

        if let Some((_, start)) = memo.get(&grid) {
            break *start;
        }
    };

    let offset = cycles - start;
    let final_index = start + offset % (memo.len() - start) - 1;

    memo_index.get(&final_index).unwrap().clone()
}

fn tilt_section(section: &Vec<&char>) -> Vec<char> {
    let (mut new_column, mut section) = section.iter().fold(
        (vec![], VecDeque::new()),
        |(mut new_column, mut section), &&c| {
            match c {
                '.' => section.push_back(c),
                'O' => section.push_front(c),
                '#' => {
                    section.push_back(c);
                    while let Some(e) = section.pop_front() {
                        new_column.push(e);
                    }
                }
                _ => unreachable!(),
            }
            (new_column, section)
        },
    );

    // make sure we empty the last section
    while let Some(e) = section.pop_front() {
        new_column.push(e);
    }
    new_column
}

fn tilt(grid: Grid<char>, direction: Direction) -> Grid<char> {
    match direction {
        Direction::NORTH => Grid::from_cols(
            grid.columns()
                .iter()
                .map(|column| tilt_section(column))
                .collect::<Vec<_>>(),
        ),
        Direction::WEST => Grid::from_rows(
            grid.rows()
                .iter()
                .map(|row| tilt_section(row))
                .collect::<Vec<_>>(),
        ),
        Direction::SOUTH => Grid::from_cols(
            grid.columns()
                .iter()
                .map(|column| {
                    tilt_section(&column.iter().rev().map(|&c| c).collect::<Vec<_>>())
                        .iter()
                        .rev()
                        .map(|&c| c)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        ),
        Direction::EAST => Grid::from_rows(
            grid.rows()
                .iter()
                .map(|row| {
                    tilt_section(&row.iter().rev().map(|&c| c).collect::<Vec<_>>())
                        .iter()
                        .rev()
                        .map(|&c| c)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        ),
    }
}

fn calculate_load(grid: &Grid<char>) -> usize {
    grid.columns()
        .iter()
        .map(|column| {
            column
                .iter()
                .enumerate()
                .map(|(index, &&c)| if c == 'O' { grid.height - index } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use utils::grid::Grid;

    use crate::{part_one, part_two, tilt, Direction};

    #[test]
    fn part_one_test() {
        assert_eq!(136, part_one("data/example.txt"));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(64, part_two("data/example.txt"));
    }

    #[test]
    fn cycle_north_test() {
        let grid = Grid {
            width: 10,
            height: 10,
            data: [
                'O', '.', '.', '.', '.', '#', '.', '.', '.', '.', /* fmt */
                'O', '.', 'O', 'O', '#', '.', '.', '.', '.', '#', /* fmt */
                '.', '.', '.', '.', '.', '#', '#', '.', '.', '.', /* fmt */
                'O', 'O', '.', '#', 'O', '.', '.', '.', '.', 'O', /* fmt */
                '.', 'O', '.', '.', '.', '.', '.', 'O', '#', '.', /* fmt */
                'O', '.', '#', '.', '.', 'O', '.', '#', '.', '#', /* fmt */
                '.', '.', 'O', '.', '.', '#', 'O', '.', '.', 'O', /* fmt */
                '.', '.', '.', '.', '.', '.', '.', 'O', '.', '.', /* fmt */
                '#', '.', '.', '.', '.', '#', '#', '#', '.', '.', /* fmt */
                '#', 'O', 'O', '.', '.', '#', '.', '.', '.', '.', /* fmt */
            ]
            .to_vec(),
        };

        let expected_grid = Grid {
            width: 10,
            height: 10,
            data: [
                'O', 'O', 'O', 'O', '.', '#', '.', 'O', '.', '.', /* fmt */
                'O', 'O', '.', '.', '#', '.', '.', '.', '.', '#', /* fmt */
                'O', 'O', '.', '.', 'O', '#', '#', '.', '.', 'O', /* fmt */
                'O', '.', '.', '#', '.', 'O', 'O', '.', '.', '.', /* fmt */
                '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', /* fmt */
                '.', '.', '#', '.', '.', '.', '.', '#', '.', '#', /* fmt */
                '.', '.', 'O', '.', '.', '#', '.', 'O', '.', 'O', /* fmt */
                '.', '.', 'O', '.', '.', '.', '.', '.', '.', '.', /* fmt */
                '#', '.', '.', '.', '.', '#', '#', '#', '.', '.', /* fmt */
                '#', '.', '.', '.', '.', '#', '.', '.', '.', '.', /* fmt */
            ]
            .to_vec(),
        };

        assert_eq!(tilt(grid, Direction::NORTH).data, expected_grid.data);
    }

    #[test]
    fn cycle_south_test() {
        let grid = Grid {
            width: 10,
            height: 10,
            data: [
                'O', '.', '.', '.', '.', '#', '.', '.', '.', '.', /* fmt */
                'O', '.', 'O', 'O', '#', '.', '.', '.', '.', '#', /* fmt */
                '.', '.', '.', '.', '.', '#', '#', '.', '.', '.', /* fmt */
                'O', 'O', '.', '#', 'O', '.', '.', '.', '.', 'O', /* fmt */
                '.', 'O', '.', '.', '.', '.', '.', 'O', '#', '.', /* fmt */
                'O', '.', '#', '.', '.', 'O', '.', '#', '.', '#', /* fmt */
                '.', '.', 'O', '.', '.', '#', 'O', '.', '.', 'O', /* fmt */
                '.', '.', '.', '.', '.', '.', '.', 'O', '.', '.', /* fmt */
                '#', '.', '.', '.', '.', '#', '#', '#', '.', '.', /* fmt */
                '#', 'O', 'O', '.', '.', '#', '.', '.', '.', '.', /* fmt */
            ]
            .to_vec(),
        };

        let expected_grid = Grid {
            width: 10,
            height: 10,
            data: [
                '.', '.', '.', '.', '.', '#', '.', '.', '.', '.', /* fmt */
                '.', '.', '.', '.', '#', '.', '.', '.', '.', '#', /* fmt */
                '.', '.', '.', 'O', '.', '#', '#', '.', '.', '.', /* fmt */
                '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', /* fmt */
                'O', '.', 'O', '.', '.', '.', '.', 'O', '#', 'O', /* fmt */
                'O', '.', '#', '.', '.', 'O', '.', '#', '.', '#', /* fmt */
                'O', '.', '.', '.', '.', '#', '.', '.', '.', '.', /* fmt */
                'O', 'O', '.', '.', '.', '.', 'O', 'O', '.', '.', /* fmt */
                '#', 'O', 'O', '.', '.', '#', '#', '#', '.', '.', /* fmt */
                '#', 'O', 'O', '.', 'O', '#', '.', '.', '.', 'O', /* fmt */
            ]
            .to_vec(),
        };

        assert_eq!(tilt(grid, Direction::SOUTH).data, expected_grid.data);
    }

    #[test]
    fn cycle_west_test() {
        let grid = Grid {
            width: 10,
            height: 10,
            data: [
                'O', '.', '.', '.', '.', '#', '.', '.', '.', '.', /* fmt */
                'O', '.', 'O', 'O', '#', '.', '.', '.', '.', '#', /* fmt */
                '.', '.', '.', '.', '.', '#', '#', '.', '.', '.', /* fmt */
                'O', 'O', '.', '#', 'O', '.', '.', '.', '.', 'O', /* fmt */
                '.', 'O', '.', '.', '.', '.', '.', 'O', '#', '.', /* fmt */
                'O', '.', '#', '.', '.', 'O', '.', '#', '.', '#', /* fmt */
                '.', '.', 'O', '.', '.', '#', 'O', '.', '.', 'O', /* fmt */
                '.', '.', '.', '.', '.', '.', '.', 'O', '.', '.', /* fmt */
                '#', '.', '.', '.', '.', '#', '#', '#', '.', '.', /* fmt */
                '#', 'O', 'O', '.', '.', '#', '.', '.', '.', '.', /* fmt */
            ]
            .to_vec(),
        };

        let expected_grid = Grid {
            width: 10,
            height: 10,
            data: [
                'O', '.', '.', '.', '.', '#', '.', '.', '.', '.', /* fmt */
                'O', 'O', 'O', '.', '#', '.', '.', '.', '.', '#', /* fmt */
                '.', '.', '.', '.', '.', '#', '#', '.', '.', '.', /* fmt */
                'O', 'O', '.', '#', 'O', 'O', '.', '.', '.', '.', /* fmt */
                'O', 'O', '.', '.', '.', '.', '.', '.', '#', '.', /* fmt */
                'O', '.', '#', 'O', '.', '.', '.', '#', '.', '#', /* fmt */
                'O', '.', '.', '.', '.', '#', 'O', 'O', '.', '.', /* fmt */
                'O', '.', '.', '.', '.', '.', '.', '.', '.', '.', /* fmt */
                '#', '.', '.', '.', '.', '#', '#', '#', '.', '.', /* fmt */
                '#', 'O', 'O', '.', '.', '#', '.', '.', '.', '.', /* fmt */
            ]
            .to_vec(),
        };

        assert_eq!(tilt(grid, Direction::WEST).data, expected_grid.data);
    }

    #[test]
    fn cycle_east_test() {
        let grid = Grid {
            width: 10,
            height: 10,
            data: [
                'O', '.', '.', '.', '.', '#', '.', '.', '.', '.', /* fmt */
                'O', '.', 'O', 'O', '#', '.', '.', '.', '.', '#', /* fmt */
                '.', '.', '.', '.', '.', '#', '#', '.', '.', '.', /* fmt */
                'O', 'O', '.', '#', 'O', '.', '.', '.', '.', 'O', /* fmt */
                '.', 'O', '.', '.', '.', '.', '.', 'O', '#', '.', /* fmt */
                'O', '.', '#', '.', '.', 'O', '.', '#', '.', '#', /* fmt */
                '.', '.', 'O', '.', '.', '#', 'O', '.', '.', 'O', /* fmt */
                '.', '.', '.', '.', '.', '.', '.', 'O', '.', '.', /* fmt */
                '#', '.', '.', '.', '.', '#', '#', '#', '.', '.', /* fmt */
                '#', 'O', 'O', '.', '.', '#', '.', '.', '.', '.', /* fmt */
            ]
            .to_vec(),
        };

        let expected_grid = Grid {
            width: 10,
            height: 10,
            data: [
                '.', '.', '.', '.', 'O', '#', '.', '.', '.', '.', /* fmt */
                '.', 'O', 'O', 'O', '#', '.', '.', '.', '.', '#', /* fmt */
                '.', '.', '.', '.', '.', '#', '#', '.', '.', '.', /* fmt */
                '.', 'O', 'O', '#', '.', '.', '.', '.', 'O', 'O', /* fmt */
                '.', '.', '.', '.', '.', '.', 'O', 'O', '#', '.', /* fmt */
                '.', 'O', '#', '.', '.', '.', 'O', '#', '.', '#', /* fmt */
                '.', '.', '.', '.', 'O', '#', '.', '.', 'O', 'O', /* fmt */
                '.', '.', '.', '.', '.', '.', '.', '.', '.', 'O', /* fmt */
                '#', '.', '.', '.', '.', '#', '#', '#', '.', '.', /* fmt */
                '#', '.', '.', 'O', 'O', '#', '.', '.', '.', '.', /* fmt */
            ]
            .to_vec(),
        };

        assert_eq!(tilt(grid, Direction::EAST).data, expected_grid.data);
    }
}
