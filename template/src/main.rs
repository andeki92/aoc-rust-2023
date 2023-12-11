use utils::string::{read, read_string};

type SolutionType = u32;

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day {{.day}} - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt");
    println!("Day {{.day}} - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> SolutionType {
    let input = read(file_name);
    todo!()
}

fn part_two(file_name: &str) -> SolutionType {
    let input = read_string(file_name);
    todo!()
}

#[cfg(test)]
mod test {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(0, part_one("data/example.txt"))
    }

    #[test]
    fn part_two_test() {
        assert_eq!(0, part_two("data/example.txt"))
    }
}
