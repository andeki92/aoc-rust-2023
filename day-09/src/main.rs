use utils::string::read;

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day 9 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt");
    println!("Day 9 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> i32 {
    let input = read(file_name);

    input
        .iter()
        .map(|line| parse_history(line))
        .map(|seq| predict_value(seq))
        .sum()
}

fn part_two(file_name: &str) -> i32 {
    let input = read(file_name);

    input
        .iter()
        .map(|line| parse_history(line))
        .map(|seq| seq.iter().copied().rev().collect::<Vec<_>>())
        .map(|rev_seq| predict_value(rev_seq))
        .sum()
}

fn predict_value(sequence: Vec<i32>) -> i32 {
    let mut current_sequence = sequence;
    let mut last_values: Vec<i32> = vec![];

    while current_sequence.iter().any(|&e| e != 0) {
        last_values.push(*current_sequence.last().unwrap());

        let next_sequence = current_sequence
            .windows(2)
            .map(|win| win[1] - win[0])
            .collect::<Vec<_>>();

        current_sequence = next_sequence
    }

    // starting at 0 we can fold the reversed list back up to get our prediction
    last_values.iter().rev().fold(0, |acc, e| acc + e)
}

fn parse_history(input: &str) -> Vec<i32> {
    input
        .trim()
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(114, part_one("data/example.txt"));
        assert_eq!(1696140818, part_one("data/input.txt"));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(2, part_two("data/example.txt"));
        assert_eq!(1152, part_two("data/input.txt"));
    }
}
