use utils::{grid::Grid, string::read_string, vector::UniquePermutations};

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day 11 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt", 1_000_000);
    println!("Day 11 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> usize {
    let input = read_string(file_name);
    let universe = Grid::parse(&input);
    let (row_expansion, col_expansion) = universe_expansion(&universe);

    find_inter_galactic_distances(universe, row_expansion, col_expansion, 2)
}

fn part_two(file_name: &str, expansion_factor: usize) -> usize {
    let input = read_string(file_name);
    let universe = Grid::parse(&input);
    let (row_expansion, col_expansion) = universe_expansion(&universe);

    find_inter_galactic_distances(universe, row_expansion, col_expansion, expansion_factor)
}

fn find_inter_galactic_distances(
    universe: Grid<char>,
    row_expansion: Vec<usize>,
    col_expansion: Vec<usize>,
    multiplier: usize,
) -> usize {
    let galaxy_indices = universe.find_all('#');
    let galaxy_permutations = galaxy_indices.unique_permutations();

    // since we can only move in cardinal directions the manhattan distance is easy to calculate
    galaxy_permutations
        .iter()
        .fold(0, |acc, ((x1, y1), (x2, y2))| {
            let row_crossings = row_expansion
                .iter()
                .filter(|&index| index > y1.min(y2) && index < y1.max(y2))
                .collect::<Vec<_>>()
                .len();

            let col_crossings = col_expansion
                .iter()
                .filter(|&index| index > x1.min(x2) && index < x1.max(x2))
                .collect::<Vec<_>>()
                .len();

            let delta_x = multiplier * col_crossings - col_crossings + *x2.max(x1) - *x2.min(x1);
            let delta_y = multiplier * row_crossings - row_crossings + *y2.max(y1) - *y2.min(y1);

            acc + delta_x + delta_y
        })
}

fn universe_expansion(universe: &Grid<char>) -> (Vec<usize>, Vec<usize>) {
    let mut row_expansion = vec![];
    for row_idx in 0..universe.height {
        if universe.row(row_idx).iter().all(|&&c| c == '.') {
            row_expansion.push(row_idx)
        }
    }
    let mut col_expansion = vec![];
    for col_idx in 0..universe.width {
        if universe.col(col_idx).iter().all(|&&c| c == '.') {
            col_expansion.push(col_idx)
        }
    }
    (row_expansion, col_expansion)
}

#[cfg(test)]
mod test {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(374, part_one("data/example.txt"));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(1030, part_two("data/example.txt", 10));
        assert_eq!(8410, part_two("data/example.txt", 100));
    }
}
