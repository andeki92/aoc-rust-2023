use utils::string::read;

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day 6 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt");
    println!("Day 6 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> u32 {
    let input = read(file_name);
    let race = parse_race(input);

    race.iter()
        .map(|&[time, record]| {
            (0..time)
                .map(|speed| speed * (time - speed))
                .filter(|&distance| distance > record)
                .count()
        })
        .fold(1, |acc, result| acc * result as u32)
}

fn part_two(file_name: &str) -> u64 {
    let input = read(file_name);
    let (time, record) = parse_single_race(input);

    let cutoff = (0..time)
        .take_while(|charge| charge * (time - charge) <= record)
        .last()
        .unwrap();

    time - (2 * (cutoff + 1)) + 1
}

fn parse_single_race(input: Vec<String>) -> (u64, u64) {
    let time_parts = input[0].split(":").collect::<Vec<_>>();
    let time = time_parts[1].replace(" ", "").parse::<u64>().unwrap();

    let record_parts = input[1].split(":").collect::<Vec<_>>();
    let record = record_parts[1].replace(" ", "").parse::<u64>().unwrap();

    (time, record)
}

fn parse_race(input: Vec<String>) -> Vec<[u32; 2]> {
    let time_parts = input[0].split(":").collect::<Vec<_>>();
    let times = parse_nums(time_parts[1]);

    let record_parts = input[1].split(":").collect::<Vec<_>>();
    let records = parse_nums(record_parts[1]);

    times
        .iter()
        .enumerate()
        .map(|(idx, t)| [*t, records[idx]])
        .collect()
}

fn parse_nums(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(288, part_one("data/example.txt"))
    }

    #[test]
    fn part_two_test() {
        assert_eq!(71503, part_two("data/example.txt"))
    }
}
