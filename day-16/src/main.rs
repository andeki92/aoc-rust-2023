use std::collections::HashSet;

use utils::{
    grid::Grid,
    point::{Direction, Directional, Point},
    string::read_string,
};

type SolutionType = usize;

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day 16 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt");
    println!("Day 16 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> SolutionType {
    let input = read_string(file_name);
    let grid: Grid<char> = Grid::parse(&input);

    energized_cells(&grid, (Point::new(0, 0), Direction::RIGHT))
}

fn part_two(file_name: &str) -> SolutionType {
    let input = read_string(file_name);
    let grid: Grid<char> = Grid::parse(&input);

    let mut max_cells = 0;
    let mut starting_positions = vec![];

    for x in 0..grid.width {
        starting_positions.push((Point::new(x as i64, 0), Direction::DOWN));
        starting_positions.push((Point::new(x as i64, grid.height as i64 - 1), Direction::UP));
    }

    for y in 0..grid.height {
        starting_positions.push((Point::new(0, y as i64), Direction::RIGHT));
        starting_positions.push((Point::new(grid.width as i64 - 1, y as i64), Direction::LEFT));
    }

    for start in starting_positions {
        let energized_cells = energized_cells(&grid, start);
        max_cells = max_cells.max(energized_cells);
    }
    max_cells
}

fn energized_cells(grid: &Grid<char>, start: (Point, Direction)) -> usize {
    // The positions would be HEAVILY cacheable - maybe something to look into in the future...

    let mut history: HashSet<(Point, Direction)> = HashSet::new();
    let mut beams: Vec<(Point, Direction)> = vec![];

    beams.push(start);
    history.insert(start);

    let in_grid =
        |Point { x, y }: Point| x >= 0 && x < grid.width as i64 && y >= 0 && y < grid.height as i64;

    loop {
        if let Some((current, direction)) = beams.pop() {
            if let Some(char) = grid
                .row(current.y as usize)
                .and_then(|row| row.get(current.x as usize).and_then(|&&c| Some(c)))
            {
                let next: Vec<(Point, Direction)> = match char {
                    '-' if (direction == Direction::DOWN || direction == Direction::UP) => {
                        vec![current.right(), current.left()]
                    }
                    '|' if (direction == Direction::LEFT || direction == Direction::RIGHT) => {
                        vec![current.up(), current.down()]
                    }
                    '.' | '-' | '|' => vec![current.follow(&direction)],
                    // TODO: Implement clockwise and counterclockwise for point...
                    '/' if (direction == Direction::RIGHT) => {
                        vec![current.up()]
                    }
                    '/' if (direction == Direction::LEFT) => {
                        vec![current.down()]
                    }
                    '/' if (direction == Direction::DOWN) => {
                        vec![current.left()]
                    }
                    '/' if (direction == Direction::UP) => {
                        vec![current.right()]
                    }
                    '\\' if (direction == Direction::RIGHT) => {
                        vec![current.down()]
                    }
                    '\\' if (direction == Direction::LEFT) => {
                        vec![current.up()]
                    }
                    '\\' if (direction == Direction::DOWN) => {
                        vec![current.right()]
                    }
                    '\\' if (direction == Direction::UP) => {
                        vec![current.left()]
                    }
                    _ => unreachable!(),
                };

                for n in next {
                    if !history.contains(&n) && in_grid(n.0) {
                        history.insert(n);
                        beams.push(n)
                    }
                }
            }
        } else {
            break;
        }
    }

    let mut energized_cells: HashSet<Point> = HashSet::new();
    history.iter().for_each(|&(point, _)| {
        energized_cells.insert(point);
    });

    energized_cells.iter().len()
}

#[cfg(test)]
mod test {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(46, part_one("data/example.txt"));
        assert_eq!(6855, part_one("data/input.txt"));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(51, part_two("data/example.txt"));
        assert_eq!(7513, part_two("data/input.txt"));
    }
}
