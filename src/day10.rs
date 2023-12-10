use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use num::Integer;

pub struct Map {
    grid: Vec<Vec<u8>>,
    start: Point,
    width: usize,
    height: usize,
}

pub type Point = (usize, usize);

impl Map {
    fn get(&self, (x, y): Point) -> u8 {
        self.grid[y][x]
    }

    fn east(&self, (x, y): Point) -> Option<Point> {
        if x == self.width - 1 {
            None
        } else {
            let x = x + 1;
            match self.grid[y][x] {
                b'7' | b'J' | b'-' | b'S' => Some((x, y)),
                _ => None
            }
        }
    }

    fn west(&self, (x, y): Point) -> Option<Point> {
        if x == 0 {
            None
        } else {
            let x = x - 1;
            match self.grid[y][x] {
                b'F' | b'L' | b'-' | b'S' => Some((x, y)),
                _ => None
            }
        }
    }

    fn north(&self, (x, y): Point) -> Option<Point> {
        if y == 0 {
            None
        } else {
            let y = y - 1;
            match self.grid[y][x] {
                b'|' | b'7' | b'F' | b'S' => Some((x, y)),
                _ => None
            }
        }
    }

    fn south(&self, (x, y): Point) -> Option<Point> {
        if y == self.height - 1 {
            None
        } else {
            let y = y + 1;
            match self.grid[y][x] {
                b'|' | b'J' | b'L' | b'S' => Some((x, y)),
                _ => None
            }
        }
    }

    fn next(&self, cell: &(u8, Point)) -> (u8, Point) {
        match (cell.0, self.get(cell.1)) {
            (b'N', b'|') => (b'N', self.south(cell.1).unwrap()),
            (b'N', b'L') => (b'W', self.east(cell.1).unwrap()),
            (b'N', b'J') => (b'E', self.west(cell.1).unwrap()),

            (b'S', b'|') => (b'S', self.north(cell.1).unwrap()),
            (b'S', b'7') => (b'E', self.west(cell.1).unwrap()),
            (b'S', b'F') => (b'W', self.east(cell.1).unwrap()),

            (b'W', b'-') => (b'W', self.east(cell.1).unwrap()),
            (b'W', b'7') => (b'N', self.south(cell.1).unwrap()),
            (b'W', b'J') => (b'S', self.north(cell.1).unwrap()),

            (b'E', b'-') => (b'E', self.west(cell.1).unwrap()),
            (b'E', b'L') => (b'S', self.north(cell.1).unwrap()),
            (b'E', b'F') => (b'N', self.south(cell.1).unwrap()),

            _ => unreachable!(),
        }
    }

    fn replace_start(&self) -> u8 {
        match (
            self.north(self.start),
            self.east(self.start),
            self.south(self.start),
            self.west(self.start),
        ) {
            (Some(_), Some(_), None, None) => b'L',
            (Some(_), None, Some(_), None) => b'|',
            (Some(_), None, None, Some(_)) => b'J',
            (None, Some(_), Some(_), None) => b'F',
            (None, Some(_), None, Some(_)) => b'-',
            (None, None, Some(_), Some(_)) => b'7',
            _ => unreachable!(),
        }
    }

    fn first_step(&self) -> (u8, Point) {
        self.north(self.start).map(|p| (b'S', p))
            .or(self.east(self.start).map(|p| (b'W', p)))
            .or(self.south(self.start).map(|p| (b'N', p)))
            .or(self.west(self.start).map(|p| (b'E', p)))
            .unwrap()
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Map {
    let grid = input.lines().map(|line| line.bytes().collect()).collect::<Vec<Vec<_>>>();

    let start = grid.iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find(|(_, b)| *b == &b'S')
                .map(|(x, _)| (x, y))
        }).unwrap();

    Map {
        width: grid[0].len(),
        height: grid.len(),
        grid,
        start,
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(map: &Map) -> usize {
    let mut cell = map.first_step();
    let mut steps = 1;
    while cell.1 != map.start {
        steps += 1;
        cell = map.next(&cell);
    }
    steps / 2
}

#[aoc(day10, part2)]
pub fn solve_part2(map: &Map) -> usize {
    let mut cell = map.first_step();

    let mut borders = HashMap::new();
    borders.insert(map.start, map.replace_start());

    while cell.1 != map.start {
        let c = map.get(cell.1);
        borders.insert(cell.1, c);
        cell = map.next(&cell);
    }

    let mut inside = 0;
    for (y, line) in map.grid.iter().enumerate() {
        let mut crossed = 0;
        let mut prev = None;
        for (x, _) in line.iter().enumerate() {
            match borders.get(&(x, y)) {
                None if crossed.is_odd() => inside += 1,
                Some(c) => match c {
                    b'|' => crossed += 1,
                    b'7' if prev == Some(b'L') => crossed += 1,
                    b'J' if prev == Some(b'F') => crossed += 1,
                    c @ (b'L' | b'F') => prev = Some(*c),
                    _ => {}
                },
                _ => {}
            }
        }
    }
    inside
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    static INPUT2: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 8);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT2)), 8);
    }
}
