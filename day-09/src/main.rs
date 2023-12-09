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
        .map(|seq| predict_next_value(seq))
        .sum()
}

fn part_two(file_name: &str) -> i32 {
    let input = read(file_name);

    input
        .iter()
        .map(|line| parse_history(line))
        .map(|seq| predict_previous_value(seq))
        .sum()
}

fn predict_next_value(sequence: Vec<i32>) -> i32 {
    let mut history: Vec<Vec<i32>> = vec![sequence];
    let mut current_sequence = 0;

    loop {
        // if the last element is all zeroes we can start extrapolating
        if history.last().unwrap().iter().all(|&e| e == 0) {
            (0..history.len() - 1).rev().for_each(|idx| {
                let last = *history[idx].last().unwrap();
                let previous = *history[idx + 1].last().unwrap_or_else(|| &0);
                history[idx].push(last + previous);
            });
            break;
        }

        let next_sequence = history[current_sequence]
            .windows(2)
            .map(|win| win[1] - win[0])
            .collect::<Vec<_>>();
        history.push(next_sequence);

        current_sequence += 1
    }

    *history.first().unwrap().last().unwrap()
}

fn predict_previous_value(sequence: Vec<i32>) -> i32 {
    let mut history: Vec<Vec<i32>> = vec![sequence];
    let mut current_sequence = 0;

    loop {
        // if the last element is all zeroes we can start extrapolating
        if history.last().unwrap().iter().all(|&e| e == 0) {
            (0..history.len() - 1).rev().for_each(|idx| {
                let first = *history[idx].first().unwrap();
                let previous = *history[idx + 1].first().unwrap_or_else(|| &0);
                history[idx].insert(0, first - previous);
            });
            break;
        }

        let next_sequence = history[current_sequence]
            .windows(2)
            .map(|win| win[1] - win[0])
            .collect::<Vec<_>>();
        history.push(next_sequence);

        current_sequence += 1
    }

    *history.first().unwrap().first().unwrap()
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
        assert_eq!(114, part_one("data/example.txt"))
    }

    #[test]
    fn part_two_test() {
        assert_eq!(2, part_two("data/example.txt"))
    }
}
