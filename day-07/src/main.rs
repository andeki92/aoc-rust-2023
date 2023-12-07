use std::{cmp::Ordering, collections::HashMap};

use utils::read;

#[derive(Debug, PartialEq, Eq)]
struct Cards {
    cards: [u32; 5],
}

#[derive(Debug, PartialEq, Eq)]
enum Hand {
    FiveOfAKind(Cards),
    FourOfAKind(Cards),
    FullHouse(Cards),
    ThreeOfAKind(Cards),
    TwoPair(Cards),
    OnePair(Cards),
    HighCard(Cards),
}

#[derive(Debug)]
struct Round {
    hand: Hand,
    bid: u32,
}

fn card_cmp(cards: &[u32; 5], other: &[u32; 5]) -> Ordering {
    cards
        .iter()
        .zip(other.iter())
        .find_map(|(card, other)| {
            if card != other {
                Some(card.cmp(&other))
            } else {
                None
            }
        })
        .unwrap_or_else(|| Ordering::Equal)
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (
                Hand::FiveOfAKind(Cards { cards: hand_one }),
                Hand::FiveOfAKind(Cards { cards: hand_two }),
            ) => card_cmp(hand_one, hand_two),
            (Hand::FiveOfAKind(_), _) => Ordering::Greater,
            (_, Hand::FiveOfAKind(Cards { cards: _ })) => Ordering::Less,
            (
                Hand::FourOfAKind(Cards { cards: hand_one }),
                Hand::FourOfAKind(Cards { cards: hand_two }),
            ) => card_cmp(hand_one, hand_two),
            (Hand::FourOfAKind(_), _) => Ordering::Greater,
            (_, Hand::FourOfAKind(_)) => Ordering::Less,
            (
                Hand::FullHouse(Cards { cards: hand_one }),
                Hand::FullHouse(Cards { cards: hand_two }),
            ) => card_cmp(hand_one, hand_two),
            (Hand::FullHouse(_), _) => Ordering::Greater,
            (_, Hand::FullHouse(_)) => Ordering::Less,
            (
                Hand::ThreeOfAKind(Cards { cards: hand_one }),
                Hand::ThreeOfAKind(Cards { cards: hand_two }),
            ) => card_cmp(hand_one, hand_two),
            (Hand::ThreeOfAKind(_), _) => Ordering::Greater,
            (_, Hand::ThreeOfAKind(_)) => Ordering::Less,
            (
                Hand::TwoPair(Cards { cards: hand_one }),
                Hand::TwoPair(Cards { cards: hand_two }),
            ) => card_cmp(hand_one, hand_two),
            (Hand::TwoPair(_), _) => Ordering::Greater,
            (_, Hand::TwoPair(_)) => Ordering::Less,
            (
                Hand::OnePair(Cards { cards: hand_one }),
                Hand::OnePair(Cards { cards: hand_two }),
            ) => card_cmp(hand_one, hand_two),
            (Hand::OnePair(_), _) => Ordering::Greater,
            (_, Hand::OnePair(_)) => Ordering::Less,
            (
                Hand::HighCard(Cards { cards: hand_one }),
                Hand::HighCard(Cards { cards: hand_two }),
            ) => card_cmp(hand_one, hand_two),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let part_one_solution = part_one("data/input.txt");
    println!("Day 7 - Part 1 solution is '{part_one_solution:?}'");

    let part_two_solution = part_two("data/input.txt");
    println!("Day 7 - Part 2 solution is '{part_two_solution:?}'");
}

fn part_one(file_name: &str) -> u32 {
    play_camel_cards(file_name, false)
}

fn part_two(file_name: &str) -> u32 {
    play_camel_cards(file_name, true)
}

fn play_camel_cards(file_name: &str, with_joker: bool) -> u32 {
    let mut rounds = read(file_name)
        .iter()
        .map(|i| parse_round(i, with_joker))
        .collect::<Vec<_>>();

    rounds.sort_by(|round, other| round.hand.cmp(&other.hand));

    rounds
        .iter()
        .map(|hand| hand.bid)
        .enumerate()
        .fold(0, |acc, (idx, bid)| acc + ((idx as u32 + 1) * bid))
}

fn parse_round(input: &str, with_joker: bool) -> Round {
    let parts = input.split_whitespace().collect::<Vec<_>>();
    let cards: [u32; 5] = parts[0]
        .chars()
        .take(5)
        .map(|c| map_card_value(&c, with_joker))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let bid = parts[1].parse::<u32>().unwrap();
    let hand = parse_hand(cards, with_joker);

    Round { hand, bid }
}

fn map_card_value(card: &char, with_joker: bool) -> u32 {
    let j_value = if with_joker { 1 } else { 11 };

    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => j_value,
        'T' => 10,
        _ => card
            .to_digit(10)
            .unwrap_or_else(|| panic!("{} is not a valid card!", card)),
    }
}

fn parse_hand(cards: [u32; 5], with_joker: bool) -> Hand {
    let mut card_counts: HashMap<u32, u32> = HashMap::new();

    cards
        .iter()
        .for_each(|&card| *card_counts.entry(card).or_insert(0) += 1);

    if with_joker {
        if let Some(max) = card_counts
            .iter()
            .filter(|&(c, _)| *c != 1)
            .max_by_key(|(_, v)| *v)
            .map(|(k, _)| k)
        {
            let joker_count = *card_counts
                .get(&1 /* joker */)
                .map(|count| count)
                .unwrap_or_else(|| &0);

            *card_counts.entry(*max).or_insert(0) += joker_count;
            card_counts.remove(&1 /* joker */);
        }
    }

    let mut counts: Vec<&u32> = card_counts.values().collect();
    counts.sort();
    counts.reverse();

    match counts.first().unwrap() {
        5 => Hand::FiveOfAKind(Cards { cards }),
        4 => Hand::FourOfAKind(Cards { cards }),
        3 => match counts[1] {
            2 => Hand::FullHouse(Cards { cards }),
            _ => Hand::ThreeOfAKind(Cards { cards }),
        },
        2 => match counts[1] {
            2 => Hand::TwoPair(Cards { cards }),
            _ => Hand::OnePair(Cards { cards }),
        },
        _ => Hand::HighCard(Cards { cards }),
    }
}

#[cfg(test)]
mod test {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(6440, part_one("data/example.txt"));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(5905, part_two("data/example.txt"));
    }
}
