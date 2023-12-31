use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};

use regex::bytes::Regex;
use atoi::atoi;
use lazy_static::lazy_static;

lazy_static! {
    static ref NUMBER_PATTERN: Regex = Regex::new(r"\d+").unwrap();
}

fn range(start: usize, end: usize, max: usize) -> impl Iterator<Item=usize> {
    start.saturating_sub(1)..=(end).min(max - 1)
}

#[derive(Debug, Clone)]
struct Number {
    line: usize,
    head: usize,
    tail: usize,
    value: u64,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Neighbor {
    x: usize,
    y: usize,
    byte: u8,
}

#[derive(Debug)]
pub struct Schema {
    height: usize,
    width: usize,
    grid: Vec<Vec<u8>>,
}

impl Schema {
    fn new(grid: Vec<Vec<u8>>) -> Self {
        Self {
            height: grid.len(),
            width: grid[0].len(),
            grid,
        }
    }

    fn numbers(&self) -> impl Iterator<Item=Number> + '_ {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, line)|
                NUMBER_PATTERN.find_iter(line)
                    .map(move |m| Number {
                        line: y,
                        head: m.start(),
                        tail: m.end(),
                        value: atoi(m.as_bytes()).unwrap(),
                    })
            )
    }

    fn neighbors<'a>(&'a self, number: &'a Number) -> impl Iterator<Item=Neighbor> + 'a {
        range(number.line, number.line + 1, self.height).flat_map(move |y|
            range(number.head, number.tail, self.width).filter_map(move |x|
                if y != number.line || (x < number.head || number.tail <= x) {
                    Some(Neighbor { x, y, byte: self.grid[y][x] })
                } else {
                    None
                }
            )
        )
    }

    fn is_adjacent_to_symbol(&self, number: &Number) -> bool {
        self.neighbors(number).any(|n| n.byte != b'.' && !n.byte.is_ascii_digit())
    }

    fn adjacent_gears<'a>(&'a self, number: &'a Number) -> impl Iterator<Item=Neighbor> + '_ {
        self.neighbors(number).filter(|n| n.byte == b'*')
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Schema {
    Schema::new(input.lines().map(|s| s.as_bytes().to_vec()).collect())
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Schema) -> u64 {
    input.numbers()
        .filter_map(|n| if input.is_adjacent_to_symbol(&n) { Some(n.value) } else { None })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Schema) -> u64 {
    let mut gears = HashMap::<Neighbor, (usize, u64)>::new();
    for number in input.numbers() {
        for gear in input.adjacent_gears(&number) {
            gears.entry(gear)
                .and_modify(|(count, power)| {
                    *count += 1;
                    *power *= number.value;
                })
                .or_insert((1, number.value));
        }
    }
    gears.values()
        .filter_map(|(count, power)| if *count == 2 { Some(power) } else { None })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 4361);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 467835);
    }
}
