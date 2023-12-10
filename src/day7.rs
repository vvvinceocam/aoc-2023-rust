use std::cmp::Ordering;
use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};

fn hand_value(cards: &[u8]) -> usize {
    cards.iter().fold(HashMap::<u8, usize>::new(), |mut acc, c| {
        acc.entry(*c).and_modify(|i| *i += 1).or_insert(1usize);
        acc
    }).values().fold(0, |mut acc, v| {
        acc += v.pow(2);
        acc
    })
}

fn card_value(card: &u8) -> usize {
    match card {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => 11,
        b'T' => 10,
        c => (c - b'0').into(),
    }
}

fn joker_hand_value(cards: &[u8]) -> usize {
    let mut groups = cards.iter().fold(HashMap::<u8, usize>::new(), |mut acc, c| {
        acc.entry(*c).and_modify(|i| *i += 1).or_insert(1usize);
        acc
    });

    if let Some(jokers) = groups.remove(&b'J') {
        if let Some(max_count) = groups.values().max() {
            let figures = groups.iter().filter(|(_, v)| *v == max_count).map(|(k, _)| *k).collect::<Vec<_>>();
            let figure = figures.iter().max_by_key(|v| joker_card_value(v)).unwrap();
            groups.entry(*figure).and_modify(|v| *v += jokers);
        } else {
            groups.insert(b'J', 5);
        }
    }

    groups.values().fold(0, |mut acc, v| {
        acc += v.pow(2);
        acc
    })
}

fn joker_card_value(card: &u8) -> usize {
    match card {
        b'J' => 1,
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'T' => 10,
        c => (c - b'0').into(),
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hand {
    cards: Vec<u8>,
    bid: usize,
    value: usize,
}

impl Hand {
    pub fn new(cards: String, bid: usize) -> Self {
        let cards = cards.bytes().collect::<Vec<_>>();
        Self {
            value: hand_value(&cards),
            cards,
            bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value.cmp(&other.value) {
            Ordering::Equal => {
                let (a, b) = self.cards.iter().zip(other.cards.iter())
                    .map(|(a, b)| (card_value(a), card_value(b)))
                    .find(|(a, b)| a != b).unwrap();
                a.cmp(&b)
            },
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HandJoker {
    cards: Vec<u8>,
    bid: usize,
    value: usize,
}

impl HandJoker {
    pub fn new(cards: String, bid: usize) -> Self {
        let cards = cards.bytes().collect::<Vec<_>>();
        Self {
            value: joker_hand_value(&cards),
            cards,
            bid,
        }
    }
}

impl Ord for HandJoker {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value.cmp(&other.value) {
            Ordering::Equal => {
                let (a, b) = self.cards.iter().zip(other.cards.iter())
                    .map(|(a, b)| (joker_card_value(a), joker_card_value(b)))
                    .find(|(a, b)| a != b).unwrap();
                a.cmp(&b)
            },
            ord => ord,
        }
    }
}

impl PartialOrd for HandJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day7, part1)]
pub fn input_generator1(input: &str) -> Vec<Hand> {
    input.split_whitespace()
        .array_chunks()
        .map(|[cards, value]| Hand::new(cards.to_string(), value.parse().unwrap()))
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[Hand]) -> usize {
    let mut hands = input.to_vec();
    hands.sort();
    hands.iter().enumerate().map(|(k, hand)| (k + 1) * hand.bid).sum()
}

#[aoc_generator(day7, part2)]
pub fn input_generator2(input: &str) -> Vec<HandJoker> {
    input.split_whitespace()
        .array_chunks()
        .map(|[cards, value]| HandJoker::new(cards.to_string(), value.parse().unwrap()))
        .collect()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[HandJoker]) -> usize {
    let mut hands = input.to_vec();
    hands.sort();
    hands.iter().enumerate().map(|(k, hand)| (k + 1) * hand.bid).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator1(INPUT)), 6440);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator2(INPUT)), 5905);
    }
}
