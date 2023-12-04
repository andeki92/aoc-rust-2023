use std::{collections::HashMap, fs};

fn main() {
    let part_one_example = part_one("data/example.txt");
    assert_eq!(part_one_example, 13);

    let part_one_solution = part_one("data/input.txt");
    println!("Day 4 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_example = part_two("data/example.txt");
    assert_eq!(part_two_example, 30);

    let part_two_solution = part_two("data/input.txt");
    println!("Day 4 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_path: &str) -> u32 {
    fs::read_to_string(file_path)
        .expect("Failed to read file")
        .trim_end()
        .split("\n")
        .map(|line| parse_cards(line))
        .filter_map(|matches| {
            if matches > 0 {
                Some(u32::pow(2, (matches - 1) as u32))
            } else {
                None
            }
        })
        .sum::<u32>()
}

fn part_two(file_path: &str) -> u32 {
    let mut card_count = HashMap::new();

    fs::read_to_string(file_path)
        .expect("Failed to read file")
        .trim_end()
        .split("\n")
        .map(|line| parse_cards(line))
        .enumerate()
        .fold(0, |acc, (idx, matches)| {
            let id = idx + 1;
            let current_card_count = *card_count.entry(id).or_insert(1);
            let next_id = id + 1;

            (next_id..next_id + matches).for_each(|next_id|
                // it is kinda cool that this works
                *card_count.entry(next_id).or_insert(1) += current_card_count);

            acc + current_card_count
        })
}

fn parse_cards(line: &str) -> usize {
    let mut split_line = line.split(":");
    let _ = split_line.next(); // this would contain the card id

    let nums = split_line.next().unwrap();

    let mut split_nums = nums.splitn(2, "|");
    let target_nums = parse_nums(split_nums.next().unwrap());
    let owned_nums = parse_nums(split_nums.next().unwrap());

    target_nums
        .iter()
        .filter(|wn| owned_nums.contains(wn))
        .count()
}

fn parse_nums(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}
