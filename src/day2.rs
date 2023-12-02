use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Default)]
pub struct CubeSet {
    blue: usize,
    green: usize,
    red: usize,
}

impl CubeSet {
    fn from_str(raw_set: &str) -> Self {
        let mut set = CubeSet::default();
        for group in raw_set.split(", ") {
            let (count, color) = group.split_once(' ').unwrap();
            match color {
                "blue" => set.blue = count.parse().unwrap(),
                "green" => set.green = count.parse().unwrap(),
                "red" => set.red = count.parse().unwrap(),
                _ => unreachable!(),
            }
        }
        set
    }

    fn smaller_than(&self, other: &Self) -> bool {
        self.blue <= other.blue && self.green <= other.green && self.red <= other.red
    }

    fn power(&self) -> usize {
        self.blue * self.green * self.red
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green),
            red: self.red.max(other.red),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    id: usize,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn from_str(raw_game: &str) -> Self {
        let (header, raw_sets) = raw_game.split_once(": ").unwrap();
        let id = header.split_once(' ').unwrap().1.parse().unwrap();
        let sets = raw_sets.split("; ").map(CubeSet::from_str).collect();

        Self {
            id,
            cube_sets: sets,
        }
    }

    fn max_set(&self) -> CubeSet {
        self.cube_sets.iter().cloned().reduce(|a, b| a.max(&b)).unwrap()
    }
}


#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input.lines().map(Game::from_str).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> usize {
    let max_set = CubeSet {
        blue: 14,
        green: 13,
        red: 12,
    };

    input.iter()
        .filter(|game| game.cube_sets.iter().all(|set| set.smaller_than(&max_set)))
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Game]) -> usize {
    input.iter()
        .map(|game| game.max_set().power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 8);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 2286);
    }
}

