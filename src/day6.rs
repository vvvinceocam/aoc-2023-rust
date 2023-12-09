use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6, part1)]
pub fn input_generator1(input: &str) -> Vec<(usize, usize)> {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_whitespace().skip(1).map(|n| n.parse::<usize>().unwrap());
    let distances = lines.next().unwrap().split_whitespace().skip(1).map(|n| n.parse::<usize>().unwrap());
    times.zip(distances).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[(usize, usize)]) -> usize {
    input.iter().map(|(time, distance)| {
        (1..*time).map(|i| i * (time - i)).filter(|c| c > distance).count()
    }).product()
}

#[aoc_generator(day6, part2)]
pub fn input_generator2(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let time = lines.next().unwrap().split_whitespace().skip(1).collect::<Vec<_>>().join("").parse::<usize>().unwrap();
    let distance = lines.next().unwrap().split_whitespace().skip(1).collect::<Vec<_>>().join("").parse::<usize>().unwrap();

    (time, distance)
}

#[aoc(day6, part2)]
pub fn solve_part2((time, distance): &(usize, usize)) -> usize {
    (1..*time).map(|i| i * (time - i)).filter(|c| c > distance).count()
}


#[cfg(test)]
mod tests {
    use super::{input_generator1, solve_part1, input_generator2, solve_part2};

    static INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator1(INPUT)), 288);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator2(INPUT)), 71503);
    }
}
