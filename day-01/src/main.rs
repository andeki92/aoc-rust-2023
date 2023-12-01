use std::fs;

fn main() {
    let part_one_example = part_one("data/example.txt");
    assert_eq!(part_one_example, 142);

    let part_one_solution = part_one("data/input.txt");
    println!("Day 1 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_example = part_two("data/example_2.txt");
    assert_eq!(part_two_example, 281);

    let part_two_solution = part_two("data/input.txt");
    println!("Day 1 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_path: &str) -> u32 {
    fs::read_to_string(file_path)
        .expect("Failed to read file")
        .trim_end()
        .split("\n")
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

fn part_two(file_path: &str) -> u32 {
    fs::read_to_string(file_path)
        .expect("Failed to read file")
        .trim_end()
        .split("\n")
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
