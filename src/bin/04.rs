use itertools::Itertools;
use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(l.split(' ').collect::<Vec<&str>>())
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    parse(input)
        .into_iter()
        .filter(|words| {
            let set: HashSet<&&str> = HashSet::from_iter(words.iter());
            set.len() == words.len()
        })
        .count() as u32
}

pub fn part_two(input: &str) -> u32 {
    parse(input)
        .into_iter()
        .filter(|words| {
            let set: HashSet<Vec<char>> =
                HashSet::from_iter(words.iter().map(|w| w.chars().sorted_unstable().collect()));
            set.len() == words.len()
        })
        .count() as u32
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 4), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa";
        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn test_part_two() {
        let input = "abcde fghij\nabcde xyz ecdab\na ab abc abd abf abj\niiii oiii ooii oooi oooo\noiii ioii iioi iiio is";
        assert_eq!(part_two(&input), 3);
    }
}
