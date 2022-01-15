fn parse(input: &str) -> usize {
    input.lines().next().unwrap().parse().unwrap()
}

pub fn part_one(input: &str) -> usize {
    let shift = parse(input);
    let mut lock: Vec<usize> = vec![0];
    let mut offset = 0;

    for i in 1..=2017 {
        offset = (offset + shift + 1) % i;
        lock.insert(offset, i);
    }

    lock[offset + 1]
}

pub fn part_two(input: &str) -> usize {
    let shift = parse(input);
    let mut offset = 0;
    let mut after_zero = None;

    for i in 1..50000000 {
        offset = (offset + shift + 1) % i;

        if offset == 0 {
            after_zero = Some(i);
        }
    }

    after_zero.unwrap()
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 17), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 17);
        assert_eq!(part_one(&input), 638);
    }
}
