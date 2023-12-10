use std::usize;

use utils::string::read_string;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day 10 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt");
    println!("Day 10 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> i32 {
    let input = read_string(file_name);
    let (cost, _) = parse_grid(&input);
    cost
}

fn part_two(file_name: &str) -> i32 {
    let input = read_string(file_name);
    let (_, area) = parse_grid(&input);
    area
}

/**
 * To calculate the area we use the Shoelace formula (https://en.wikipedia.org/wiki/Shoelace_formula)
 * along with Pick's Theorem (https://en.wikipedia.org/wiki/Pick%27s_theorem) (here to find the interior points).
 * The rearranged formula looks like:
 *      A = i + b/2 - 1 -> i = A - b / 2 - 1
 *
 * To use this we need to track the corners. The first corner is the start position.
 */
fn parse_grid(input: &str) -> (i32, i32) {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = grid[0].len();

    let mut corner = grid
        .iter()
        .flatten()
        .position(|&h| h == 'S')
        .map(|idx| (idx % width, idx / width))
        .unwrap();

    // TODO: This could be extracted into utils
    let follow = |(x, y): (usize, usize), direction: Direction| match direction {
        Direction::UP => (x, y - 1),
        Direction::DOWN => (x, y + 1),
        Direction::LEFT => (x - 1, y),
        Direction::RIGHT => (x + 1, y),
    };

    // To make things easier we only follow either up or down on the initial corner (not left or right)
    let mut direction = if matches!(grid[corner.1 - 1][corner.0], '|' | '7' | 'F') {
        Direction::UP
    } else {
        Direction::DOWN
    };
    let mut current = follow(corner, direction);

    let mut steps = 1;
    let mut area = 0;

    let determinant =
        |a: (usize, usize), b: (usize, usize)| a.0 as i32 * b.1 as i32 - a.1 as i32 * b.0 as i32;

    loop {
        // simply follow straight paths - can skip doing fancy maths here...
        while grid[current.1][current.0] == '-' || grid[current.1][current.0] == '|' {
            current = follow(current, direction); // reuse the direction since we're moving in a straight line
            steps += 1
        }

        direction = match grid[current.1][current.0] {
            '7' if direction == Direction::UP => Direction::LEFT,
            'F' if direction == Direction::UP => Direction::RIGHT,
            'J' if direction == Direction::DOWN => Direction::LEFT,
            'L' if direction == Direction::DOWN => Direction::RIGHT,
            'J' | 'L' => Direction::UP,
            '7' | 'F' => Direction::DOWN,
            _ => {
                area += determinant(corner, current);
                break;
            }
        };

        // update steps and the shoelace area
        steps += 1;
        area += determinant(corner, current);

        // we just turned a corner!
        corner = current;
        current = follow(current, direction);
    }

    let furthest_distance = steps / 2;
    let area = area.abs() / 2;
    let interior_points = area - steps / 2 + 1;
    (furthest_distance, interior_points)
}

#[cfg(test)]
mod test {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(4, part_one("data/example.txt"));
        assert_eq!(8, part_one("data/example_2.txt"));
        assert_eq!(6875, part_one("data/input.txt"));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(1, part_two("data/example.txt"));
        assert_eq!(4, part_two("data/example_3.txt"));
        assert_eq!(471, part_two("data/input.txt"));
    }
}
