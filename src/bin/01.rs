fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn sum_values(values: &[u32], get_target: fn(usize, usize) -> usize) -> u32 {
    let len = values.len();
    values
        .iter()
        .enumerate()
        .filter_map(|(i, &val)| {
            if val == values[get_target(i, len) % len] {
                Some(val)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_one(input: &str) -> u32 {
    sum_values(&parse(input), |i, _| i + 1)
}

pub fn part_two(input: &str) -> u32 {
    sum_values(&parse(input), |i, len| i + len / 2)
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 1), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("1122"), 3);
        assert_eq!(part_one("1111"), 4);
        assert_eq!(part_one("1234"), 0);
        assert_eq!(part_one("91212129"), 9);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("1212"), 6);
        assert_eq!(part_two("1221"), 0);
        assert_eq!(part_two("123425"), 4);
        assert_eq!(part_two("123123"), 12);
        assert_eq!(part_two("12131415"), 4);
    }
}
