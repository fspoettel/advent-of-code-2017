use hashbrown::HashSet;
use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                let words = l.split(' ').collect::<Vec<&str>>();
                Some(words)
            }
        })
        .collect()
}

fn count_valid_passphrases(words: &[Vec<&str>], policy: fn(words: &[&str]) -> bool) -> u32 {
    words.iter().filter(|words| policy(words)).count() as u32
}

pub fn part_one(input: &str) -> u32 {
    count_valid_passphrases(&parse(input), |words| {
        let valid_words: HashSet<&&str> = HashSet::from_iter(words.iter());
        valid_words.len() == words.len()
    })
}

pub fn part_two(input: &str) -> u32 {
    count_valid_passphrases(&parse(input), |words| {
        let valid_words: HashSet<Vec<char>> =
            HashSet::from_iter(words.iter().map(|w| w.chars().sorted_unstable().collect()));
        valid_words.len() == words.len()
    })
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
