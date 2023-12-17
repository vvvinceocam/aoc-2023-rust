use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included};
use aoc_runner_derive::{aoc, aoc_generator};

pub struct Galaxy {
    x: usize,
    y: usize,
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<Galaxy> {
    input.lines()
        .enumerate()
        .flat_map(|(y, line)|
            line.bytes()
                .enumerate()
                .filter_map(move |(x, b)|
                    if b == b'#' {
                        Some(Galaxy { x, y })
                    } else {
                        None
                    }
                )
        ).collect::<Vec<_>>()
}

fn generic_solver(galaxies: &[Galaxy], factor: usize) -> usize {
    let columns = galaxies.iter().map(|g| g.x).collect::<BTreeSet<_>>();
    let rows = galaxies.iter().map(|g| g.y).collect::<BTreeSet<_>>();

    let mut total = 0;

    for (i, src) in galaxies.iter().enumerate() {
        for dst in &galaxies[i+1..] {
            let x1 = src.x.min(dst.x);
            let x2 = src.x.max(dst.x);
            if x1 < x2 {
                let normals = columns.range((Included(x1 + 1), Excluded(x2))).count();
                total += (x2 - x1 - normals - 1) * factor + normals + 1;
            }

            let y1 = src.y.min(dst.y);
            let y2 = src.y.max(dst.y);
            if y1 < y2 {
                let normals = rows.range((Included(y1 + 1), Excluded(y2))).count();
                total += (y2 - y1 - normals - 1) * factor + normals + 1;
            }
        }
    }

    total
}

#[aoc(day11, part1)]
fn solve_part1(galaxies: &[Galaxy]) -> usize {
    generic_solver(galaxies, 2)
}

#[aoc(day11, part2)]
fn solve_part2(galaxies: &[Galaxy]) -> usize {
    generic_solver(galaxies, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 374);
    }

    #[test]
    fn generic_solver_match_example_1() {
        assert_eq!(generic_solver(&input_generator(INPUT), 10), 1030);
    }

    #[test]
    fn generic_solver_match_example_2() {
        assert_eq!(generic_solver(&input_generator(INPUT), 100), 8410);
    }
}
