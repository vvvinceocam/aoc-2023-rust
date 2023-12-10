use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use crate::day8::Direction::{Left, Right};

#[derive(Debug)]
pub enum Direction {
    Right,
    Left,
}

impl Direction {
    pub fn from_byte(b: &u8) -> Self {
        match b {
            b'R' => Right,
            b'L' => Left,
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> (Vec<Direction>, HashMap<String, (String, String)>) {
    let mut block = input.split("\n\n");
    let dirs = block.next().unwrap().bytes().map(|b| Direction::from_byte(&b)).collect::<Vec<_>>();
    let map = block.next().unwrap()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|str| !str.is_empty())
        .array_chunks()
        .map(|[key, left, right]| (key.to_string(), (left.to_string(), right.to_string())))
        .collect();

    (dirs, map)
}

#[aoc(day8, part1)]
pub fn solve_part1((dirs, map): &(Vec<Direction>, HashMap<String, (String, String)>)) -> usize {
    let mut steps = 0;
    let mut state = "AAA".to_string();

    for dir in dirs.iter().cycle() {
        steps += 1;

        state = match dir {
            Left => map[&state].0.to_string(),
            Right => map[&state].1.to_string(),
        };

        if state == "ZZZ" {
            break;
        }
    }

    steps
}

#[aoc(day8, part2)]
pub fn solve_part2((dirs, map): &(Vec<Direction>, HashMap<String, (String, String)>)) -> usize {
    map.keys()
        .filter(|k| k.ends_with('A'))
        .map(|start| {
        let mut steps = 0;
        let mut state = start;
        for dir in dirs.iter().cycle() {
            steps += 1;

            state = match dir {
                Left => &map[state].0,
                Right => &map[state].1,
            };

            if state.ends_with('Z') {
                break;
            }
        }

        steps
    }).reduce(num::integer::lcm).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;


    static INPUT1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    static INPUT2: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 2);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT2)), 6);
    }
}