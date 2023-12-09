use std::cmp::Ordering;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Interval {
    start: isize,
    end: isize,
}

/// Closed interval with shift value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Shifter {
    start: isize,
    end: isize,
    shift: isize,
}

impl Shifter {
    pub fn from_str(str: &str) -> Self {
        let mut numbers = str.split_whitespace().map(|sub| sub.parse::<isize>().unwrap());
        let dst = numbers.next().unwrap();
        let src = numbers.next().unwrap();
        let width = numbers.next().unwrap();
        Self {
            start: src,
            end: src + width - 1,
            shift: dst - src,
        }
    }

    #[inline]
    pub fn compare(&self, i: isize) -> Ordering {
        if i < self.start {
            Ordering::Greater
        } else if self.end < i {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }

    #[inline]
    pub fn shift(&self, i: isize) -> isize {
        i + self.shift
    }

    pub fn partition(&self, interval: &Interval) -> (Option<Interval>, Option<Interval>, Option<Interval>) {
        let prefix = if interval.start < self.start {
            Some(Interval {
                start: interval.start,
                end: interval.end.min(self.start - 1),
            })
        } else {
            None
        };

        let intersection = if interval.end < self.start || self.end < interval.start {
            None
        } else {
            Some(Interval {
                start: self.start.max(interval.start) + self.shift,
                end: self.end.min(interval.end) + self.shift,
            })
        };

        let suffix = if self.end < interval.end {
            Some(Interval {
                start: interval.start.max(self.end + 1),
                end: interval.end,
            })
        } else {
            None
        };

        (prefix, intersection, suffix)
    }
}

impl Ord for Shifter {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Shifter {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Map(Vec<Shifter>);

impl Map {
    #[inline]
    pub fn shift_id(&self, i: isize) -> isize {
        match self.0.binary_search_by(|s| s.compare(i)) {
            Ok(j) => self.0[j].shift(i),
            Err(_) => i,
        }
    }

    pub fn shift_interval(&self, mut interval: Interval) -> Vec<Interval> {
        let mut out = vec![];
        for shifter in self.0.iter().skip_while(move |s| s.end < interval.start) {
            let (prefix, intersection, suffix) = shifter.partition(&interval);
            out.extend(prefix.iter());
            out.extend(intersection.iter());
            match suffix {
                Some(suffix) => interval = suffix,
                None => break
            }
        }

        if self.0.last().unwrap().end < interval.start {
            out.push(interval);
        }

        out
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<isize>, Vec<Map>) {
    let mut groups = input.split("\n\n");
    let seeds = groups.next().unwrap().split_once(": ").unwrap().1.split_whitespace().map(|s| s.parse::<isize>().unwrap()).collect();

    let maps = groups.map(|group| {
        let mut shifters = group.lines().skip(1).map(Shifter::from_str).collect::<Vec<_>>();
        shifters.sort();
        Map(shifters)
    }).collect();

    (seeds, maps)
}

#[aoc(day5, part1)]
pub fn solve_part1((seeds, maps): &(Vec<isize>, Vec<Map>)) -> isize {
    seeds.iter().map(|seed| {
        maps.iter().fold(*seed, |state, map| map.shift_id(state))
    }).min().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2((seeds, maps): &(Vec<isize>, Vec<Map>)) -> isize {
    let mut intervals = seeds.chunks(2)
        .map(|se| Interval { start: se[0], end: se[0] + se[1] - 1 })
        .collect::<Vec<_>>();

    for map in maps {
        intervals = intervals.iter()
            .fold(vec![], |mut acc, &interval| {
                acc.extend(map.shift_interval(interval));
                acc
            });
    }

    intervals.iter().map(|i| i.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2};

    static INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 35);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 46);
    }
}
