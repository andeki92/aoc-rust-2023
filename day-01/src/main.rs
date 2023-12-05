use utils::read;

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day 1 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt");
    println!("Day 1 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> u32 {
    read(file_name)
        .iter()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| {
            10 * vec
                .first()
                .expect("Every line MUST contain at least one digit")
                + vec
                    .last()
                    .expect("Every line MUST contain at least one digit")
        })
        .sum::<u32>()
}

fn part_two(file_name: &str) -> u32 {
    read(file_name)
        .iter()
        .map(|line| {
            line.to_string()
                .replace("zero", "zero0zero")
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum::<u32>()
}

#[cfg(test)]
mod test {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(142, part_one("data/example.txt"))
    }

    #[test]
    fn part_two_test() {
        assert_eq!(281, part_two("data/example_2.txt"))
    }
}
