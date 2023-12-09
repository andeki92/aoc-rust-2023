use utils::string::read_string;

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day 5 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt");
    println!("Day 5 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> usize {
    let input = read_string(file_name);
    let (seeds, maps) = parse(&input);

    seeds
        .iter()
        .map(|&s| maps.iter().fold(s, |s, maps| lookup_seed(&s, maps)))
        .min()
        .unwrap()
}

fn part_two(file_name: &str) -> usize {
    let input = read_string(file_name);
    let (seeds, maps) = parse(&input);

    let seed_ranges: Vec<[usize; 2]> = seeds.chunks(2).map(|ch| [ch[0], ch[0] + ch[1]]).collect();

    maps.iter()
        .fold(seed_ranges, |seed_ranges, map| {
            seed_ranges
                .iter()
                .flat_map(|&[start, end]| {
                    let mut mapped = Vec::new();
                    let mut unmapped = vec![[start, end]];

                    for &[dst, src, len] in map {
                        let mut internal_unmapped = Vec::new();

                        for [start, end] in unmapped {
                            // closed start, open ended range
                            if start < end.min(src) {
                                internal_unmapped.push([start, end.min(src)]);
                            }

                            // open start, close ended range
                            if (src + len).max(start) < end {
                                internal_unmapped.push([(src + len).max(start), end]);
                            }

                            // closed range, no more processing, yay!
                            if start.max(src) < (src + len).min(end) {
                                mapped.push([
                                    start.max(src) - src + dst,
                                    (src + len).min(end) - src + dst,
                                ]);
                            }
                        }
                        unmapped = internal_unmapped
                    }
                    mapped.extend(unmapped);
                    mapped
                })
                .collect()
        })
        .iter()
        .map(|r| r[0])
        .min()
        .unwrap()
}

/**
 * Find the first entry whose range matches the seed, and determin its next value.
 * If none of the ranges matches, we return the original seed value.
 */
fn lookup_seed(seed: &usize, maps: &Vec<[usize; 3]>) -> usize {
    maps.iter()
        .find(|&[_, src, len]| src <= seed && src + len >= *seed)
        .and_then(|&[dst, src, _]| Some(seed - src + dst))
        .unwrap_or_else(|| *seed)
}

fn parse(input: &str) -> (Vec<usize>, Vec<Vec<[usize; 3]>>) {
    let sections = input.trim().split("\n\n").collect::<Vec<_>>();
    let seeds = parse_seeds(sections[0]);
    let maps = sections[1..].iter().map(|s| parse_maps(s)).collect();

    (seeds, maps)
}

fn parse_seeds(input: &str) -> Vec<usize> {
    let parts = input.trim().split(": ").collect::<Vec<_>>();
    parts[1]
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_maps(input: &str) -> Vec<[usize; 3]> {
    let lines = input.trim().lines().skip(1);

    lines
        .map(|line| {
            let parts = line
                .trim()
                .to_string()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            [parts[0], parts[1], parts[2]]
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(35, part_one("data/example.txt"))
    }

    #[test]
    fn part_two_test() {
        assert_eq!(46, part_two("data/example.txt"))
    }
}
