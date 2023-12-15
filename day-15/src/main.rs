use std::vec;

use utils::string::read_string;

type SolutionType = usize;

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day 15 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt");
    println!("Day 15 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> SolutionType {
    read_string(file_name)
        .split(",")
        .map(|sequence| hash_string(sequence.trim()))
        .sum()
}

fn part_two(file_name: &str) -> SolutionType {
    const V: Vec<(&str, usize)> = vec![];
    let mut boxes = [V; 256];

    let content = read_string(file_name);
    let trimmed = content.trim_end_matches('\n');

    trimmed.split(",").for_each(|s| {
        if let Some(label) = s.strip_suffix('-') {
            // remove the label from the box
            println!("Removing {}", label);
            boxes[hash_string(label)].retain(|&(l, _)| l != label);
        } else if let Some((label, focal_length)) = s.split_once("=") {
            let box_content = &mut boxes[hash_string(label)];
            let focal_length = focal_length.parse::<usize>().unwrap();
            if let Some(index) = box_content.iter().position(|&(l, _)| l == label) {
                box_content[index] = (label, focal_length)
            } else {
                box_content.push((label, focal_length));
            }
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(box_index, foci)| {
            (box_index + 1)
                * foci
                    .iter()
                    .enumerate()
                    .map(|(index, (_, focal_length))| (index + 1) * focal_length)
                    .sum::<usize>()
        })
        .sum()
}

fn hash_string(input: &str) -> usize {
    let mut current_value = 0;
    for char in input.chars() {
        current_value += char as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

#[cfg(test)]
mod test {
    use crate::{hash_string, part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(1320, part_one("data/example.txt"));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(145, part_two("data/example.txt"))
    }

    #[test]
    fn calculate_hash_test() {
        assert_eq!(52, hash_string(r"HASH"));
    }

    #[test]
    fn linked_hashmap_test() {}
}
