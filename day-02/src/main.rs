use regex::Regex;
use std::u32;
use utils::string::read;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Cube {
    color: Color,
    count: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Vec<Cube>>,
}

static RED_THRESHOLD: u32 = 12;
static GREEN_THRESHOLD: u32 = 13;
static BLUE_THRESHOLD: u32 = 14;

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day 2 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt");
    println!("Day 2 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> u32 {
    read(file_name)
        .iter()
        .map(|line| parse_game(line))
        .filter(|game| {
            game.rounds.iter().all(|round| {
                round.iter().all(|cube| match cube {
                    Cube {
                        count,
                        color: Color::Red,
                    } => count < &RED_THRESHOLD,
                    Cube {
                        count,
                        color: Color::Green,
                    } => count < &GREEN_THRESHOLD,
                    Cube {
                        count,
                        color: Color::Blue,
                    } => count < &BLUE_THRESHOLD,
                })
            })
        })
        .map(|game| game.id)
        .sum()
}

fn part_two(file_name: &str) -> u32 {
    read(file_name)
        .iter()
        .map(|line| parse_game(line))
        .map(|game| {
            let mut min_red: u32 = 0;
            let mut min_green: u32 = 0;
            let mut min_blue: u32 = 0;

            game.rounds.iter().for_each(|round| {
                round.iter().for_each(|cube| match cube {
                    Cube {
                        count,
                        color: Color::Red,
                    } => {
                        if count > &min_red {
                            min_red = *count
                        }
                    }
                    Cube {
                        count,
                        color: Color::Green,
                    } => {
                        if count > &min_green {
                            min_green = *count
                        }
                    }
                    Cube {
                        count,
                        color: Color::Blue,
                    } => {
                        if count > &min_blue {
                            min_blue = *count
                        }
                    }
                })
            });

            min_red * min_green * min_blue
        })
        .sum()
}

fn parse_game(line: &str) -> Game {
    let re = Regex::new(r"^Game ([0-9]+):").unwrap();
    let Some(caps) = re.captures(line) else {
        panic!("Failed to match game_id in line '{line:?}'")
    };

    let game_id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();

    let rounds: Vec<Vec<Cube>> = re
        .replace(line, "")
        .split(";")
        .map(|round| {
            round
                .trim()
                .split(",")
                .map(|cube| {
                    let cubes: Vec<&str> = cube.split_ascii_whitespace().collect();

                    match &cubes[..] {
                        [count, "red"] => Cube {
                            color: Color::Red,
                            count: count.parse::<u32>().unwrap(),
                        },
                        [count, "blue"] => Cube {
                            color: Color::Blue,
                            count: count.parse::<u32>().unwrap(),
                        },
                        [count, "green"] => Cube {
                            color: Color::Green,
                            count: count.parse::<u32>().unwrap(),
                        },
                        _ => panic!("Failed to match any colors. How?"),
                    }
                })
                .collect()
        })
        .collect();

    return Game {
        id: game_id,
        rounds: rounds,
    };
}

#[cfg(test)]
mod test {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(8, part_one("data/example.txt"))
    }

    #[test]
    fn part_two_test() {
        assert_eq!(2286, part_two("data/example.txt"))
    }
}
