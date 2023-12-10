use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<isize>> {
    input.lines()
        .map(|line| line.split_whitespace()
            .map(|n| n.parse::<isize>().unwrap())
            .collect()).
        collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[Vec<isize>]) -> isize {
    input.iter().map(|line| {
        let mut ends = vec![];
        let mut seq = line.to_vec();

        while !seq.iter().all(|n| *n == 0) {
            ends.push(*seq.last().unwrap());
            seq = seq.array_windows().map(|[a, b]| b - a).collect();
        }

        ends.into_iter().sum::<isize>()
    }).sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[Vec<isize>]) -> isize {
    input.iter().map(|line| {
        let mut ends = vec![];
        let mut seq = line.to_vec();

        while !seq.iter().all(|n| *n == 0) {
            ends.push(*seq.first().unwrap());
            seq = seq.array_windows().map(|[a, b]| b - a).collect();
        }

        ends.into_iter().rfold(0, |a, b| b - a)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

   static INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 114);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 2);
    }
}
