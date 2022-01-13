use itertools::Itertools;

const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;
const DIV: u64 = 2147483647;

fn parse(input: &str) -> (u64, u64) {
    input
        .lines()
        .filter_map(|l| l.split(' ').last().unwrap().parse().ok())
        .collect_tuple()
        .unwrap()
}

fn generate(x: u64, factor: u64) -> u64 {
    (x * factor) % DIV
}

fn generate_with_fit(x: u64, factor: u64, fit: u64) -> u64 {
    let y = generate(x, factor);
    if y % fit == 0 {
        y
    } else {
        generate_with_fit(y, factor, fit)
    }
}

fn low_16_match(a: u64, b: u64) -> bool {
    a & 0xFFFF == b & 0xFFFF
}

pub fn part_one(input: &str) -> u64 {
    let (mut a, mut b) = parse(input);
    let mut matches: u64 = 0;

    for _ in 0..40000000 {
        a = generate(a, FACTOR_A);
        b = generate(b, FACTOR_B);
        if low_16_match(a, b) {
            matches += 1;
        }
    }

    matches
}

pub fn part_two(input: &str) -> u64 {
    let (mut a, mut b) = parse(input);
    let mut matches: u64 = 0;

    for _ in 0..5000000 {
        a = generate_with_fit(a, FACTOR_A, 4);
        b = generate_with_fit(b, FACTOR_B, 8);
        if low_16_match(a, b) {
            matches += 1;
        }
    }

    matches
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 15), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 15);
        assert_eq!(part_one(&input), 588);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 15);
        assert_eq!(part_two(&input), 309);
    }
}
