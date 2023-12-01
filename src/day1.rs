use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

static PATTERN: &str = r".*(one|two|three|four|five|six|seven|eight|nine|\d)";
lazy_static! {
    static ref FIRST: Regex = Regex::new(&PATTERN[2..]).unwrap();
    static ref LAST: Regex = Regex::new(&PATTERN).unwrap();
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[String]) -> u64 {
    input
        .iter()
        .map(|line| {
            let digits = line
                .bytes()
                .filter(|b| b.is_ascii_digit())
                .map(|b| b - b'0')
                .collect::<Vec<_>>();
            ((digits.first().unwrap() * 10) + (digits.last().unwrap())) as u64
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[String]) -> u64 {
    input
        .iter()
        .map(|line| extract(line, &FIRST) * 10 + extract(line, &LAST))
        .sum()
}

fn convert(raw: &str) -> u64 {
    match raw {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        raw => (raw.as_bytes()[0] - b'0') as u64
    }
}

fn extract(line: &str, pattern: &Regex) -> u64 {
    convert(pattern.captures_iter(line).next().unwrap().get(1).unwrap().as_str())
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2};

    static INPUT1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    static INPUT2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 142);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT2)), 281);
    }
}
