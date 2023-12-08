use std::collections::HashMap;

use utils::num::lcm;
use utils::string::read_string;

#[derive(Debug)]
struct Map {
    nodes: HashMap<String, (String, String)>,
    input: Vec<char>,
}

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day 8 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt");
    println!("Day 8 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> u32 {
    let input = read_string(file_name);
    let map = parse_nodes(input);

    find_output_node(&map, "AAA")
}

fn part_two(file_name: &str) -> u64 {
    let input = read_string(file_name);
    let map = parse_nodes(input);

    let starting_nodes: Vec<&String> = map
        .nodes
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<_>>();

    let node_distances = starting_nodes
        .iter()
        .map(|&node| find_output_node(&map, node) as u64)
        .collect::<Vec<_>>();

    node_distances
        .iter()
        .fold(1 as u64, |acc, &distance| lcm(acc, distance))
}

fn find_output_node(map: &Map, start_node: &str) -> u32 {
    let mut current_node: &str = start_node;
    let mut step: usize = 0;
    let input_size = map.input.len();

    while !current_node.ends_with("Z") {
        let next_step = map.input[step % input_size];

        if next_step == 'R' {
            current_node = &map.nodes.get(current_node).unwrap().1;
        } else {
            current_node = &map.nodes.get(current_node).unwrap().0;
        }

        step += 1;
    }

    return step as u32;
}

fn parse_nodes(input: String) -> Map {
    let sections = input.trim().split("\n\n").collect::<Vec<_>>();
    let input = sections[0].trim().chars().collect::<Vec<_>>();

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    sections[1].lines().for_each(|line| {
        let parts = line.split('=').collect::<Vec<_>>();

        let start_node = parts[0].trim().to_string();
        let node_string = parts[1].replace('(', "").replace(')', "");
        let node_pair = node_string.split(',').collect::<Vec<_>>();

        let left_node = node_pair[0].trim().to_string();
        let right_node = node_pair[1].trim().to_string();

        nodes.insert(start_node, (left_node, right_node));
    });

    Map { nodes, input }
}

#[cfg(test)]
mod test {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(2, part_one("data/example.txt"))
    }

    #[test]
    fn part_two_test() {
        assert_eq!(6, part_two("data/example_2.txt"))
    }
}
