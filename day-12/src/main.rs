use std::collections::HashMap;

use utils::string::read_string;

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day 12 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt");
    println!("Day 12 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> usize {
    let input = read_string(file_name);

    parse_hotsprings(&input)
        .iter()
        .map(|(pattern, counts)| spring_arrangements(&pattern, counts))
        .sum()
}

fn part_two(file_name: &str) -> usize {
    let input = read_string(file_name);

    parse_hotsprings(&input)
        .iter()
        .map(|(pattern, counts)| {
            let pattern = [*pattern; 5].join("?");
            let counts = counts.repeat(5);
            spring_arrangements(&pattern, &counts)
        })
        .sum()
}

fn combinations(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    s: &[char],
    current_group: Option<usize>,
    remaining_groups: &[usize],
) -> usize {
    if s.is_empty() {
        return match (current_group, remaining_groups.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == remaining_groups[0] => 1,
            _ => 0,
        };
    }
    if current_group.is_some() && remaining_groups.is_empty() {
        return 0;
    }

    // calculate cache key and check the cache
    let key = (s.len(), current_group.unwrap_or(0), remaining_groups.len());
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let cost = match (s[0], current_group) {
        ('.', Some(g)) if g != remaining_groups[0] => 0,
        ('.', Some(_)) => combinations(cache, &s[1..], None, &remaining_groups[1..]),
        ('.', None) => combinations(cache, &s[1..], None, remaining_groups),
        ('#', Some(g)) => combinations(cache, &s[1..], Some(g + 1), remaining_groups),
        ('#', None) => combinations(cache, &s[1..], Some(1), remaining_groups),
        ('?', Some(g)) => {
            let mut ans = combinations(cache, &s[1..], Some(g + 1), remaining_groups);
            if g == remaining_groups[0] {
                ans += combinations(cache, &s[1..], None, &remaining_groups[1..])
            }
            ans
        }
        ('?', None) => {
            combinations(cache, &s[1..], Some(1), remaining_groups)
                + combinations(cache, &s[1..], None, remaining_groups)
        }
        _ => unreachable!(),
    };
    cache.insert(key, cost);
    cost
}

fn spring_arrangements(springs: &str, counts: &[usize]) -> usize {
    let mut cache: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let chars = springs.chars().collect::<Vec<_>>();
    combinations(&mut cache, &chars, None, counts)
}

fn parse_hotsprings(input: &str) -> Vec<(&str, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let pattern = parts.next().unwrap();
            let counts = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (pattern, counts)
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{combinations, part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(21, part_one("data/example.txt"));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(525152, part_two("data/example.txt"));
    }

    #[test]
    fn combination_test() {
        assert_eq!(
            2,
            combinations(
                &mut HashMap::new(),
                &"?#?#?#?#?#?#?#?".chars().collect::<Vec<_>>(),
                None,
                &[1, 3, 1, 6]
            )
        );
        assert_eq!(
            2,
            combinations(
                &mut HashMap::new(),
                &"#.#.###".chars().collect::<Vec<_>>(),
                None,
                &[1, 1, 3]
            )
        );
    }
}
