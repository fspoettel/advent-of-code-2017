use std::cmp::{max, min};

use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(
                    l.split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect(),
                )
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    parse(input)
        .into_iter()
        .map(|mut row| {
            row.sort_unstable();
            row.last().unwrap() - row.first().unwrap()
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    parse(input)
        .into_iter()
        .map(|row| {
            row.into_iter()
                .tuple_combinations()
                .find_map(|(a, b)| {
                    let ceil = max(a, b);
                    let floor = min(a, b);
                    if ceil % floor == 0 {
                        Some(ceil / floor)
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .sum()
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 2), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_one(&input), 18);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_two(&input), 9);
    }
}
