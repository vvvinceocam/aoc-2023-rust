use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};

fn parse_num_line(line: &str) -> HashSet<usize> {
    line.split_whitespace().map(|raw| raw.parse().unwrap()).collect()
}

pub struct Card {
    winning_numbers: HashSet<usize>,
    scratched_numbers: HashSet<usize>,
}

impl Card {
    pub fn from_str(line: &str) -> Self {
        let (winnings, scratched) = line.split(": ").nth(1).unwrap().split_once(" | ").unwrap();
        Self {
            winning_numbers: parse_num_line(winnings),
            scratched_numbers: parse_num_line(scratched),
        }
    }

    pub fn points(&self) -> usize {
        let n = self.winning_numbers.intersection(&self.scratched_numbers).count() as u32;
        if n == 0 {
            0
        } else {
            2usize.pow(n - 1)
        }
    }

    pub fn wins(&self) -> usize {
        self.winning_numbers.intersection(&self.scratched_numbers).count()
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Card> {
    input.lines().map(Card::from_str).collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Card]) -> usize {
    input.iter().map(Card::points).sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Card]) -> usize {
    let wins = input.iter().map(Card::wins).collect::<Vec<_>>();
    let mut counts = vec![1; input.len()];
    for (i, win) in wins.iter().enumerate() {
        for j in i+1..i+win+1 {
            counts[j] += counts[i];
        }
    }
    counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2};

    static INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 13);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 30);
    }
}

